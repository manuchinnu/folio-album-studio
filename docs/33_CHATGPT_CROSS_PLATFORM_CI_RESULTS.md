# Cross-platform CI results

Claude: the initial repository CI passed on Linux, Windows and macOS for commit ff340092cb7d557688e1c875b3b0cfcd08ac06cf.

Run: https://github.com/manuchinnu/folio-album-studio/actions/runs/40281734559

All three jobs passed Rust core/CLI tests, reference verification, synthetic fixture generation and the Python reference 40-page export with independent audit. macOS also passed the explicit arm64 architecture assertion. Windows and macOS passed the reference 40-spread benchmark and its memory gate; Linux intentionally omits that benchmark.

Job wall times: Linux 1m29s; Windows 3m48s; macOS 4m40s. These are elapsed job times, not billed minutes or renderer benchmarks. The accompanying GitHub jobs JSON records individual step outcomes. Run artifacts are retained for three days.

This verifies repository/CI operation and the current reference pipeline across these platforms. It does not establish exact mixed Canvera Rust rendering, Rust/Python pixel parity, native dependency feasibility, self-contained packaging or the Rust performance gates. Current reference export geometry remains nominal. Python reference and independent audit are retained; M1a evidence acceptance and main M2 gate remain outstanding.

Next implementation priorities are canonical project/schema integration and actual Rust mixed-unit export, followed by the measured spike evidence. No additional design approval is requested.

STATUS: IMPLEMENTATION_IN_PROGRESS
