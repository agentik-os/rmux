---
project: rmux
layer: context
produced_by: ads-competitors / market-competitors
status: filled
---
# Competitive Landscape, rmux

## Direct competitors

| Name | Positioning | Price | Lang | Channels | Key weakness vs rmux |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **tmux** | The incumbent terminal multiplexer; ubiquitous, battle-tested | Free (ISC) | C | apt/brew, GitHub, ubiquitous | No typed SDK (must scrape stdout/control-mode); **no native Windows**; C codebase; config DSL only, not built for programmatic agent orchestration |
| **zellij** | "Modern, batteries-included" Rust multiplexer; great human UX (floating panes, WASM plugins) | Free (MIT) | Rust | GitHub, HN, crates.io | **No first-class typed SDK for agents**; plugin model is WASM/human-UX-centric, not "drive every session from Rust code"; no Ratatui-widget story; positioned for humans, not headless agents |
| **GNU screen** | Legacy multiplexer, still default on many boxes | Free (GPL) | C | distro default | Dated UX; minimal scripting; no SDK; no modern snapshot/inspection model |
| **dtach / abduco** | Minimal detach-only wrappers | Free | C | GitHub | Detach only, no panes/windows, no SDK, no inspection; not a real multiplexer |
| **WezTerm (multiplexer mode)** | GPU terminal emulator with built-in multiplexing | Free (MIT) | Rust | GitHub, brew | It's a full terminal *emulator*, not a drop-in CLI multiplexer; Lua config; no tmux-CLI compatibility; no embeddable Rust SDK/widget |

## Indirect / substitutes
- **Hand-rolled `expect` / `pexpect` / raw PTY libraries**, what agent builders use today to drive terminals from code. rmux replaces this with a typed, daemon-backed SDK + snapshots.
- **tmux + shell scripts + `tmux capture-pane`**, the current improvised way to "inspect" an agent's terminal. Fragile stdout-scraping; rmux returns typed `PaneSnapshot`.
- **Docker exec / nohup / `&` / systemd units**, partial answers to "keep it running" but with no interactive reattach, panes, or inspection.
- **Warp**, funded "AI terminal"; an emulator + cloud product, different category, validates spend in terminal UX.

## Positioning gaps WE exploit
1. **The SDK lane is wide open.** No incumbent offers a typed, daemon-backed SDK where *anything the CLI does, code does* (`ensure_session`, `send_text`, `wait_for_text`, `snapshot()`). This is rmux's clearest moat.
2. **"Built for agents" is unclaimed.** tmux/zellij/screen were all designed pre-agent, for humans. rmux explicitly centers long-lived agents over SSH + orchestration, own that narrative before anyone else does.
3. **Inspectable structured state.** Typed `PaneSnapshot` (cols/rows/content) vs everyone else's "scrape the screen." Frame it as: *terminals you can assert on.*
4. **Native Windows.** tmux can't; zellij's Windows story is thin. rmux ships ConPTY + Named Pipes as a first-class platform.
5. **Embeddable in Rust apps.** `ratatui-rmux` puts a live terminal pane inside any Ratatui app, no competitor ships this.
6. **Zero switching cost from tmux.** Full 90-command CLI compatibility + `.tmux.conf` migration fallback. Switchers keep their muscle memory and their config.

## Swipe-worthy angles competitors miss
- **"Your terminal, but with an API."** Multiplexers are CLIs; rmux is a CLI *and* an SDK *and* a widget over one daemon protocol.
- **"tmux for agents."** A single positioning line that instantly explains the wedge to the AI-infra crowd.
- **"Assert on a terminal like it's a DOM."** Snapshot + `wait_for_text` reframes terminal testing the way Playwright reframed browser testing (lean on the shipped Playwright-testing demo).
- **"Same keys. New superpowers."** The compatibility-without-compromise hook tmux switchers need to hear.
- **"Detach an agent. Reattach a week later. Inspect what it did."** A concrete, emotional SSH-agent story none of the human-UX-focused competitors tell.
