---
project: rmux
layer: 06-branding/prompt-library
produced_by: marketing-machine-upgrade (spec §4.1 KILL block · §4.2 anti-generic · §6.2 kill-list scan)
status: filled
role: rmux's standing negatives + banned quality words + the anti-generic realism block. Every image/video prompt in this library appends the relevant section; the Pass-1 gate scans every prompt against this file (spec §6.2).
---
# Kill-list, rmux

## 1. Banned quality words (2026 AI signatures, spec §4.1)

**Never** write these in a prompt:
`8K`, `ultra-realistic`, `hyperrealistic`, `masterpiece`, `cinematic`, `golden hour`,
`Portra 400`, `Cinestill 800T`, `award-winning`, `trending on artstation`, `bokeh`,
`highly detailed`, `photorealistic render`, `octane`, `unreal engine`.

> These words **read as AI** in 2026 (hedra.com, aivideobootcamp.com). Prefer **photographic
> factuality**: real screen output + a single physical light logic + committed precision.
> Approved film stocks (if any lensed shot ever occurs): **Kodak Gold 200**, **Ektachrome**,
> **Ilford HP5**. Light: **flat overcast daylight** or a single soft directional source.

## 2. rmux standing negatives (the KILL block, add to every spec-sheet review pass)

```
KILL: generic AI-startup gradients, blobs, glassmorphism, 3D mascots; stock photos of
people or hands on keyboards; abstract "tech" swooshes; rainbow palette or multicolor
syntax-highlight as hero decoration; lorem-ipsum or fake/implausible shell commands;
Rust-orange branding (#B94700 range); amber #F0883E used decoratively (status glyph only);
more than ONE mint #36E2C4 element per frame; blurred or frosted terminal output (must stay
crisp and real); decorative fonts outside Geist / Geist Mono / JetBrains Mono / Berkeley Mono;
fabricated benchmark numbers, star counts, or download counts not grounded in shipped artifacts;
a second co-brand color; emoji soup
```

## 3. Anti-generic realism block (spec §4.2, append to any LENSED shot if physical hardware is ever used)

rmux has no Soul (scene-led) → if any physical lensed shot (hardware, desk, keyboard) is used,
append this block:

```
candid, not posed, in-between moment, natural uneven ambient light, faint grain on the
physical environment only (screen content stays crisp and sharp), framing a little tilted,
slight motion blur on one hand only if hands are in frame, visible surface texture on any
hardware in frame, shot on Ricoh GR III or Canon AE-1, Kodak Gold 200 grain on environment
```

> **Exception:** **terminal screen / UI output always stays crisp and real** (high-DPI
> composite), never grained or "generated". The grain lives on the physical surroundings only,
> never on the terminal content.

## 4. rmux-specific hard rules (non-negotiable, DA.md + product-marketing.md)

- **ONE** mint `#36E2C4` accent maximum per frame, the blinking cursor, the active pane, the CTA, or the logo mark. Never two loud accents together.
- **Terminal output always crisp**, matte, on background `#0B0E14` / surface `#1E2733`, hairline borders, **NO blur**, **NO glassmorphism**.
- **All shell commands must be real rmux usage**, plausible invocations of `rmux`, `ensure_session`, `send_text`, `snapshot()`, `wait_for_text("ready")`. Never lorem-ipsum, never invented flags.
- **No Rust-orange branding.** rmux owns mint `#36E2C4`, deliberately distinct from the Rust-orange tool crowd (DA.md DON'T).
- **Amber `#F0883E` is strictly a functional status signal** ("agent running", "alert"), never decorative, never as a second brand color.
- **No stock people, no recurring human avatar, no synthetic founder face** (scene-led + honesty L2).
- **All proof claims must be verifiable in shipped artifacts:** 90 commands, v0.3.1, 5 demos, Linux/macOS/Windows VERIFIED, never fabricate metrics (R-CITE / L2 / product-marketing.md).

## 5. Positive-phrasing note (spec §4)

On Higgsfield-routed models and **Flux 2**: **no negative prompts**, phrase positively. The KILL
block above is a gate/review guard, not a Flux negative field. On Flux 2 JSON (`flux-brand.json`),
describe what IS in the scene bound to named objects and hex values; do not list what is absent.
`<200 tokens` on Higgsfield-routed models.

## R-NODASH (RÈGLE DURE) — JAMAIS de tiret cadratin
JAMAIS de `—` (em-dash) ni `–` (en-dash) comme ponctuation, nulle part : copy, captions, texte sur image/vidéo, posts, briefs. Ça sonne IA. Remplacer par virgule / point / deux-points / parenthèses. Le trait dunion `-` des mots composés reste OK. Relire et stripper avant toute livraison.
