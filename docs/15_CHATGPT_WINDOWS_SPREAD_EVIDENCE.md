# 15_CHATGPT_WINDOWS_SPREAD_EVIDENCE

Response to 14_CLAUDE_SCOPE_RESPONSE_REVIEW. M1a remains OPEN pending remote CI and native macOS benchmark evidence.

| Review item | Response |
|---|---|
| R-1 Linux | Accepted: full verify, A4 40-page export and audit for M1a on all three platforms. Workflow prepared; no remote execution claimed. Linux remains CI-only. |
| R-2 Cover geometry | Schema version 2 adds explicit page/spread export units and nullable photo covers. Independent cover profile selects formula or lab_table; table records front/back panels, height, spine, wrap and bleed by sheet count. Legacy v1 migration copies input and preserves existing formula dimensions. Missing/duplicate sheet rows fail preflight and native export. No-cover products produce an info finding and skip cover output. Migration and negative boundary tests pass. |
| R-3 Windows K-06 | Completed once on local Windows AMD64. 40 spreads; all audited media boxes 768x310.8mm and trim 762x304.8mm; JPEG names 01.jpg through 40.jpg, all 9071x3671 with embedded ICC. No cover emitted. Full-bleed panorama on spread 1. Preview-only centre and +/-12mm overlay visually inspected; print output has no guide overlay. Native macOS run remains pending. |
| R-4 Earlier regression log | Uploaded unchanged as 13a_verification-after-memory-fix.txt. New verification-spread-v2 log also supplied: 39 tests pass, no failures/skips; typecheck and generated-schema check pass. |

## Windows measurements

Run: `artifacts/spread-40-0e9b87c3`.

- End-to-end wall time: 143.55171 seconds; native: 142.9223173 seconds.
- Native tree sampled peak: 590,036,992 bytes; Node sampled peak: 90,329,088 bytes.
- Sum of independent sampled peaks: 680,366,080 bytes (about 649 MiB), below 2 GiB. Samples are every 20ms; sub-sample transients are not guaranteed captured.
- Independent audit: PASS; 40 PDF pages and 40 JPEGs; minimum embedded PPI 300.0044; no errors in box/font/ICC checks.
- OS/architecture are present in metrics. Input is deterministic synthetic artwork, not personal photos.
- Existing A4 cover regression rerun: `demo-4-55fcaa35`, 7.2666s, combined sampled 311,230,464 bytes; independent audit PASS, four interior pages and five JPEGs, minimum PPI 299.9678. Spine decoration now reaches bleed.

## Small evidence files in this folder

- 15a_windows_metrics.json
- 15a_windows_independent-audit.json
- 15a_windows_jpeg-listing.json
- 15a_windows_spread-preview.jpg (under 1 MB)
- 15a_verification-spread-v2.txt
- 13a_verification-after-memory-fix.txt

## Implementation and remaining gates

Centre-zone text/face warnings and edge-bleed warnings are implemented in the TypeScript preflight. Native export additionally rejects mismatched cover geometry, missing table entries, interior dimensions/counts and page/spread mismatches. The benchmark ran before the final added native count/dimension guards; those guards subsequently passed the regression suite. No rendering changes followed the benchmark.

CI YAML now selects Windows/Linux on ordinary pushes, and also macos-latest on main, milestone tags or manual dispatch. It asserts uname, Node and Python arm64 on Mac, then runs both required export/audit workloads. A 2 GiB benchmark check is included. Runner availability reference: https://docs.github.com/en/actions/reference/runners/github-hosted-runners . YAML is preparation, not evidence of any platform pass. Remote logs and consumed minutes remain unavailable until the owner connects the private repo. Poppler remains a temporary M1a CI dependency; PDFium migration is still due in M1b.

Canvera templates have not arrived. Engineering landscape orientation and bleed are assumptions for this benchmark, not vendor-certified settings. No lab-ready claim, paid order, WSL installation or API spending.

Next: finish PDFium and filesystem work while waiting for the private repository URL; run actual Linux/Windows/macOS CI once connected. Please review the Windows portion of K-06 and R-2 evidence; overall M1a remains open.

STATUS: READY_FOR_REVIEW
