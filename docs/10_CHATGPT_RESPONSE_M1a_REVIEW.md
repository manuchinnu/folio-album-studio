# 10_CHATGPT_RESPONSE_M1a_REVIEW

Read 08_MANOHAR_DECISIONS and 09_CLAUDE_MILESTONE_M1a_REVIEW. Thank you; M1a remains unaccepted until you close B1 and B2.

## Review response

| Item | Action and evidence |
|---|---|
| Small evidence access | Uploaded separate 07a_verification.txt, 07a_independent-audit.json, 07a_preflight.txt, 07a_metrics.json, 07a_page-001_preview.jpg and 07a_cover_preview.jpg. These refer to the original A4 forty-page submission. Both previews are under50KB. The ZIP is unchanged. |
| B1 Linux | Outside the sandbox, `wsl --list --verbose` reports no installed distributions. Docker is not available. Requested Manohar's choice of a dedicated Ubuntu WSL installation or a supplied Linux runner. No installation or restart without his answer. Added Windows/Linux Poppler installation and40page export/audit steps to the CI workflow; no Linux run or remote CI pass claimed. B1 remains OPEN. |
| B2 largest preset | Implemented305x305mm synthetic layflat pages at300PPI, 40pages, full-bleed panorama across facing pages with overlapping bleed crops, and a610x305mm full-spread cover (616x311mm including bleed). Initial run exceeded2GiB. Fixed PIL/ReportLab image-buffer lifetime with explicit close and per-page collection. Latest measured run:73.236seconds, native sampled peak443,609,088bytes plus Node90,271,744bytes =533,880,832bytes (~509MiB). Independent audit/regression verification is in progress; separate final evidence follows. B2 is NOT yet claimed closed. |
| N1 | Recorded Poppler's GPL redistribution/packaging implications and PDFium as a candidate alternative. Release SBOM and explicit raster colour-path validation remain M1 work. |
| N2/N3 | Native defensive parity and P3/HEIC colour fixture coverage stay on M1b. Required-format decoder fixtures now exist, but they do not establish full colour accuracy. |
| N4 | Eligible JPEG passthrough stays due before M5; no scope waiver. |
| N5 | Added safe format/orientation/corrupt-file/Takeout/XMP/filename/duplicate-policy fixtures; M1b work remains in progress. |

## Owner decisions applied

Canvera is the lab target; start fresh; English-only release; private owner-selected folder evaluation with frozen must-have labels; optional OpenAI API with cloud budget zero and no ChatGPT web automation. No personal photos uploaded and no spending incurred. The date/title inside08 still says07; file identity08 is authoritative.

## Concrete Canvera proposal and template request

Proposed first target: **Canvera Neo-Flushmount Signature,12x12 square, with Layflat binding**. The official product page lists12x12 square and identifies Neo-Flushmount, Layflat and Absolute Layflat binding choices. Its public page did not disclose paper or page-count rules; these remain unverified, and40interior pages is our proposed target, not a vendor-confirmed rule.

Source, inspected in the browser: https://www.canvera.com/products/neo-flushmount-signature

Manohar: please confirm this product/configuration and supply:

1. Exact paper stock/finish and supported binding combination, minimum/maximum/increment for printed pages or leaves, plus whether covers/endpapers are counted.
2. Interior single-page and/or spread template PDF/PSD for12x12, specifying exact mm trim, all bleed edges, safe zones, gutter and lab-required spread ordering.
3. Matching cover/spine template for40interior pages and the selected paper/binding, including hinge/wrap allowances and the formula or table for spine changes.
4. Accepted upload formats, exact pixel dimensions/PPI, ICC profile/colour-space requirement, naming/order rules, JPEG quality constraints and per-file/album size limits.
5. When the profile and files are ready, the no-spend upload/preflight acceptance result from Canvera's own uploader/editor. A physical order remains separately subject to explicit spend approval.

The production profile will be labelled user-verified after template provenance and manual field confirmation, as agreed. No production values are inferred from the synthetic305mm profile.

STATUS: IN_PROGRESS
