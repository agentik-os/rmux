---
project: rmux
layer: calendar
horizon: 14 days
window: J1 (Wed 2026-07-02) → J14 (Tue 2026-07-15)
timezone: Europe/Paris
cadence: 1-3 posts/day (per 01-strategy/content-strategy.md)
status: filled
---
# 14-Day Plan, rmux

> Legend MODE: 🤖 AUTO (auto-posted via poster) · 🙋 MANUAL (you do it by hand).
> Voice = engineer-to-engineer. Show the command, not the adjective. Every post below carries its **full ready-to-publish text**.
> Channels & cadence reused from `01-strategy/content-strategy.md`; copy reused from `02-copy/*`; AUTO visuals = filled Higgsfield prompts from `03-visual-identity/higgsfield/`.
>
> **Manual count over 14d = 4** (staggered, max 1/day): J3 Reddit r/rust · J8 screencast recording · J10 Reddit r/commandline AMA · J13 Show HN. Everything else is 🤖 AUTO.
> **Honest scope note (from product-marketing.md):** public distribution channels (rmux.io, crates.io, Helvesec releases) are upstream `Helvesec/rmux`. Copy describes the product; links point to `github.com/Helvesec/rmux`.

---

### J1, Wednesday · 2026-07-02
- **Post 1** · 09:00 · LinkedIn · pillar **Build-in-Public** · 🤖 AUTO · ~10 min
 - **Text:**
  > Most terminal multiplexers were designed before AI agents existed.
  >
  > tmux and zellij are built for humans at a keyboard. But more and more of my terminal time is now *agents* running over SSH, and they need something different: to be detached, reattached, and **inspected from code**.
  >
  > That's why rmux exists. It's a Rust, tmux-compatible multiplexer with a daemon-backed typed SDK. Anything the CLI can do, your code can do too, `ensure_session`, `send_text`, `wait_for_text`, `snapshot()`.
  >
  > Same keys you already know. A real API you always wanted. Open-source (MIT/Apache).
  >
  > `cargo install rmux` → github.com/Helvesec/rmux
 - **Visuel :** 🤖 Higgsfield (Pillar 5, 1:1), `{SYSTEM} Subject: a box-and-connector architecture diagram, three labeled surfaces (CLI / rmux-sdk / ratatui-rmux) converging on one central "daemon" node. Scene/context: schematic on canvas-black, thin #1E2733 connectors. Format: 1:1 for LinkedIn. Action/emotion: authoritative, calm, engineered order. Lighting: flat schematic glow, single mint focal light on the daemon node only. --- on-brand constraints: palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent on the daemon node; monospaced "rmux" wordmark lower-left; matte; no people.`
 - **CTA :** `cargo install rmux` · Star → github.com/Helvesec/rmux

- **Post 2** · 14:00 · X/Twitter · pillar **Agentic Terminal (flagship thread)** · 🤖 AUTO · ~12 min
 - **Text:**
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
 - **Visuel :** 🤖 Higgsfield (Pillar 1, 16:9, on post 3/), `{SYSTEM} Subject: a near-black terminal split into four panes, each labeled with a running agent, top-left pane active with a blinking mint cursor. Scene/context: a multi-agent orchestration session over SSH, one pane showing a small amber "running" glyph. Format: 16:9. Action/emotion: calm, controlled orchestration; quiet power. Lighting: flat UI glow with a single mint focal light on the active pane. --- on-brand constraints: palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced "rmux" wordmark lower-left; multiplex-grid composition; matte; real plausible shell log lines; no people.`
 - **CTA :** `cargo install rmux`

---

