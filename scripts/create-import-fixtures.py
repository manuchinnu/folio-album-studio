"""Small safe format and orientation fixtures, never personal media."""
from pathlib import Path
import json
from PIL import Image,ImageDraw,ImageCms
from pillow_heif import register_heif_opener
register_heif_opener()
root=Path('fixtures/import');(root/'nested').mkdir(parents=True,exist_ok=True)
image=Image.new('RGB',(120,80));draw=ImageDraw.Draw(image)
for box,color in [((0,0,59,39),'red'),((60,0,119,39),'green'),((0,40,59,79),'yellow'),((60,40,119,79),'blue')]:draw.rectangle(box,fill=color)
icc=ImageCms.ImageCmsProfile(ImageCms.createProfile('sRGB')).tobytes()
for orientation in range(1,9):
    exif=Image.Exif();exif[274]=orientation
    image.save(root/f'orientation-{orientation}.jpg',exif=exif,quality=100,subsampling=0,icc_profile=icc)
for fmt,name in [('PNG','nested/image.png'),('WEBP','image.webp'),('HEIF','image.heic')]:image.save(root/name,format=fmt,icc_profile=icc)
image.save(root/'misnamed.jpeg',format='PNG',icc_profile=icc)
(root/'corrupt.jpg').write_bytes(b'not an image')
(root/'camera.cr2').write_bytes(b'RAW placeholder - unsupported preview must be reported')
(root/'nested/image.png.json').write_text(json.dumps({'title':'image.png','photoTakenTime':{'timestamp':'1711972800'},'geoDataExif':{'latitude':0,'longitude':0,'altitude':0},'geoData':{'latitude':12.97,'longitude':77.59}}),encoding='utf-8')
for name in ['image.webp.json','duplicate.json']:(root/name).write_text(json.dumps({'title':'image.webp','photoTakenTime':{'timestamp':'1711972800'}}),encoding='utf-8')
(root/'orientation-1.xmp').write_text('<x:xmpmeta xmlns:x="adobe:ns:meta/" xmlns:exif="http://ns.adobe.com/exif/1.0/"><photo exif:DateTimeOriginal="2024-04-01T12:30:00+05:30" exif:GPSLatitude="12,30N" exif:GPSLongitude="77,30E"/></x:xmpmeta>',encoding='utf-8')
(root/'orientation-2.xmp').write_text('<!DOCTYPE x [<!ENTITY external SYSTEM "file:///nonexistent-folio-synthetic-fixture">]><x>&external;</x>',encoding='utf-8')
(root/'too-deep.json').write_text('['*25+'0'+']'*25,encoding='utf-8')
print('Created safe import fixtures')
