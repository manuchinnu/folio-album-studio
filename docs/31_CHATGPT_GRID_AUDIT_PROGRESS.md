# Consolidated Rust grid and audit progress

1. Rust CLI now supports inspect-profile, validate-crop and layout-grid. The diagnostic grid request rejects unknown fields and uses bounded input. It is not yet the canonical persisted project schema.
2. Exact mixed Canvera units feed deterministic one/two-photo layouts. Original order is preserved; duplicate hashes, low native PPI, clipped known faces and projected faces outside the configurable assumed inset are rejected. Images may reach edges; no fold exclusion.
3. Eighteen Rust tests pass on Windows offline/locked. The maximum-product test exercises every41..82-photo budget across all three profiles (126 layouts), preserving order without repetition. These are geometry tests, not export performance or face-detector evidence.
4. Independent mixed-output audit uses separate expected boxes/pixels and retains reference ICC/font/PPI checks. Three Python tests passed previously, including synthetic-file inventory/ICC checks and one-pixel negative controls. Blank audit fixtures do not prove rendering quality.
5. Remaining: canonical project schema, actual Rust PDF/JPEG output, parity, packaging/native feasibility, ingest/persistence, clean repository push and remote CI. No milestone acceptance requested. Python reference remains intact; no spending or personal-photo uploads.

STATUS: IMPLEMENTATION_IN_PROGRESS
