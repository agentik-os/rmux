---
project: rmux
layer: copy
produced_by: social-content (/omg-social-content)
status: filled
---
# Organic Social Content, rmux

> Each item maps to a slot in 04-publishing/calendar.json. Voice = engineer-to-engineer.
> Primary organic channels: X, Reddit, LinkedIn, GitHub Discussions, HN (manual). Show real commands/output.

## Hooks bank
- "tmux is great. But it was built for humans, not agents."
- "Your terminal has no API. rmux fixes that."
- "I lost a 3-hour agent run to a dropped SSH connection. So I started using rmux."
- "All 90 tmux commands. Rewritten in Rust. Now on Windows."
- "What if you could `wait_for_text()` on a terminal like it's a webpage?"
- "Stop scraping `capture-pane`. Get a typed snapshot instead."
- "Same keybindings as tmux. Plus an SDK. Plus a Ratatui widget."
- "Detach an agent. Reattach a week later. Inspect everything it did."
- "One daemon. Three surfaces. CLI = SDK = widget."
- "`#![forbid(unsafe_code)]` in a terminal multiplexer. In Rust. Of course."

## LinkedIn (1-2×/week, Build-in-Public + Programmable pillars)
**Post 1, the why**
> Most terminal multiplexers were designed before AI agents existed.
>
> tmux and zellij are built for humans at a keyboard. But more and more of my terminal time is now *agents* running over SSH, and they need something different: to be detached, reattached, and **inspected from code**.
>
> That's why rmux exists. It's a Rust, tmux-compatible multiplexer with a daemon-backed typed SDK. Anything the CLI can do, your code can do too, `ensure_session`, `send_text`, `wait_for_text`, `snapshot()`.
>
> Same keys you already know. A real API you always wanted. Open-source (MIT/Apache).
>
> `cargo install rmux` → github.com/Helvesec/rmux

**Post 2, the SDK**
> Driving a terminal from code used to mean scraping stdout and praying the format never changed.
>
> rmux returns a typed `PaneSnapshot`, cols, rows, content, and lets you `wait_for_text("ready")` before reading. Deterministic. Testable. No regex archaeology.
>
> Here's the whole quickstart 👇 [SDK snippet]

## X / Twitter (3-4×/week)
**Thread, "tmux for agents" (flagship)**
> 1/ I kept losing long-running agent sessions when my SSH dropped. tmux kept them alive, but I couldn't *inspect* what the agent did without attaching and squinting.
>
> 2/ So the fix wasn't "a better TUI." It was: give the terminal an API.
>
> 3/ rmux = a Rust, tmux-compatible multiplexer (all 90 commands) + a daemon-backed typed SDK. CLI, `rmux-sdk`, and a `ratatui-rmux` widget, all over one local protocol.
>
> 4/ `pane.send_text(...)` → `pane.wait_for_text("ready")` → `pane.snapshot()`. Typed cols/rows/content. No stdout scraping.
>
> 5/ Detach an agent. Reattach later. Read exactly what it did. Orchestrate N of them.
>
> 6/ Linux, macOS, AND Windows (ConPTY + Named Pipes). forbid-unsafe. No network at runtime. MIT/Apache.
>
> 7/ `cargo install rmux`, it's a fresh public preview, file issues, they get fixed. github.com/Helvesec/rmux

**Single posts**
- "Your `.tmux.conf` migrates automatically. Your muscle memory comes along. rmux just adds the SDK tmux never had. [GIF]"
- "Assert on a terminal like it's a DOM: `wait_for_text` + typed snapshots. Terminal testing, finally deterministic. [demo link]"
- "tmux on Windows? With rmux, natively, ConPTY + Named Pipes, no WSL shim."

## Instagram / TikTok (scripts, low priority, dev-creator only)
**Reel/Short (≤30s):**
- 0-2s (hook, on-screen): "Your terminal needs an API."
- 2-10s: screen-record split panes; an agent running; SSH drops; reattach instantly.
- 10-22s: code panel, `send_text` → `wait_for_text("ready")` → `snapshot()` prints content.
- 22-30s (CTA card, brand DA): "rmux, tmux for agents. Rust. Open-source. `cargo install rmux`."

## Reddit / communities (1×/week, value-first, NEVER announcement-spam)
- **r/rust:** "I built a tmux-compatible multiplexer in Rust with a typed SDK, here's the daemon + 3-surface architecture and why." (lead with the engineering, link last)
- **r/commandline:** "tmux users: rmux migrates your `.tmux.conf` and runs natively on Windows, AMA about compatibility."
- **r/devops:** "Driving long-lived terminal jobs over SSH from code: typed snapshots instead of `capture-pane` scraping."
- **Ratatui Discord:** "`cargo add ratatui-rmux`, drop a live terminal pane into your TUI. Feedback welcome."
- **HN (manual, launch only):** "Show HN: rmux, tmux for agents (Rust multiplexer with a typed SDK)."
