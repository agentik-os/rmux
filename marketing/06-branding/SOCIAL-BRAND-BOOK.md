---
project: rmux
layer: 06-branding
produced_by: marketing-machine-upgrade (docs/marketing-machine-upgrade-research.md §2)
status: filled
role: >
 The compiled, machine-readable social brand system for rmux. Every content skill
 (social-content, ad_designer, scriptwriter, creator-media-engine, higgsfield-generate)
 reads THIS + tokens.json before producing anything, same SSOT contract as
 .agents/product-marketing.md. Derived from 03-visual-identity/DA.md (the input DA) and
 marketing/00-context/product-marketing.md (R-CITE). 03-visual-identity = input; 06-branding = the system.
language: English prose (engineer-to-engineer voice, R-STYLE) · tokens.json + machine files universal
---

# SOCIAL BRAND BOOK, rmux

> **"The programmable terminal made visible."** A dark, monospaced, grid-first system that *looks
> like the tool it sells*: terminal panes, mint cursors, precise chrome, and exactly one signature
> accent. The quality comes from **restraint, type, and real artifacts**, not decoration. Consistency
> is **enforced by the files**, not by memory (spec §1.1). This book + `tokens.json` are that law.

**Product one-liner:** *"The universal Rust terminal multiplexer for the agentic era, detachable, scriptable, and inspectable."* (`product-marketing.md`)
**Brand concept:** *"The programmable terminal made visible."* (`DA.md`)

---

## 1. Master canvas + grid

*(spec §2.1.1)*

- **Master design canvas: `1080x1920` (9:16).** Every other size is **recomposed, never cropped**:
 `1080x1350` (4:5, IG feed / carousel / LinkedIn PDF), `1080x1440` (3:4), `1080x1080` (1:1, X / cover center-safe), `1920x1080` (16:9, OG card / YouTube thumbnail / screencap demo).
- **Grid:** 4 columns for ~80% of content; 12 columns for text-heavy slides (>50 words).
- **Hard margins:** ~60px from left edge, ~150px from top. **Left-aligned by default**, terminal-native composing reads from the left, matching the `$`/`❯` anchor of real shell output.
- **The pane grid is the hero motif:** composition built around thin `#1E2733` dividers organizing content into rectangular blocks, like the multiplex grid (`DA.md §Composition`). One mint element per frame earns its attention through restraint.
- **One idea per screen.** Max two text blocks per slide. The argument is readable from the headlines alone. Real shell output and plausible rmux commands only, never lorem-ipsum.

## 2. Safe zones (2026 pixel values)

*(spec §2.1.2, shipped as **locked, non-exportable layers** in every template under `templates/`)*

- **Universal zone: `900x1400` centered on `1080x1920`.** Every critical element (headline, command, stat, CTA, mint cursor, wordmark) lives here.
- **Meta (unified Stories/Reels, March 2026):** keep critical elements out of top 14% (270px), bottom 20% Stories / 35% Reels (670px), 6% sides.
- **TikTok:** usable zone `900x1492`, 108px top, 320px bottom, 60px left, 120px right (action bar).
- **Paid placements:** reserve an additional 50-80px bottom, the CTA never drops into that band.
- **Rule:** safe-zone errors *"effectively stop happening"* when the zone is a locked layer, not an intention.

## 3. AI-caption reserve

*(spec §2.1.3)*

- Auto-captions are **default-on** on TikTok / Reels / Shorts: they consume 80px (≤10 words) to 250px (≤40 words).
- **rmux caption reserve = 250px bottom**, held free above the lower safe-zone boundary. No brand text lands there.
- **Max on-screen text by format:** hook/cover ≤ **12 words** (~0.7s read); burned subtitle **4-7 words per cut**, held ≥2s, high contrast.
- **Contextual continuity:** audio, on-screen text, and caption share the same semantic keywords (`rmux`, `snapshot`, `tmux-compatible`, `agent`, `SDK`, `forbid-unsafe`).

## 4. Type scale (mobile-first)

*(spec §2.1.4, see `tokens.json.type`)*

- **2 families max:** **Geist Mono** (wordmark, terminal output, code) + **Geist** or **Inter** (headlines, body on marketing surfaces). Both are the same Vercel/neutral-engineering family, coherent, no exotic decoration.
- **Display:** weight 600, tracking -0.03em, line-height 1.0. On the 1080-master, a display line lands ~80-140px.
- **Statement / anchor:** 500, -0.025em. **Body / lead:** 400, LH 1.5, `textMuted`.
- **Code / terminal:** Geist Mono (or JetBrains Mono), weight 400, tracking 0em, every shell line is real, plausible rmux output.
- **Real jump display→body ≈ 2.4x** (not 1.2x micro-steps). Arm's-length squint test: the headline stays readable.
- **Eyebrow:** UPPERCASE, +0.14em, `textFaint`, **rare** (platform label, pillar tag), never decorative wallpaper.
- **Numbers:** `tabular-nums` everywhere (version numbers, line counts, LOC, benchmark ms).

