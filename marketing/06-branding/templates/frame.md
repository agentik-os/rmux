---
project: rmux
layer: 06-branding/templates
produced_by: marketing-machine-upgrade (spec §2.5 · hyperframes frame.md)
status: filled
role: >
 The hyperframes COMPOSITION DIRECTIVES for rmux, "frame.md directs the composition"
 (pacing / scale / dwell / motion), the counterpart to design.md's brand descriptor. Consumed by
 hyperframes animation adapters via data-start/data-duration + Remotion sequence props.
---
# frame.md, rmux composition directives

## Canvas & grid (see tokens.json.canvas)
- Master **1080x1920**. Recompose to 1080x1350 / 1080x1440 / 1080x1080 / 1920x1080, **never crop**.
- 4 columns (default) / 12 columns (>50 words). Hard margins 60px left / 150px top. **Left-aligned** (terminal output anchors left, `$`/`❯` glyph at the left edge).
- **Safe-zone 900x1400 locked** + caption reserve **250px** bottom (non-exportable layers). Platform bands: see `tokens.canvas.safeZones`.
- The **pane-grid motif** is the compositional backbone: thin `#1E2733` dividers organizing the frame into rectangles, mirroring the product itself.

## Scale (hierarchy on the frame)
- **One** dominant element per frame: the pane grid, or the headline, or the key stat, or the code snippet. Not all at once.
- Real jump display headline → body ≈ 2.4x. One idea per screen, max two text blocks.
- Terminal output occupies a surface `#11161F` panel with hairline `#1E2733` borders, **NO blur**, sharp and real.
- Argument is readable from headlines alone; body text supports, never competes.

## Pacing (short-form, spec §5.1)
- **Hook delivered ≤2.5s**, big readable text or a visible terminal action in frame 1, **no** logo intro / "hey there" cold open.
- **Visual reset every 3-5s** (pane split / new code panel / text change / zoom on a command), encoded as a beat grid.
- **Reverse structure:** payoff first ("your agents survive SSH drops"), method after (the SDK call).
- Default length **15-35s**; SDK tutorial +5-10s; conversion-intent 30-60s.
- **Loop the ending** (Shorts): last frame → first frame; the cursor-blink provides a natural loop point.

## Dwell (hold times)
- Each text block held **≥2s**. Burned subtitle 4-7 words per cut. Cover ≤12 words.
- The mint `#36E2C4` cursor blinks for at least one full cycle (530ms) in any "live" context, it earns the frame's attention.
- Code snippets that require reading: held ≥3s minimum, pull-in with `emphasis` 400ms.
- A shell command resolving (the `snapshot()` return) is the "Confirm moment" of rmux, hold ≥2s so it reads.

## Motion (bind to tokens.json.motion)
- `enter` 150ms `cubic-bezier(0,0,0.2,1)`, terminal snap.
- `splitReveal` 250ms `cubic-bezier(0.33,1,0.68,1)`, pane divides / new content slides in.
- `emphasis` 400ms `cubic-bezier(0.2,0,0,1)`, stat / snapshot value pops in.
- Transitions allowed: **cut · fast-fade (≤150ms) · match-cut between pane states**. Nothing else.
- **PROHIBITED:** bounce, overshoot, animated gradients, stretched type, motion-blur-as-style, SaaS slide-fade, 3D plastic spin (`tokens.motion.prohibited`).
- Reduced-motion: cursor static, pane splits instant, all reveals immediate, hierarchy preserved.

## Audio (spec §4.3, audio is 50% of the reel)
- Always directed: `SFX:`, `Ambient noise:`. No avatar dialogue (scene-led, no talking-head). Captions word-synced (Whisper) added in montage, never auto or hand-timed.
- Sound grammar for rmux: **soft terminal keypress** (enter key), **soft resolve chime** (snapshot returns), **quiet room tone** (ambient dark engineering). No music-for-hype; subtle bed only if present; FFmpeg sidechain ducking.
- The **mint cursor blink** can be paired with a barely-audible 530ms pulse SFX for the "live" scene, optional but reinforces the brand moment.

## Export
- Dual format from one composition: **9:16** (captions burned, safe-zone) + **16:9** (clean, for OG/YouTube).
- `rmux` wordmark in `#E6EDF3` or `#36E2C4`, corner, small, no fill, in all video exports.

## Frame checklist (before render)
- [ ] Critical elements inside 900x1400 · caption reserve 250px free · platform bands respected.
- [ ] ONE mint `#36E2C4` accent per frame · amber `#F0883E` only as status glyph · tokens only, no raw hex in comp.
- [ ] Hook in frame 1 · visual reset ≤5s · loop option (Shorts) · shell command resolving is held ≥2s.
- [ ] Motion in whitelist · audio directed · dual export ready.
- [ ] All shell commands are plausible rmux usage, verified before render.
