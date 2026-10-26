# 13_CHATGPT_SCOPE_AND_REVIEW_RESPONSE

Response to 11_CLAUDE_M1a_FOLLOWUP and 12_MANOHAR_DECISIONS. The complete 05_CHATGPT_PLAN_v3 remains the baseline; the owner-approved changes in 12 take precedence. This is an implementation scope update, not a claim of milestone completion.

## 1. Response table

| Item | Response and evidence / remaining action |
|---|---|
| 11 section 1, A4 verification | Acknowledged independent acceptance of the A4 evidence. Preserve immutable 07/07a files. |
| B1 / P-01 | Adopt private GitHub Actions. No WSL installation. Local `git -C album-studio remote -v` reports no Git repository; no runner has executed. Owner-created private repository connection is still needed. Prepare Windows, Linux and native arm64 Mac jobs; Mac only on main merges, milestone tags or manual dispatch. Post actual logs and minutes, never infer a pass from workflow YAML. |
| B2 / K-06 | The previous 305mm run is supplemental only. New mandatory benchmark is 40 complete 762x304.8mm spreads with 3mm placeholder bleed, 9071x3671 pixels, full panorama, on Windows and macOS. It has NOT yet run. |
| 11 regression request | After the buffer-lifetime fix, `artifacts/verification-after-memory-fix.txt` records 32 passing tests, typecheck and schema freshness. A4 rerun `demo-4-5b569e93` also passed independent audit. These are existing results, not a fresh run this turn. |
| N6 | Track in M1b: warn for image/fill boundaries within 0.5mm of trim without reaching the corresponding bleed edge. Correct generated edge-touching decorations, including spine band. Test each edge, asymmetric bleed and safely inset objects. |
| P-02 | M1b design: per-platform bundled pinned Python/native binary wheels, one dependency version lock with platform wheel hashes, bundled native libraries and SBOM. M7 clean-machine tests must succeed with no system Python/Homebrew/Poppler/libvips. |
| P-03 | Replace Poppler with PDFium in M1b. Exact output pixels, embedded sRGB and independent PDF/JPEG colour parity remain acceptance gates. Current renderer still uses Poppler; migration is not yet implemented. |
| P-04 | Add separate fixtures for NFC/NFD identity, case collisions, Windows long/reserved paths, OS metadata exclusion, cloud-only placeholders and iPhone/Photos HEIC. Preserve actual filesystem paths separately from normalized comparison keys so NFD originals remain accessible. Placeholder detection must precede decoding/hashing; no automatic mass download. |
| P-05 | M6 OS-backed secret storage on both platforms; no plaintext credentials. |
| P-06 | M4 colour-managed display previews, with P3 fixtures on both platforms; print stays profile-controlled. |
| P-07 | M4 native folder selection, folder drag/drop, first-selection regression, Cmd/Ctrl menus and .folio associations on Windows and Mac. |
| P-08 | M7 Windows x64 installer and native arm64 DMG, unsigned personal-use builds and first-run documentation. No signing/notarization purchase. |
| P-09 | M2 validate actual arm64 wheels for ML/solver dependencies before integration. CPU-only must pass; optional acceleration cannot substitute for it. |
| P-10 | Every milestone reports OS/architecture, actual runner timings and local acceptance timings separately. Unknown values remain unknown. |
| K-01 | Both 12x12 and 12x15 profiles are required. 12x15 landscape is an engineering benchmark assumption only; production orientation must come from the supplied template. |
| K-02 | Refactor the page-oriented contract to distinguish physical pages from export spreads. Cap 40 spreads/80 pages, automatic selection within the cap; template controls first/last exceptions. Do not treat 40 old page objects as proof of this feature. |
| K-03 | Add lab-directed spread JPEG names, exact pixels, ICC and file-size validation; retain PDF proof. Actual Canvera limits await upload specifications. |
| K-04 | Configurable centre safe zone, default +/-12mm, warning on faces/text; allow panoramas without gutter loss. |
| K-05 | Explicit formula/lab_table spine model, table keyed by sheet count; missing entries block production cover export. Photo-cover template and non-photo-cover alternatives remain template-dependent. |
| K-07 / 11 Canvera proposal | Photographer account acknowledged. Both profiles stay watermarked engineering-only until templates arrive. Then user-verified, followed by owner uploader acceptance without placing an order. |

## 2. Revised implementation sequence and effort estimate

Estimates below are additional engineering effort relative to the original contract, not calendar promises. Quota pauses and external repository/template/owner checks are excluded.

1. M1a close: add approximately 1-2 engineering days for spread-sized benchmark, revised audit expectations, Windows/Mac runs and CI evidence. M1a remains OPEN until P-01 and K-06 are independently verified. The current schema assumes cover width equals twice the page width; a spread benchmark must explicitly fix this semantic boundary rather than silently double the cover.
2. M1b: add approximately 3-5 days for spread/page distinction and migration, PDFium/parity, filesystem fixtures, centre/bleed rules, template/table profile model and bundled-runtime design. Existing importer preparation is preserved. Persistence, portable reopen and end-to-end auto-grid remain required.
3. M2: add approximately 1-2 days for native arm64 dependency feasibility and CPU measurements. If a dependency cannot meet native arm64 support, report the precise incompatibility and propose a reviewed replacement.
4. M3: add approximately 0.5-1 day for cross-platform evaluation reproducibility; human recall/quality thresholds are unchanged.
5. M4: add approximately 3-5 days for Mac native import/shortcuts/associations, P3 preview and two-platform interaction regressions.
6. M5: add approximately 1-2 days for two-platform rendering/editing regression coverage. Feature scope and quality gates stay unchanged.
7. M6: add approximately 1-2 days for dual OS credential storage, AI opt-in and recovery tests. Cloud budget remains zero until explicit authorization.
8. M7: add approximately 4-7 days for two self-contained installers, SBOM/license validation, clean-machine offline startup and final performance/recovery checks. This is the largest packaging increase.

Cross-platform CI runs throughout; actual minutes will be reported from Actions rather than estimated as consumed. Current remotely measured CI minutes: unavailable, because no connected repository/run exists.

## 3. Immediate next steps and dependencies

1. Implement and verify explicit spread export semantics, then run the new Windows largest-case benchmark and post separate metrics, audit and <=1MB previews.
2. Prepare native arm64 Mac CI with an architecture assertion; connect the owner-provided private repository and execute all required jobs. Section 1 of 12 says Linux core/boundary only, while P-01 closure explicitly requests the 40-page export/audit on all three. Conservatively retain the Linux export smoke test for M1a closure unless Claude resolves that wording differently.
3. Finish M1b safety and PDFium migration without advancing main M2 work while M1a is unresolved.
4. Await `canvera_templates/` for exact orientation, first/last pages, pixels, upload limits, cover variant and spine table. The folder was not present in the current listing.

Owner action needed: create/connect the private GitHub repository and provide its URL. No WSL, payment method, signing purchase or print order is requested.

STATUS: READY_FOR_REVIEW
