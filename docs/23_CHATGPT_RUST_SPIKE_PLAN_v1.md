# 23_CHATGPT_RUST_SPIKE_PLAN_v1

| Review 22 item | Response |
|---|---|
| Rust engine and Tauri direction | Agree to the gated direction. Retain the current implementation as a reference until the spike passes. React/TypeScript remains UI-only in the target architecture. |
| Single Rust source of truth | Agree: folio-core, folio-ingest, folio-render, folio-store, folio-cli; generated schema/types. No permanent duplicate preflight. |
| Independent audit | Preserve audit assertions and Python dev-only verifier. Concrete compatibility issue: existing audit geometry/count expectations describe uniform pages or spreads, not the new mixed template structure. Add a parameterized mixed-geometry fixture configuration, without weakening assertions; literally unchanged expectations cannot validate the new geometry. Please confirm this interpretation. |
| Spike count | 40 spreads plus first/last singles is 82 printed pages. Propose using that only as an explicitly oversized stress fixture; ordinary albums remain capped at 80 pages (provisionally 39 spreads plus 2 singles). Actual sheet-count mapping remains pending Canvera. Please confirm. |
| Performance | Accept <=2GiB and <=99s target on same Windows machine; report fixture differences and actual results, not a claimed speedup from language choice. Rust/native dependencies can still exceed memory limits; measure process-tree peak and buffer lifetimes. |
| Packaging and HEIC/ICC/ONNX | Accept as spike gates, not presumed benefits. PDFium, libheif codecs, lcms2 and ONNX may require bundled native libraries; self-contained means no system-installed dependencies, not necessarily literally one file. Pin licences and build provenance. |
| Repository | Actual private repository URL/access still pending. This blocks remote evidence, not local spike work. |

## Numbered spike plan

1. Inventory Rust/compiler/linker availability and current reference outputs. Preserve baseline source and immutable evidence. Create a separate Rust workspace alongside existing code.
2. Implement exact independent template pixel dimensions and provisional singles/spreads structure in folio-core. Track printJoin, layflatType, orientation, verification provenance and unknown cover sheet count. Test odd-width folds, count cap, migrations, assumed subject inset and production-cover block.
3. Build the minimum PDF/JPEG renderer with pinned native dependencies. Prove one mixed synthetic album before the 40-spread stress fixture. Keep the independent audit criteria for boxes, fonts, ICC, PPI and exact pixels.
4. Compare against equivalent Python reference geometry. Emit mean DeltaE2000, count/percentage above 2, and a <=1MB heat-map; investigate non-edge outliers. Do not compare unlike geometry.
5. Measure wall time and process-tree memory on Windows. Package and test with system dependency paths unavailable. Validate HEIC, sRGB/P3 transform, and a licensed small CPU ONNX model.
6. Repeat on macOS arm64 and run Linux/Windows/macOS CI when repository access exists. Record logs, runner architecture, artifact hashes and billed minutes. No paid capacity or reset credits.
7. Submit measured go/no-go report. If a gate fails, propose only the smallest necessary fallback. Retire Python runtime workers only after the spike passes; keep independent Python audits dev-only.
8. Continue M1b persistence/filesystem/end-to-end work in Rust after core contracts are established. Eyes-on-fold preference remains M3. Do not claim M1a closed without CI and spike evidence.

## Effort and current status

Estimate: 3-5 engineering days for the spike if toolchains/dependencies cooperate, with native packaging and missing macOS access the main uncertainties. This is effort, not a calendar completion promise. Tentative migration allowance: +6-10 engineering days in M1b, with a possible 4-7 day packaging reduction in M7, to be revised from measured spike results. No Rust migration or spike result is claimed in this plan.

STATUS: READY_FOR_REVIEW
