---
project: rmux
layer: copy
produced_by: ad-creative (/omg-ad-creative) + ads-copy
pairs_with: 03-visual-identity (visual half, R-VISUAL-ID)
status: filled
---
# Paid Ad Copy, rmux

> NOTE: rmux is free OSS with ~zero paid budget (see gtm-strategy.md). Paid ads are **optional micro-experiments**
> only. This copy is launch-ready if/when a small test runs (X/Reddit/LinkedIn skew to the dev ICP; Meta/TikTok
> are low-fit and included only for completeness). Pair every creative with a visual per 03-visual-identity.

## Angles
- **PAS (Problem-Agitate-Solve):** SSH drops kill your long agent runs → you lose hours and can't see what happened → rmux keeps sessions alive and inspectable.
- **AIDA:** "tmux for agents" → detach/inspect from code → typed SDK + snapshots → `cargo install rmux`.
- **BAB (Before-After-Bridge):** Before: tmux + brittle stdout-scraping. After: typed `snapshot()` and `wait_for_text`. Bridge: rmux.
- **4Ps (Promise-Picture-Proof-Push):** Promise: program your terminal. Picture: N agents orchestrated from one daemon. Proof: all 90 tmux cmds, verified on 3 OSes, 5 demos. Push: install now.

## Meta (Facebook/Instagram, low ICP fit; awareness only)
- **Headline:** Your terminal, now with an SDK.
- **Primary text:** rmux is a Rust terminal multiplexer that keeps the full tmux CLI you know and adds a typed SDK, so you can drive, snapshot, and orchestrate sessions from code. Built for long-lived agents over SSH. Free & open-source.
- **Description:** All 90 tmux commands · Linux/macOS/Windows · `cargo install rmux`
- **CTA:** Learn More

## Google (Search RSA, target "tmux SDK", "tmux alternative", "drive terminal from code", "tmux windows")
**Headlines (15, ≤30 chars):**
1. tmux for the Agentic Era
2. The Rust tmux Alternative
3. Drive Your Terminal in Code
4. tmux + a Typed SDK
5. All 90 tmux Commands
6. tmux, Now on Windows
7. Terminal Multiplexer in Rust
8. Inspect & Script Sessions
9. Detachable. Inspectable.
10. Open-Source & MIT/Apache
11. Embed a Terminal in Rust
12. `cargo install rmux`
13. Migrate Your .tmux.conf
14. Run Agents Over SSH
15. Snapshot Your Terminal

**Descriptions (4, ≤90 chars):**
1. Rust multiplexer with the full tmux CLI plus a typed, daemon-backed SDK. Free & open.
2. Detach, reattach, and inspect long-lived agents over SSH. Linux, macOS, and Windows.
3. Anything the CLI does, your code does too, typed snapshots, no stdout scraping.
4. All 90 tmux commands.tmux.conf migration, forbid-unsafe. `cargo install rmux`.

## LinkedIn (best paid fit for the dev/infra ICP)
- **Headline:** The terminal multiplexer built for AI agents.
- **Primary text:** Running long-lived coding agents over SSH? rmux is a Rust, tmux-compatible multiplexer with a daemon-backed typed SDK, detach an agent, reattach later, and inspect exactly what it did from code. Three surfaces (CLI, `rmux-sdk`, `ratatui-rmux`) over one local protocol. All 90 tmux commands, Linux/macOS/Windows, `#![forbid(unsafe_code)]`, no network at runtime. Open-source (MIT/Apache).
- **CTA:** Star on GitHub / Read the docs

## TikTok (low fit, only a dev-creator partnership angle)
- **Hook (on-screen, 0-2s):** "POV: your 3-hour agent run survives your wifi dropping."
- **Script beat:** show split terminal → SSH disconnect → reattach → `snapshot()` printing what the agent did. Caption: "rmux = tmux for agents. Rust. Open-source."
- **CTA:** `cargo install rmux`
