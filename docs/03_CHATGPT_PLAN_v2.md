# 03_CHATGPT_PLAN_v2 — Folio Photo Album Tool

Author: ChatGPT/Codex. Reviewer and go-ahead authority: Claude. Owner and final arbiter: Manohar S.

This is the complete plan submitted under 00_PROTOCOL and 01_CLAUDE_REQUIREMENTS_BRIEF. It supersedes the preliminary FOLIO_PLAN_v1.md and unnumbered review request. Those preliminary files predated the brief and are retained as history, not the implementation contract. Initial untested browser scaffolding was written under the user's first instruction to begin immediately; building stopped when the user introduced the GO_AHEAD gate. That scaffold is not a completed milestone and its pixel geometry, data-URI assets, bounded snapshot history and browser-print assumptions will not be carried into the production core.

No implementation, installation for implementation, hosting deployment or production claim will proceed until Claude posts GO_AHEAD in this folder. Planning, research, review responses and continuity records continue. Poll cadence: five minutes, quiet when unchanged. Revised plans will be complete and open with an item-by-item Change Log; disputed items after three rounds go to Manohar under the protocol.


## Change Log — CLAUDE_REVIEW_v1.md

The review of the preliminary FOLIO_PLAN_v1.md crossed with the later 02_CHATGPT_PLAN_v1. This complete v2 supersedes both. Numbered naming follows Manohar's latest request and 00_PROTOCOL; this is the full v2 requested by Claude. No approval is presumed.

| Finding | Disposition | Resolution |
|---|---|---|
| R1-01 | Accepted | Automatic folder/settings/fixed-page-budget draft is the primary product (§1–2,14). |
| R1-02 | Accepted | Local measured quality, faces/eyes, embeddings, dedupe, events, clock correction, budget optimization and reasons (§5). |
| R1-03 | Accepted | Canonical mm geometry, gutter/binding semantics, protected crops, native DPI repair and seeded pacing (§4,6,13). |
| R1-04 | Accepted with modification | PDF/JPEG/cover/preflight in M1; native ReportLab+pikepdf adapter rather than browser pdf-lib, avoiding canvas limits (§3,9,14). |
| R1-05 | Accepted | Reference-only manifest/history; SQLite and immutable binaries, streaming ZIP64, migrations from M1 (§4,10). |
| R1-06 | Accepted | HEIC/HEIF and EXIF move explicitly into M1, with recursive native/FSA/webkitdirectory paths and reselection tests (§2,14). |
| R1-07 | Accepted with modification | Bounded native processes replace web workers; virtualized proxies start in M1, full budgets and 3,000-photo tests follow early (§3,12–14). |
| R1-08 | Accepted | Strict TypeScript UI-independent core from first approved code; Python only injected media/ML/solver/export adapters; Node tests and generated boundary schemas (§3–4). |
| R1-09 | Accepted with modification | M1 gains baseline quality, near-duplicate grouping, time clusters and budgeted selection. M2/M3 isolate analysis/quality risks before full editor; AI remains after print (§14). |
| R1-10 | Accepted | Annotated varied fixtures, real-like/licensed images, independent PDF/golden checks and per-draft quality metrics (§13). Drawn landscapes are not curation evidence. |
| R1-11 | Accepted | M1 external subgate is a successful chosen-lab upload/soft-proof validator or physical proof with explicit spend approval (§9,14). |
| R1-12 | Accepted | Local keychain adapter, cloud off/zero budget, 512–1024px inputs, project consent and itemized album quote/cap (§11). |
| R1-13 | Accepted | Complete 17-section plan, all-MUST traceability, measurable milestones, questions/defaults (§1–17). |
| R1-14 | Accepted with modification | All building stays paused under Manohar's later instruction. Existing tree is disclosed below; requested code/test/schema evidence is deferred to M1 after GO_AHEAD. |
| R1-15 | Accepted | Parametric 1–9 photos/page, full-spread imagery with gutter protection, breathing spreads (§6). |
| R1-16 | Accepted | Auto-enhance, white balance/tint, straighten, before/after, presets (§7). |
| R1-17 | Accepted | One-action ranked better/similar replacement (§7). |
| R1-18 | Accepted with modification | Reversible low-value filtering; missing EXIF and low resolution alone do not justify discarding old family photos (§5). |
| R1-19 | Accepted | ARCHITECTURE.md, DECISIONS.md, STATUS.md exist as planning records and update every milestone (§10,14). |
| R1-20 | Accepted | Explicit benchmark dimensions and evidence-before-superiority policy (§1). |
| R1-21 | Accepted | HarfBuzz/Pango, licensed Noto fonts, glyph checks and Indic fixtures (§8, M7). |

### Existing evidence and user gate

Read-only current inventory: package.json; .gitignore; .openai/hosting.json; dist/{index.html,style.css,model.js,storage.js,render.js,demo.js}; ARCHITECTURE.md; DECISIONS.md; STATUS.md; docs/{PRODUCT_PLAN.md,CODEX_TO_CLAUDE_REVIEW_REQUEST.md,02_CHATGPT_PLAN_v1.md,PROGRESS.md}, plus this v2. There is no core/geometry implementation, preset test suite or TypeScript schema implementation yet. The preliminary scaffold is incomplete and untested, with no app entrypoint/server. No working application or tests are claimed. Creating the requested implementation evidence now would violate Manohar's explicit no-building-before-GO_AHEAD instruction; it will accompany M1 instead.

## 1. Vision, users, boundaries and benchmark targets

Pick a folder, choose page size/count/lab/style, and receive a beautiful, story-ordered, print-correct album with an explainable selection and simple refinement. Families need reliable automatic storytelling; photographers need speed and alternatives; designers need predictable geometry, masks, typography and print control. Originals remain immutable.

The initial product is a local-first Windows application with a browser UI and local native processing service; a packaged desktop shell and CLI consume the same core. The existing workspace's photos remain local. Cloud hosting is not the production architecture for large personal libraries. The private Site registered before the protocol stays unpublished; a later hosted demo would require an accepted scope change and would contain only nonpersonal fixtures.

Benchmark targets, not unverified superiority claims: match SmartAlbums on fast spread relayout and drag-swap; match Lightroom Book on measured print geometry and preflight; match Photos book workflows on automatic event coverage; match Mixbook/Shutterfly on a three-step start. Aim to exceed these comparison dimensions with transparent pick/drop reasons, editable lab specifications, offline operation and reproducible seeded drafts. A timed human comparison is a release evaluation, not marketing evidence already obtained.

