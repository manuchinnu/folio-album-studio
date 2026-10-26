"""Synthetic high-resolution panorama for largest-format print geometry."""
import hashlib,json
from pathlib import Path
from PIL import Image,ImageDraw,ImageCms
root=Path('fixtures/large');root.mkdir(parents=True,exist_ok=True)
image=Image.new('RGB',(7392,3732),'#ccd7cc');draw=ImageDraw.Draw(image)
for x in range(0,7500,50):draw.rectangle((x,0,x+49,3799),fill=(80+x*150//7500,130,190-x*90//7500))
draw.polygon([(0,3000),(1700,500),(3750,1900),(5400,800),(7500,2600),(7500,3800),(0,3800)],fill='#405b50')
for y in range(0,3800,200):draw.line((0,y,7500,y),fill='white',width=3)
draw.line((3696,0,3696,3732),fill='#fae6ab',width=12)
path=root/'panorama.jpg';image.save(path,'JPEG',quality=97,subsampling=0,icc_profile=ImageCms.ImageCmsProfile(ImageCms.createProfile('sRGB')).tobytes())
asset={'id':'large-panorama','relativePath':path.as_posix(),'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'width':7392,'height':3732,'mime':'image/jpeg','faces':[],'analysis':'complete'}
(root/'manifest.json').write_text(json.dumps(asset,indent=2),encoding='utf-8')
