---
project: rmux
layer: 06-branding
produced_by: marketing-machine-upgrade (spec §2 design-guidelines.md)
status: filled
role: The human-readable grid / safe-zone / type / logo / DO-DON'T reference. Machine values live in tokens.json; this file explains how to apply them on an rmux canvas.
---
# Design Guidelines, rmux

> **Law:** "The programmable terminal made visible." The quality comes from restraint, type, and
> real artifacts. All numeric values are authoritative from `tokens.json` (SSOT). In case of
> doubt, `marketing/03-visual-identity/DA.md` is canon (R-CITE).

## 1. Grid & composition

- **Master `1080x1920`.** Recompose to 4:5 / 3:4 / 1:1 / 16:9, **never crop** (spec §2.1.1).
- **4 columns** for ~80% of visuals; **12 columns** when a slide exceeds 50 words.
- **Hard margins:** 60px left, 150px top. **Left-aligned by default**, terminal output reads from the left. The `$`/`❯` prompt glyph anchors every composition.
- **The multiplex pane grid is the hero motif.** Compose around thin `#1E2733` dividers organizing content into rectangular blocks, the visual grammar of the product itself (DA.md §Composition).
- **One idea per screen.** Max two text blocks per slide. Argument readable from headlines alone.
- **Real shell output only.** Every command, flag, or function call visible on screen must be a plausible rmux invocation. No lorem-ipsum, no invented APIs.

## 2. Safe zones (locked layers, spec §2.1.2)

| Platform | Usable zone | Bands to avoid |
|---|---|---|
| Universal | `900x1400` centered | everything else |
| Meta (Stories/Reels, Mar 2026) |, | top 14% (270px) · bottom 20% Stories / 35% Reels (670px) · sides 6% |
| TikTok | `900x1492` | 108px top · 320px bottom · 60px left · **120px right (action bar)** |
| Paid placements |, | +50-80px additional bottom |

- **AI-caption reserve: 250px bottom**, held free above the lower safe-zone. No brand text lands there.
- Headline, key stat, CTA, and `rmux` wordmark live **always** inside the universal zone.

## 3. Type scale (spec §2.1.4 · `tokens.json.type`)

- **2 families:** Geist or Inter (headlines, body on marketing surfaces) + Geist Mono / JetBrains Mono (wordmark, terminal output, code). **Nothing else.**
- **Display** 600 / -0.03em / LH 1.0 · **Body / lead** 400 / LH 1.5 / `textMuted`.
- **Mono (code/terminal):** 400 / 0em tracking / LH 1.6, every shell line is real, plausible rmux syntax.
- **Real jump display→body ≈ 2.4x.** Cover ≤12 words (~0.7s). Burned subtitle 4-7 words/cut.
- **Eyebrow** UPPERCASE +0.14em `textFaint`, **rare** (pillar label, platform tag only).
- **Numbers** in `tabular-nums` (versions, LOC counts, command counts, benchmark ms).

## 4. Color (60/30/10 · spec §2.1.5)

- **60%** terminal canvas (`bg #0B0E14` / `bg2 #11161F`) · **30%** pane chrome (`surface #1E2733` / `surface2 #1A2030`, `border #1E2733`) · **10%** accent `#36E2C4` ("rmux mint", one focal element/frame).
- **`amber #F0883E` = functional status only**, reserved for "agent running / alert" glyphs. **Never two loud accents in the same frame.**
- WCAG AA 4.5:1 on all text. Light mode available (`bg #FBFCFD`, `text #0B0E14`, `accent #1A8C7D`).
- R-CITE: all hex values from DA.md §Color palette.

## 5. Logo spec (spec §2.1.8)

- **Wordmark:** lowercase `rmux` in Geist Mono, tight tracking, `text #E6EDF3` on `bg #0B0E14`. The monospaced font is the brand, it *is* a terminal tool.
- **Icon variant:** `r>` prompt glyph or 2×2 pane-grid icon in `accent #36E2C4`, for avatar/profile use.
- **Clear space:** 1 X-height on all sides. Min size: ≥64px in posts, ≥40px as overlay.
- **Avatars/profiles:** icon-only. **Banners:** full wordmark lockup.
- **Video watermark:** `rmux` in `#E6EDF3` or `#36E2C4`, corner, small, no fill.
- **Approved backgrounds:** `#0B0E14`, `#11161F`, `#1E2733` only.

## 6. DO / DON'T (anti-generic guardrails, DA.md §DO/DON'T)

**DO**
- Dark canvas `#0B0E14` by default; monospaced type everywhere it's load-bearing.
- Exactly **one** mint `#36E2C4` accent per frame, the logo mark, the blinking cursor, the active pane, the CTA. Let it mean "active / important."
- Show **real** terminal output, real plausible rmux commands (`ensure_session`, `send_text`, `snapshot()`), real `$` prompts, real `PASS` / agent status text.
- Keep chrome grey and quiet; earn attention through restraint.
- Compose around the **pane-grid motif**: thin `#1E2733` dividers, rectangular blocks.
- Respect platform aspect ratios: 1:1 / 4:5 social, 16:9 demos/OG, 9:16 shorts.
- `tabular-nums` on all stats, versions, and line counts.

**DON'T**
- ❌ Generic AI-startup gradients, blobs, glassmorphism, or 3D mascots.
- ❌ Rainbow palettes or multicolor syntax-highlight as hero decoration (one accent only).
- ❌ Stock photos of people, hands on keyboards, abstract "tech" swooshes.
- ❌ Lorem-ipsum or fake/implausible commands, every shell line must be valid rmux usage.
- ❌ Amber `#F0883E` as a second identity color; it is a functional status signal only.
- ❌ Rust-orange branding, rmux owns mint, deliberately distinct from the Rust-orange tool crowd (DA.md DON'T).
- ❌ Motion: bounce, cartoon-overshoot, animated gradients, motion-blur-as-style, 3D plastic spin (see `tokens.json.motion.prohibited`).
- ❌ Blur or glassmorphism on the terminal content, pane output is always crisp and real.
- ❌ Fabricated benchmark numbers, fabricated star counts, or any metric not grounded in shipped artifacts (L2 / R-CITE).
