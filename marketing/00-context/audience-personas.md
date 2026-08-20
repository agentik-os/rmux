---
project: rmux
layer: context
produced_by: ads-audience
status: filled
---
# Audience Personas, rmux

> 5 forensic personas for an open-source Rust dev tool. "Targeting" here is mostly **organic
> community placement** (subreddits, HN, newsletters, Discords) since rmux is free OSS with
> negligible paid spend; paid-channel notes included where a small experiment could make sense.

---

## 1. "Orchestrator Olivia", AI-Agent Infrastructure Engineer (PRIMARY)
- **Demographics:** 27-40, senior/staff eng or founding engineer at an AI startup or platform team; remote; works in `$SHELL` + SSH all day.
- **Psychographics:** systems thinker, automates everything, allergic to GUIs, reads source before docs, early adopter of agent tooling.
- **Pains:** long-running agents over SSH die on disconnect; no way to inspect what an agent's terminal is doing programmatically; orchestrating N agents = a pile of brittle tmux + shell glue; stdout-scraping breaks constantly.
- **Triggers:** "I lost a 3-hour agent run when my SSH dropped"; "I need to know what the agent typed without attaching"; a new agent-orchestration framework launch.
- **Objections:** "Why not just tmux + control mode?" / "Is the SDK actually typed and stable?" / "Will it run headless in CI?"
- **Where they live:** HN, X (AI-infra/agent circle), r/rust, GitHub, niche agent-tooling Discords.
- **Message that lands:** *"tmux for agents, detach, reattach, and inspect long-lived agent sessions from typed Rust code."*
- **Paid (optional):** X promoted posts targeting followers of agent-framework / Rust accounts.

## 2. "Rustacean Ravi", Rust / TUI Developer
- **Demographics:** 24-38, Rust dev building CLIs/TUIs, often in the Ratatui ecosystem; OSS contributor.
- **Psychographics:** values type safety, zero-unsafe, clean APIs; will adopt a crate if the API is elegant and the docs have a real example.
- **Pains:** wants to embed a live terminal pane in a Ratatui app; doesn't want to write PTY/IPC plumbing; tmux integration means shelling out and parsing text.
- **Triggers:** "I need a terminal widget in my TUI"; browsing crates.io / This Week in Rust; a Ratatui showcase thread.
- **Objections:** "Is `ratatui-rmux` maintained?" / "How heavy is the dependency?" / "Does the SDK lock me to a daemon?"
- **Where they live:** r/rust, Ratatui Discord, This Week in Rust, lobste.rs, crates.io.
- **Message that lands:** *"`cargo add ratatui-rmux`, a live terminal pane in your TUI, backed by a typed SDK."*

## 3. "tmux-veteran Theo", Power-User / Platform Engineer
- **Demographics:** 30-50, 8+ years of tmux, hundreds of lines of `.tmux.conf`, now on Windows for half his work.
- **Psychographics:** efficiency maximalist, deeply invested in muscle memory, skeptical of "new and shiny", will not relearn keybindings.
- **Pains:** tmux has no native Windows; juggling WSL is friction; wants scriptability without leaving his config behind.
- **Triggers:** new laptop/Windows machine; a "modern tmux alternative" thread; frustration with a tmux limitation.
- **Objections:** "I'm not relearning tmux." / "Will my `.tmux.conf` work?" / "Is it actually compatible or just 'inspired by'?"
- **Where they live:** r/commandline, r/tmux, HN, lobste.rs, dotfiles communities.
- **Message that lands:** *"All 90 tmux commands. Your `.tmux.conf` migrates. Now on Windows too."*

## 4. "Platform Priya", DevOps / SRE Lead
- **Demographics:** 30-45, owns CI/CD and remote automation for a mid-size eng org.
- **Psychographics:** security- and reproducibility-minded; vets dependencies; cares about supply-chain posture.
- **Pains:** needs reliable headless terminal automation; nervous about unsafe code and runtime network calls in infra tooling.
- **Triggers:** a security review; a flaky CI terminal job; standardizing remote-automation tooling.
- **Objections:** "What's the unsafe/network posture?" / "Is it locked/reproducible?" / "Windows + Linux from one tool?"
- **Where they live:** r/devops, HN, internal platform Slacks, security-conscious newsletters.
- **Message that lands:** *"`#![forbid(unsafe_code)]`, no network at runtime, locked builds, Linux/macOS/Windows, one multiplexer for your whole fleet."*

## 5. "QA Quinn", Test / Automation Engineer
- **Demographics:** 26-42, builds E2E and integration suites, increasingly testing CLIs/TUIs.
- **Psychographics:** assertion-driven; loves deterministic, snapshot-based tests; already uses Playwright for web.
- **Pains:** no good way to assert on a terminal app's output; existing approaches scrape raw bytes.
- **Triggers:** a TUI app to test; flaky `expect` scripts; discovering the Playwright-for-terminal demo.
- **Objections:** "Can I `wait_for_text` and snapshot reliably?" / "Does it work in CI containers?"
- **Where they live:** r/QualityAssurance, testing Discords, dev.to, HN.
- **Message that lands:** *"Assert on a terminal like it's a DOM, `wait_for_text`, typed snapshots, deterministic terminal tests."*

---

## Negative audiences (do NOT target)
- Non-technical users / general consumers, zero fit; rmux is a developer primitive.
- GUI-only / no-terminal developers.
- People seeking a paid/enterprise product with SLAs, it's a free OSS preview.
- Anti-Rust zealots and "tmux is perfect, never changing" hardliners, high friction, low conversion; let advocacy reach them indirectly.
- Enterprises needing a vendor-supported, certified tool today (preview stage).
