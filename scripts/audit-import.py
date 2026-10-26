"""Verify normalized proxy pixels against synthetic corner identities, without EXIF transpose."""
import argparse,json
from pathlib import Path
from PIL import Image
parser=argparse.ArgumentParser();parser.add_argument('directory',type=Path);args=parser.parse_args()
data=json.loads((args.directory/'library.json').read_text(encoding='utf-8'))
expected={1:(255,0,0),2:(0,128,0),3:(0,0,255),4:(255,255,0),5:(255,0,0),6:(255,255,0),7:(0,0,255),8:(0,128,0)}
checks=[]
for n,color in expected.items():
    item=next(i for i in data['images'] if i['relativePath']==f'orientation-{n}.jpg')
    with Image.open(args.directory/'proxies'/item['proxyPath']) as image:
        actual=image.convert('RGB').getpixel((5,5));difference=max(abs(a-b) for a,b in zip(actual,color))
        assert difference<=10,(n,actual,color)
        assert image.getexif().get(274,1)==1
        assert image.info.get('icc_profile')
        checks.append({'orientation':n,'expectedTopLeft':color,'actualTopLeft':actual,'maximumChannelError':difference})
(args.directory/'orientation-audit.json').write_text(json.dumps({'passed':True,'checks':checks},indent=2),encoding='utf-8')
print(json.dumps({'passed':True,'orientations':len(checks),'maximumChannelError':max(c['maximumChannelError'] for c in checks)}))