### J2, Thursday · 2026-07-03
- **Post 1** · 10:00 · X/Twitter · pillar **tmux, Upgraded** · 🤖 AUTO · ~6 min
 - **Text:**
  > Your `.tmux.conf` migrates automatically. Your muscle memory comes along. Same keybindings, same workflow.
  >
  > rmux just adds the SDK tmux never had, typed snapshots, `wait_for_text`, a Ratatui widget.
  >
  > Switching costs you one `cargo install`. 👇
 - **Visuel :** 🤖 Higgsfield (Pillar 3, 1:1), `{SYSTEM} Subject: a clean "tmux → rmux" migration frame, a .tmux.conf file on the left flowing into an identical rmux session on the right, a small native "Windows" badge in the corner. Scene/context: muscle-memory keys carried across. Format: 1:1. Action/emotion: effortless continuity. Lighting: flat. --- on-brand constraints: palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent on the rmux side only; monospaced "rmux" wordmark; comparison grid; matte; no people.`
 - **CTA :** GIF demo + `cargo install rmux`

- **Post 2** · 16:00 · dev.to (blog) · pillar **Programmable Terminal** · 🤖 AUTO · ~35 min
 - **Text (full post):**
  > # Drive a terminal from Rust: the rmux-sdk quickstart
  >
  > Automating a terminal has always meant one ugly thing: spawn a process, scrape its stdout, and pray the format never changes. `tmux` has control mode, but you're still parsing text.
  >
  > rmux takes a different route. It's a tmux-compatible multiplexer with a daemon behind it, and that daemon exposes a **typed** SDK. Here's the whole loop.
  >
  > ## Install
  > ```bash
  > cargo install rmux --locked
  > cargo add rmux-sdk
  > ```
  >
  > ## Ensure a session, send input, wait, snapshot
  > ```rust
  > use rmux_sdk::connect_or_start;
  >
  > let client = connect_or_start()?;       // daemon lifecycle handled for you
  > let pane = client.ensure_session("build")?;  // idempotent: attach or create
  >
  > pane.send_text("cargo build --release\n")?;
  > pane.wait_for_text("Compiling")?;       // block until it's actually running
  >
  > let snap = pane.snapshot()?;          // typed PaneSnapshot
  > println!("{}x{}", snap.cols, snap.rows);
  > assert!(snap.content.contains("Compiling"));
  > ```
  >
  > No regex archaeology. `snapshot()` returns a structured `PaneSnapshot`, `cols`, `rows`, `content`, so you assert on terminal state like it's a DOM node.
  >
  > ## Why this matters
  > A session you started by hand can be detached, reattached, snapshotted, and driven from code, on Linux, macOS, and Windows. The same daemon powers three surfaces: the `rmux` CLI, the `rmux-sdk` crate, and the `ratatui-rmux` widget. Anything one can do, the others can too.
  >
  > It's an honest v0.3.1 preview, all 90 tmux commands work, bugs get fixed fast. Try it and file an issue: **github.com/Helvesec/rmux**
 - **Visuel :** 🤖 Higgsfield (Pillar 2, 16:9 cover), `{SYSTEM} Subject: side-by-side, left a Rust code panel showing an rmux_sdk snippet (ensure_session, send_text, wait_for_text, snapshot), right a live terminal pane responding with a typed PaneSnapshot rendered as structured fields. Scene/context: code driving a terminal. Format: 16:9 blog cover. Action/emotion: the snapshot resolving; wait_for_text("ready") flashing mint. Lighting: even, low-noise. --- on-brand constraints: palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, mint #36E2C4 only on the active result; monospace alignment; plausible Rust; matte; no people.`
 - **CTA :** Read → link the post from X; `cargo add rmux-sdk`

---