Non-goals for this release: payments, ordering on the user's behalf, social publishing, multi-user live editing, facial identity lookup, a Photoshop replacement, and universal RAW development. RAW embedded-preview support is included; full RAW editing and collaborative cloud sync are later proposals. Generic press-ready PDF/X certification is not claimed unless a target lab requires it and the resulting file passes that validator.

## 2. End-to-end flow

The three-step happy path is: (1) Pick folder, recursively; (2) Choose size, pages, orientation, binding, lab/product/paper and style; (3) Make my album. Optional title and dates never block creation.

Before expensive analysis, validate lab product compatibility and page rules. Display interior pages separately from facing spreads and physical sheets/leaves. A page is one printed side; a reader spread is the two facing pages, with first/last single-page handling; sheets and leaves depend on binding/manufacturing and are computed only through the selected product's rule. Covers/endpapers are explicitly included or excluded by the profile, never silently counted.

Folder enumeration yields thumbnails and progress quickly. Analysis is progressive; an early draft is labeled provisional until dedupe, face and print constraints pass. A clock-skew suggestion appears only when cross-camera evidence supports it. The user can ignore it; the saved timestamps retain both original and corrected values. The final draft opens in a spread editor with chapter navigator and ranked unused/alternate filmstrip. A Review step exposes rejected photos, required-event coverage, people balance and print issues. Export writes interiors, cover, preflight report and project package to a user-chosen folder. Ordering remains manual.

Native folder dialogs are the default. Browser import prefers showDirectoryPicker with a tested recursive webkitdirectory fallback. Copy FileList entries into the queue before resetting the input; test first selection and reselection. Stream bytes to staging with validated relative paths, never base64. Formats: JPEG, PNG, WebP and HEIC/HEIF are required; detect by decoded content as well as extension. Use native libheif decoding through the media adapter. Read CR2/NEF/ARW/DNG embedded previews via ExifTool/rawpy where available; mark preview resolution explicitly. If none exists, report unsupported RAW preview rather than silently dropping it. Respect all eight EXIF orientation values before scoring and store normalized coordinates. Read capture time, offset/timezone when present, camera model/serial when available, GPS, pixel dimensions and ICC profile. Missing values stay missing; filesystem time is only a labeled fallback.

## 3. Architecture and stack

```text
React/TypeScript UI       Node CLI / later Electron shell
          +---- typed commands/jobs ----+
                        |
              @folio/core (TypeScript)
geometry / curation / layout / preflight / history / render plan
          |             |                  |
 persistence port   analysis/solver port   export/provider port
 SQLite + blobs       bounded Python native worker processes
                      libvips/libheif, OpenCV, MediaPipe, ONNX
                      CP-SAT, ReportLab+pikepdf, LittleCMS
```

Strict TypeScript from the first approved implementation commit. @folio/core has no DOM, React, filesystem, HTTP or subprocess dependencies. It owns canonical schemas, mm geometry, curation policy, parametric layout, hard validation, preflight decisions and the display list. Effects use typed ports; core is tested directly in Node and used unchanged by CLI and desktop shell. Generated JSON Schema validates the native boundary with strict Pydantic models; no second hand-maintained domain model exists in Python.

Python adapters perform expensive image kernels, ML inference, constraint solving, font shaping and PDF/image writing. Core sends the exact objective/constraints to CP-SAT and independently validates returned selections. Native rendering consumes core's explicit mm display list, crop transforms and font runs; it does not choose layout or reinterpret gutters. This split accepts Claude's typed-core requirement while using mature native imaging and print libraries for throughput and packaging.

React/TypeScript manages the editor's synchronized panels, filmstrip, focus and job states. A Node loopback host owns the SQLite/blob repository, root grants and bounded subprocess jobs; framed stdin/stdout connects Python workers without a second exposed HTTP server. CLI imports the same core with the same adapters. Electron later supplies a thin native-dialog/launch shell. libvips is primary demand-driven image processing, with bounded Pillow fallback; ExifTool reads metadata; MediaPipe/OpenCV/ONNX provide local signals. No full-album pixel buffer is permitted. Library choice is informed by libvips documentation (https://www.libvips.org/) and verified by our own benchmarks.

The host binds only 127.0.0.1, validates Host/Origin, uses a per-session capability token and resolves opaque asset IDs only inside user-authorized roots. Commands carry revision preconditions; jobs carry input/model hashes. The renderer is sandboxed and imports cannot execute code. Pin dependency/model versions, DLLs, hashes and licenses after M1/M2 compatibility checks; bundle required models for offline use. No production cloud hosting is needed.

Packages: packages/core/{schema,geometry,curation,layout,commands,preflight,render-plan,ports}; apps/{editor,local-host,cli}; native/folio_workers/{media,analysis,solver,pdf,jpeg,providers}; fixtures; tests; docs. Node tests cover core, pytest covers adapters, contract tests cover the boundary and browser tests cover workflows. Geometry and curation policy are never reimplemented in the shell.

## 4. Data model and invariants

Canonical models use strict TypeScript discriminated unions, branded Mm/Pixels/PageIndex types, readonly records and runtime-validated payloads from M1. Reject nonfinite geometry, dangling IDs, invalid crops and unsupported schema versions. Native adapters consume generated schemas. Preview pixels never replace authoritative mm coordinates.

Project: UUID, schemaVersion, appVersion, revision, created/modified timestamps, title/date labels, seed, root grants, assets, analysisVersion, interior page count, trimMm, orientation, binding, style, immutable lab-profile snapshot/hash, constraints, spreads, cover, command journal and consent records.

Photo: content SHA-256, source-relative path, immutable blob reference, MIME, byte size, normalized pixel dimensions, original EXIF orientation and metadata, ICC hash, thumbnail/proxy keys, missing-file state, override {normal/mustInclude/exclude/prefer}, tags, derivative provenance. No base64/data URI representation in projects, API payloads or history.

Analysis: photo hash + algorithm/model version, face rectangles/landmarks/confidence, eye/expression signals, subject/saliency regions, technical score components and normalization, embedding blob, pHash, shot type, palette, low-value flags, analysis status and explanation records. Face cluster IDs are local anonymous IDs with optional user labels, never inferred real-world identities.

EventCluster: ID, member IDs, corrected-time extent, optional GPS centroid, embedding centroid, significance, user split/merge overrides, title, hero candidates and analysis version. DuplicateGroup: representative, members, burst/near/exact relation, pairwise evidence and ranked alternates. PersonCluster: embedding references, confidence, user merge/split, inclusion/maximum-share constraints.

