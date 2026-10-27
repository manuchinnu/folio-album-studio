"""Generate a locked Cargo dependency/license inventory (not a license audit).

Run with the configured cargo executable. Offline/locked metadata prevents
dependency updates; all target dependencies in the lock graph are included.
"""
import argparse
import json
import subprocess
import tomllib
from pathlib import Path

root = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser()
parser.add_argument('--cargo', default='cargo')
parser.add_argument('--toolchain', default='1.85.1')
parser.add_argument('--output', type=Path, required=True)
args = parser.parse_args()
metadata = json.loads(subprocess.check_output([
    args.cargo, '+' + args.toolchain, 'metadata', '--manifest-path', str(root / 'native/Cargo.toml'),
    '--format-version', '1', '--locked', '--offline'
], text=True))
lock = tomllib.loads((root / 'native/Cargo.lock').read_text(encoding='utf-8'))
checksums = {(p['name'], p['version'], p.get('source')): p.get('checksum')
             for p in lock['package']}
packages = []
for package in metadata['packages']:
    if package['source'] is None:
        continue  # Own workspace, not a third-party dependency.
    packages.append({
        'name': package['name'], 'version': package['version'],
        'source': package['source'],
        'checksum': checksums.get((package['name'], package['version'], package['source'])),
        'declaredLicense': package['license'],
        'licenseFile': Path(package['license_file']).name if package['license_file'] else None,
        'repository': package['repository'],
    })
packages.sort(key=lambda p: (p['name'], p['version']))
missing = [p['name'] for p in packages
           if not p['checksum'] or not (p['declaredLicense'] or p['licenseFile'])]
report = {
    'scope': 'Locked Cargo dependency graph; all targets; no absolute local paths',
    'limitations': 'Declared metadata only. Does not replace license-text distribution, legal review, model provenance or a complete application SBOM.',
    'packages': packages, 'missingMetadata': missing,
}
args.output.parent.mkdir(parents=True, exist_ok=True)
args.output.write_text(json.dumps(report, indent=2) + '\n', encoding='utf-8')
print(json.dumps({'packages': len(packages), 'missingMetadata': missing}))
raise SystemExit(bool(missing))
