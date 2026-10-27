"""Exercise Rust executable against generated synthetic PNG, never user media."""
import hashlib
import json
import os
from pathlib import Path
import subprocess

root = Path(__file__).resolve().parents[1]
fixture_root = root / 'fixtures/import'
fixture = fixture_root / 'nested/image.png'
digest = hashlib.sha256(fixture.read_bytes()).hexdigest()
binary = root / 'native/target/debug' / ('folio-cli.exe' if os.name == 'nt' else 'folio-cli')

def run(sha, budget):
    return subprocess.run([str(binary), 'inspect-image', str(fixture_root),
        'nested/image.png', sha, str(budget), '10000000'], capture_output=True, text=True)

success = run(digest, 10000000)
if success.returncode:
    raise RuntimeError(success.stderr)
result = json.loads(success.stdout)
assert (result['width'], result['height']) == (120, 80), result
assert result['orientationApplied'] is True and result['colorNormalized'] is False
assert result['iccBytes'] > 0, result
for sha, budget in [(digest, 1), ('0' * 64, 10000000)]:
    rejected = run(sha, budget)
    assert rejected.returncode == 2 and not rejected.stdout and rejected.stderr
out = root / 'artifacts/rust-decoder-smoke.json'
out.parent.mkdir(exist_ok=True)
out.write_text(json.dumps({'passed': True, 'syntheticPng': result,
    'rejected': ['encoded-byte-budget', 'changed-source-identity'],
    'scope': 'Decoder smoke only; no color or rendering parity claim'}, indent=2) + '\n')
print('Rust decoder synthetic smoke passed')
