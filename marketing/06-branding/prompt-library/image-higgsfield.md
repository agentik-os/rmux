---
project: rmux
layer: 06-branding/prompt-library
produced_by: marketing-machine-upgrade (spec §4.1 6-block spec sheet · §2.4 pre-filled per brand)
status: filled
role: >
 rmux's recurring image shot types, each as a filled 6-block spec sheet bound to rmux tokens.
 Generate WITHOUT --soul-id (scene-led). Append kill-list.md §2 (KILL) to every prompt.
 Routing: stylized OG/social cards and diagram motifs → Higgsfield/Flux 2; readable text / feature
 grids / code-accurate output → Nano Banana Pro (never send readable text to a video model).
 Real high-DPI screencaps of shipped demos beat synthetic generation, always prefer real output
 first (shotlist.md Production notes / DA.md §Photography).
---
# Image prompts, Higgsfield (rmux)

> Spec sheet = technical brief, **not** a sentence (spec §4). Front-load the subject. `<200 tokens`. Positive phrasing.
> Tokens: bg #0B0E14 · pane chrome #1E2733 · fg #E6EDF3 · muted #7D8590 · accent #36E2C4 (ONE focal element only) · amber #F0883E (status glyph only) · Geist Mono wordmark · matte dark studio · real terminal output.

---

## SHOT A, Multi-agent orchestration hero · Nano Banana Pro / Flux 2
*Pillar 1: Agentic Terminal. Calendar slots: `rmux-tmux-for-agents-thread` (OG 16:9), `lost-agent-ssh-story` (1:1).*

```
[SUBJECT: a terminal canvas divided into four panes by thin grid-line dividers; each pane shows
 a different agent's live output, plausible rmux session names and shell prompts; the top-left
 pane has a blinking cursor in mint hex #36E2C4 marking the active pane; one pane shows the
 text "agent running" with a small amber glyph in hex #F0883E; the rmux wordmark in Geist Mono
 lowercase anchors the bottom-left corner in #E6EDF3]
[FRAMING: straight-on terminal view, 16:9 filling the frame, slight low angle to read all panes]
[LIGHT: dark studio, a single matte key from upper-left grazing the screen surface; the screen
 glows the only real light; no spill, no reflections]
[ATMOSPHERE: near-black #0B0E14 field; pane dividers in muted border #1E2733; terminal text in
 #E6EDF3; ONE mint #36E2C4 cursor as the sole accent; amber #F0883E only on the status glyph;
 no other color; matte dark hardware surround]
[TECHNIQUE: high-DPI screen composite, crisp sharp terminal text, no grain on screen content,
 16:9, straight render]
KILL: (kill-list.md §2) + any non-rmux branding, fake/implausible shell commands, more than one
 bright color, amber used decoratively, Rust-orange, blurred or frosted terminal output
```

## SHOT B, SDK code panel (`snapshot()` resolving) · Nano Banana Pro
*Pillar 2: Programmable Terminal. Calendar slots: `sdk-quickstart-tutorial` (16:9), `stop-scraping-capture-pane` (4:5).*

```
[SUBJECT: a split terminal + editor view: left half shows a dark code editor with Rust SDK
 code, `ensure_session("worker-1")`, `send_text("cargo test")`, `snapshot()`, `wait_for_text("ready")`
, in Geist Mono; right half shows the terminal output resolving with mint hex #36E2C4 on the
 returned PaneSnapshot type; the rmux wordmark small in the corner]
[FRAMING: 16:9 or 4:5, slight left-alignment, code fills 60% of the frame]
[LIGHT: screen glow only, matte, dark studio; no external key light]
[ATMOSPHERE: bg #0B0E14; code panel bg #11161F; pane divider #1E2733; source text #E6EDF3;
 the ONE mint #36E2C4 accent on the resolved snapshot value; muted #7D8590 on comments; no
 other color; no decorative elements]
[TECHNIQUE: precise code composite, monospaced Geist Mono font, crisp text, no grain on screen,
 4:5 or 16:9]
KILL: (kill-list.md §2) + invented function signatures, non-Rust syntax, more than one accent
 color, any rainbow syntax-highlight theme used as hero art
```

## SHOT C, rmux vs tmux comparison split · Nano Banana Pro
*Pillar 3: tmux, Upgraded. Calendar slots: `rmux-vs-tmux-comparison` (1:1), `tmux-on-windows-finally` (16:9).*

