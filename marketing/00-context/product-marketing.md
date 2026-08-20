---
project: rmux
layer: context
produced_by: product-marketing-context (/omg-product-marketing-context)
inputs: [README.md, README.fr.md, RULES.md, docs/README.md, spec/feature-inventory-v1.yaml, Cargo.toml]
status: filled
---
# Product Marketing Context, rmux

> SSOT every other marketing file reads (R-MARKETING: run FIRST). No `.agents/product-marketing.md`
> existed at generation time; this file is the canonical context. No `brand-book/` or `VISION.md`/`PRD*.md`
> found in the repo, context below is grounded strictly in shipped artifacts (see Inputs cited below).

### Inputs cited (R-CITE)
- `README.md`, product one-liner, three public surfaces, demos, install, architecture, platform table, v0.3.1 status.
- `README.fr.md`, French positioning parity (confirms multilingual intent: EN · FR · 简体中文 · 日本語).
- `RULES.md`, canonical project identity, stack, fork/upstream relationship, objective.
- `docs/README.md`, fork-vs-upstream distribution reality (rmux.io/docs belongs to upstream).
- `spec/feature-inventory-v1.yaml`, VERIFIED feature surface (SDK facade, ensure-session, snapshots, lifecycle) per-platform pass matrix.
- `Cargo.toml` / workspace crates, `rmux`, `rmux-sdk`, `ratatui-rmux`, daemon/IPC crates; dual MIT OR Apache-2.0.

## One-liner
**The universal Rust terminal multiplexer for the agentic era, detachable, scriptable, and inspectable, with a tmux-compatible CLI, a daemon-backed typed SDK, and native Ratatui integration.** (`README.md`)

## Category & positioning
- **Category:** terminal multiplexer / terminal session runtime (the tmux/zellij/screen lineage).
- **Sub-category we create:** the *programmable, agent-ready* multiplexer, a multiplexer with a real typed SDK and structured, inspectable pane snapshots, not just a human TUI.
- **Positioning statement:** For developers and AI-agent builders who run long-lived terminal work over SSH and need to inspect, script, and orchestrate it, **rmux** is a Rust terminal multiplexer that keeps the full tmux-compatible CLI (90 commands) you already know while adding a daemon-backed SDK (`rmux-sdk`) and a native Ratatui widget (`ratatui-rmux`), so anything a human can do at the prompt, code can do programmatically. Unlike tmux (C, human-only, no SDK, no native Windows) and zellij (batteries-included human UX, no programmatic agent surface), rmux is built so humans, headless CLI workflows, and autonomous agents share one runtime.
- **Honest scope note (load-bearing):** this checkout is the `agentik-os/rmux` fork; the hosted channels (`rmux.io`, `crates.io`, helvesec GitHub releases) belong to **upstream `Helvesec/rmux`** (`README.md`, `docs/README.md`). Marketing claims describe the rmux product/codebase; any "go connect/publish" step on owned channels must respect that the public distribution is upstream's. Build-from-source: `cargo install --path . --locked`.

## ICP (ideal customer profile)
1. **AI-agent infrastructure engineers**, running long-lived coding/autonomous agents (Claude Code, Codex-style, custom agents) over SSH, who need to detach, reattach, inspect state, and orchestrate many sessions programmatically. **Primary ICP.**
2. **Rust / TUI developers**, already in the Ratatui ecosystem; want to embed a live terminal pane in their own app via `ratatui-rmux`, or drive sessions from Rust via `rmux-sdk`.
3. **Power-user tmux refugees**, heavy tmux users who want the same muscle memory plus native Windows support, scriptability, and Rust safety.
4. **Platform / DevOps engineers**, headless CI, remote automation, and terminal orchestration where "no network at runtime" and locked, reproducible builds matter.
5. **QA / automation engineers**, terminal automation and Playwright-style testing of TUIs via structured snapshots (`pane.snapshot()`, `wait_for_text`).