Spread: stable ID, page IDs, chapter/event assignment, layout family/seed, background, lock state, slots, text and layer ordering. Slot: photo ID, page-side/spread anchor, rectangle in mm, normalized original crop, rotation, mask/vector path, opacity, adjustment stack, protected regions, derived effective DPI and lock state. Crop/rotation coordinates always refer to orientation-normalized original dimensions.

LabPreset: versioned JSON described in §9. Job: id, stage, progress, cancellation token, input hashes/revision, checkpoints, errors and output refs. PreflightIssue: code, severity, object/page, measured/required values and suggested fixes. Every reference resolves before export. A missing analysis is not a passing analysis; final-draft status requires all mandatory signals or an explicit unresolved-state report.

## 5. Curation pipeline: algorithms, execution and cost

All required curation runs locally without cloud APIs or usage charges. The installer includes licensed, hash-pinned model assets; offline operation after installation is tested. Model downloads are installation work, never unannounced photo uploads. Optional aesthetic/vision features have separate gates.

Stage A — enumerate and fingerprint. Stream SHA-256, extract metadata, normalize orientation, generate 256px thumbnails and 1024px analysis proxies. Cache by content hash and algorithm version; skip exact duplicate decoding. Reject decompression bombs by dimension/pixel budgets and report corrupt files individually. Enumerate without following symlinks outside the granted root.

Stage B — measure technical quality. On detected faces or the salient subject ROI, compute multiscale Laplacian/Tenengrad sharpness; separately measure clipped highlight/shadow fractions, robust high-frequency residual noise in low-gradient regions, edge-direction anisotropy and spectral attenuation for motion blur, and native resolution. Normalize per image scale and compare within burst/event cohorts. Keep raw measures, calibrated scores and confidence. A large blurred background must not downgrade a sharp face. Technical score is a configurable weighted combination; starting weights: subject sharpness .35, exposure .20, noise .15, motion .15, usable resolution .15, calibrated on annotated fixtures before acceptance.

Stage C — human signals. MediaPipe Face Landmarker supplies landmarks and blendshape signals; eyeBlinkLeft/right plus eye-aspect ratios produce open/closed/unknown estimates. Face size and detection confidence determine the principal subject unless a user identifies key people. Expression is a weak preference signal, not an inference of mood or attractiveness. For comparable burst siblings, a confidently open-eyed main-subject frame outranks a closed-eyed one before aesthetic tie-breaking. Unknown/occluded eyes remain unknown and trigger an alternate-review cue. This gives a testable guarantee on annotated detected subjects; no model can honestly guarantee every real-world face is found. Reference: https://ai.google.dev/edge/mediapipe/solutions/vision/face_landmarker and its documented blendshapes.

Stage D — duplicate/burst groups. Exact SHA matches collapse. A 64-bit pHash candidate index and temporal neighborhoods generate near-pairs; a local semantic embedding plus aligned visual comparison verifies similarity so two different photos of the same place are not conflated. Use OpenCV Zoo MobileNet image-classification penultimate-layer embeddings exported to ONNX as the baseline compact local embedding; pin the model and its license. Burst links use corrected time and camera identity. Initial pHash Hamming ≤8 is a candidate cutoff, not a final truth; final thresholds are calibrated against positive/negative fixtures. Avoid transitive chain overmerging by requiring representative similarity and splitting inconsistent connected components. Pick one representative; keep every sibling as a ranked alternate. After layout, audit every selected pair in candidate groups; violations re-enter selection. Zero measured near-duplicates is required for an accepted automatic draft.

Stage E — time and story. Match highly similar cross-camera moments, estimate a robust median time offset with RANSAC/outlier rejection, and offer per-device/model offsets with sample evidence and confidence. Never auto-shift with fewer than three corroborating pairs or ambiguous event alignment. Build pairwise normalized distances from corrected time, GPS distance and embedding cosine distance; initial weights .45/.25/.30. Renormalize over only jointly available axes; without GPS use .60 time/.40 semantic; without time use available semantic/GPS; if neither time nor GPS exists cluster visually and show uncertain chronology. Run DBSCAN with calibrated epsilon and min_samples, retaining isolated meaningful photos as candidate singleton events. A significant event defaults to a cluster of at least three nonduplicate viable photos, or any event with a must-include; users can mark any event significant, merge or split clusters. Significance rules are visible and frozen for regression, not adjusted to hide failures.

Stage F — budgeted selection. Style defines a target occupancy range: minimal 1–2 photos/page, classic 2–3, collage 3–5; title/end pages and locked slots subtract from the budget. Allocate at least one eligible representative per significant event, then allocate remaining slots by event size/quality using diminishing returns. Optimize a binary selection problem over a bounded candidate shortlist (OR-Tools CP-SAT, fixed seed, time bound), with hard constraints for exclude/mustInclude, one per duplicate group, event coverage, usable resolution and explicit people rules. Soft objective combines technical score, weak expression/aesthetic preference, people-coverage balance, shot-type diversity, hero value and chronology. Default people policy penalizes exceeding 35% of person-bearing selected slots for one person when alternatives exist; no automatic demographic inference. User rules override this soft balance.

An infeasible budget never silently drops a must-include or significant event: show the conflicting rules and propose more pages, fewer constraints or a smaller print format. A deterministic greedy allocator with repair provides a bounded fallback when the solver times out; it must pass the same hard-constraint validator or return a draft-with-conflicts, never final success. Record picked/dropped reasons: duplicate of X, insufficient postcrop resolution, event quota, excluded by user, better burst sibling, budget tradeoff, plus the score breakdown. Each card offers must-include/exclude/prefer with one action and undo.

SHOULD features included in M2/M3: on-device SFace embeddings for people clustering with merge/split correction; screenshot/document detection from aspect ratios, OCR/text density and EXIF/app hints; recompressed-copy flags require multiple signals, not missing EXIF alone. A learned aesthetic model is deferred from the required fast path until license and CPU benchmarks pass; a local composition/color heuristic is the default and is not labeled a learned predictor. Optional CLIP aesthetic inference is a separately benchmarked M7 enhancement.

## 6. Constraint-based layout design

All physical geometry is mm. Convert to points exactly at the PDF boundary (72/25.4) and to pixels only at preview/export raster boundaries. Presets include A4 portrait 210×297 mm and A4 landscape 297×210 mm with independent orientation assertions; custom sizes retain exact decimals. Bleed, safe zones and gutter are derived from the frozen lab profile. Different inside/outside margins and left/right binding are explicit.

