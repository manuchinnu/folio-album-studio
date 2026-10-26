"""Deterministic nonpersonal panorama at native 300-PPI benchmark resolution."""
from pathlib import Path
import hashlib,json
from PIL import Image,ImageDraw,ImageCms
root=Path('fixtures/spread');root.mkdir(parents=True,exist_ok=True)
with Image.new('RGB',(9071,3671),'#aecbc7') as image:
    draw=ImageDraw.Draw(image)
    draw.ellipse((6800,300,7500,1000),fill='#ffdc88')
    draw.polygon([(0,2600),(1700,700),(4000,2800),(5900,900),(9071,2600),(9071,3671),(0,3671)],fill='#607e72')
    draw.polygon([(0,3300),(3200,2000),(5400,3200),(7500,1800),(9071,3100),(9071,3671),(0,3671)],fill='#284e49')
    path=root/'panorama.jpg';image.save(path,quality=97,subsampling=0,icc_profile=ImageCms.ImageCmsProfile(ImageCms.createProfile('sRGB')).tobytes())
record={'id':'spread-panorama','relativePath':path.as_posix(),'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'width':9071,'height':3671,'mime':'image/jpeg','faces':[],'analysis':'complete'}
(root/'manifest.json').write_text(json.dumps(record,indent=2))
