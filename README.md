# Folio Album Studio

Local-first album software under implementation against `docs/05_CHATGPT_PLAN_v3.md`, approved by Claude in Drive `06_GO_AHEAD` on 2026-10-08. This checkout currently provides the M1a fixed-layout print CLI, not the finished album editor.

## Run the synthetic print demo

Prerequisites: Node 24, Python 3.12+, with the pinned native dependencies below (PDFium is included as a wheel; Poppler is no longer required). From this directory:

```powershell
npm.cmd ci
python -m venv .venv
.venv/Scripts/python.exe -m pip install -r requirements.txt
.venv/Scripts/python.exe scripts/create-fixtures.py
npm.cmd run verify
npm.cmd run demo -- 40
```

On Linux use `.venv/bin/python` and `npm`. `FOLIO_PYTHON` can override the interpreter path. The CLI prints a fresh output directory; inspect it with:

```powershell
.venv/Scripts/python.exe scripts/inspect-pdf.py artifacts/<job-id> --expected-pages 40
```

Outputs include interior PDF, cover PDF, 300-PPI JPEGs, preflight JSON/text, and measured metrics. The independent inspector adds PDF resource/box/PPI evidence. Source fixtures are synthetic illustrations and a licensed bundled Vera font. The three illustrations intentionally repeat to exercise the renderer; they do not demonstrate curation or layout variety.

All demo outputs are engineering proofs with an unverified synthetic lab profile. A real lab/product/template and acceptance evidence are required before print readiness can be claimed. See STATUS.md for other open gates. No personal photos or paid services are needed by these commands.

The historical `dist/` scaffold predates the approved plan and is not the current application.

## Import checkpoint (M1b preparation)

`npm run import -- <source-folder> <new-project-folder>` reads linked originals and writes proxies plus `library.json` to a new destination outside the source folder. Existing destinations are not overwritten. The checkpoint includes metadata evidence/alternatives, explicit decode failures, baseline duplicate groups/event selection and quality signals. It is not yet the finished album project or an export-ready draft.

Use `fixtures/import` for safe format/orientation/Takeout/XMP cases; `scripts/create-fixtures.py` also creates these. `scripts/audit-import.py <project-folder>` independently verifies all eight fixture orientation proxies. RAW preview extraction, fuzzy sidecar reconciliation, NCLX-only colour, full P3 testing, persistent history and portable projects remain open. HEIC decoder and secure XML parser dependencies are pinned in requirements.txt.

## Spread engineering benchmark

Run `.venv/Scripts/python.exe scripts/create-spread-fixture.py`, then `npx --no-install tsx apps/cli/spread-demo.ts`. Audit the returned directory with `scripts/inspect-pdf.py <directory> --expected-pages 40 --spread-12x15`. The profile is an engineering assumption pending Canvera templates. Schema v2 supports explicit spread units and optional template/table-driven covers; `migrateProject` copies and upgrades legacy v1 documents.
