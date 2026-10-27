"""Exercise Rust executable against generated synthetic PNG, never user media."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import struct
import tempfile
import zlib
from PIL import Image, ImageCms
import io

root = Path(__file__).resolve().parents[1]
fixture_root = root / 'fixtures/import'
fixture = fixture_root / 'nested/image.png'
digest = hashlib.sha256(fixture.read_bytes()).hexdigest()
binary = root / 'native/target/debug' / ('folio-cli.exe' if os.name == 'nt' else 'folio-cli')

def run(sha, budget, command='inspect-image'):
    return subprocess.run([str(binary), command, str(fixture_root),
        'nested/image.png', sha, str(budget), '10000000'], capture_output=True, text=True)

success = run(digest, 10000000)
if success.returncode:
    raise RuntimeError(success.stderr)
result = json.loads(success.stdout)
assert (result['width'], result['height']) == (120, 80), result
assert result['orientationApplied'] is True and result['colorNormalized'] is False
assert result['iccBytes'] > 0, result
normalized = run(digest, 10000000, 'inspect-normalized-image')
if normalized.returncode:
    raise RuntimeError(normalized.stderr)
normalized_result = json.loads(normalized.stdout)
assert (normalized_result['width'], normalized_result['height']) == (120, 80)
assert normalized_result['orientationApplied'] is True
assert normalized_result['colorNormalized'] is True
assert normalized_result['colorType'] == 'Rgb8' and normalized_result['iccBytes'] > 0
for sha, budget in [(digest, 1), ('0' * 64, 10000000)]:
    rejected = run(sha, budget)
    assert rejected.returncode == 2 and not rejected.stdout and rejected.stderr

# Build PNG16 directly to avoid a fixture library silently reducing RGB depth.
def chunk(kind, data):
    return struct.pack('>I', len(data)) + kind + data + struct.pack('>I', zlib.crc32(kind + data))

with Image.open(fixture) as source:
    icc = source.info['icc_profile']
depth_results = []
reference_result = None
(root / 'artifacts').mkdir(exist_ok=True)
with tempfile.TemporaryDirectory(prefix='folio-synthetic16-', dir=root / 'artifacts') as temporary:
    # Independent Pillow/ImageCms reference across a deterministic RGB cube.
    cube = Image.new('RGB', (32, 32))
    cube.putdata([(x * 255 // 7, y * 255 // 7, z * 255 // 15)
                  for x in range(8) for y in range(8) for z in range(16)])
    cube_path = Path(temporary) / 'cube.png'
    cube.save(cube_path, icc_profile=icc)
    checked = subprocess.run([str(binary), 'sample-normalized-image', temporary,
        cube_path.name, hashlib.sha256(cube_path.read_bytes()).hexdigest(), '1000000', '4096'],
        capture_output=True, text=True)
    assert checked.returncode == 0, checked.stderr
    samples = json.loads(checked.stdout)['samples']
    reference = ImageCms.profileToProfile(cube, ImageCms.ImageCmsProfile(io.BytesIO(icc)),
        ImageCms.createProfile('sRGB'), renderingIntent=1, outputMode='RGB').tobytes()
    assert len(samples) == len(reference)
    errors = [abs(a-b) for a,b in zip(samples, reference)]
    assert max(errors) <= 1, max(errors)
    reference_result = {'pixels': 1024, 'profile': 'sRGB', 'maxChannelError': max(errors),
        'meanChannelError': sum(errors) / len(errors), 'reference': 'Pillow ImageCms relative colorimetric'}
    for color_type, channels, expected in [(2, 3, 'Rgb16'), (6, 4, 'Rgba16')]:
        pixels = [12345, 23456, 34567] + ([45678] if channels == 4 else [])
        scanline = b'\0' + struct.pack('>' + 'H' * channels, *pixels)
        png = (b'\x89PNG\r\n\x1a\n' + chunk(b'IHDR', struct.pack('>IIBBBBB', 1, 1, 16, color_type, 0, 0, 0))
               + chunk(b'iCCP', b'sRGB\0\0' + zlib.compress(icc))
               + chunk(b'IDAT', zlib.compress(scanline)) + chunk(b'IEND', b''))
        path = Path(temporary) / 'synthetic.png'
        path.write_bytes(png)
        checked = subprocess.run([str(binary), 'sample-normalized-image', temporary,
            path.name, hashlib.sha256(png).hexdigest(), '1000000', '100'], capture_output=True, text=True)
        assert checked.returncode == 0, checked.stderr
        info = json.loads(checked.stdout)
        assert info['colorType'] == expected and (info['width'], info['height']) == (1, 1), info
        assert info['colorNormalized'] is True and info['iccBytes'] > 0, info
        assert len(info['samples']) == channels
        assert all(abs(a - b) <= 8 for a, b in zip(info['samples'][:3], pixels[:3])), info
        if channels == 4:
            assert info['samples'][3] == pixels[3], info
        depth_results.append(info)
# The diagnostic cap applies even when the caller supplies a larger budget.
oversized = run(digest, 10000000, 'sample-normalized-image')
assert oversized.returncode == 2 and not oversized.stdout and oversized.stderr
out = root / 'artifacts/rust-decoder-smoke.json'
out.parent.mkdir(exist_ok=True)
out.write_text(json.dumps({'passed': True, 'syntheticPng': result,
    'normalizedSyntheticPng': normalized_result,
    'normalized16BitPng': depth_results,
    'srgbReferenceComparison': reference_result,
    'rejected': ['encoded-byte-budget', 'changed-source-identity', 'diagnostic-pixel-cap'],
    'scope': 'Decoder smoke only; no color or rendering parity claim'}, indent=2) + '\n')
print('Rust decoder synthetic smoke passed')
