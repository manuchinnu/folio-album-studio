# 07_MANOHAR_DECISIONS — Owner decisions (relayed by Claude)

**Date:** 2026-10-08 · **Applies to:** `05_CHATGPT_PLAN_v3` (agreed contract) and `06_GO_AHEAD`
These are Manohar's answers to the open questions in §17 of the plan. They replace the safe defaults below.

| # | Question | Manohar's decision | What it means for the build |
|---|---|---|---|
| 1 | First real print target | **Canvera** (canvera.com) | Canvera is the M1 external lab subgate and the M5 "one fully verified Indian product". He has named the lab but not the exact product or paper. **Codex:** from Canvera's current official catalogue, propose 1–2 specific photobook products (size, binding, paper, page-count rules) in your next report, with source URLs. Manohar confirms one. Until then, use a clearly labelled synthetic profile. |
| 2 | Can he supply lab templates? | **Yes** | **Codex:** list exactly which Canvera template files and specifications you need (e.g. interior page template PDF/PSD with trim/bleed, cover/spine template for N pages, file-format and size limits, colour profile). Put the list in the M1a report or a short `NN_CHATGPT_QUESTION`. The profile built from them is labelled *user-verified* under the C-05 workflow. |
| 3 | Reuse earlier source (Mémoire etc.)? | **Start fresh** | Don't request or port earlier code. Lessons are already captured in `01_CLAUDE_REQUIREMENTS_BRIEF` §8. |
| 4 | Folder for the C-02 must-have evaluation | **Any folder** | Manohar approves using any of his own photo folders for the private local evaluation. The data stays local and nothing is uploaded, per the contract. **Codex:** at M3, provide a simple way for him to point at a folder and mark his must-have set (e.g. a quick picker that freezes the list before scoring). The ≥ 80% recall gate is unchanged. |
| 5 | AI provider and spend ceiling | **ChatGPT (he has the Plus plan)** | **Important:** a ChatGPT Plus subscription does **not** include OpenAI API access. API usage is billed separately, pay-as-you-go, through platform.openai.com. So: provider = **OpenAI API** for the optional M6 features (captions and generative edits). **No API spend is authorised yet** and the cloud budget stays at zero. Manohar will set a cap if and when he creates an API key. Never automate the ChatGPT web app as a substitute for the API. The full album pipeline must still work fully offline (unchanged). |
| 6 | Indic text in first release | **No — English only** | Kannada/Hindi/Tamil shaping (§8, M7) is **descoped from this release**. Ship Latin fonts with glyph-coverage checks, keep text handling Unicode-safe, and drop the Indic fixtures. It is a possible future change request. |

## Updated safe defaults

- Lab target: Canvera, synthetic profile until the template arrives.
- Linked originals, Travel theme, 40 interior pages, 300 PPI.
- AI off, cloud budget 0.
- English only.

## Not changed

Everything else in `05_CHATGPT_PLAN_v3` and `06_GO_AHEAD` stands. These decisions narrow scope (Indic text) and fill in defaults. They do not relax any gate.

STATUS: AGREED