### J3, Friday · 2026-07-04
- **Post 1** · 11:00 · Reddit r/rust · pillar **Engineering / Build-in-Public** · 🙋 MANUAL · ~35 min
 - **Text (post body):**
  > **Title:** I built a tmux-compatible multiplexer in Rust with a typed SDK, here's the daemon + 3-surface architecture and why
  >
  > I run a lot of long-lived work over SSH, builds, TUIs, and increasingly autonomous agents. tmux kept sessions alive but I couldn't *inspect* them from code without scraping `capture-pane`. So I built rmux around one idea: give the terminal a real API.
  >
  > **Architecture:** one local daemon, three equal surfaces over the same IPC protocol,
  > - the `rmux` CLI (all 90 tmux-compatible commands),
  > - `rmux-sdk` (a typed Rust crate: `ensure_session`, `send_text`, `wait_for_text`, `snapshot()`),
  > - `ratatui-rmux` (drop a live terminal pane into any Ratatui app).
  >
  > Panes return a structured `PaneSnapshot { cols, rows, content }` instead of a blob of bytes, so you can assert on terminal state deterministically. `#![forbid(unsafe_code)]` in the upper crates, the OS/PTY boundary isolated, no network at runtime (enforced by a build guardrail). Cross-platform including native Windows via ConPTY + Named Pipes, no WSL shim.
  >
  > It's a v0.3.1 public preview, dual MIT/Apache. Happy to go deep on the daemon protocol, the snapshot model, or the Windows PTY work in the comments. Repo: github.com/Helvesec/rmux
 - **Visuel :** 🙋 MANUAL, no image; Reddit r/rust favors text + inline code. Optionally attach the same architecture asciinema/GIF used on J8. Lead with engineering, link last, then reply to every comment for the first 2 hours.
 - **CTA :** engage in comments (no hard CTA); repo link at the end

- **Post 2** · 15:00 · X/Twitter · pillar **Programmable Terminal** · 🤖 AUTO · ~6 min
 - **Text:**
  > Stop scraping `capture-pane`. Get a typed snapshot instead.
  >
  > ```rust
  > let snap = pane.snapshot()?; // PaneSnapshot { cols, rows, content }
  > assert!(snap.content.contains("ready"));
  > ```
  >
  > Deterministic. Testable. No regex archaeology. That's the rmux-sdk.
 - **Visuel :** 🤖 Higgsfield (Pillar 2, 4:5), `{SYSTEM} Subject: a single Rust code card, pane.snapshot() returning a typed PaneSnapshot with cols/rows/content fields highlighted. Scene/context: the typed fields resolving cleanly. Format: 4:5. Action/emotion: precision; the snapshot fields locking in. Lighting: even, low-noise. --- on-brand constraints: palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, mint #36E2C4 only on the resolved snapshot; monospace; plausible Rust; matte; no people.`
 - **CTA :** `cargo add rmux-sdk`

---

### J4, Saturday · 2026-07-05
- **Post 1** · 12:00 · X/Twitter · pillar **Engineering** · 🤖 AUTO · ~5 min
 - **Text:**
  > `#![forbid(unsafe_code)]` in a terminal multiplexer. In Rust. Of course.
  >
  > Upper crates forbid unsafe, the OS/PTY boundary is isolated, and there's no network at runtime, enforced by a build guardrail, not a promise. Locked, dual MIT/Apache.
  >
  > Infra-grade posture for something you leave running for days.
 - **Visuel :** 🤖 Higgsfield (Pillar 5, 1:1), `{SYSTEM} Subject: a matte badge motif on canvas-black, "#![forbid(unsafe_code)]" and "no network at runtime" rendered as two clean status glyphs. Scene/context: a safety-posture card. Format: 1:1. Action/emotion: quiet authority. Lighting: schematic, flat, one mint rim-light on the forbid-unsafe glyph. --- on-brand constraints: palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced "rmux" wordmark; matte; no people.`
 - **CTA :** github.com/Helvesec/rmux

---

### J5, Sunday · 2026-07-06
- **Post 1** · 17:00 · X/Twitter · pillar **Terminal Testing** · 🤖 AUTO · ~5 min
 - **Text:**
  > What if you could `wait_for_text()` on a terminal like it's a webpage?
  >
  > ```rust
  > pane.wait_for_text("Server listening")?;
  > let snap = pane.snapshot()?;
  > assert!(snap.content.contains("0 errors"));
  > ```
  >
  > Playwright-style assertions for TUIs and CLIs. Deterministic terminal tests, finally.
 - **Visuel :** 🤖 Higgsfield (Pillar 4, 16:9), `{SYSTEM} Subject: a test panel asserting on terminal output, a CI-style pass row where wait_for_text + snapshot resolve to a single mint "PASS". Scene/context: "assert on a terminal like a DOM". Format: 16:9. Action/emotion: a passing assertion turning mint. Lighting: even. --- on-brand constraints: palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, one mint #36E2C4 "PASS"; deterministic clean layout; monospace; matte; no people.`
 - **CTA :** Docs → rmux.io/docs

