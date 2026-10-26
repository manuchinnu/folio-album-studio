"""Sequential bounded local media inspection; originals remain linked and unchanged."""
import argparse,hashlib,io,json,math,os,sys,warnings
from pathlib import Path
from PIL import Image,ImageCms,ImageOps,ImageStat
from pillow_heif import register_heif_opener
from jsonschema import Draft7Validator
from boundary import compile_type
from xmp import extract_xmp

register_heif_opener(thumbnails=False,decode_threads=1)
Image.MAX_IMAGE_PIXELS=90_000_000
warnings.simplefilter('error',Image.DecompressionBombWarning)
SRGB=ImageCms.ImageCmsProfile(ImageCms.createProfile('sRGB'));ICC=SRGB.tobytes()
SCHEMAS=Path(__file__).resolve().parents[2]/'packages/core/generated'
def schema(name):return json.loads((SCHEMAS/(name+'.schema.json')).read_text(encoding='utf-8'))
REQUEST=schema('import-request');RESPONSE=schema('import-response')
Request=compile_type(REQUEST,'ImportRequest')
Response=compile_type(RESPONSE,'ImportResponse')
COS=[[math.cos(math.pi*(2*x+1)*u/64) for x in range(32)] for u in range(8)]
def phash(gray):
    pixels=list(gray.resize((32,32),Image.Resampling.LANCZOS).getdata())
    rows=[[sum(pixels[y*32+x]*COS[u][x] for x in range(32)) for u in range(8)] for y in range(32)]
    coefficients=[sum(rows[y][u]*COS[v][y] for y in range(32)) for v in range(8) for u in range(8)]
    median=sorted(coefficients[1:])[31];bits=0
    for index,value in enumerate(coefficients):bits=(bits<<1)|int(index>0 and value>median)
    return f'{bits:016x}'

def safe_path(root,relative):
    candidate=root/relative
    if Path(relative).is_absolute():raise ValueError('Absolute source path is forbidden')
    path=candidate.resolve(strict=True)
    if not path.is_relative_to(root):raise ValueError('Path escapes granted root')
    cursor=candidate
    while cursor!=root:
        if cursor.is_symlink():raise ValueError('Symbolic links are not followed')
        cursor=cursor.parent
    if not path.is_file():raise ValueError('Not a regular source file')
    return path

def gps_value(gps):
    def degrees(v):return float(v[0])+float(v[1])/60+float(v[2])/3600
    try:
        if gps.get(1) not in ('N','S') or gps.get(3) not in ('E','W'):return None
        lat=degrees(gps[2])*(-1 if gps[1]=='S' else 1);lon=degrees(gps[4])*(-1 if gps[3]=='W' else 1)
        alt=float(gps[6])*(-1 if gps.get(5) in (1,b'\x01') else 1) if 6 in gps else None
        if not math.isfinite(lat) or not math.isfinite(lon) or abs(lat)>90 or abs(lon)>180:return None
        return {'latitude':lat,'longitude':lon,'altitude':alt}
    except (KeyError,TypeError,ValueError,ZeroDivisionError,IndexError):return None

