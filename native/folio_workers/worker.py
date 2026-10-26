"""Bounded M1a native rendering adapter; JSON-lines protocol over stdin/stdout."""
import argparse
import gc
import hashlib
import io
import json
import math
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import threading
import time

from PIL import Image, ImageCms, ImageOps
import pikepdf
import psutil
from reportlab.pdfgen import canvas
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.lib.utils import ImageReader
from boundary import validate_request
from cover import cover_geometry

PT = 72 / 25.4
Image.MAX_IMAGE_PIXELS = 90_000_000
SRGB = ImageCms.ImageCmsProfile(ImageCms.createProfile('sRGB'))
ICC = SRGB.tobytes()

def source_path(root: Path, relative: str) -> Path:
    path = (root / relative).resolve()
    if not path.is_relative_to(root.resolve()): raise ValueError('Asset path escapes authorized root')
    if not path.is_file(): raise ValueError(f'Missing asset: {relative}')
    return path

def checked_asset(root, record):
    path = source_path(root, record['relativePath'])
    with path.open('rb') as handle:
        digest = hashlib.file_digest(handle, 'sha256').hexdigest()
    if digest != record['sha256']: raise ValueError('Asset hash mismatch: '+record['id'])
    return path

def cancelled(req, output):
    if req['cancelled'] or (output / (req['jobId']+'.cancel')).exists():
        raise InterruptedError('Job cancelled; no finished output published')

def color(c): return tuple(int(c[i:i+2],16)/255 for i in (1,3,5))

def render_image(path, asset, crop, rect):
    with Image.open(path) as raw:
        normalized = ImageOps.exif_transpose(raw)
        try:
            if normalized.size != (asset['width'],asset['height']): raise ValueError('Declared image dimensions do not match decoded source')
            profile = raw.info.get('icc_profile')
            converted = ImageCms.profileToProfile(normalized, ImageCms.ImageCmsProfile(io.BytesIO(profile)), SRGB, renderingIntent=1, outputMode='RGB', flags=0x2000) if profile else normalized.convert('RGB')
        finally:normalized.close()
        try:
            x,y,w,h = crop['x'],crop['y'],crop['w'],crop['h']
            if x+w>1+1e-7 or y+h>1+1e-7: raise ValueError('Crop exceeds image')
            image=converted.crop((round(x*converted.width),round(y*converted.height),round((x+w)*converted.width),round((y+h)*converted.height)))
        finally:converted.close()
    try:
        dpi = min(image.width/(rect['w']/25.4),image.height/(rect['h']/25.4))
        if dpi>350:
            resized=image.resize((round(rect['w']/25.4*300),round(rect['h']/25.4*300)),Image.Resampling.LANCZOS);image.close();image=resized
        with io.BytesIO() as buf:
            image.save(buf,'JPEG',quality=95,subsampling=0,icc_profile=ICC)
            return buf.getvalue()
    finally:image.close()

