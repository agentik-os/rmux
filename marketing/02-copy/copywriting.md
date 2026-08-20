---
project: rmux
layer: copy
produced_by: mk-copywriting
status: filled
---
# Core Copy, rmux

> Engineer-to-engineer voice (product-marketing.md). Show the command, not the adjective.
> All claims grounded in README.md / spec. English (project is EN-primary).

## Taglines (5)
1. **tmux for the agentic era.**
2. **Same keys. New superpowers.**
3. **Your terminal, now with an SDK.**
4. **Detachable. Scriptable. Inspectable.**
5. **The multiplexer humans and agents share.**

## Hero headline + subhead variants

**A, Agent wedge (primary)**
> ### The Rust multiplexer built for agents and humans.
> Run long-lived agents over SSH, detach, reattach, and inspect them from typed Rust code, with the full tmux-compatible CLI you already know.

**B, tmux switcher**
> ### All 90 tmux commands. In Rust. Now on Windows.
> rmux keeps your muscle memory and migrates your `.tmux.conf`, then adds a daemon-backed SDK and structured snapshots tmux never had.

**C, SDK / developer**
> ### Anything the CLI can do, your code can do too.
> One local daemon, three surfaces: the `rmux` CLI, the `rmux-sdk` Rust crate, and the `ratatui-rmux` widget. Drive, snapshot, and orchestrate terminals programmatically.

## Value-prop blocks
- **Compatible, not a compromise.** The full tmux-compatible CLI, all 90 commands implemented, so switching costs you nothing but a `cargo install`.
- **Programmable by design.** `ensure_session`, `send_text`, `wait_for_text`, `snapshot()`, typed, daemon-backed, no stdout-scraping.
- **Inspectable state.** Panes return a structured `PaneSnapshot` (cols, rows, content). Assert on a terminal like it's a DOM.
- **Built for the agentic era.** Detach a long-running agent over SSH, reattach later, inspect what it did, orchestrate many at once.
- **Fast, safe, everywhere.** Rust with fat-LTO release builds, `#![forbid(unsafe_code)]` in the upper crates, no network at runtime, Linux, macOS, and Windows (ConPTY + Named Pipes).
- **Embeddable.** `cargo add ratatui-rmux` drops a live terminal pane into any Ratatui app.

## Feature → benefit table

| Feature (README/spec) | Benefit |
| :--- | :--- |
| Tmux-compatible CLI, 90 commands | Zero relearning; your keybindings and `.tmux.conf` come with you |
| `rmux-sdk` daemon-backed typed SDK | Drive terminals from code without scraping stdout |
| Structured `PaneSnapshot` (cols/rows/content) | Inspect and test terminal state deterministically |
| `wait_for_text` + `snapshot()` | Playwright-style assertions for TUIs/CLIs |
| `ratatui-rmux` widget | Embed a live terminal pane in your own Rust TUI |
| Persistent, detachable sessions | Long-lived agents survive SSH drops; reattach anytime |
| ConPTY + Named Pipes | Native Windows, not just WSL |
| `#![forbid(unsafe_code)]` + no network at runtime | Infra-grade safety and supply-chain posture |
| `.tmux.conf` migration fallback | Bring your existing config; opt out with one env var |
| Graphics passthrough (Kitty/SIXEL) | Images and rich output survive the multiplexer |

## CTAs
- **`curl -fsSL https://rmux.io/install.sh | sh`** *(upstream-hosted installer)*
- **`cargo install rmux --locked`**
- **`cargo add rmux-sdk`**, drive it from code
- **Star it on GitHub** → `Helvesec/rmux`
- **Read the docs** → rmux.io/docs
- **File an issue**, it's a fresh preview, your bug report shapes v0.4

## Objection-handling copy
- **"I'm not relearning tmux."** → You won't. All 90 tmux commands are implemented and your `.tmux.conf` migrates automatically. Same keys, same workflow.
- **"Why not just tmux + control mode?"** → Because control-mode means parsing text. rmux gives you a *typed* SDK, `wait_for_text`, typed `PaneSnapshot`, that doesn't break when output formatting changes.
- **"Is it stable?"** → It's an honest public preview (v0.3.1); all 90 commands work, the feature matrix is verified across Linux/macOS/Windows, and bugs get fixed fast. File one and watch.
- **"zellij already exists."** → zellij is a great *human* UX. rmux is built for the case zellij doesn't serve: driving and inspecting terminals (and agents) from code, with full tmux-CLI compatibility.
- **"Does it phone home?"** → No. No network at runtime (enforced by a build guardrail), `forbid(unsafe_code)` in the upper crates, dual MIT/Apache, locked builds.
- **"Windows really?"** → Yes, native ConPTY + per-user Named Pipes, not a WSL shim.
