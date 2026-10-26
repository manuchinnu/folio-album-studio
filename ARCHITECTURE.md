# Architecture

M1b preparation now includes generated import request/response schemas, a sequential native HEIC/JPEG/PNG/WebP inspection adapter, managed 512px proxies, content hashes, EXIF orientation, pHash/dHash and baseline technical signals. Node enumerates linked roots, bounds JSON sidecars, preserves field provenance and resolves exact Takeout matches. Native XMP uses defusedxml with DTD/entities/external access disabled. Pure core curation retains alternatives and reports event-budget infeasibility; pure mm grid generation is independently tested. CLI import creates an intermediate checkpoint, not the final SQLite/portable project. GUI, storage/history, full colour fixtures and pipeline integration are still in progress.

Contract: `docs/05_CHATGPT_PLAN_v3.md`; authorisation: Drive `06_GO_AHEAD`.

Implemented M1a flow:

1. `apps/cli/demo.ts` composes a typed fixed display list of image, fill and single-line text elements, using mm geometry and linked source references.
2. `packages/core/schema.ts` is the canonical TypeBox contract. Generated JSON Schema is checked for freshness on every verification run. `geometry.ts` and `preflight.ts` are strict TypeScript, independent of DOM, file access and HTTP.
3. The CLI validates and performs blocking preflight before calling `native-port.ts`, which sends JSON-lines over stdin/stdout to a local Python process. Cancellation uses a job-specific marker file, checked between pages/elements and while Poppler runs. Staged outputs publish atomically after completion.
4. Python validates against the generated schema and dynamically builds strict Pydantic models from it. Source hashes and paths are checked. Pillow/LittleCMS normalises ICC, applies crop and bounded downsampling; ReportLab embeds fonts and draws the PDF. pikepdf supplies ICC-based image resources and the output intent. Poppler rasterises one page at a time.
5. `scripts/inspect-pdf.py` independently reads PDF operators, boxes, font resources and ICC resources to verify actual outputs rather than trusting renderer metadata.

Current native module is a local trusted adapter; it is not an authenticated server. Clients must invoke canonical core preflight before export. No HTTP service is exposed. Additional native preflight parity hardening is tracked in STATUS.md.

Planned ports: media ingest, analysis, solver, persistence, AI and packaging. Node loopback host, React UI, SQLite project storage and native media/ML workers follow the agreed milestones. The current demo is not the future interactive UI or automatic layout algorithm.