def inspect(path,relative,output,size):
    before=path.stat()
    if before.st_size>512*1024*1024:raise ValueError('Source exceeds 512 MiB inspection limit')
    with path.open('rb') as handle:sha=hashlib.file_digest(handle,'sha256').hexdigest()
    with Image.open(path,formats=['JPEG','PNG','WEBP','HEIF']) as raw:
        fmt=raw.format;notes=[]
        xmp=[]
        embedded=raw.info.get('xmp') or raw.info.get('XML:com.adobe.xmp')
        if embedded:
            try:xmp.append({'source':'embedded','sourceFile':relative,'fields':extract_xmp(embedded)})
            except Exception as error:notes.append('Embedded XMP rejected: '+str(error)[:500])
        if getattr(raw,'n_frames',1)>1:notes.append('Only the primary still image is imported')
        exif=raw.getexif();orientation=int(exif.get(274,1));details=exif.get_ifd(34665);gps=exif.get_ifd(34853)
        if orientation not in range(1,9):raise ValueError('Invalid EXIF orientation')
        profile=raw.info.get('icc_profile');image=ImageOps.exif_transpose(raw)
        width,height=image.size
        if width>30000 or height>30000:raise ValueError('Source dimensions exceed supported bounds')
        if profile:image=ImageCms.profileToProfile(image,ImageCms.ImageCmsProfile(io.BytesIO(profile)),SRGB,renderingIntent=1,outputMode='RGB',flags=0x2000)
        else:
            if raw.info.get('nclx_profile'):raise ValueError('NCLX-only colour requires a validated transform; ICC source required for now')
            image=image.convert('RGB');notes.append('Untagged source: assumed sRGB')
        image.thumbnail((size,size),Image.Resampling.LANCZOS)
        gray=image.convert('L');small=gray.resize((9,8),Image.Resampling.LANCZOS);pixels=list(small.getdata());bits=0
        for y in range(8):
            for x in range(8):bits=(bits<<1)|int(pixels[y*9+x]>pixels[y*9+x+1])
        samples=list(gray.resize((64,64),Image.Resampling.BILINEAR).getdata())
        detail=sum(abs(samples[y*64+x]-samples[y*64+x+1]) for y in range(64) for x in range(63))/(64*63)
        proxy=output/(sha+f'-{size}.jpg');temp=proxy.with_suffix('.tmp')
        try:image.save(temp,'JPEG',quality=88,subsampling=0,icc_profile=ICC);os.replace(temp,proxy)
        finally:temp.unlink(missing_ok=True)
        def text(value):return str(value)[:4096] if value is not None else None
        result={'relativePath':relative,'sha256':sha,'format':fmt,'width':width,'height':height,'orientation':orientation,'bytes':before.st_size,'mtimeSeconds':before.st_mtime,'proxyPath':proxy.name,'iccSha256':hashlib.sha256(profile).hexdigest() if profile else None,'colorAssumption':'embedded-icc' if profile else 'assumed-srgb','captureTime':text(details.get(36867) or exif.get(36867)),'captureOffset':text(details.get(36881) or exif.get(36881)),'camera':text(exif.get(272)),'gps':gps_value(gps),'dhash':f'{bits:016x}','meanLuma':ImageStat.Stat(gray).mean[0],'detail':detail,'warnings':notes}
    after=path.stat()
    if (before.st_size,before.st_mtime_ns)!=(after.st_size,after.st_mtime_ns):raise ValueError('Source changed during inspection; retry import')
    result['phash']=phash(gray);result['xmp']=xmp
    return result

def main():
    parser=argparse.ArgumentParser();parser.add_argument('--root',type=Path,required=True);parser.add_argument('--output',type=Path,required=True);args=parser.parse_args()
    raw=sys.stdin.buffer.readline(4_000_001)
    if len(raw)>4_000_000:raise ValueError('Import request exceeds limit')
    data=json.loads(raw);Draft7Validator(REQUEST).validate(data);req=Request.model_validate(data).model_dump();root=args.root.resolve(strict=True);args.output.mkdir(parents=True,exist_ok=True)
    result={'protocolVersion':1,'images':[],'errors':[]}
    for relative in req['paths']:
        try:
            path=safe_path(root,relative);item=inspect(path,relative,args.output,req['proxySize'])
            sidecars=[p for p in dict.fromkeys([Path(relative).with_suffix('.xmp'),Path(relative+'.xmp')]) if (root/p).exists()]
            if len(sidecars)>1:item['warnings'].append('Ambiguous XMP sidecars; metadata left unresolved')
            elif sidecars:
                try:
                    sidecar=safe_path(root,str(sidecars[0]))
                    with sidecar.open('rb') as handle:raw_xmp=handle.read(1_048_577)
                    item['xmp'].append({'source':'sidecar','sourceFile':sidecars[0].as_posix(),'fields':extract_xmp(raw_xmp)})
                except Exception as error:item['warnings'].append('XMP sidecar rejected: '+str(error)[:500])
            result['images'].append(item)
        except Exception as error:result['errors'].append({'relativePath':relative,'message':str(error)[:4096]})
    Draft7Validator(RESPONSE).validate(result);Response.model_validate(result);print(json.dumps(result))
if __name__=='__main__':main()