---

### J6, Monday · 2026-07-07
- **Post 1** · 09:00 · LinkedIn · pillar **Programmable Terminal** · 🤖 AUTO · ~10 min
 - **Text:**
  > Driving a terminal from code used to mean scraping stdout and praying the format never changed.
  >
  > rmux returns a typed `PaneSnapshot`, cols, rows, content, and lets you `wait_for_text("ready")` before you read. Deterministic. Testable. No regex archaeology.
  >
  > One daemon, three surfaces: the CLI you type, the `rmux-sdk` crate your code calls, and a `ratatui-rmux` widget you can embed. Same protocol underneath.
  >
  > If you've ever wanted your terminal to have an API, this is it. `cargo add rmux-sdk` → github.com/Helvesec/rmux
 - **Visuel :** 🤖 Higgsfield (Pillar 2, 4:5), `{SYSTEM} Subject: left a Rust code panel calling rmux_sdk, right a live terminal pane returning a structured PaneSnapshot. Scene/context: code driving a terminal, the snapshot rendered as typed fields. Format: 4:5 for LinkedIn. Action/emotion: wait_for_text("ready") flashing mint as the pane responds. Lighting: even, low-noise. --- on-brand constraints: palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, mint #36E2C4 only on the active result; monospace alignment; plausible Rust; matte; no people.`
 - **CTA :** `cargo add rmux-sdk`

- **Post 2** · 14:00 · X/Twitter · pillar **Agentic Terminal** · 🤖 AUTO · ~6 min
 - **Text:**
  > Detach an agent. Reattach a week later. Inspect everything it did.
  >
  > I lost a 3-hour agent run to a dropped SSH connection once. Now the session outlives the connection, and I can read exactly what the agent typed, from code, without attaching.
  >
  > That's the whole point of rmux.
 - **Visuel :** 🤖 Higgsfield (Pillar 1, 1:1), `{SYSTEM} Subject: a single terminal pane labeled with a long-running agent, an SSH connection dropping in the corner while the pane keeps running, a mint cursor still blinking. Scene/context: session persistence across a dropped connection. Format: 1:1. Action/emotion: reassurance, continuity, quiet power. Lighting: flat UI glow, one mint focal light on the surviving pane. --- on-brand constraints: palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced "rmux" wordmark; multiplex-grid; matte; real plausible agent log lines; no people.`
 - **CTA :** `cargo install rmux`

---

### J7, Tuesday · 2026-07-08
- **Post 1** · 10:00 · X/Twitter · pillar **tmux, Upgraded** · 🤖 AUTO · ~5 min
 - **Text:**
  > tmux on Windows? With rmux, natively, ConPTY + per-user Named Pipes, no WSL shim.
  >
  > All 90 tmux commands, the same keybindings, and your `.tmux.conf` migrates automatically. It just also runs where tmux never could.
 - **Visuel :** 🤖 Higgsfield (Pillar 3, 16:9), `{SYSTEM} Subject: a clean rmux session running natively with a small "Windows · native ConPTY" badge, a matching Linux/macOS row beneath it, same session, three platforms. Scene/context: cross-platform parity. Format: 16:9. Action/emotion: "finally, everywhere". Lighting: flat. --- on-brand constraints: palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent on the Windows badge; monospaced "rmux" wordmark; comparison grid; matte; no people.`
 - **CTA :** github.com/Helvesec/rmux

