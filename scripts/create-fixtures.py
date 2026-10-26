"""Nonpersonal, deterministic test assets; no user's source photographs touched."""
from pathlib import Path
import hashlib,json,shutil
from PIL import Image,ImageDraw,ImageCms
import reportlab

root=Path('fixtures/media');root.mkdir(parents=True,exist_ok=True)
icc=ImageCms.ImageCmsProfile(ImageCms.createProfile('sRGB')).tobytes()
assets=[]
for n,palette in enumerate([('#bac9bd','#68887e','#334d49'),('#dfc6a3','#ae8f6c','#67654a'),('#aebfce','#7b929a','#4a646a')]):
    image=Image.new('RGB',(3000,4000),palette[0]);draw=ImageDraw.Draw(image)
    draw.ellipse((1800,420,2450,1070),fill='#f8e8bd')
    draw.polygon([(0,2600),(600,1300),(1500,2200),(2300,1400),(3000,2400),(3000,4000),(0,4000)],fill=palette[1])
    draw.polygon([(0,3300),(1000,2200),(2100,3100),(3000,2100),(3000,4000),(0,4000)],fill=palette[2])
    # Fiducials and color patches let tests detect orientation and clipping.
    for x,col in enumerate(['#ff0000','#00ff00','#0000ff','#ffffff']):draw.rectangle((100+x*180,3600,230+x*180,3730),fill=col)
    path=root/f'landscape-{n}.jpg';image.save(path,quality=97,subsampling=0,icc_profile=icc)
    assets.append({'id':f'landscape-{n}','relativePath':path.as_posix(),'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'width':3000,'height':4000,'mime':'image/jpeg','faces':[],'analysis':'complete'})
font_source=Path(reportlab.__file__).parent/'fonts'/'Vera.ttf'
font=root/'Vera.ttf';shutil.copyfile(font_source,font)
license_source=font_source.parent/'bitstream-vera-license.txt'
if license_source.exists():shutil.copyfile(license_source,root/'FONT-LICENSE.txt')
else:(root/'FONT-LICENSE.txt').write_text('Bitstream Vera bundled with ReportLab. See https://www.gnome.org/fonts/ and the ReportLab distribution for its license.',encoding='utf-8')
record={'assets':assets,'fonts':[{'id':'Vera','relativePath':font.as_posix(),'sha256':hashlib.sha256(font.read_bytes()).hexdigest()}]}
(root/'manifest.json').write_text(json.dumps(record,indent=2),encoding='utf-8')
print(json.dumps({'fixtureCount':len(assets),'personalPhotos':False}))
import runpy
runpy.run_path('scripts/create-import-fixtures.py')