## 5. Color roles by ratio (as tokens)

*(spec §2.1.5, **60/30/10**, raw hex never in templates, named tokens only, R-CITE: DA.md §Color palette)*

| Role | Ratio | Token(s) | Usage |
|---|---|---|---|
| **Dominant, terminal black** | ~60% | `bg #0B0E14`, `bg2 #11161F` | Primary canvas, everywhere (DA.md `#0B0E14` / `#11161F`) |
| **Secondary, pane chrome** | ~30% | `surface #1E2733`, `surface2 #1A2030` | Pane borders, grid dividers, code block chrome (DA.md `#1E2733`) |
| **Foreground** | text | `text #E6EDF3`, `textMuted #7D8590` | Primary code/headline ink, secondary labels (DA.md `#E6EDF3` / `#7D8590`) |
| **★ Signature accent, "rmux mint"** | ~10% | `accent #36E2C4` | ONE focal element per frame: the logo mark, blinking cursor, active pane, CTA. Use sparingly. (DA.md `#36E2C4`) |
| **Functional amber (status only)** | rare | `amber #F0883E` | "Agent running" / alert status glyph ONLY, never a second brand color (DA.md `#F0883E`) |

- **WCAG AA 4.5:1** on all text (`text #E6EDF3` on `bg #0B0E14` is well above threshold).
- **Discipline (operator Monogram doctrine):** grey chrome + **ONE** signature accent. Amber lives as a *functional status signal*, never identity. No gradients as decoration, no rainbow syntax-highlight as hero art.
- **Light-mode canvas** (`#FBFCFD`) for the rare light surfaces (README light-header parity, DA.md). Light-mode ink: `#0B0E14`.

## 6. Motion tokens

*(spec §2.1.6, see `tokens.json.motion`, consumed by hyperframes via `frame.md`)*

