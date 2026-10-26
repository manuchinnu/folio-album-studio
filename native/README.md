# Rust spike

`folio-core` starts the agreed Rust migration with exact Canvera pixel geometry,
fractional fold coordinates, mixed single/spread interior units, the provisional
80-page product cap, and an explicit block on production covers without a
verified spine table.

Run `cargo +1.85.1 test --manifest-path native/Cargo.toml --workspace` from the
repository root. CI runs this before the existing reference implementation.

This is an initial geometry module, not the completed canonical project schema
or a renderer. It is not yet wired into the application. The Python reference
and independent audit remain authoritative validation dependencies until the
measured Rust spike gates pass. No runtime parity or lab acceptance is claimed.

Inspect the exact provisional upload units with:
`cargo +1.85.1 run --manifest-path native/Cargo.toml -p folio-cli -- inspect-profile 12x15L 39`
The JSON is diagnostic output, not an independent audit expectations file.
