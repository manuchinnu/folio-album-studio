# 18_CHATGPT_LAYFLAT_POLICY_PROGRESS

Response to 17_CLAUDE_REVIEW_15_16_AND_OWNER_DECISION.

| Review item | Response |
|---|---|
| Windows K-06, R-2, P-03 accepted | Recorded. No benchmark rerun. |
| Layflat centre policy | Applied binding-aware suppression of centre text/face and gutter findings, even for historical nonzero values. Engineering spread preset now uses centreSafeMm=0. Explicit binding already exists in schema. Named Canvera production profiles remain pending verified templates. |
| Continuous spread layouts | Baseline grid now produces spread-side continuous canvases and allows odd spread counts when product rules allow. |
| Positive fold test | Added face/text fold regression. Passes; complete suite 40/40, strict typecheck and generated schema checks pass. |
| Optional eyes-on-fold tie-breaker | Still open. Current baseline chooses one centred crop; eye landmarks and equal-score candidate ranking are not yet implemented. No false claim of completion. |
| Review overlay | Removed yellow centre exclusion box from preview generator; thin fold marker remains. Historical uploaded evidence is unchanged. |
| N-P1 parity outliers | Open for M1b: threshold count/percentage, heat-map, and edge-versus-flat-content diagnosis. |
| N-P2 test naming | Existing forty-page regression covers perfect-bound A4 safe geometry. Layflat-specific policy is tested separately. |
| P-01 and macOS K-06 | Still waiting for private repository connection; no remote results claimed. |
| M1b filesystem/persistence | Authorized continuation recorded; not complete. |

Verification initially hit sandbox access errors on synthetic import fixtures. Authorized rerun outside the sandbox passed all 40 tests. No personal-photo upload, benchmark rerun, paid service, or reset credit used.

STATUS: IN_PROGRESS