Generate candidates for 1–9 photos per page from parametric layout families: single hero, paired portraits, asymmetric 2+1, modular grid, panorama plus details, contact-sheet collage, title/breathing page. Parameters depend on trim, safe box, gap, image count, aspect ratios and minimum usable area. Choose a photo-to-slot assignment and crop jointly, minimizing crop loss, face/subject loss, gutter conflict, red DPI, imbalance and repeated family. This is a bounded beam search over candidates with a hard validator; not fixed pixel templates.

Expand detected face boxes to conservative head rectangles; preserve all high-confidence faces and salient regions within each mask's visible area, not merely its bounding rectangle. If a fill crop cannot preserve these regions, choose contain/letterbox, a different slot ratio or another layout. A face spanning a center fold is forbidden; crossing-background panoramic imagery is allowed only with protected regions outside the gutter exclusion band. The user can correct protected regions; constraints follow those corrections.

For normalized crop width cw and height ch on W×H source pixels in a w×h mm slot, effective DPI = min(W*cw/(w/25.4), H*ch/(h/25.4)); include rotation and actual transformed visible axes. Upsampling never raises this native-DPI measure. Default green ≥300, amber 200–299, red <200, overridden only by a verified lab rule. Before finalizing, shrink the slot/recompose, reduce crop or swap a higher-resolution photo to eliminate red. If impossible, return an actionable conflict, not a completed album. Selection/layout iterate until event, people, duplicate, face and DPI constraints all pass or the bounded solver reports infeasibility.

Pacing: chronological chapters by corrected time; best feasible hero opens each significant event; no identical layout family on consecutive facing spreads; alternate dense and breathing spreads within the style's occupancy limits. LAB k-means palette similarity, inward gaze when reliably measured and horizon consistency are secondary objectives. Same content, model versions, profile and seed produce identical results. Shuffle changes only the selected spread seed; unrelated/locked spreads remain byte-for-byte unchanged.

## 7. Editor, library, adjustments and masks

Three regions: chapter/spread navigator, central proof canvas, and context inspector; bottom filmstrip ranks unused photos by score with the selected photo's alternates first. Library supports folder/event/person groups, filename/tag/date search, favorites, used/unused, score filters, exclusions and duplicate siblings. Every photo exposes why it was selected or rejected.

Required actions: drag photos between slots to swap; drag unused photo to replace; one-click better/similar alternative; relayout selected spread; increase/decrease photo count; lock spread or individual placement; pan/zoom a crop; rotate/straighten; crop reset; multi-select, alignment and layer order. Frequent actions have shortcuts and visible keyboard equivalents; focusable slot controls and list navigation make drag unnecessary. The UI offers mm numeric geometry for expert editing, with immediate preflight feedback.

Global changes run as a preview transaction: show which unlocked spreads reflow and which constraints fail, then apply as one undoable command. Style/page-count changes preserve locked work. A new page size cannot always preserve a locked spread's physical geometry; default is to block the incompatible change and offer keep old size or explicitly unlock selected spreads. Never silently scale or crop locked work. A page-count decrease below locked pages or required-event capacity is blocked with alternatives.

Unlimited undo/redo means no arbitrary operation-count cap: append reversible metadata commands to the disk journal, checkpoint every 100 commands, page old history from disk. Asset references are reused; no image copies in history. Redo branches after a new command; crash recovery and reopen preserve undo. Finite disk capacity is checked and surfaced; do not delete history automatically. Autosave commits commands transactionally with a visible dirty/saved/error indicator and a ≤1 second idle target.

Per-photo nondestructive adjustments: auto-enhance, exposure, contrast, white balance/tint, saturation/hue, B&W, sharpness, sepia, vignette, straighten and flip. Store an ordered operation graph and use identical core processing for previews/export; compare-before/after and reset are available. Masks are a user requirement despite being COULD in the brief: rectangle, rounded, ellipse, arch, diamond, hexagon, user SVG path with sanitized geometry and feathering. Border/background/shadow parameters are editable; mask coverage participates in crop/preflight. Decorative assets are restrained and optional. Originals are never overwritten.

## 8. Text and captions

Title page, chapter headings and optional captions use licensed bundled fonts, embedded/subset in PDF. All glyph extents, including rotated text, must remain inside safe zones; overset text blocks export. Text layout belongs to the core display list, not browser-specific line breaking. Use HarfBuzz/Pango shaping for Indic text when enabled, with packaged Noto fonts and glyph-coverage checks. Kannada/Hindi/Tamil are a M7 capability with explicit fixture validation; unsupported glyphs fail visibly rather than becoming boxes.

Offline captions use real capture dates and user event labels. Optional AI suggestions process distinct event batches; prompts receive event IDs and allowed facts. Normalize and compare captions for exact and semantic repetition across an album, regenerate within budget or fall back to a unique factual template. Do not invent places; GPS reverse geocoding is optional network access with separate consent and cached provenance. Captions stay editable and suggestions are never silently applied.

## 9. Print, export, preflight and lab profiles

The core produces PDF interiors with correct MediaBox/TrimBox/BleedBox, per-page or per-spread JPEG packages at exact lab pixel dimensions, a separate computed cover/spine file, machine-readable JSON and human-readable preflight, and a re-openable project package. JPEG dimensions follow round(mm/25.4 × targetPPI) unless the lab provides explicit pixel dimensions, which take priority and are validated against the physical profile. Export names encode page/spread sequence and front/back cover without changing user titles into unsafe paths.

PDF generation uses ReportLab for text/vector/image placement and pikepdf to set/verify boxes, image ICCBased color spaces and document OutputIntent. Native-resolution cropped imagery is retained unless effective resolution exceeds 350 PPI, then downsample to the profile's target (normally 300). Do not rasterize full pages into the PDF merely to simplify typography. Transparent masks/adjustments may require localized image rasterization at target resolution; text stays vector. sRGB is the default with embedded profile; source ICC is transformed through LittleCMS, not simply relabeled. CMYK is enabled only for a verified requirement with the actual lab ICC profile; missing profile blocks that export. No browser print dialog qualifies as the print renderer.

Preflight blocks print PDF/JPEG export for: red DPI, text/glyph bounds outside safe zone, head/face inside gutter or clipped by crop/mask, missing/unembeddable font, invalid page count/product size/binding, missing image, uncompleted required analysis, invalid/unknown mandatory lab fields and impossible cover geometry. Warn on amber DPI, close-to-edge subjects, unknown dates, uncertain eyes, and large file sizes before encoding. Enforce max file size after encoding; reduce JPEG quality only within the preset's floor, never reduce pixel dimensions silently. If limits still fail, report that and offer a lab-allowed split. Diagnostic previews/reports may export with a prominent NOT FOR PRINT watermark while blocked; this is distinct from print output.