def render_pdf(pages, project, assets, fonts, target, req, output):
    first_font=next(iter(fonts))
    cv=canvas.Canvas(str(target),pageCompression=1,initialFontName=first_font,invariant=1)
    cv.setTitle(project['title']+' — SYNTHETIC PROOF' if req['proof'] else project['title'])
    lab=project['lab'];b=lab['bleed']
    for page in pages:
        cancelled(req,output)
        width=page['widthMm']+b['left']+b['right'];height=page['heightMm']+b['top']+b['bottom']
        cv.setPageSize((width*PT,height*PT))
        cv.setTrimBox((b['left']*PT,b['bottom']*PT,(b['left']+page['widthMm'])*PT,(b['bottom']+page['heightMm'])*PT))
        cv.setBleedBox((0,0,width*PT,height*PT))
        cv.setFillColorRGB(*color(page['background']));cv.rect(0,0,width*PT,height*PT,stroke=0,fill=1)
        for element in page['elements']:
            cancelled(req,output)
            r=element['rect'];x=(r['x']+b['left'])*PT;y=(page['heightMm']+b['bottom']-r['y']-r['h'])*PT
            if element['type']=='fill':
                cv.setFillColorRGB(*color(element['color']));cv.rect(x,y,r['w']*PT,r['h']*PT,fill=1,stroke=0)
            elif element['type']=='image':
                asset,path=assets[element['assetId']]
                data=render_image(path,asset,element['crop'],r)
                with io.BytesIO(data) as buffer, Image.open(buffer) as decoded:
                    reader=ImageReader(decoded)
                    cv.drawImage(reader,x,y,width=r['w']*PT,height=r['h']*PT)
                del reader,data
            else:
                font=element['fontId'];size=element['fontSizeMm']*PT
                face=pdfmetrics.getFont(font).face
                if any(ord(ch) not in face.charToGlyph for ch in element['text'] if not ch.isspace()): raise ValueError('Missing font glyph in '+element['id'])
                if pdfmetrics.stringWidth(element['text'],font,size)>r['w']*PT: raise ValueError('Overset text: '+element['id'])
                asc,desc=pdfmetrics.getAscentDescent(font,size)
                if asc-desc>r['h']*PT: raise ValueError('Text height exceeds box: '+element['id'])
                cv.setFont(font,size);cv.setFillColorRGB(*color(element['color']))
                cv.drawString(x,y+r['h']*PT-asc,element['text'])
        cv.showPage()
        gc.collect()
    cv.save()
    with pikepdf.open(target) as pdf:
        profile=pdf.make_stream(ICC);profile.N=3;profile.Alternate=pikepdf.Name.DeviceRGB
        for page in pdf.pages:
            for image in page.images.values():
                if image.get('/ColorSpace')==pikepdf.Name.DeviceRGB:image.ColorSpace=pikepdf.Array([pikepdf.Name.ICCBased,profile])
        pdf.Root.OutputIntents=pikepdf.Array([pdf.make_indirect(pikepdf.Dictionary(Type=pikepdf.Name.OutputIntent,S=pikepdf.Name.GTS_PDFA1,OutputConditionIdentifier='sRGB IEC61966-2.1',Info='sRGB output; not PDF/A certification',DestOutputProfile=profile))])
        tmp=target.with_suffix('.color.pdf');pdf.save(tmp)
    os.replace(tmp,target)

