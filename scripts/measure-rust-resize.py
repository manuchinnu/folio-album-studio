"""Synthetic candidate/reference comparison; channel errors are not DeltaE gates."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tempfile
import random
from PIL import Image

root = Path(__file__).resolve().parents[1]
binary = root / 'native/target/debug' / ('folio-cli.exe' if os.name == 'nt' else 'folio-cli')
out = root / 'artifacts'
out.mkdir(exist_ok=True)
results = []
with tempfile.TemporaryDirectory(prefix='resize-reference-', dir=out) as directory:
    for name in ['gradient', 'checker', 'impulse', 'noise']:
        random_source = random.Random(20261028)
        source = Image.new('RGB', (31, 29))
        samples = []
        for y in range(29):
            for x in range(31):
                if name == 'gradient':
                    pixel = (x * 255 // 30, y * 255 // 28, (x+y) * 255 // 58)
                elif name == 'checker':
                    pixel = (255, 0, 128) if (x+y) % 2 else (0, 255, 64)
                elif name == 'noise':
                    pixel = tuple(random_source.randrange(256) for _ in range(3))
                else:
                    pixel = (255, 255, 255) if (x,y) == (15,14) else (0,0,0)
                samples.append(pixel)
        source.putdata(samples)
        path = Path(directory) / 'source.png'
        source.save(path)
        sha = hashlib.sha256(path.read_bytes()).hexdigest()
        for width, height in [(13,11), (47,43), (31,29), (1,1), (31,7), (9,29)]:
            proc = subprocess.run([str(binary), 'sample-resized-image', directory, path.name,
                sha, '1000000', '4096', str(width), str(height)], capture_output=True, text=True)
            if proc.returncode:
                raise RuntimeError(proc.stderr)
            actual = json.loads(proc.stdout)['samples']
            expected = source.resize((width,height), Image.Resampling.LANCZOS).tobytes()
            assert len(actual) == len(expected)
            errors = [abs(a-b) for a,b in zip(actual,expected)]
            results.append({'fixture':name, 'size':[width,height], 'maxChannelError':max(errors),
                'meanChannelError':sum(errors)/len(errors), 'channelsOverOne':sum(e>1 for e in errors)})
report = {'scope':'Synthetic RGB8 channel comparison; not DeltaE2000 or production parity acceptance',
          'reference':'Pillow Lanczos', 'candidate':'image 0.25.5 Lanczos3', 'results':results}
(out / 'rust-resize-reference.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(results))
assert all(r['maxChannelError'] <= 1 for r in results), 'Synthetic channel regression; see saved report'
