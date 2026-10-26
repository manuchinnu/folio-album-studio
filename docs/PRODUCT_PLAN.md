# Folio — Photo Album Studio

Plan version: 1.0 • 8 October 2026 • Author: Codex
Review status: submitted for Claude review; agreement has NOT yet been received.

## 1. Product promise
Turn a personal photo collection into a beautiful, editable, print-aware album without losing the originals. Folio combines a searchable photo library, an intuitive spread editor, reusable layouts, shape masks, nondestructive photo adjustments, and explicit AI editing. The visual direction is a quiet professional creative tool: warm paper, charcoal controls, restrained terracotta accents, large image previews, and precise typography.

Primary workflow: import → organize → choose album dimensions → compose spreads → refine images → review print warnings → save and export.

## 2. Users and success criteria
- Families: a first attractive spread within five minutes; no design terminology required for core tasks.
- Photographers: repeatable templates, original preservation, accurate composition, reliable export.
- Designers: layer ordering, numeric geometry, masks, crop control, typography, undo, reusable documents.
- Acceptance: imported photos survive reload; a saved portable project reopens with all assets and edits; undo/redo works for all document mutations; exported pages match composition; failed AI jobs never replace originals.
- World-class is a release quality bar, not a claim for the initial implementation. Each phase below has a measurable exit gate.

## 3. Feature scope
### Photo library
Multi-file and drop import; JPEG/PNG/WebP initially; dimensions and file size; search by filename/tags; favorites; used/unused indicators; sort; duplicate detection by content hash; collections; thumbnails; missing/unsupported-file feedback. Originals stay immutable. Large-file limits and storage quota errors must be visible. Later: EXIF/date grouping, HEIC/RAW conversion through a dedicated decoder, virtualized thousands-of-photo browsing, optional connected cloud imports with explicit selection.

### Album and layout editing
Album title; portrait/landscape/square presets; multiple spreads; add/duplicate/reorder/delete spread; free composition; curated one-, two-, three-, four-, and six-image layouts; replace selected frame; drag and resize; numeric position/size; rotate; fit/fill and focal point; zoom; guides and snapping; background color; borders; shadows; text captions; layer list and ordering; lock/hide; undo/redo; keyboard operations. Applying a layout must preserve selected photos predictably and remain undoable. Templates store geometry rather than destructive raster output.

### Masks and image editing
Rectangle, rounded rectangle, circle/ellipse, arch, diamond, and hexagon masks. Nondestructive exposure/brightness, contrast, saturation, grayscale, sepia/warmth, blur, flip, rotation, crop and focal position. Before/after toggle; reset adjustments; presets. Later: curves, white balance, local brush adjustments, perspective, red-eye, lens profiles, feathered masks, vector masks, color-managed export. UI adjustments and export use one rendering model.

### AI photo editing
A clearly marked AI panel accepts instructions and a selected source photo. Operations include object removal, background replacement/removal, restoration, relighting, expansion, and creative transformations according to provider capability. User sees what will be sent before invoking a remote provider. A provider adapter owns request validation, authentication, cancellation, retry, status, and error translation. Keys are server-side only. Results become new library assets with provenance and original references; explicit accept/reject controls govern replacement. No fake success or substitution of ordinary filters for AI. Initial release can use a configured compatible endpoint; unavailable provider is honestly shown as unconfigured. Production gate requires a real configured provider integration and end-to-end success/failure tests.

### Saving, export and recovery
Autosave to IndexedDB, debounced writes, visible save status, schema version, migration validation and recovery. Download/upload a portable JSON project with embedded assets for the first vertical slice; later ZIP packages with binary originals, manifest, checksums, and previews. Export selected spread as PNG/JPEG at selectable resolution; export all spreads; print/PDF workflow with explicit limits. Preflight checks effective DPI, edge overflow, empty frames and missing images. Later production PDF: trim, bleed, ICC profiles, font embedding, printer-specific profiles and proofing; browser print must not be advertised as press-ready PDF.

## 4. Architecture
Initial browser-first application: ES modules, CSS, Canvas 2D export, DOM/SVG composition, IndexedDB persistence, and a small local development server. No framework dependency is required for this core. This keeps the first build runnable on the available machine and deployable as a private static Site. Modules: document model and commands; library/assets; editor UI; renderer/export; persistence; AI adapter; validation. Split modules as responsibilities stabilize.

Document schema: {schemaVersion, id, title, createdAt, updatedAt, page:{width,height,unit,bleed,safeMargin}, assets:[{id,name,mime,width,height,data,tags,favorite,sourceAssetId,provenance}], spreads:[{id,name,background,elements:[{id,type,assetId,x,y,w,h,rotation,opacity,mask,fit,focalX,focalY,adjustments,text,fontSize,color,z,locked}]}]}.

