---
project: rmux
layer: visual-identity/higgsfield
produced_by: higgsfield-generate (per-shot layer)
status: filled
---
# Higgsfield PRE-PROMPT template, rmux

> Per-generation prompt template. Combine SYSTEM (system-prompt.md) + this PRE-PROMPT + shot variables.
> rmux uses NO soul-id (product/scene-led). Fill `{...}` from the shotlist + content-strategy pillars.

## Template
```
{SYSTEM}
Subject: {subject}
Scene/context: {scene}
Format: {aspect_ratio} for {platform}
Action/emotion: {action}
Lighting: {lighting}
--- on-brand constraints from DA: palette = canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590,
ONE mint #36E2C4 focal accent only; monospaced wordmark "rmux"; multiplex-grid composition; matte,
engineered, calm; real plausible shell/Rust text; {extra_style}
```

## Variable presets (per content pillar, from content-strategy.md)

### Pillar 1, Agentic Terminal (flagship hero)
- subject: a dark terminal split into 3-4 panes, each running a labeled agent; one pane active with a mint cursor
- scene: an orchestration session over SSH; subtle "agent running" amber status glyph on one pane
- action: calm control, sessions persisting, one pane highlighted as detach/reattach happens
- lighting: flat UI glow, single mint focal light
- aspect: 16:9 (OG/demo), 1:1 (social), 9:16 (short)
- extra_style: clean #1E2733 pane dividers; real-looking agent log lines; no people

### Pillar 2, Programmable Terminal (SDK)
- subject: side-by-side, left a Rust code panel (`rmux_sdk` snippet), right a live terminal pane responding
- scene: code driving a terminal; a typed `PaneSnapshot` rendered as structured fields
- action: the snapshot resolving; `wait_for_text("ready")` flashing mint
- lighting: even, low-noise
- aspect: 16:9, 4:5
- extra_style: monospace alignment, plausible Rust; mint only on the active result

### Pillar 3, tmux, Upgraded (comparison/conversion)
- subject: a clean "rmux vs tmux" split frame, or a `.tmux.conf` migrating into rmux
- scene: muscle-memory keys + a Windows badge (native Windows angle)
- action: a tmux command running identically in rmux
- lighting: flat
- aspect: 1:1, 16:9
- extra_style: restrained, comparison grid, mint accent on the rmux side only

### Pillar 4, Terminal Testing
- subject: a test panel asserting on terminal output; green/mint pass state; `wait_for_text` + snapshot
- scene: "assert on a terminal like a DOM"; CI-style pass row
- action: a passing assertion turning mint
- lighting: even
- aspect: 16:9, 4:5
- extra_style: deterministic, clean; one mint "PASS"

### Pillar 5, Engineering / Build-in-Public
- subject: the architecture diagram, three surfaces (CLI / rmux-sdk / ratatui-rmux) → one daemon
- scene: box-and-connector schematic on canvas-black; or a forbid-unsafe / no-network badge motif
- action: static, authoritative
- lighting: schematic, flat
- aspect: 16:9 (OG), 1:1
- extra_style: thin #1E2733 connectors, mint on the daemon node only; matte
```
EXAMPLE (filled, Pillar 1, OG card):
{SYSTEM}
Subject: a near-black terminal split into four panes, each labeled with a running agent, the top-left pane active with a blinking mint cursor
Scene/context: a multi-agent orchestration session over SSH, one pane showing a small amber "running" glyph
Format: 16:9 for an OpenGraph card
Action/emotion: calm, controlled orchestration; quiet power
Lighting: flat UI glow with a single mint focal light on the active pane
--- on-brand constraints from DA: palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced "rmux" wordmark lower-left; multiplex-grid composition; matte, engineered, calm; real plausible shell log lines; no people
```