Proof toggles show trim, bleed, safe zone and gutter; color-managed soft proof is offered only when a lab ICC is present, and display calibration limits are stated. M1 requires a successful chosen-lab soft-proof/upload validator or a small physical proof with Manohar's explicit approval for any spend. Evidence identifies the exact profile and export hashes. Local PDF checks cannot substitute for that lab subgate. A physical proof remains recommended before the first full order. No order is placed automatically.

Lab JSON schema (illustrative keys, not fabricated lab values):

```json
{
  "id": "lab.product.paper.binding", "version": 1,
  "lab": "Printique", "product": "specific SKU", "verification": "unverified",
  "sourceEvidence": [{"url":"official product/spec URL","retrievedAt":"ISO timestamp","sha256":"captured source hash","fields":["trimMm"]}],
  "trimMm": {"width": 0, "height": 0}, "bleedMm": {"top":0,"right":0,"bottom":0,"left":0},
  "safeMm": {"outer":0,"inner":0}, "gutterMm":0,
  "pageRules":{"min":0,"max":0,"multiple":2,"allowedCounts":[],"coversIncluded":false,"endpapersIncluded":false},
  "binding":"perfect-bound", "colorSpace":"sRGB", "iccRef":"profile hash",
  "outputs":{"interior":"pages","formats":["jpeg","pdf"],"ppi":300,"exactPixels":null,"maxBytes":0,"jpegQualityFloor":90},
  "dpi":{"green":300,"redBelow":200},
  "cover":{"formula":"leafThickness","thicknessUnit":"perLeaf","paperThicknessMm":0,"fixedSpineMm":0,"hingeMm":0,"wrapMm":0,"boardMm":0},
  "validFrom":"ISO date", "verifiedBy":"review evidence reference"
}
```

Zeroes above are schema illustrations, rejected for production wherever positive values are required. Actual profiles must carry source evidence for every mandatory value. Spine = physical leaf count × calibrated thickness per leaf + fixed allowance, or an exact published table/formula. If a lab quotes thickness per printed page, use that unit explicitly. Never blindly multiply a two-sided page count by leaf thickness. Perfect-bound, layflat and spiral use distinct cover rules; spiral may have no spine panel, and hinge/punch exclusions remain explicit. Full cover width combines back + front + spine + wrap/bleed/hinge terms only as the product requires.

Verification process: create records for Zoomin, Photojaanic, Canvera, Printo and Printique; obtain current official SKU templates/specs, preserve source URL/date/hash, transcribe into JSON, independently review dimensions/page rules, validate sample exports, and freeze profile snapshots in projects. Check upstream before release and at least every 90 days; update by explicit migration, never silently mutate an existing album. A marketing size list is insufficient evidence for bleed, gutter, file limits or spine. Where official published specifications are absent, keep the record visible but unverified and block lab-ready export. Obtain a lab-issued template through Manohar if needed; do not contact a lab on his behalf without authorization. A generic custom profile enables a clearly labeled proof, not a claim of compatibility with an unverified lab.

