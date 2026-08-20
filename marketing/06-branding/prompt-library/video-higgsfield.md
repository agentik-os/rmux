---
project: rmux
layer: 06-branding/prompt-library
produced_by: marketing-machine-upgrade (spec §4.3 video · §3.1 routing)
status: filled
role: >
 rmux's recurring video shot types, Veo 3.1 / Seedance 2.0 / Kling 3.0 + image-to-video delta
 templates, bound to rmux tokens. Scene-led, no --soul-id. Directed not described (spec §4.3):
 subject noun byte-identical at every timestamp; audio ALWAYS directed (SFX:, Ambient noise:,
 no subtitles, captions added in montage). Never text-to-video cold, keyframe first
 (image-higgsfield.md), then i2v.
 Routing (spec §3.1): b-roll screen transitions → Kling 3.0 (cheap, ≤15s labeled shots);
 UGC-style demo captures → Seedance 2.0; hero + narrated audio → Veo 3.1.
---
# Video prompts, Higgsfield (rmux)

> Tokens: bg #0B0E14 · pane chrome #1E2733 · fg #E6EDF3 · muted #7D8590 · accent #36E2C4 (ONE focal element) · amber #F0883E (status glyph only) · Geist Mono · matte dark studio · real terminal output.
> Motion signature: 150ms ease-out snap for terminal content landing; 250ms pane split-reveal; cursor blinks at terminal cadence (~530ms). **Prohibited:** bounce, overshoot, animated gradients, motion-blur-as-style (DA.md §Motion). Audio = 50% of the reel.

---

## V1, Agent orchestration demo (detach → reattach → inspect) · Veo 3.1 timeline
*Pillar 1: Agentic Terminal. Slots: `orchestration-demo-short` (9:16 Shorts), `rmux-tmux-for-agents-thread` (16:9 OG). Hook in frame 1.*

```
[00:00-00:02] Straight-on view of a 4-pane terminal canvas: four agent sessions in
 near-black #0B0E14 with thin #1E2733 dividers; the mint cursor #36E2C4 blinks in one pane;
 on-screen text (Geist 600): "Your agents survive SSH drops."
[00:02-00:04] The terminal canvas goes dark, SSH disconnect. Panes grey to #7D8590.
 SFX: a single soft disconnect click.
[00:04-00:06] Same 4-pane canvas reappears, rmux reattach. Mint cursor re-blinks. The
 agent output resumes mid-line exactly where it left off. Emotion: relief, control.
 SFX: a short soft reconnect chime.
[00:06-00:08] Push-in on one pane showing `snapshot()` returning typed output. On-screen
 text (Geist 600 #36E2C4): "`cargo install rmux`". SFX: one clean terminal keypress.
matte near-black #0B0E14, pane dividers #1E2733, #E6EDF3 text, ONE mint #36E2C4 cursor +
CTA text, amber #F0883E strictly on any "agent running" glyph only. 9:16, 8s.
(no subtitles, captions added in montage)
```
*Subject noun "the 4-pane terminal canvas" byte-identical at every timestamp (spec §4.3).*

## V2, SDK `snapshot()` call resolving · Seedance 2.0 timeline
*Pillar 2: Programmable Terminal. Slots: `sdk-quickstart-tutorial` (16:9), `stop-scraping-capture-pane` (9:16). Hook in frame 1.*

```
[00:00-00:02] Dark editor split: left panel shows `capture_pane` bash with stdout scraping
 (grey #7D8590 muted); right panel shows rmux SDK code `snapshot()` call in Geist Mono.
 On-screen: "stop scraping. use a snapshot."
[00:02-00:04] Left panel fades darker; right panel: the `snapshot()` call resolves; the
 PaneSnapshot struct appears in mint #36E2C4 on the return type. Emotion: clean, typed.
 SFX: one soft resolve chime.
[00:04-00:06] Right panel fills frame; the resolved `content` field shows real plausible
 terminal output. Label below (Geist 400 #7D8590): "typed. no scraping." Ambient: quiet.
matte near-black, #11161F panel bg, ONE mint #36E2C4 on the resolved snapshot. 16:9 or 9:16, 6s.
(no subtitles)
```
*Subject noun "the rmux SDK code panel" byte-identical at every timestamp.*

