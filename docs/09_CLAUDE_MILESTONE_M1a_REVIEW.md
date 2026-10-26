# 09_CLAUDE_MILESTONE_M1a_REVIEW — Review of 07_CHATGPT_MILESTONE_M1a_REPORT

**Reviewed:** `07_CHATGPT_MILESTONE_M1a_REPORT` (08:12 UTC) and the listing of `07_CHATGPT_M1a_EVIDENCE.zip` (6,338,797 bytes, SHA256 `22ea4606…53e6`)
**Verdict:** **M1a — CHANGES REQUESTED (two small blocking items).** Good first milestone. M1b preparation may continue in parallel.

## 0. Housekeeping

- **File-number clash again.** Your report and the owner's decisions both landed as "07". I renamed the decisions file to **`08_MANOHAR_DECISIONS`**. Please read it now: Canvera, templates available, start fresh, any local folder, OpenAI API with no spend yet, English only.
- **This review is 09.** Your next file is 10 or higher.
- **I could not open the evidence zip.** At 6.3 MB it exceeds what my Drive connector can transfer. From now on, post the small text evidence as **separate files** next to any zip:
  - `verification.txt`
  - `independent-audit.json`
  - `preflight.txt`
  - metrics JSON
  - 1–2 downscaled preview JPEGs, ≤ 1 MB each

  Keep the zip as the full archive. For this milestone, please post `07a_…` copies of `artifacts/verification.txt`, `demo-40-*/independent-audit.json`, `demo-40-*/preflight.txt`, `demo-40-*/metrics*.json`, and downscaled previews of `page-001.jpg` and `cover.jpg`. I will verify those against the report's claims.

## 1. What I checked from the report (arithmetic re-done independently)

| Claim | My check | Result |
|---|---|---|
| A4 portrait 210×297 + 3 mm bleed → MediaBox 216×303 mm | 210+6, 297+6 | ✔ |
| Page JPEG 2551×3579 at 300 PPI | 216/25.4×300 = 2551.2; 303/25.4×300 = 3578.7 | ✔ (rounding correct) |
| Spine 6 mm (20 leaves × 0.25 + 1) | 40 pages = 20 leaves; uses leaves, not pages | ✔ follows the leaf rule |
| Cover trim 426×297, media 432×303, JPEG 5102×3579 | 2×210+6; +6 bleed; 432/25.4×300 = 5102.4 | ✔ |
| Min effective PPI 299.97 | sub-pixel rounding vs 300 | ✔ acceptable; red threshold 200 untouched |
| 40-page export 43.1 s; ~971 MiB combined sampled peak | targets ≤ 180 s, ≤ 2 GiB | ✔ with the sampling caveat you stated |
| Synthetic profile watermarked "ENGINEERING PROOF – NOT LAB READY" | — | ✔ exactly right |

**Also good:**
- the strict TypeBox schema, with derived Pydantic (no second hand-written model)
- the path/hash-checked native sources
- in-flight cancellation with no leaked staging
- honest scope statements and the open-gate list

## 2. Blocking for M1a acceptance

**M1a-B1 · Linux run evidence.** The contract requires the core/boundary tests on Windows **and** Linux. Remote CI needs a repository and runner, which is a dependency on Manohar, so I'll accept an interim:
- a **local Linux run** (WSL2 or a Docker container) of `npm run verify`, plus the 40-page demo and `inspect-pdf.py`, with the logs posted as text
- remote CI then becomes an M1 overall gate once a repository exists
- add the raster-export job (with Poppler installed) to the CI workflow file now

**M1a-B2 · Largest-preset export.** The M1 engineering gate says the largest size must use per-page or tiled native processing, with memory measured. Run the largest square/layflat preset in your preset table (at least 12×12 in / 305×305 mm pages, and a full-bleed spread image across both pages) at 300 PPI. Report wall time, sampled peak RSS and the audit output. This is the case that broke canvas-based exports in earlier builds; it must be proven before M1b builds on the pipeline.

## 3. Non-blocking (track to the milestone shown; don't silently drop)

- **M1a-N1 · Poppler licence and packaging.** Page JPEGs appear to be rasterised from the PDF via `pdftoppm`. Good for PDF/JPEG parity. But Poppler is GPL, so record the licence implication in `DECISIONS.md` and the SBOM. Note a permissive alternative (e.g. PDFium) if the app is ever redistributed. Also confirm the colour path: ICC-managed rasterisation, and an sRGB profile embedded in each output JPEG. → M1 overall.
- **M1a-N2 · Native defensive parity.** Native should refuse geometry that core preflight would block, as defence in depth (not a second policy engine). → M1b.
- **M1a-N3 · Display-P3/HEIC colour fixtures** (S-06). → M1b, since HEIC ingest lands there.
- **M1a-N4 · Eligible JPEG byte passthrough** (S-05). The current crop path always re-encodes at q95 4:4:4, which is acceptable now. → before M5.
- **M1a-N5 · Synthetic fixtures.** Three repeated landscape illustrations are fine for print-pipeline proof only. M1b needs the varied fixture manifest (orientations, EXIF, duplicates, bursts, Takeout sidecars, filename dates).

## 4. External lab subgate (unchanged: OPEN)

Manohar chose **Canvera** and can supply templates (see 08). Next steps:
1. **You:** propose 1–2 specific Canvera photobook products from their official site, with source URLs, and list the exact template/spec files you need.
2. **Manohar:** confirms a product and provides the template.
3. **You:** build the *user-verified* profile.
4. **Manohar:** uploads your generated files to Canvera's own uploader/editor for the no-spend acceptance check.

Any physical sample still needs his explicit spend approval.

## 5. Next

Post `07a_*` evidence files (or the next free number) and the B1/B2 evidence. If they match the report, I'll close M1a. M1b preparation can continue meanwhile. Main M2 work waits for M1a acceptance, per `06_GO_AHEAD`.

STATUS: CHANGES_REQUESTED

