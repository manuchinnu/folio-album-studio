# Rust decoder and composition progress

Claude: commit 1c2000e5475d8e9fb026d3ae2398d38f796a8bfb passes CI on Linux, Windows and macOS:
https://github.com/manuchinnu/folio-album-studio/actions/runs/40435338581

Since report 33, implemented mixed proof geometry with frozen independent 80-page expectations, grid-to-PDF placement matrices/clips, exclusive proof-plan publication, bounded hash-verified source snapshots, and Rust JPEG/PNG decoding with EXIF orientation and retained ICC metadata. The CLI can inspect verified images and emit composition plans. All 27 Rust tests pass, including an independent coordinate oracle for all eight EXIF orientations. CI also checks a generated synthetic PNG and rejection of changed source identity and insufficient byte budget.

The locked dependency inventory records 37 third-party packages with declared license/checksum metadata locally. CI retains its own inventory and decoder smoke JSON. This inventory is not a complete application SBOM or license-text bundle.

The preceding run failed because offline all-target metadata needed a crate absent from host-only build downloads. Added a locked fetch before testing; the new three-platform run confirms that fix. No dependency versions were changed by the fix.

Remaining: ICC color conversion and measured parity, HEIC/native feasibility, actual Rust PDF/JPEG rendering, fonts, full canonical Project integration/persistence, packaging and performance evidence. Current Rust output is geometry/inspection JSON, not a rendered album. Python reference and independent audit remain intact. No M1a completion or M2 clearance is claimed. Continue the approved implementation without awaiting another design decision.

STATUS: IMPLEMENTATION_IN_PROGRESS
