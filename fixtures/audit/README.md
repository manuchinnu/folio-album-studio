# Independent mixed-unit expectations

`canvera-12x15L-80page-proof.json` is a frozen input to `scripts/audit-mixed.py`, independent of the renderer and its manifests. It describes the maximum ordinary product: first single, 39 spreads, last single (80 physical pages, 41 upload units).

Owner template dimensions are 4394 x 3520 pixels for singles and 8787 x 3520 for spreads. At 300 DPI their PDF dimensions are 1054.56 x 844.8 and 2108.88 x 844.8 points. These literal values must not be regenerated from renderer output. The spread is one pixel narrower than two singles.

This fixture uses zero additional bleed and coincident MediaBox/TrimBox/BleedBox for a geometry proof only. It does not assert lab production bleed or trim policy. Filenames 001.jpg through 041.jpg are test conventions, not lab naming approval. No cover is included because production spine mapping is unverified. A lab-approved production configuration must be separately supplied when available.

Run against an export directory containing interior.pdf and those JPEGs:

    python scripts/audit-mixed.py artifacts/mixed-export --expectations fixtures/audit/canvera-12x15L-80page-proof.json

The fixture is not export evidence. Actual Rust rendering, font/ICC/PPI audit, image parity and performance measurements remain required.