Commands mutate a serializable document; selection and zoom are session state. History records document changes with gesture coalescing. Asset binary data should be normalized out of history in the scalability phase. IDs remain stable through save/reopen. Rendering uses asset resolution plus a shared adjustment/mask specification. Tests validate invariants and integration behavior, not merely function mirrors.

Production evolution: typed document contracts; tiled/worker rendering; blob store; thumbnail workers; secure AI job service with signed uploads and rate limits; optional authenticated cloud sync with revisions and conflict handling. Collaboration and billing follow a proven single-user core rather than blocking it.

## 5. Delivery phases and exit gates
### P0 — Plan, review, foundations
Publish this plan and a review request in the user-specified Drive folder. Claude responds with explicit status and blocking/nonblocking findings. Address findings, publish a revision and decision log, and obtain explicit agreement. Development can proceed in parallel because the user explicitly requested implementation now. Gate: genuine Claude agreement; never infer silence as approval.

### P1 — Usable composition vertical slice
Build the complete three-panel editor, photo import/library/search/favorites, templates, masks, core adjustments, text, spread management, geometry controls, undo/redo, autosave, portable project save/reopen, image export and preflight. Seed with generated illustrative demo assets rather than uploading personal photos into hosting. Gate: browser-driven import→compose→save→reload→export verification, document-model tests, responsive and keyboard checks, no console errors in exercised flows.

### P2 — Real AI and robust media pipeline
Implement provider interface and a working secured provider path; job states and cancel/retry; result comparison and preservation; validate file limits and hostile document inputs; storage quota recovery; binary assets outside history; batch import progress. Gate: genuine successful AI edit with configured provider plus failure/cancellation coverage; no secret in bundle, logs, or project files. External provider credentials/account capabilities are a dependency, not something to fabricate.

### P3 — Professional production
Arbitrary vector masks, advanced image controls, snapping and alignment/distribution, reusable user templates, multi-selection/grouping, calibrated export, PDF pipeline, batch exports, print profiles, performance optimization and accessibility audit. Gate: 50-spread/500-photo benchmark on agreed hardware; no missing/incorrect imagery in exports; print proof reviewed against a named printer specification; WCAG 2.2 AA review for core workflows.

### P4 — Release readiness
Optional secure cloud library and collaboration; onboarding/help; error reporting with consent; migrations/backups; browser matrix; security review; documentation; deployment and recovery runbook. Gate: published acceptance checklist, resolved critical defects, explicit Claude final review and user-visible limitations.

## 6. Quality strategy
Model tests: template bounds, reference integrity, undo/redo branch behavior, spread deletion minimum, serialization round-trip, malformed project rejection and limits. Browser tests: file import, image insertion/replacement, drag/resize, masks/filters, keyboard shortcuts outside inputs, save/reload, export dimensions and image contents, AI unconfigured state and errors. Visual checks at desktop and narrow widths. Accessibility: named controls, visible focus, label relationships, readable contrast, keyboard alternatives to drag. Performance: compressed preview assets, bounded history, debounced autosave, no silent eviction.

## 7. Risks and explicit limitations
- A universal image editor is a multi-phase product; initial implementation must be labeled honestly.
- Browser storage may be cleared; portable backup is prominently available.
- AI availability, pricing and provider limits depend on configured account; no paid jobs run implicitly.
- Color management, PDF bleed and RAW decoding require dedicated validation before professional claims.
- Photo privacy: local imports remain in the browser unless user invokes an identified remote AI/cloud action. Existing workspace originals are untouched.
- The Drive folder is asynchronous communication, not a mechanism to start Claude. A reviewer must read and reply there.

## 8. Review protocol and execution policy
Shared folder: https://drive.google.com/drive/folders/1jJZnKQJCoIXKxyiJrLV8ZZPpW9mk1bQy
Files: FOLIO_PLAN_v1.md; CODEX_TO_CLAUDE_REVIEW_REQUEST.md; CLAUDE_REVIEW*.md; CODEX_REVIEW_RESPONSE*.md; implementation progress reports.
Claude should identify the reviewed version, state AGREED / AGREED WITH CONDITIONS / CHANGES REQUIRED, prioritize findings, and specify acceptance evidence. Codex addresses findings and cites the actual response. Only AGREED, or explicitly closed agreed conditions, satisfies the agreement gate.

Check account usage at milestones. If a relevant remaining allowance falls below 10%, persist exact progress, current blockers, next actions, and review IDs; pause the goal per the user's instruction. A recurring heartbeat checks for replenishment and continues work only when allowance is sufficient. Do not spend reset credits without user instruction. Keep unchanged monitoring quiet. Mark the overall goal complete only after agreed scope and acceptance gates are actually satisfied.
