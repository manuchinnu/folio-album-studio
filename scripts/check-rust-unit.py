"""Synthetic unit composition through executable, audited independently by Pillow."""
import copy
import hashlib
import io
import json
import os
from pathlib import Path
import subprocess
import tempfile
from PIL import Image, ImageCms, JpegImagePlugin

root = Path(__file__).resolve().parents[1]
binary = root / 'native/target/debug' / ('folio-cli.exe' if os.name == 'nt' else 'folio-cli')
artifacts = root / 'artifacts'
artifacts.mkdir(exist_ok=True)
with tempfile.TemporaryDirectory(prefix='unit-proof-', dir=artifacts) as directory:
    folder = Path(directory)
    sources = []
    colors = [(200,40,60),(20,120,210)]
    for i,color in enumerate(colors):
        path = folder / f'{i}.png'
        Image.new('RGB',(16,16),color).save(path)
        sources.append({'relative':path.name,'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),
                        'orientedWidth':16,'orientedHeight':16})
    mm = 25.4/300
    request = {'width':64,'height':32,'sources':sources,'placements':[
        {'source_index':i,'rect':{'x':i*32*mm,'y':0,'width':32*mm,'height':32*mm},
         'crop':{'x':0,'y':0,'w':1,'h':1}} for i in range(2)]}
    def run(data,name):
        return subprocess.run([str(binary),'save-unit-proof',directory,str(folder/name)],
                              input=json.dumps(data),text=True,capture_output=True)
    result = run(request,'proof.jpg')
    assert result.returncode == 0, result.stderr
    assert json.loads(result.stdout)['assumedSrgbSources'] == [0,1]
    with Image.open(folder/'proof.jpg') as image:
        assert image.size == (64,32) and image.info['dpi'] == (300,300)
        assert JpegImagePlugin.get_sampling(image) == 0
        ImageCms.ImageCmsProfile(io.BytesIO(image.info['icc_profile']))
        for x,color in [(16,colors[0]),(48,colors[1])]:
            assert all(abs(a-b)<=3 for a,b in zip(image.getpixel((x,16)),color))
    original = (folder/'proof.jpg').read_bytes()
    assert run(request,'proof.jpg').returncode == 2
    assert (folder/'proof.jpg').read_bytes() == original
    for mutation in ['unknown-field','dimensions','hash','bounds','crop','empty','too-many']:
        invalid = copy.deepcopy(request)
        if mutation == 'unknown-field': invalid['unexpected'] = True
        elif mutation == 'dimensions': invalid['sources'][0]['orientedWidth'] = 17
        elif mutation == 'hash': invalid['sources'][0]['sha256'] = '0'*64
        elif mutation == 'bounds': invalid['placements'][1]['rect']['x'] = 100
        elif mutation == 'crop': invalid['placements'][0]['crop']['w'] = 2
        elif mutation == 'empty': invalid['placements'] = []
        else: invalid['placements'] = invalid['placements'] * 33
        failed = run(invalid,'rejected.jpg')
        assert failed.returncode == 2 and not failed.stdout, mutation
        assert not (folder/'rejected.jpg').exists(), mutation
    oversized = subprocess.run([str(binary),'save-unit-proof',directory,str(folder/'rejected.jpg')],
        input=' ' * 1_048_577,text=True,capture_output=True)
    assert oversized.returncode == 2 and not oversized.stdout
    assert not (folder/'rejected.jpg').exists()
(artifacts/'rust-unit-smoke.json').write_text(json.dumps({'passed':True,'size':[64,32],
    'verified':['two-source-placement','ICC','300DPI','4:4:4','assumed-provenance','no-overwrite',
                'unknown-field','dimensions','hash','bounds','crop','empty','too-many','input-byte-cap'],
    'scope':'Synthetic opaque RGB unit only; not full album or parity acceptance'},indent=2)+'\n')
print('Rust unit composition smoke passed')
