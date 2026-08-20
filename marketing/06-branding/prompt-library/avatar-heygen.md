---
project: rmux
layer: 06-branding/prompt-library
produced_by: marketing-machine-upgrade (spec §4.5 avatar scene brief · §3.3 avatar edit grammar)
status: filled-conditional
role: >
 HeyGen avatar scene briefs, DORMANT for rmux. rmux is scene-led with NO recurring human
 avatar (avatar-persona.md / avatar-soul.md), so no talking-head series exists today. This file
 documents HOW HeyGen would plug in IF a founder/maintainer series is ever activated, so the
 pipeline is ready without inventing social proof now (honesty L2).
---
# Avatar scene briefs, HeyGen (rmux), **DORMANT / conditional**

## Status: no talking-head today

`heygen_avatar_id: null` · `hasRecurringAvatar: false` (see `tokens.json.avatar` + `avatar-persona.md`).
rmux is **scene-led / product-led**. There is **no human spokesperson**, therefore **no HeyGen scenes
are produced now**. This file exists so the pipeline is **ready** if the operator ever activates a
founder/maintainer explainer series (Phase 2), without fabricating social proof that does not yet exist.

## What plays the "talking-head" role today

For explainers, rmux uses the **faceless grammar**: screen-recordings of the real shipped demos
(via `website-to-hyperframes`), product b-roll (`video-higgsfield.md`), and voiceover off.
This is the lo-fi-beats-polish format recommended for developer brands (spec §5.1), and it is
consistent with DA.md's "no people" doctrine and the engineer-to-engineer voice.

---

## IF activated later, prerequisites (NOT done now)

1. **Explicit operator decision** to launch a founder/maintainer series (Phase 2, see conditional
  brief in `avatar-persona.md`).
2. Train identity from the **real project maintainer**, not a synthetic face (honesty L2, R-CITE).
  - `higgsfield-soul-id` for identity-consistent stills → fill `soul_id` in `avatar-persona.md` + `tokens.json.avatar`.
  - **HeyGen Avatar V** for talking-head → fill `heygen_avatar_id` here + in `tokens.json.avatar`.
3. Define the voice tone: *precise · direct · no-fluff · engineer-to-engineer · honest about preview status* (5 brand adjectives from `product-marketing.md §Voice & tone`).

## Scene brief template, HeyGen (spec §4.5, fill only after activation)

```
scene: [n] layout: [fullscreen | pip-corner | graphic-only]
avatar: [heygen_avatar_id, EMPTY until activated]
line: "[one breath, ≤8s, from a scriptwriter beat]"
tone: precise, direct, no-fluff, engineer-to-engineer, honest about preview status
background: [brand template #0B0E14/#1E2733 OR real rmux screencap / demo recording]
wardrobe: plain dark top, matte, no loud color, nothing competing with accent #36E2C4
captions: from HeyGen API word-timings, Hormozi style, 9:16 only, within the safe-zone band
```

## 3-layout montage grammar (spec §3.3, when the series exists)

Alternate per scene to simulate an editor's pacing:
1. **Fullscreen avatar** (direct address), the maintainer explains.
2. **PiP corner avatar** over a screen-recording of the real rmux demo (the SDK call, the agent orchestration, the Playwright test passing).
3. **Graphic-only + VO**, the architecture diagram or the comparison split, voice off.

Plus: FFmpeg sidechain ducking (music under voice) · SFX/music chosen by scene context · dual export 9:16 (captions burned) / 16:9 (clean) from one composition · captions from HeyGen word-timings, **never** hand-timed.

## Standing content rules for any future avatar series

- Every demo clip shows **real** rmux output, plausible commands, real function names (`ensure_session`, `snapshot`, `wait_for_text`).
- No fabricated benchmarks or invented star counts mentioned by the avatar (L2 / R-CITE).
- If the `.tmux.conf` migration is demonstrated, show the real migration path honestly.
- Mention **upstream / fork reality** accurately when relevant, the honest scope note from `product-marketing.md`.

> **Safeguard:** while this file remains *dormant*, no avatar scene enters the calendar. rmux's identity is the terminal + the pane grid + the mint cursor.
