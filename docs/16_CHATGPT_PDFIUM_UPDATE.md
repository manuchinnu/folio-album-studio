# 16_CHATGPT_PDFIUM_UPDATE

Follow-up to 15, under approved P-03. M1a remains OPEN on remote CI and Mac K-06; this does not claim full M1b completion.

| Item | Status |
|---|---|
| P-03 renderer replacement | Implemented pinned pypdfium2 5.14.0 in a cancellable single-page subprocess; exact requested pixels, embedded sRGB, explicit buffer disposal. Poppler is no longer used by the exporter or installed by CI. Historical 15a evidence is retained unchanged. |
| R-3 Windows K-06 | Revalidated after renderer replacement: 40 spreads, 9071x3671, 01.jpg..40.jpg, correct PDF boxes/fonts/ICC; independent audit passes. |
| R-2 A4 regression | PDFium A4 four-page plus photo cover export and independent audit pass. |
| Tests | All 39 regressions pass with strict TS and schema freshness. Final centreSafeMm=0 handling checked by the four focused cover/centre tests. |
| P-01 / Mac K-06 | Still pending owner repository connection. YAML is ready, not remotely tested. |

## Measured results

PDFium spread run `spread-40-1284ca60`: 99.1596s end-to-end; native sampled peak590,008,320 bytes, Node89,677,824 bytes, combined679,686,144 bytes (~648MiB). Below2GiB. Same20ms sampling limitations as15. Audit:40PDF pages/40JPEG, min embedded300.0044PPI. Preview visually checked with fold/zone overlay only in review image.

A4 `demo-4-b70e68d6`:6.7802s, combined332,161,024 bytes; audit PASS4PDF pages/5JPEG,min299.9678PPI.

Initial raster comparison against historical Poppler, ICC-managed Lab and CIEDE2000 at matched pixel coordinates:
- A4 page:63,687 samples, mean0.09529,p95=0,max39.5655.
- Full spread:102,816 samples, mean0.02465,p95=0,max55.0953.
- Deterministic spatial sampling, no excluded samples; mean passes<=2. This is NOT a guarantee over every pixel. Large isolated differences are explicitly reported; no maximum threshold has been substituted for the agreed mean gate.
- The colour-distance implementation checks a published reference pair (expected2.0425). Full P3, adjustments, masks and cross-platform preview/export fixtures remain due; P-03/M1b is not claimed fully accepted from two synthetic comparisons.

Small files posted:16a_pdfium_metrics.json,16a_pdfium_audit.json,16a_pdfium_jpegs.json,16a_pdfium_preview.jpg,16a_pdfium_parity.json,16a_pdfium_tests.txt.

Implementation follows PDFium's documented raster API: https://pypdfium2.readthedocs.io/en/stable/python_api.html . Self-contained application packaging/SBOM remains M7, runtime design M1b; installing a development wheel is not a finished installer.

Next independent work: filesystem/cloud-placeholder fixtures and persistence. External dependencies remain private repo URL/connection and Canvera templates. No paid API/print/signing operation and no reset credit invoked.

STATUS: READY_FOR_REVIEW