- **Post 2** · 16:00 · dev.to (blog) · pillar **tmux, Upgraded** · 🤖 AUTO · ~35 min
 - **Text (full post):**
  > # rmux vs tmux: what stays the same, what's finally different
  >
  > If you've typed `tmux` for a decade, the switching cost is the whole question. Short answer: your muscle memory is safe, and you gain three things tmux structurally can't give you.
  >
  > ## What stays the same
  > - **All 90 tmux commands** are implemented. `split-window`, `select-pane`, `new-session`, copy-mode, same verbs.
  > - **Your keybindings.** Your `.tmux.conf` migrates automatically on first run (opt out with one env var).
  > - **The workflow.** Prefix key, panes, windows, sessions, identical mental model.
  >
  > ## What's finally different
  > 1. **A typed SDK.** `rmux-sdk` gives you `ensure_session`, `send_text`, `wait_for_text`, and `snapshot()` returning a structured `PaneSnapshot { cols, rows, content }`. tmux's control mode still hands you text to parse.
  > 2. **Native Windows.** ConPTY + per-user Named Pipes, no WSL shim. tmux has no native Windows.
  > 3. **Rust safety posture.** `#![forbid(unsafe_code)]` in the upper crates, no network at runtime (build-guardrail enforced), locked dual MIT/Apache builds.
  >
  > ## And one more surface
  > `cargo add ratatui-rmux` drops a live terminal pane into your own Ratatui TUI. Same daemon, third surface.
  >
  > ## Try it
  > ```bash
  > cargo install rmux --locked
  > ```
  > It's a v0.3.1 public preview, the feature matrix is verified across Linux/macOS/Windows, and bugs get fixed fast. Repo + issues: **github.com/Helvesec/rmux**
 - **Visuel :** 🤖 Higgsfield (Pillar 3, 16:9 cover), `{SYSTEM} Subject: a restrained "rmux vs tmux" split comparison frame, left column tmux (grey), right column rmux (one mint accent) listing: 90 commands · SDK · native Windows · forbid-unsafe. Scene/context: conversion/comparison card. Format: 16:9 blog cover. Action/emotion: clear, honest contrast. Lighting: flat. --- on-brand constraints: palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, mint #36E2C4 on the rmux side only; monospaced "rmux" wordmark; comparison grid; matte; no people.`
 - **CTA :** Read → link from X; `cargo install rmux --locked`

---