## V3, rmux vs tmux split reveal · Kling 3.0 (b-roll, ≤15s)
*Pillar 3: tmux, Upgraded. Slot: `rmux-vs-tmux-comparison` (1:1 / 4:5).*

```
Shot 1, flat-on, left half of frame labeled "tmux": grey terminal, plain cursor, manual
 `tmux capture-pane -p` scraping visible. Right half labeled "rmux": mint #36E2C4 cursor
 active, `snapshot()` returning cleanly. Thin #1E2733 divider between. Camera static.
 SFX: a soft typing sound on the tmux side.
Shot 2, camera trucks right, staying flat-on; the rmux side fills 75% of frame; the
 "Windows ✓ Linux ✓ macOS ✓" badge appears in tabular-nums #E6EDF3 below the rmux label.
 SFX: three short ticks as each platform badge appears. 4s. seamless loop option.
(no subtitles). matte #0B0E14, ONE mint accent. 1:1, 4s.
```

## V4, Architecture diagram animation (three surfaces → daemon) · Kling 3.0 or i2v from SHOT D
*Pillar 5: Engineering Craft. Slot: `three-surfaces-one-daemon` (16:9 / 4:5).*

```
Shot 1, flat-on diagram: three boxes "CLI", "rmux-sdk", "ratatui-rmux" in #1E2733 chrome,
 unconnected. Camera static. bg #0B0E14. SFX: silence.
Shot 2, thin connector lines draw outward from each box toward a central "rmux daemon" box,
 sequentially: CLI first (150ms), then rmux-sdk (mint #36E2C4 highlight on this connector,
 250ms), then ratatui-rmux (150ms). The daemon box lights its border in #36E2C4 on connect.
 SFX: three brief soft connection tones, one per connector. Ambient: quiet room tone.
Shot 3, push-in on the central daemon box; the badge "#![forbid(unsafe_code)]" appears below
 in #7D8590. Camera holds. SFX: one clean resolution tone.
matte #0B0E14, ONE mint #36E2C4 on the rmux-sdk connector and daemon border. 16:9, 6s.
(no subtitles)
```
*If using i2v from SHOT D keyframe: `[start_image: SHOT D render]` Camera: slow dolly-in on the daemon box then hard stop (150ms ease-out). Motion: the mint connector line draws in; the daemon box border pulses once in #36E2C4. Physics: realistic. Audio: three soft connection tones + one resolution chime. 16:9, 5s. (no subtitles)*

## V5, Image-to-video delta (any keyframe → motion) · template
*Use to animate any image from `image-higgsfield.md`. Never redescribe the frame (spec §4.3).*

```
[start_image: <SHOT_x keyframe>] Camera: slow dolly-in then hard stop (150ms ease-out feel).
Motion: the mint #36E2C4 cursor blinks once in the active pane; a single light sweep grazes the
terminal screen surface; terminal text stays razor-sharp and still. Physics: realistic, restrained.
Audio: quiet room tone + one soft terminal keypress on the final beat. 9:16 or 16:9, 5s.
(no subtitles)
```
*Loops: start = end image + `seamless loop, cyclic motion, 3-6s` (for the cursor-blink loop version).*

---

### Doctrine (spec §3.2 · §4.3)
- **Keyframe first** (`image-higgsfield.md`), then i2v, **never** text-to-video cold.
- Subject noun **byte-identical** at every timestamp (Seedance); labeled shot blocks for Kling (never pronouns; but rmux has no human character, the "subject" is the terminal canvas / code panel / diagram).
- **Audio always directed:** `SFX:`, `Ambient noise:`. No dialogue since there is no avatar. Captions added in montage (word-synced, Whisper), never auto or hand-timed.
- **Beat grid ≤5s** visual reset · hook in frame 1 · loop for Shorts · dual export 9:16 (captions burned) + 16:9 (clean).
- Without `--soul-id`. Terminal output must be real and plausible, never lorem-ipsum.