```
[SUBJECT: a clean 50/50 split frame, left side labeled "tmux" in muted #7D8590, showing
 grey terminal output with a plain cursor; right side labeled "rmux" in #E6EDF3, showing
 the same terminal with a mint #36E2C4 cursor active and a `snapshot()` line resolving;
 thin #1E2733 divider between; bottom of the rmux side: a "Windows ✓ Linux ✓ macOS ✓" badge
 in tabular-nums #E6EDF3; rmux wordmark in Geist Mono small bottom-right]
[FRAMING: 1:1 or 16:9, balanced halves, flat-on]
[LIGHT: flat even, screen glow, no dramatic shadows]
[ATMOSPHERE: bg #0B0E14; left side slightly dimmer (#11161F tint); right side normal; ONE mint
 accent on the rmux cursor and the Windows badge; clean separation; no gradient between halves]
[TECHNIQUE: clean editorial composite, crisp type, tabular-nums on badge, no grain on screen, 1:1]
KILL: (kill-list.md §2) + fake benchmark numbers, biased/strawman comparison, decorative color
 on the tmux side, more than one accent
```

## SHOT D, Three-surfaces → one-daemon architecture diagram · Nano Banana Pro / Flux 2
*Pillar 5: Engineering Craft. Calendar slots: `three-surfaces-one-daemon` (16:9 / 4:5), `forbid-unsafe-no-network` (1:1).*

```
[SUBJECT: a clean box-and-connector diagram, three source boxes labeled "CLI", "rmux-sdk",
 "ratatui-rmux" connected with thin lines to a central "rmux daemon" box; the active connector
 line from "rmux-sdk" is drawn in mint hex #36E2C4; box borders in #1E2733; label text in
 #E6EDF3; a small badge below: "#![forbid(unsafe_code)] no-network-at-runtime" in #7D8590
 tabular text; rmux wordmark Geist Mono small corner]
[FRAMING: 16:9 or 4:5, centered diagram, generous whitespace around boxes, flat-on]
[LIGHT: flat even editorial, no dramatic lighting]
[ATMOSPHERE: bg #0B0E14 field; box fill #11161F; box borders #1E2733; ONE mint #36E2C4 accent
 on the active connector; all other connectors #7D8590; heavy whitespace; no decorative elements]
[TECHNIQUE: clean vector-precise diagram render, crisp hairline connectors, no grain, 16:9]
KILL: (kill-list.md §2) + invented crate names, extra connector types, more than one accent
 color, decorative arrows or 3D styling
```

## SHOT E, Terminal testing assertion panel · Nano Banana Pro
*Pillar 4: Terminal Testing. Calendar slots: `playwright-for-terminals` (16:9).*

```
[SUBJECT: a test runner panel in a dark terminal, `pane.snapshot()` call, then
 `assert!(snapshot.contains("server ready"))`, then a large mint text "PASS" resolving
 in hex #36E2C4; the rmux wordmark Geist Mono in the corner; a step counter "3/5 tests"
 in tabular-nums #7D8590]
[FRAMING: 16:9 or 4:5, terminal fills the frame, slight upward crop to show full "PASS"]
[LIGHT: screen glow, dark studio, matte]
[ATMOSPHERE: bg #0B0E14; terminal surface #11161F; text #E6EDF3; the ONE mint #36E2C4 on
 "PASS" only; muted #7D8590 on the step counter; no other color]
[TECHNIQUE: high-DPI screen composite, Geist Mono, crisp, tabular-nums, no grain on screen, 16:9]
KILL: (kill-list.md §2) + invented test names or function signatures, extra accent colors,
 fabricated passing percentage claims
```

## SHOT F, Type-led stat card (social proof / pillar proof) · Nano Banana Pro / Flux 2
*All pillars. Calendar slots: any stat-backed post (`90-tmux-commands`, `5-demos-shipped`).*

```
[SUBJECT: a type-led editorial layout, one large tabular number centered-left: "90" or "5" or
 "v0.3.1"; a single-line label below in #7D8590: "tmux-compatible commands" / "shipped demos"
 / "current version"; the mint #36E2C4 only on the large number; rmux wordmark Geist Mono
 small bottom-right corner; generous whitespace]
[FRAMING: 1:1 or 4:5, left-aligned type block, flat-on]
[LIGHT: flat editorial, even]
[ATMOSPHERE: bg #0B0E14; text #E6EDF3; number in ONE mint #36E2C4; label #7D8590; massive
 whitespace; no secondary colors; no decorative elements]
[TECHNIQUE: clean typographic composition, tabular-nums, Geist Mono on number, Geist on label,
 no photo, no grain, 1:1]
KILL: (kill-list.md §2) + fabricated numbers (use only verified counts from product-marketing.md),
 more than one accent, decorative font outside Geist/JetBrains Mono
```

---

### Usage
1. Choose the shot (A-F) by pillar/calendar slot (`03-visual-identity/higgsfield/shotlist.md`).
2. **Prefer real screencaps first**, high-DPI captures of the shipped demos beat synthetic generation for this ICP (shotlist.md Production notes).
3. **Append `kill-list.md §2`** (KILL block) to every prompt · **`kill-list.md §3`** (anti-generic) to any lensed physical shot.
4. **Without `--soul-id`** (scene-led). Readable text → Nano Banana Pro / Flux 2, never a video model.
5. Run through Pass-1 gate (safe-zones, tokens, kill-list scan) before any render.