### J8, Wednesday · 2026-07-09
- **Post 1** · 11:00 · YouTube / asciinema · pillar **Agentic Terminal** · 🙋 MANUAL · ~75 min
 - **Text (video description / caption):**
  > **rmux, orchestrating N agents from one daemon (60s screencast)**
  >
  > A real, un-cut terminal recording: start four agents in split panes, detach the whole session, drop the SSH connection, reattach, and read exactly what each agent did via `pane.snapshot()`. No edits, no stdout scraping.
  >
  > rmux is a Rust, tmux-compatible multiplexer with a daemon-backed typed SDK. `cargo install rmux` → github.com/Helvesec/rmux
 - **Visuel :** 🙋 MANUAL, **record** the Multi-Agent Orchestration demo (the shipped ~514-LOC demo) as a 60s asciinema/screen capture: 4 agent panes → detach → drop SSH → reattach → `snapshot()` prints each pane. Keep the brand terminal theme (canvas #0B0E14, mint cursor). This is the **hero asset**, repurpose per the content-strategy repurposing engine.
 - **CTA :** `cargo install rmux` · watch full demo link

- **Post 2** · 15:00 · X/Twitter · pillar **Agentic Terminal** · 🤖 AUTO · ~6 min
 - **Text:**
  > New: a 60s screen recording of rmux orchestrating four agents from one daemon.
  >
  > Start them → detach → drop SSH → reattach → `snapshot()` each pane. Nothing lost, nothing scraped. 👇
  >
  > [link to the J8 screencast]
 - **Visuel :** 🤖 Reuse the J8 screencast as the attached video/GIF (no separate Higgsfield generation, the recording is the visual).
 - **CTA :** Watch → screencast link; `cargo install rmux`

---

### J9, Thursday · 2026-07-10
- **Post 1** · 09:00 · LinkedIn · pillar **Engineering / Build-in-Public** · 🤖 AUTO · ~10 min
 - **Text:**
  > A design decision I keep coming back to: why rmux has *three* surfaces instead of one.
  >
  > There's a single local daemon. On top of it sit three equal clients over the same protocol:
  > • the `rmux` CLI, what a human types,
  > • `rmux-sdk`, what your Rust code calls,
  > • `ratatui-rmux`, a live terminal pane you embed in your own TUI.
  >
  > Because they share one protocol, there's no "scripting API that lags behind the real thing." Anything the CLI can do, the SDK can do, the widget can do. A session a human starts by hand is the same session code drives and inspects.
  >
  > That symmetry is the whole product. It's what makes a terminal something agents and humans can genuinely share.
  >
  > github.com/Helvesec/rmux
 - **Visuel :** 🤖 Higgsfield (Pillar 5, 16:9 OG), `{SYSTEM} Subject: the three-surfaces → one-daemon architecture diagram, CLI / rmux-sdk / ratatui-rmux as three boxes connected by thin #1E2733 connectors to a central daemon node. Scene/context: authoritative schematic on canvas-black. Format: 16:9 OG. Action/emotion: static, engineered clarity. Lighting: schematic, flat, mint focal light on the daemon node only. --- on-brand constraints: palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent on the daemon; monospaced "rmux" wordmark; matte; no people.`
 - **CTA :** Star → github.com/Helvesec/rmux

---

### J10, Friday · 2026-07-11
- **Post 1** · 11:00 · Reddit r/commandline · pillar **tmux, Upgraded** · 🙋 MANUAL · ~35 min
 - **Text (post body):**
  > **Title:** tmux users: rmux migrates your `.tmux.conf` and runs natively on Windows, AMA about compatibility
  >
  > rmux is a Rust terminal multiplexer with all 90 tmux commands implemented. Your `.tmux.conf` migrates automatically on first run (opt out with one env var), so the keybindings and workflow you already have come with you. What's new on top: native Windows (ConPTY + Named Pipes, no WSL shim), a typed SDK for driving/inspecting sessions from code, and a Ratatui widget to embed a live pane.
  >
  > I'd rather answer real compatibility questions than pitch, so: ask me anything about specific commands, copy-mode, nested sessions, your exact `.tmux.conf`, or the Windows behavior and I'll tell you honestly what works and what's still rough in this v0.3.1 preview.
  >
  > Repo: github.com/Helvesec/rmux
 - **Visuel :** 🙋 MANUAL, no image (text AMA). Have your `.tmux.conf` and a Windows terminal ready to screenshot live answers. Reply to every top comment; be honest about preview rough edges.
 - **CTA :** answer questions live; repo link at the end

- **Post 2** · 15:00 · X/Twitter · pillar **Programmable Terminal** · 🤖 AUTO · ~5 min
 - **Text:**
  > One daemon. Three surfaces. CLI = SDK = widget.
  >
  > A session you start by hand is the same session your code drives with `rmux-sdk` and the same one you embed with `ratatui-rmux`. No second-class scripting API.
  >
  > `cargo add rmux-sdk`
 - **Visuel :** 🤖 Higgsfield (Pillar 5, 1:1), `{SYSTEM} Subject: three small labeled surface icons (CLI / SDK / widget) linked by thin connectors to one central daemon node. Scene/context: "one runtime, three surfaces" quote card. Format: 1:1. Action/emotion: symmetry, clean equality between the three. Lighting: flat, mint focal light on the daemon node. --- on-brand constraints: palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced "rmux" wordmark; matte; no people.`
 - **CTA :** `cargo add rmux-sdk`

---

### J11, Saturday · 2026-07-12
- **Post 1** · 13:00 · X/Twitter · pillar **Agentic Terminal** · 🤖 AUTO · ~6 min
 - **Text:**
  > Orchestrate N agents from one daemon:
  >
  > ```rust
  > for name in ["planner", "coder", "tester"] {
  >   let pane = client.ensure_session(name)?;
  >   pane.send_text("start\n")?;
  > }
  > ```
  >
  > Broadcast a command to many panes, then `snapshot()` each to see where they are. tmux for agents.
 - **Visuel :** 🤖 Higgsfield (Pillar 1, 16:9), `{SYSTEM} Subject: a near-black terminal split into three labeled agent panes (planner / coder / tester) all running, one broadcast command echoed across them, a mint cursor on the active pane. Scene/context: broadcast-to-many orchestration. Format: 16:9. Action/emotion: coordinated control across panes. Lighting: flat UI glow, single mint focal light. --- on-brand constraints: palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced "rmux" wordmark; multiplex-grid; matte; real plausible agent log lines; no people.`
 - **CTA :** `cargo install rmux`

---

### J12, Sunday · 2026-07-13
- **Post 1** · 17:00 · X/Twitter · pillar **Programmable Terminal (tip card)** · 🤖 AUTO · ~5 min
 - **Text:**
  > Embeddable terminal, one line:
  >
  > ```bash
  > cargo add ratatui-rmux
  > ```
  >
  > Drop a live, real terminal pane into any Ratatui app, driven by the same daemon as your CLI and SDK. Not a mock. A real pane.
 - **Visuel :** 🤖 Higgsfield (Pillar 2, 4:5), `{SYSTEM} Subject: a Ratatui TUI app window with one embedded live terminal pane glowing mint at its center, surrounded by grey widget chrome. Scene/context: ratatui-rmux embedding a real pane. Format: 4:5. Action/emotion: a live pane sitting inside a custom TUI. Lighting: even, one mint focal light on the embedded pane. --- on-brand constraints: palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent on the embedded pane; monospaced "rmux" wordmark; matte; no people.`
 - **CTA :** `cargo add ratatui-rmux`

---

### J13, Monday · 2026-07-14
- **Post 1** · 09:00 · Hacker News · pillar **Agentic Terminal (flagship / launch moment)** · 🙋 MANUAL · ~40 min
 - **Text (Show HN):**
  > **Title:** Show HN: rmux, tmux for agents (Rust multiplexer with a typed SDK)
  >
  > **Body:** rmux is a Rust, tmux-compatible terminal multiplexer (all 90 commands) with something tmux never had: a daemon-backed *typed* SDK. One local daemon drives three equal surfaces over the same protocol, the `rmux` CLI, the `rmux-sdk` crate, and a `ratatui-rmux` widget you can embed.
  >
  > The motivating use case: long-lived agents over SSH. You can detach a session, reattach later, and `pane.snapshot()` to read a structured `PaneSnapshot { cols, rows, content }`, inspecting what an agent did without attaching and scraping `capture-pane`. `wait_for_text("ready")` lets you assert on a terminal like it's a DOM, which also makes TUI/CLI testing deterministic.
  >
  > Cross-platform including native Windows (ConPTY + per-user Named Pipes, no WSL shim). `#![forbid(unsafe_code)]` in the upper crates, no network at runtime (build-guardrail enforced), dual MIT/Apache. It's an honest v0.3.1 preview, all 90 commands work and the feature matrix is verified across Linux/macOS/Windows; bugs get fixed fast.
  >
  > Repo: github.com/Helvesec/rmux, feedback and issues very welcome.
 - **Visuel :** 🙋 MANUAL, HN is text-only. Have the J8 orchestration screencast linked in the first comment, and stay on the thread for the first few hours to answer.
 - **CTA :** engage in comments; repo link in the post

- **Post 2** · 14:00 · X/Twitter · pillar **Agentic Terminal** · 🤖 AUTO · ~5 min
 - **Text:**
  > rmux is on Hacker News today: "tmux for agents", a Rust multiplexer with a typed SDK.
  >
  > If the idea of `pane.snapshot()` and `wait_for_text()` on a real terminal resonates, I'd love your take (and your bug reports). 👇
  >
  > [HN link]
 - **Visuel :** 🤖 Higgsfield (Pillar 1, 16:9), `{SYSTEM} Subject: a hero multiplex grid of four agent panes with a mint cursor, "tmux for agents" set as a quiet monospaced caption. Scene/context: flagship launch card. Format: 16:9. Action/emotion: confident, controlled. Lighting: flat UI glow, single mint focal light. --- on-brand constraints: palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced "rmux" wordmark; multiplex-grid; matte; no people.`
 - **CTA :** Discuss on HN → link

---

### J14, Tuesday · 2026-07-15
- **Post 1** · 10:00 · LinkedIn · pillar **Terminal Testing** · 🤖 AUTO · ~10 min
 - **Text:**
  > Terminal apps are notoriously hard to test, you end up snapshotting bytes and diffing brittle strings.
  >
  > rmux gives you the two primitives a terminal test actually needs: `wait_for_text("...")` to synchronize, and a typed `PaneSnapshot { cols, rows, content }` to assert on. It's Playwright-for-terminals, deterministic, CI-friendly, no sleeps-and-hope.
  >
  > One of the five demos we ship is exactly this: a Playwright-style testing harness (~1,495 LOC) driving and asserting on a TUI.
  >
  > If you maintain a CLI or TUI, this is worth a look → github.com/Helvesec/rmux
 - **Visuel :** 🤖 Higgsfield (Pillar 4, 4:5), `{SYSTEM} Subject: a CI test report panel, a column of terminal assertions (wait_for_text, snapshot.content.contains) each resolving to a mint "PASS". Scene/context: deterministic terminal testing in CI. Format: 4:5. Action/emotion: a green/mint run completing cleanly. Lighting: even. --- on-brand constraints: palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, mint #36E2C4 on the PASS states only; deterministic clean layout; monospace; matte; no people.`
 - **CTA :** github.com/Helvesec/rmux

- **Post 2** · 15:00 · X/Twitter · pillar **Terminal Testing** · 🤖 AUTO · ~5 min
 - **Text:**
  > Assert on a terminal like it's a DOM:
  >
  > ```rust
  > pane.wait_for_text("PASS")?;
  > assert!(pane.snapshot()?.content.contains("0 failed"));
  > ```
  >
  > Deterministic TUI/CLI tests in CI. No sleeps, no flaky diffs. Ship rmux-sdk in your test suite.
 - **Visuel :** 🤖 Higgsfield (Pillar 4, 1:1), `{SYSTEM} Subject: a compact Rust test-assertion card, wait_for_text + snapshot content check resolving to a single mint "PASS" row. Scene/context: deterministic terminal test. Format: 1:1. Action/emotion: the assertion locking green. Lighting: even, one mint focal light on PASS. --- on-brand constraints: palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, mint #36E2C4 on PASS only; monospace; plausible Rust; matte; no people.`
 - **CTA :** `cargo add rmux-sdk`

---

## Summary
- **Total posts:** 23 over 14 days (min 1/day, max 2/day; weekends kept to 1).
- **Modes:** 19 🤖 AUTO · 4 🙋 MANUAL (J3 Reddit r/rust · J8 screencast recording · J10 Reddit r/commandline AMA · J13 Show HN), staggered, max 1 manual/day.
- **Pillar spread:** Agentic Terminal (flagship, wedge) heaviest; Programmable, tmux-Upgraded, Terminal Testing, Engineering/Build-in-Public rotated so no pillar repeats within a day.
- **Channel spread:** X 12 · LinkedIn 4 · dev.to 2 · Reddit 2 · YouTube/asciinema 1 · HN 1, matches `content-strategy.md` cadence.
- **Hero repurposing:** the J8 Multi-Agent Orchestration screencast is the hero asset; J8 X post, J13 HN first comment, and the J2/J7 blogs all pull from the same demo (repurposing engine).