- **`enter`:** 150ms `cubic-bezier(0,0,0.2,1)`, fast ease-out snap, the feel of a terminal command landing.
- **`splitReveal`:** 250ms `cubic-bezier(0.33,1,0.68,1)`, a pane divides / new pane slides into frame; the motion signature of the multiplex grid.
- **`cursorBlink`:** 530ms interval, on/off, only in "live/active" context; the mint `#36E2C4` cursor blinks at a real terminal cadence.
- **`emphasis`:** 400ms `cubic-bezier(0.2,0,0,1)`, used for key data reveals (snapshot payload appearing, benchmark popping in).
- **Logo intro:** wordmark `rmux` types in one character at a time at ~60ms/char, then pauses, mechanical, purposeful.
- **Transition whitelist:** cut · fast fade (≤150ms) · match-cut between pane states. Nothing else.
- **PROHIBITED (per DA.md DON'T + Klarna principle):** bounce / cartoon-overshoot, animated gradients, stretched type, motion-blur-as-style, drifting pastel blobs, rainbow gradients, slide-fade SaaS animations, 3D plastic spin.
- **Reduced-motion:** cursor static (no blink), reveals instant, pane splits appear immediately; hierarchy preserved.
- **Terminal-native motion principle:** motion is mechanical and purposeful, never decorative. Every animation has a name and a reason: `enter` = content arrives, `splitReveal` = structure expands, `cursorBlink` = system is live, `emphasis` = a fact lands.

## 7. Template registry

*(spec §2.1.7, pointers to `templates/`; locked layers vs editable slots)*

| Template | File | LOCKED layers | EDITABLE slots |
|---|---|---|---|
| Brand descriptor (video) | `templates/design.md` | palette, type, motion, multiplex-grid grammar | scene, subject |
| Composition directives | `templates/frame.md` | grid, safe-zones, pacing, beat-grid, audio | shot, beat |
| 6 Still archetypes | `templates/stills/README.md` | logo, safe-zone, grid, color roles, pane chrome | headline, code/UI, stat, CTA |

- **Changing a token in `tokens.json` propagates everywhere**, a non-designer produces an on-brand post in ~10 min.
- Editable slots are: **headline, real terminal output / screenshot, stat or benchmark, CTA line**. Everything else is locked.

## 8. Logo spec

*(spec §2.1.8, source: DA.md §Typography)*

- **Mark:** lowercase `rmux` wordmark set in **Geist Mono** (or JetBrains Mono), tight tracking, in `text #E6EDF3` on `bg #0B0E14`. The monospaced wordmark is intentional, it *is* a terminal tool.
- **Icon variant:** a monospaced `r❯` glyph (prompt symbol) or a simple 2×2 pane-grid icon in `accent #36E2C4` on near-black, for avatar/profile use where full wordmark is too small.
- **Clear space:** one X-height (of the `rmux` cap height) on all four sides. Min size: ≥64px wordmark in posts, ≥40px overlay.
- **Avatars/profiles:** icon-only (`r❯` or pane-grid). **Banners:** full `rmux` wordmark lockup.
- **Video watermark:** `rmux` in `text #E6EDF3` or mint, corner, small, no background fill.
- **Approved backgrounds:** `#0B0E14`, `#11161F`, `#1E2733`. Never on a saturated or noisy photo.
- **DON'T:** no re-coloring the wordmark in amber or rainbow, no stretching, no glossy 3D treatment, no lorem-ipsum commands adjacent to the logo.

## 9. Avatar / persona spec

*(spec §2.1.9, detail in `avatar-persona.md`)*

- **rmux has NO recurring human avatar.** Decision: **scene-led / product-led** (`03-visual-identity/higgsfield/avatar-soul.md`). Identity is carried by the **terminal itself**: canvas `#0B0E14`, mint cursor `#36E2C4`, the split-pane grid, the lowercase monospaced `rmux` wordmark.
- **No `soul_id`, no `heygen_avatar_id`.** Generate **without `--soul-id`**.
- The recurring identity anchor is the **multiplex grid motif** + the **mint cursor**, the same visual that repeats across every OG card, carousel cover, and demo screencap. That *is* the face of rmux.
- See `avatar-persona.md` for the scene-led environment brief (the five recurring settings, the "wardrobe" of the terminal).

## 10. Governance

*(spec §2.1.10 + §6)*

**One-page brief (before any generation):**
1. **One message** (one pillar: P1 Agentic Terminal / P2 Programmable Terminal / P3 tmux Upgraded / P4 Terminal Testing / P5 Engineering Craft).
2. **One proof point** (e.g., "all 90 tmux commands verified", "`snapshot()` typed, no scraping", "Linux/macOS/Windows VERIFIED", "5 shipped demos", "`#![forbid(unsafe_code)]`").
3. **Audience insight** (primary ICP: Orchestrator Olivia, agent-infra engineer over SSH; or Rustacean Ravi / tmux-veteran Theo / Platform Priya / QA Quinn).
4. **2-3 past top performers** (from `pattern-ledger.md`).

**Pre-flight QA (Pass 1, deterministic, spec §6.2):**
- [ ] Safe-zones respected (critical elements inside 900x1400; platform bands OK).
- [ ] WCAG AA 4.5:1 contrast; type minimums (display headline ≥40px-equiv on canvas, body ≥24pt).
- [ ] Caption reserve 250px free; burned text 4-7 words, held ≥2s.
- [ ] Token conformance (only `tokens.json` colors/fonts; accent **10% max**, one per frame; amber = status glyph only).
- [ ] `kill-list.md` scan on every prompt (no banned quality words; standing negatives present).
- [ ] Video: hook text in frame 1, beat grid ≤5s, loop (Shorts), dual export 9:16+16:9, word-synced caption track.
- [ ] All shell commands are plausible rmux usage, never lorem-ipsum, never invented flags.

**Pass 2, Taste + brand review (adversarial, R-VERIFY):** `taste-and-aesthetic-director` + `art-director-soul` on the **rendered artifact** (screenshot/MP4, not the code, L1 runtime truth): anti-kitsch, hierarchy, "would a $150k agency ship this to engineers?", pane-grid coherence vs the last 9 posts. A delegate's "done" is never the verdict.

**Pass 3, Operator approval gate** before publish (04-publishing). Approved prompts/sources stored with the asset (audit trail).

**Performance loop (spec §6.2 + `pattern-ledger.md`):** metrics return here against the rubric, hook rate ≥75%, completion ≥65-70%, sends-per-reach (IG), swipe-completion >60% (carousels), first-hour reply SLA (Reddit). **A winning pattern is PROMOTED** into `templates/`/`prompt-library/`; **a pattern that fails twice is RETIRED.** Winner → follow-up post within 48h.

---

### Sibling files
`tokens.json` (the machine bridge) · `design-guidelines.md` (grid/safe/type/logo DO/DON'T) · `avatar-persona.md` (scene-led) · `prompt-library/*` (pre-filled rmux prompts) · `templates/*` (code, not canvases) · `pattern-ledger.md` (win/lose log).
