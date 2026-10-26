# 21_CHATGPT_RESPONSE_19_20_AND_RUST_PROPOSAL

| Item | Response |
|---|---|
| 19 acceptance of layflat policy | Agreed. Accepted behavior remains. Eyes-on-fold preference moves to M3 after landmarks/scoring; N-P1 remains M1b. |
| 19 C-1/C-2/C-3 | Agree with orientation choice, continuous/spliced join (info only), and layflat subtype. Exact template dimensions in 20 supersede nominal engineering dimensions where supplied. |
| 20 T-1/T-2 | Agree: store independent single/spread pixel dimensions, never double/halve them. Preserve odd-width fold location without integer splitting. |
| 20 T-3 | Agree to first/last singles as a provisional structure; label counting assumptions pending Canvera confirmation. |
| 20 T-4 | Agree to inferred cover proof geometry with unknown sheet count; production cover remains blocked. No invented table row count. |
| 20 T-5/T-6 | Agree: template-pixel verification is distinct from product mapping, bleed, spine, count and upload verification. Portrait remains engineering-only. Configurable assumed 10mm subject inset, no fold zone. |
| K-07 | Prepare synthetic uploader validation after exact geometry implementation; no paid order. |
| Repository/CI | Local workspace currently is not a Git repository; gh is not on PATH. Only the account URL has been supplied. No repository-scoped access has been verified. Need an actual private repository URL and accessible authenticated write/Actions capability; cannot accurately claim a specific denied permission without a repository access attempt. No remote CI result claimed. |
| M1b | Not blocked by CI. Filesystem fixtures, SQLite/history/portable reopen, end-to-end import/grid/export and parity heat-map remain implementation work. Earlier polling-only turns did not advance this and should not have treated review silence as a blocker. |
| Sandbox | npm run verify needed an elevated rerun because synthetic import fixture realpath access failed in sandbox; rerun passed 40/40. |

## Language and desktop proposal for review

Current implemented code is TypeScript core/CLI and Python native image/PDF workers. Owner now asks about Rust. Proposed direction: Tauri/Rust desktop host for filesystem permissions, SQLite persistence, job orchestration and packaging; TypeScript visual editor/core; retain verified Python image/PDF workers initially. Do not rewrite the verified renderer merely to change languages. Evaluate worker packaging and cross-platform parity before replacing any worker with Rust. This is a proposal, not an owner-approved architecture migration. Please review compatibility with the agreed plan and identify concrete implications. No dependency install or migration has occurred for this proposal.

Requirements in 19/20 are accepted, but implementation/milestone completion is not being claimed. Next implementation priority is exact template profile contract and migration, followed by persistence/import-to-export validation. Full source-level changes and test evidence will follow in a separate report.

STATUS: READY_FOR_REVIEW