## Buyer personas (summary, detail in audience-personas.md)
- **"Orchestrator Olivia"**, agent-infra engineer wiring N agents over SSH; pain = lost terminals + no programmatic state. (Primary)
- **"Rustacean Ravi"**, Ratatui/CLI builder; wants a typed, safe SDK instead of scraping tmux stdout.
- **"tmux-veteran Theo"**, 10-year tmux user, now on Windows half the time; wants parity + Windows + scripts.
- **"Platform Priya"**, DevOps lead; cares about `#![forbid(unsafe_code)]`, no-network-at-runtime, locked builds.
- **"QA Quinn"**, automates terminal apps; wants snapshot/assert primitives.

## Core value proposition
**Keep the tmux you know; gain the SDK you wished it had.** rmux gives every terminal session three equal surfaces, CLI, Rust SDK, Ratatui widget, over one local daemon protocol, so a session you start by hand can be detached, reattached, snapshotted, and driven from code on Linux, macOS, **and** Windows.

## Differentiators / moat
- **Tmux-compatible CLI, 90 commands**, zero relearning; drop-in muscle memory (`README.md`).
- **Daemon-backed typed SDK (`rmux-sdk`)**, `ensure_session`, `send_text`, `wait_for_text`, `snapshot()`, programmatic control without stdout-scraping (`spec/feature-inventory-v1.yaml`, VERIFIED).
- **Structured, inspectable snapshots**, panes return typed `PaneSnapshot` (cols/rows/content), the foundation for agent inspection and terminal testing.
- **Native Ratatui widget (`ratatui-rmux`)**, embed a live, real terminal pane inside any Rust TUI.
- **True cross-platform incl. Windows**, ConPTY + Named Pipes; tmux has no native Windows.
- **Rust safety posture**, `#![forbid(unsafe_code)]` in upper crates; OS/PTY boundary isolated; no network at runtime (guardrail script).
- **Built for the agentic era**, the explicit design center is long-lived agents over SSH + orchestration (the README's "Why" and the orchestration/broadcast demos).

## Messaging pillars (3-5)
1. **Compatible, not a compromise**, the full tmux CLI you already type, rebuilt in Rust.
2. **One runtime, three surfaces**, CLI = SDK = widget; anything one can do, the others can too.
3. **Inspectable & scriptable by design**, typed snapshots and a real SDK make terminals programmable, not scrape-able.
4. **Made for agents (and the humans who run them)**, detach, reattach, orchestrate long-lived agents over SSH.
5. **Fast, safe, everywhere**, Rust, fat-LTO release, forbid-unsafe, Linux/macOS/Windows.

## Proof / social proof
- v0.3.1 public preview, **all 90 tmux-compatible commands implemented** (`README.md`).
- Per-platform **VERIFIED** feature matrix across Linux/macOS/Windows in `spec/feature-inventory-v1.yaml`.
- Five real demos shipped (Multi-Agent Orchestration ~514 LOC, Agent Broadcast Arena ~2,171 LOC, Mini-Zellij ~944 LOC, Terminal↔Browser Mirroring ~649 LOC, Playwright Testing ~1,495 LOC) (`README.md`).
- Dual MIT OR Apache-2.0 license; CI release-validation badge; restricted-unsafe policy.
- (Stars / crates.io download counts: track upstream, see honest scope note. Do not fabricate metrics, R-CITE.)

## Pricing & monetization context
- **Free, open-source** (dual MIT/Apache-2.0). No paid tier. "Conversion" = installs, `crates.io` downloads, `rmux-sdk`/`ratatui-rmux` adoption, GitHub stars, and contributors, not revenue.
- GTM is therefore **developer-led / community-led / content-led**, optimizing for adoption and ecosystem pull, not ARR.

## Voice & tone
- **Engineer-to-engineer.** Precise, technical, no marketing fluff; show the command, show the snapshot, show the diff.
- Confident but honest, name the fork/upstream reality, name "bugs expected, it's a preview."
- Rust-native register: "blazing-fast", "typed", "locked", "forbid-unsafe" are credible here; hype adjectives without a code artifact are not.
- Outward copy in **English** (project README is EN-primary, multilingual). Code, commits, identifiers in English.