Research snapshot, checked 2026-10-08: Zoomin's official photobook page lists product-dependent sizes and page options, but this read did not establish every required technical field (https://www.zoomin.com/photobooks). Photojaanic lists formats/binding choices but not a complete machine-ready production profile in the retrieved page (https://www.photojaanic.com/photo-books). Printo lists A6/A4 and square/wedding products; do not reuse the unrelated catalogue template found on its test subdomain (https://printo.in/categories/photobook). Canvera's complete official specification was not located in the initial search; it remains an explicit evidence dependency. Printique Classic Softcover publishes 20–200 pages, .125-inch cover bleed and approximate inside trimming, but spine varies with page count and paper; those statements alone do not verify every schema field (https://www.printique.com/products/classic-softcover-photo-books/).

Proposed interpretation for Claude: all five lab adapters and verification gates are committed scope; missing external facts will never be invented to pass a review. Five fully verified usable profiles remain a release gate. If some labs do not publish the necessary facts, obtaining their templates is an external dependency and must be escalated rather than declaring that gate complete.

## 10. Persistence, recovery and migrations

Native project directory: manifest.json, project.sqlite, assets/<sha256> originals/derivatives, proxies/, fonts/, lab-snapshot.json, models manifest, export reports. Originals are copied by default; a linked-original mode is explicit and supports relink by hash. Browser transport uses Blob/object URLs only for current proxies, revoked on eviction; no photos in localStorage or data URIs. IndexedDB may cache UI metadata, but SQLite/files are authoritative. A future browser-only adapter uses OPFS/IndexedDB through the same repository interface and is not required to complete this release.

Portable .folio ZIP64 includes manifest, SQLite checkpoint, immutable images, required licensed fonts, lab profile, adjustments and checksums; exclude secrets, external absolute paths, transient caches and model binaries (record model hashes). Stream archive writes to a temporary destination and atomic rename after verification. Import validates path traversal, entry count/uncompressed size, checksums and schema before extracting; never execute embedded content. Missing optional models prompt a verified installation, not cloud fallback.

Every schema change has an explicit n→n+1 migration, fixture, backup and rollback path. Unsupported newer schemas open read-only with export/recovery guidance; stale/corrupt state is never silently replaced with a new project. SQLite WAL transactions save commands and references together; blob files are staged then atomically moved before references commit. Recovery cleans only verified unreferenced temporary files. Autosave failures and full disks are visible. A project saved mid-analysis reopens with resumable jobs keyed by input/model hashes. ARCHITECTURE.md, DECISIONS.md and STATUS.md are maintained at every milestone.

## 11. AI editing, privacy and cost

The full album pipeline works offline; optional cloud AI is never needed to finish. Local ML inference is bundled and distinct from optional generative editing. AI studio supports prompt-based background change/removal, object removal, restoration, relight and expansion when the chosen provider supports them. A provider capability manifest controls available operations; unsupported controls are disabled with a reason.

Use a typed provider port with a Python native adapter interface (capabilities, estimate, submit, poll, cancel, fetch result) and a local credential store backed by the OS keychain. No API key in UI code, project ZIP, logs, URLs or Drive. Project consent identifies provider, tasks, image handling, maximum per-request and per-project spend; consent is revocable. Only selected downscaled 512–1024px images and the minimum necessary prompt/mask are sent. Strip EXIF/GPS by default. Show exactly which image and request will leave the device. No implicit batch cloud requests.

Provider model/pricing/terms are resolved from official current documentation during M6; a concrete adapter and cost-estimation test are required before enabling it. Before submission, show an itemized per-album quote: requests by operation, provider/model unit price and retrieval date, retry allowance, estimated total and worst-case capped total. Reserve budget for each request, settle actual usage, and refuse jobs that could exceed the cap. If current prices cannot be established, require an explicit user cap before enabling cloud operations. Safe default is AI off and cloud budget zero. Network/provider failures preserve originals and allow cancel/retry with idempotency where supported. Results become new immutable derivative assets with provider/model/prompt/time/source metadata. User explicitly accepts a result; rejection keeps the original. Recalculate face/crop/DPI analyses before use. A 1024px output may fail print-DPI gates and must not silently replace a high-resolution source in a large slot. Preview-only AI results stay preview-only unless genuinely sufficient native resolution exists. This is a real constraint under the brief's cloud-size limit, not solved by pretending upscaling adds detail.

AI caption suggestions follow §8. A local provider is supported by the same interface and may process full-resolution originals with explicit choice. Tests use deterministic mock jobs for contract/error/cancellation, but M6 acceptance also needs a real configured provider call authorized by the user; mocked output is never reported as a working integration.

## 12. Performance plan

Reference target: Windows laptop, 4-core/8-thread CPU, 16 GB RAM, SSD, integrated graphics, no discrete GPU; record actual model/OS/runtime in benchmarks. The requested budgets remain targets: cold ingest+required analysis of 1,000 unique 12MP JPEGs ≤300 seconds; final 40-page draft ≤60 seconds after analysis; p95 editor action feedback <100 ms; 40-page export ≤180 seconds; combined service/worker/UI peak resident memory ≤2 GB. HEIC/RAW timing is measured separately and reported, not hidden in the JPEG target. First results appear progressively.

Bound enumeration queue at 64, decode concurrency at 2 initially, model inference at 1–2 bounded sessions, metadata concurrency at 4; tune by measured memory. A 256MB proxy LRU and 256MB native tile cache prevent library-size memory growth. Keep full-resolution buffers only for active decoding/export; normalize/hash once and reuse. Embeddings/face records persist, thumbnails virtualize with ~100 visible items, drag renders at frame cadence and commits once per gesture. Heavy work runs in subprocess workers; UI receives throttled progress events. Keep core models under a benchmarked combined budget by batching stages/unloading unused sessions; no global all-photo tensor.

Reserve stage budgets: 40s enumeration/metadata/hash, 90s proxy/quality/detection, 120s embeddings/landmarks, 50s grouping/persistence, with overlap measured explicitly. These are engineering allocations, not observed timings. M2 performance profiling is a gate before adding more expensive aesthetic inference. If the baseline misses the budgets, optimize/reduce duplicate work or submit an explicit change request; do not silently loosen thresholds or omit required analysis. Streaming export holds one page/tile group and incremental assets, not forty full page rasters; verify the chosen PDF writer's actual buffering in M1 and replace only its adapter if memory grows with total raster size.

## 13. Tests, fixtures and quality metrics

Fixture manifest contains consented or synthetic images with known expected labels: all EXIF orientations; no EXIF; HEIC/HEIF; JPEG/PNG/WebP; RAW preview/no-preview; portrait/landscape/panorama; transparent PNG; low resolution; corrupt/truncated file; exact and near duplicates including visually similar negatives; bursts with open/closed eyes; multiple people/occlusion; two cameras with known skew; screenshots/documents; missing GPS; ICC-tagged images; safe/gutter hazards; multilingual text; full-disk/save interruption. A 3,000-photo unique-content stress corpus measures responsiveness; cloned files alone do not qualify because dedupe would short-circuit work.

Geometry/property tests assert every preset's width/height/orientation, mm↔pt↔px conversions, asymmetric bleed, page/spread/sheet arithmetic, spine units, crop transforms, mask visibility, lock invariants and invalid-count rejection. PDF tests read exported files independently with pikepdf and a second rasterizer, checking exact page boxes within 0.01pt, all fonts embedded, ICC objects/OutputIntent, image transforms and postcrop native PPI, page count and cover extent. Compare rasterized proof to a golden reference at fixed color settings; acceptable pixel tolerances are recorded per platform, not waived globally.

Curation regression counts on every accepted draft: duplicate pairs = 0; annotated faces cropped = 0; faces in gutter = 0; red-DPI slots = 0; significant events represented = 100%; must-includes = 100% when feasible; closed-eye principal selections = 0 when a viable open-eyed sibling exists. Automated detectors are evaluated against annotations for false negatives, not used as the sole truth for their own tests. Re-run after model/threshold changes. Record people distribution, shot mix, explanation correctness and infeasibility reports. Human evaluation on one user-approved trip folder targets ≥80% recall against Manohar's must-have set; no face-photo fixture is uploaded to Drive without separate permission.

Integration tests cover recursive first-click folder ingestion, same folder chosen twice, cancellation, directory permission loss, drag/swap/relayout, global changes around locks, unlimited journal paging, crash/reopen/relink, migrations from every supported schema, project round-trip/checksum/path traversal defense, offline network-disabled creation/export, AI consent and failure paths. Playwright drives UI workflows; core tests run in Node and native-adapter tests via pytest; CLI tests prove UI independence. Screenshot evidence is redacted or uses safe fixtures. Accessibility audit covers focus order, labeled controls, contrast, keyboard equivalents and reduced motion.

## 14. Milestones and objective gates

M0 — Plan agreement. Deliver this full plan, answer Claude's item IDs in subsequent complete versions, resolve or safely defer user decisions, receive written GO_AHEAD. Acceptance: all rubric scores ≥4, all MUST decisions addressed, explicit GO_AHEAD observed. No code building before this gate.

M1 — Walking skeleton, print correctness first. Demo: recursive folder → JPEG/PNG/WebP/HEIC/HEIF with EXIF normalization/metadata → basic technical scoring plus pHash/embedding near-duplicate grouping → baseline time clusters → bounded page-budget selection → simple parametric mm grid with gutter/native-DPI constraints → blocking preflight → PDF/exact-pixel page JPEGs, cover/report and ZIP project reopen. Baseline scoring is measured; it is not the full later curation model. Use safe nonportrait fixtures until face protection lands and label this limitation. Build strict TypeScript contracts/core, Node preset tests, native adapter boundary, virtualized proxies, SQLite/blob store, migrations and disk-backed history now.

M1 engineering gate: first/repeat recursive folder selection, path safety, all orientation and required-format fixtures, correct page rules before analysis, A4 portrait/landscape unit assertions, baseline quality/dedupe/time-cluster expectations, automatic 40-page grid without manual composition, zero red slots or explicit infeasibility, independent PDF boxes/fonts/ICC/native-PPI checks, exact JPEG pixels, ZIP reopen/checksums, 40-page memory profiling and early 3,000-photo library responsiveness. Largest-size export uses native per-page/tiled processing, never an oversized browser spread canvas. Include requested repo tree, core/geometry source, preset tests and TypeScript schema evidence in the milestone report.

M1 external lab subgate: one genuinely verified Indian-lab product profile plus successful lab soft-proof/upload validation, or a small physical proof with explicit Manohar spend approval. A synthetic profile proves engineering only. Missing vendor specifications or validator access keep this subgate open; identify the precise evidence needed. Independent later work may progress after GO_AHEAD, but M1 is not accepted until both subgates pass. Five-lab coverage remains M5.

M2 — Ingest and analysis at scale. Demo 3,000-photo library with advanced incremental scoring and RAW embedded previews, dedupe alternates, local faces/eyes/embeddings, clock-offset suggestions and explainability. Tests: required format fixture matrix, missing-metadata behavior, annotated quality/eye/dedupe cases, first-selection regression, resumable jobs, ≤300s for 1,000 JPEGs and <2GB memory on reference hardware. Any budget miss stays open with measured evidence.

M3 — Automatic storytelling and constraint layout. Demo 40-page generated album with complete significant-event coverage, people/shot balance, hero pacing, parametric face/mask/gutter-safe crops, varied seeded layouts and zero red DPI. Tests: all §13 curation metrics; missing-GPS weight normalization; deterministic replay; impossible budgets reported; final draft ≤60s; shuffled spread does not mutate others.

M4 — Refinement studio. Demo drag-swap/replace, ranked alternates, photo-count/relayout, locks and global reflow transaction, crop pan/zoom, full core adjustments/masks, text/captions, layers, undo/redo and autosave/recovery. Tests: command history paging and crash recovery, locked-spread invariants, export/preview parity, keyboard workflows, p95 feedback <100ms and storage quota errors. A usable album can be completed fully offline at this milestone.

M5 — Print production and lab coverage. Demo full interior PDF/JPEG package plus accurate cover and downloadable preflight for verified Zoomin, Photojaanic, Canvera, Printo and Printique products. Tests: source-backed JSON fields and date/hash evidence; independent geometry/ICC/font/PPI checks; every blocking preflight category; exact naming/pixels/file-size limits; 40-page export ≤180s and <2GB. Missing official vendor specifications remain an external release blocker for that vendor, not invented defaults. Recommend a physical test print; acceptance of production claims requires the selected lab proof evidence.

M6 — Optional AI editing/captions. Demo real provider editing and accept/reject with source preserved, local credential security, image-size/EXIF minimization, project consent/budget, cancellation/retry and derivative re-analysis; caption event grouping and uniqueness. Tests: mock contracts and real authorized provider success/failure, no keys in artifacts, offline pipeline unaffected, no AI result bypasses preflight. If credentials are unavailable, finish adapter/tests and report the real-call acceptance dependency; do not close the gate.

M7 — Packaged release, polish and handoff. Demo one-click local launch, native folder picker, CLI equivalence, portable project recovery, shipped fonts/models, Indic shaping where supported, human must-have evaluation and benchmark comparison. Tests: offline clean-machine installation, 3,000-photo stress, migration matrix, security/accessibility review, license/SBOM inventory, full regression suite and user-approved test print. Claude reviews final evidence and all limitations. Release is not complete with unresolved mandatory gates.

At each milestone post the next NN_CHATGPT_MILESTONE_Mx_REPORT in Drive with run steps, actual results, safe screenshots/sample exports and gaps; wait for Claude's milestone review and resolve blocking findings before declaring it accepted. Maintain ARCHITECTURE.md, DECISIONS.md and STATUS.md locally. Scope changes use numbered CHANGE_REQUEST and need Claude/Manohar acceptance. No calendar completion promise is made before the native tooling, model throughput and lab-evidence risks are measured.

## 15. Risks and mitigations

Lab specifications may be incomplete or require vendor templates: block unsupported lab-ready exports, retain proof mode, escalate evidence needs. Detector errors can miss a face: annotated regression, confidence/unknown states, conservative head bounds and user corrections; never advertise universal detection certainty. CPU targets may conflict with models: benchmark M2 before aesthetic extras, bounded jobs and adapter isolation. Python/native packaging can be large: pinned reproducible runtime, smoke test all DLL/model dependencies on a clean machine. PDF libraries may buffer too much: measure forty-page memory in M1, stream image assets and isolate/export adapter changes. Originals may move: managed-copy default, checksums and relink UI. AI outputs may be low-resolution or inconsistent: derivative provenance, explicit accept and the same DPI/face gates. Local biometric data is sensitive: remains local, can be deleted with analysis reset, never sent to cloud captions automatically. Unlimited history consumes disk: visible disk usage, user-chosen archive/prune only. Complex constraints may be infeasible: explain the minimal conflict rather than conceal dropped events. Earlier builds may contain reusable domain logic: request/read existing source if supplied, audit it before adopting; do not recreate known bugs or assume unavailable source exists.

## 16. MUST traceability

IDs below are assigned by this plan to the brief's requirements; Claude can reference them directly. Each row names the concrete design and acceptance gate.

| Brief requirement | Decision and plan section | Gate |
|---|---|---|
| 2.1 Recursive folder; size/units/count/orientation/binding/lab/style/title/dates | Wizard, native root grants, physical units and product checks (§2,6,9) | M1 |
| 2.2 JPEG/PNG/HEIC/HEIF/WebP | libvips/libheif media adapter, explicit decode errors (§2,3) | M1 |
| 2.3 EXIF orientation/time/camera/GPS/dimensions | Normalize all 8 orientations; preserve nullable originals (§2,4) | M1–2 |
| 2.4 Pages/spreads/sheets and lab page rules | Explicit manufacturing units and pre-analysis validation (§2,9) | M1,5 |
| 2.5 PDF, exact JPEG, cover/spine, report, project | Core export package and ZIP64 persistence (§9,10) | M1,5 |
| 3.1 ROI sharpness/exposure/noise/resolution/motion | Measured calibrated component scores (§5 B) | M2 |
| 3.2 Faces/eyes/size/expression and better burst sibling | Landmarks/blendshapes, confidence and hard sibling priority (§5 C) | M2–3 |
| 3.3 pHash+embedding dedupe, alternates, zero duplicates | Candidate/verify groups and final pair audit (§5 D) | M2–3 |
| 3.4 Time/GPS/visual DBSCAN; missing GPS weights | Availability-normalized distances and visible significance (§5 E) | M3 |
| 3.5 Multi-device clock skew | Evidence-based robust offsets offered per device (§5 E) | M2 |
| 3.6 Budget, events, people, shot types, hero selection | Constrained CP-SAT shortlist and validated fallback (§5 F) | M3 |
| 3.7 Explain picks/drops and must/exclude/prefer | Score/reason records, one-action overrides (§5 F,7) | M2–4 |
| 4.1 mm geometry, correct presets | Single core geometry and per-preset assertions (§6,13) | M1 |
| 4.2 Spread-aware margins/gutter protection | Paired-page geometry and protected-region constraints (§6) | M3 |
| 4.3 Saliency/face-safe cropping/aspect match | Joint assignment/crop with contain or alternate layout (§6) | M3 |
| 4.4 Postcrop DPI traffic lights; zero red auto-draft | Native crop formula, repair or explicit infeasibility (§6,9) | M3,5 |
| 4.5 Parametric constraints, no fixed pixels | Geometry-derived families and hard validator (§6) | M1,3 |
| 4.6 Story order, heroes, varied pacing | Chronological chapters/family exclusion/density schedule (§6) | M3 |
| 5.1 Swap/filmstrip/relayout/count/lock/crop/history/save | Editor commands and disk-backed history (§7,10) | M4 |
| 5.2 Global reflow preserves locks | Transactional preview; block incompatible lock changes (§7) | M4 |
| 5.3 Better/similar alternative | Ranked unused/duplicate alternatives in one action (§7) | M4 |
| 7.1 Five data-driven verified lab presets incl. spine | Field evidence and immutable JSON; fail-closed unknowns (§9) | M5 |
| 7.2 PDF boxes/fonts/native images/ICC/conditional CMYK | Shared render plan, ReportLab+pikepdf+LittleCMS (§9) | M1,5 |
| 7.3 Blocking preflight/report; amber warnings | Explicit blocker list and diagnostic-only previews (§9) | M1,5 |
| 7.4 Trim/bleed/safe/gutter proof view | Shared geometric overlays (§9) | M4–5 |
| 8.1 No localStorage/data URI photos | Immutable files/SQLite; temporary Blob URLs (§4,10) | M1 |
| 8.2 Version/migration/no reset | Backed-up n→n+1 migrations and read-only future schema (§10) | M1,7 |
| 8.3 Reliable directory/file input | Native directory dialog, tested recursive browser fallback (§2,13,14) | M1,7 |
| 8.4 Heavy processing off UI; proxies/streaming | Bounded native jobs and demand-driven export (§3,12) | M1–2,5 |
| 8.5 Responsive 3,000-photo library | Virtualization, disk analysis/cache and stress corpus (§12,13) | M2,7 |
| 8.6 Full offline pipeline; optional AI | Bundled local models, network-disabled acceptance (§5,11,13) | M4,7 |
| 8.7 Secure key; 512–1024 cloud images; consent | OS keychain, project budget/consent, metadata stripping (§11) | M6 |
| 8.8 UI-independent core; thin shell/CLI; stack rationale | Native core with stable adapters and display list (§3) | M1,7 |
| 8.9 Architecture/ADR/status continuity | Required milestone records and checkpoint policy (§10,14,17) | All |
| 9.1 1,000×12MP analysis ≤5m progressive | Reference hardware, stage budgets and profiling (§12) | M2 |
| 9.2 Draft ≤60s; UI <100ms; export ≤3m; RAM ≤2GB | Bounded solver, feedback metrics and memory accounting (§12) | M3–5 |
| 9.3 Required mixed fixtures and 3,000 stress | Annotated diverse and unique-content corpora (§13) | M2,7 |
| 9.4 Preset/PDF/golden/curation automation | Independent assertions and regression pipeline (§13) | M1–7 |
| 9.5 Zero duplicate/cropped face/gutter/red; 100% events | Hard final validator and annotated metrics (§6,13) | M3,7 |
| User image editing/layout/masks/library/AI | Core adjustments, reusable layouts, shape/vector masks, library and real provider path (§7,11) | M4,6 |
| Brief §10 all 17 plan sections and walking skeleton | This document's §1–17; PDF-first M1 | M0–1 |

SHOULD coverage: RAW previews (§2); on-device people clusters and low-value detection (§5); local learned aesthetics deferred with CPU/license reason (§5); harmony/seeded layouts (§6); nondestructive editing/keyboard (§7); captions/fonts/uniqueness (§8); test print (§9); must-have recall (§13). COULD scope: masks promoted to required by user; Indic shaping included conditionally at M7; cloud vision reranking is not needed for base release.

## 17. Questions, safe defaults and execution policy

M1 external evidence default: no paid order without explicit approval. Ask for the selected Indian lab/product and its validator/template when the concrete export is ready; keep the lab subgate open if none is available. The existing general planning authorization is not spend authorization.

Questions for Manohar do not block this plan: which exact lab product/paper is the first real print target; can he supply current lab templates if unpublished; which earlier source repository should be reused; which trip folder and must-have list may be used for private evaluation; which optional AI provider/account and spend ceiling; whether linked originals are preferable to managed copies; and whether Indic text is required for first release.

Safe defaults: Windows local-first; managed original copies; 40 interior pages; classic style; 300PPI target; generative AI and reverse-geocoding off; no paid requests; Latin bundled font family with explicit glyph checking; no personal-photo uploads. Use a clearly labeled generic proof profile while lab evidence is incomplete, never a fictitious verified lab. Unknown spine values prevent a lab-ready cover. Unsupported explicit people/layout constraints produce conflicts, not silent overrides. Physical-print and real-provider evidence remain named dependencies until provided.

Usage policy: check current relevant Codex allowances at milestones and before sustained work. Below 10% remaining, save STATUS.md/PROGRESS.md with exact files, tests, next step and Drive IDs, then pause as the user requested. The five-minute heartbeat may check allowance/review state; substantive work resumes after replenishment only if GO_AHEAD is already recorded. No reset credits are consumed automatically. Preserve the review gate across all restarts and scheduled wakeups. A response marked AGREED without the required GO_AHEAD does not authorize building. Notify only meaningful changes, blockers needing action, or completion. The goal is complete only after the agreed mandatory gates and Claude milestone/final reviews actually pass.

STATUS: READY_FOR_REVIEW
