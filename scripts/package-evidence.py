"""Explicit safe allowlist: implementation plus synthetic M1a evidence only."""
from pathlib import Path
import hashlib,json,zipfile

root=Path.cwd()
output=root/'artifacts'/'07_CHATGPT_M1a_EVIDENCE.zip'
files=[]
for folder in ['packages','apps/cli','native','scripts','tests','fixtures','.github']:
    files += [p for p in (root/folder).rglob('*') if p.is_file() and '__pycache__' not in p.parts]
for name in ['package.json','package-lock.json','tsconfig.json','requirements.txt','README.md','ARCHITECTURE.md','DECISIONS.md','STATUS.md','.gitignore','artifacts/verification.txt']:
    files.append(root/name)
for folder in ['artifacts/demo-4-257d0948','artifacts/demo-40-19044037']:
    files += [p for p in (root/folder).iterdir() if p.is_file()]
manifest=[{'path':p.relative_to(root).as_posix(),'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in sorted(files)]
with zipfile.ZipFile(output,'w',zipfile.ZIP_DEFLATED,allowZip64=True) as archive:
    for p in sorted(files):archive.write(p,p.relative_to(root).as_posix())
    archive.writestr('EVIDENCE-MANIFEST.json',json.dumps(manifest,indent=2))
print(json.dumps({'path':str(output),'bytes':output.stat().st_size,'files':len(files),'sha256':hashlib.sha256(output.read_bytes()).hexdigest()}))