def export(req, root, output):
    cancelled(req,output);p=req['project'];start=time.perf_counter();cg=cover_geometry(p)
    if p['lab']['verification'] in ('synthetic','unverified') and not req['proof']:raise ValueError('Unverified profile cannot produce a lab-ready export')
    lab=p['lab'];count=len(p['pages']);rules=lab['pages']
    if count<rules['min'] or count>rules['max'] or count%rules['multiple'] or (lab['exportUnit']=='page' and count%2):raise ValueError('Interior count violates profile')
    for page in p['pages']:
        if page['widthMm']!=lab['trim']['w'] or page['heightMm']!=lab['trim']['h']:raise ValueError('Interior trim mismatch')
        if (page['side']=='spread')!=(lab['exportUnit']=='spread'):raise ValueError('Interior export unit mismatch')
    assets={a['id']:(a,checked_asset(root,a)) for a in p['assets']}
    fonts={f['id']:checked_asset(root,f) for f in p['fonts']}
    if not fonts:raise ValueError('No embedded font supplied')
    for name,path in fonts.items():pdfmetrics.registerFont(TTFont(name,str(path)))
    output.mkdir(parents=True,exist_ok=True)
    destination=output/req['jobId']
    if destination.exists():raise ValueError('Job output already exists; use a fresh job ID')
    peak=[0];stop=threading.Event();process=psutil.Process()
    def monitor():
        while not stop.wait(.02):
            try:peak[0]=max(peak[0],sum(proc.memory_info().rss for proc in [process,*process.children(recursive=True)] if proc.is_running()))
            except (psutil.NoSuchProcess,psutil.AccessDenied):pass
    thread=threading.Thread(target=monitor,daemon=True);thread.start()
    try:
        with tempfile.TemporaryDirectory(prefix='folio-',dir=output) as temp:
            staging=Path(temp)
            render_pdf(p['pages'],p,assets,fonts,staging/'interior.pdf',req,output)
            cover_project={**p,'lab':{**p['lab'],'bleed':cg['bleed']}} if cg else None
            if cg:render_pdf([p['cover']],cover_project,assets,fonts,staging/'cover.pdf',req,output)
            b=p['lab']['bleed'];ppi=p['lab']['ppi']
            for filename,pages in [('interior',p['pages'])]+([('cover',[p['cover']])] if cg else []):
                b=cg['bleed'] if filename=='cover' else p['lab']['bleed']
                for index,page in enumerate(pages):
                    cancelled(req,output)
                    w=round((page['widthMm']+b['left']+b['right'])/25.4*ppi);h=round((page['heightMm']+b['top']+b['bottom'])/25.4*ppi)
                    prefix=staging/((f'{index+1:02d}' if p['lab']['exportUnit']=='spread' else f'page-{index+1:03d}') if filename=='interior' else 'cover')
                    args=[sys.executable,str(Path(__file__).with_name('pdfium_raster.py')),str(staging/(filename+'.pdf')),str(index),str(w),str(h),str(prefix.with_suffix('.jpg')),str(ppi)]
                    child=subprocess.Popen(args,stdout=subprocess.DEVNULL,stderr=subprocess.PIPE,creationflags=getattr(subprocess,'CREATE_NO_WINDOW',0))
                    try:
                        while child.poll() is None:
                            cancelled(req,output);time.sleep(.04)
                        if child.returncode:raise ValueError('PDF rasterizer failed: '+child.stderr.read().decode(errors='replace')[:500])
                    finally:
                        if child.poll() is None:child.terminate();child.wait(timeout=10)
                        child.stderr.close()
                    jpg=prefix.with_suffix('.jpg')
                    with Image.open(jpg) as im:
                        if im.size!=(w,h):raise ValueError('Rasterizer produced wrong dimensions')
                        if not im.info.get('icc_profile'):raise ValueError('Rasterizer omitted ICC')
            for file in staging.iterdir():
                if file.stat().st_size>p['lab']['maxFileBytes']:raise ValueError(f'Lab file size limit exceeded: {file.name}')
            metrics={'elapsedSeconds':time.perf_counter()-start,'peakNativeTreeBytes':peak[0],'pageCount':len(p['pages']),'exportUnit':p['lab']['exportUnit'],'rasterizer':'pypdfium2-5.14.0','os':sys.platform,'architecture':__import__('platform').machine(),'proof':req['proof'],'profile':p['lab']['id']}
            (staging/'metrics.json').write_text(json.dumps(metrics,indent=2),encoding='utf-8')
            cancelled(req,output);os.replace(staging,destination)
        return {'status':'completed','jobId':req['jobId'],'directory':str(destination),'metrics':metrics}
    finally:stop.set();thread.join(timeout=1)

def main():
    parser=argparse.ArgumentParser();parser.add_argument('--root',type=Path,required=True);parser.add_argument('--output',type=Path,required=True);args=parser.parse_args()
    for line in sys.stdin:
        try:
            if len(line)>8_000_000:raise ValueError('Request too large')
            req=validate_request(json.loads(line,parse_constant=lambda x:(_ for _ in ()).throw(ValueError('Nonfinite number'))))
            cancelled(req,args.output)
            result={'status':'valid','jobId':req['jobId']} if req['operation']=='validate' else export(req,args.root,args.output)
        except InterruptedError as error:result={'status':'cancelled','message':str(error)}
        except Exception as error:result={'status':'error','message':str(error)[:1200]}
        print(json.dumps(result),flush=True)

if __name__=='__main__':main()
