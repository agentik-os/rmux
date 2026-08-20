---
project: rmux
layer: 06-branding/templates
produced_by: marketing-machine-upgrade (spec §2.5 · hyperframes design.md)
status: filled
role: >
 The hyperframes BRAND DESCRIPTOR for rmux, "design.md describes the brand, frame.md directs
 the composition" (github.com/heygen-com/hyperframes, hyperframes.dev/design). Consumed by
 hyperframes animation adapters + Remotion composition props. Values are bound to tokens.json.
---
# design.md, rmux brand descriptor (tokens → video)

## Brand
**rmux, "The programmable terminal made visible."** A dark, monospaced, grid-first system that
looks like the tool it sells. The quality comes from restraint, type, and real artifacts.
`tokens.json` is the machine authority. DA.md is the origin.

## Palette (roles, not raw hex in compositions)
```
bg    #0B0E14  (dominant ~60%, the terminal-black canvas field, R-CITE: DA.md §Color palette)
bg2    #11161F  (pane surface, panel backgrounds)
surface  #1E2733  (chrome ~30%, pane dividers, grid lines, code block borders)
surface2 #1A2030  (secondary chrome)
text   #E6EDF3  (foreground, all terminal text, headlines)
textMuted #7D8590  (secondary text, labels, captions, eyebrows)
accent  #36E2C4  (~10%, "rmux mint", ONE focal element per frame: cursor, active pane, CTA)
amber   #F0883E  (functional status only, "agent running" / alert glyph ONLY; never a second brand color)
```
Light mode: bg `#FBFCFD` / text `#0B0E14` / textMuted `#4A5568` / accent `#1A8C7D`.

## Type
- **Display / headlines:** Geist (Inter fallback), weight 600, tracking -0.03em, LH 1.0.
- **Terminal / wordmark / code:** Geist Mono (JetBrains Mono / Berkeley Mono fallback), weight 400, tracking 0em, LH 1.6, all terminal output, the `rmux` wordmark, every shell command.
- **Body / lead:** Geist 400, LH 1.5, `textMuted #7D8590`.
- **Numbers:** `tabular-nums` everywhere (versions v0.3.1, command counts 90, LOC counts, benchmark ms).
- **2 families max.** No decorative fonts. R-CITE: DA.md §Typography.

## Material hierarchy
`pane-grid` (thin `#1E2733` dividers organizing rectangular blocks, hero motif, always present) > `surface panel` (flat `#11161F` background for code panels, hairline `#1E2733` borders, **NO blur**, **NO glassmorphism**) > `terminal text` (crisp `#E6EDF3` on `#0B0E14`, real output only).

## Light & texture
- Screen glow as the primary (often only) light source. If a dark studio key is needed: single matte directional key, no spill. R-CITE: DA.md §Photography "matte, low-noise, dark studio, a single mint rim-light on geometric pane blocks."
- Grain: **none on terminal content** (must stay crisp). At most faint grain on any physical hardware surround if a lensed shot is ever used.
- No blurred glass, no frosted panels, no pastel overlays.

## Motion (see tokens.json.motion)
- **`enter` 150ms `cubic-bezier(0,0,0.2,1)`**, the snap of a terminal command landing.
- **`splitReveal` 250ms `cubic-bezier(0.33,1,0.68,1)`**, a pane divides / new pane slides in. The motion signature of the multiplex grid.
- **`cursorBlink` ~530ms interval, `step-end`**, the mint `#36E2C4` cursor blinks at real terminal cadence. The only looping signature motion.
- **`logoIntro`:** wordmark `rmux` types in character by character (~60ms/char), cursor blinks once. Mechanical, never bouncy.
- **PROHIBITED:** bounce, cartoon-overshoot, animated gradients, stretched type, motion-blur-as-style, drifting blobs, rainbow gradients, 3D plastic spin, slide-fade SaaS (tokens.json.motion.prohibited).
- Reduced-motion: cursor static, pane splits instant, all reveals immediate.

## Recurring signatures
- **The multiplex pane grid**, thin `#1E2733` dividers, rectangular blocks, as the composition frame in every asset.
- **The mint cursor `#36E2C4`**, the single live cue, the "active" signal, the brand moment.
- **The `rmux` lowercase wordmark** in Geist Mono, always in a corner as a watermark or anchor.
- No human face, no stock people. The terminal *is* the brand character.

## Voice (for on-screen copy)
Engineer-to-engineer. Precise, technical, no fluff. Show the command, show the snapshot, show the diff. Words: `snapshot()`, `ensure_session`, `wait_for_text`, `forbid-unsafe`, `tmux-compatible`, `agent-ready`, *"typed, not scraped"*, *"your agents survive SSH drops"*, *"one runtime, three surfaces"*, `cargo install rmux`.
