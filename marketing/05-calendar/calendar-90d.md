---
project: rmux
layer: calendar
horizon: 90 days
status: filled
---
# Plan 90 jours, rmux

> 90 jours · 103 posts (88 🤖 auto / 15 🙋 manuel). Base J1 = 2026-07-02.
> MODE : 🤖 AUTO (le moteur poste) · 🙋 MANUEL (toi).


# ── Mois 1 (J1-J30) ──

### J1, jeudi 2 juil.
- **09:00 · twitter** · Agentic Terminal · 🤖 AUTO
 - tmux is great. But it was built for humans, not agents.
 
 A human attaches, looks at a pane, and decides. An agent needs to do that from code: detach, reattach, and read the pane's state without a pair of eyes.
 
 That gap is the whole reason I started building rmux.
 - _Visuel :_ {SYSTEM} Subject: a single dark terminal pane split down the middle, left half labeled 'human' with a blinking mint cursor, right half labeled 'agent' reaching in via a thin connector line to the same pane. Scene: the same session shared by a human and an agent. Format: 1:1. Action: quiet, engineered clarity. Lighting: flat UI glow, one mint focal light on the shared cursor. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark lower-left; matte; no people.
 - _CTA :_ github.com/Helvesec/rmux

### J2, vendredi 3 juil.
- **10:00 · twitter** · Programmable Terminal · 🤖 AUTO
 - Your terminal has no API.
 
 Every other layer of your stack does, your DB, your HTTP server, your queue. But the terminal? You spawn a process and scrape bytes off stdout like it's 1999.
 
 rmux fixes that. A real, typed API for the terminal you already live in.
 - _Visuel :_ {SYSTEM} Subject: a matte concept card contrasting two rows, top 'stdout: raw bytes' shown as a grey unstructured blob, bottom 'PaneSnapshot { cols, rows, content }' shown as three clean typed fields glowing mint. Scene: raw scraping vs a typed API. Format: 4:5. Action: the messy blob resolving into structured fields. Lighting: even, low-noise, one mint focal light on the typed fields. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ cargo add rmux-sdk
- **15:00 · linkedin** · Build-in-Public · 🤖 AUTO
 - A category nobody named yet: the programmable terminal multiplexer.
 
 We have terminal multiplexers built for humans (tmux, zellij, screen). We have automation frameworks built for browsers (Playwright, Puppeteer). But the terminal, where half of infrastructure and now most autonomous agents actually run, sits in between, with no first-class programmatic surface.
 
 That in-between is where I've spent the last months. rmux keeps the full tmux-compatible CLI (all 90 commands) and adds what the lineage never had: a daemon-backed typed SDK, structured pane snapshots, and a Ratatui widget, so a session a human starts by hand can be detached, reattached, and driven from code.
 
 Not a better TUI. A terminal with an API.
 
 Open-source, dual MIT/Apache → github.com/Helvesec/rmux
 - _Visuel :_ {SYSTEM} Subject: a three-column positioning map on canvas-black, 'human multiplexers (tmux/zellij)' left, 'browser automation (Playwright)' right, and a central highlighted column 'programmable terminal, rmux' bridging them. Scene: a category map. Format: 1:1. Action: rmux occupying the empty middle slot. Lighting: schematic, flat, one mint focal light on the central rmux column. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent on the rmux column; thin #1E2733 dividers; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ Star github.com/Helvesec/rmux

### J3, samedi 4 juil.
- **09:00 · twitter** · Agentic Terminal · 🤖 AUTO
 - I lost a 3-hour agent run to a dropped SSH connection.
 
 The agent was halfway through a refactor. My laptop slept, the tunnel died, and the whole session went with it. No log I could reattach to, no state I could read back.
 
 That night I stopped treating the terminal as disposable. A long-lived agent needs a runtime that outlives the connection, and lets you inspect what it did afterward. So I started using rmux.
 - _Visuel :_ {SYSTEM} Subject: a terminal pane labeled 'agent · refactor 3h' still running, with an SSH tunnel icon breaking apart in the top corner and a mint cursor that keeps blinking regardless. Scene: session survives a dropped connection. Format: 16:9. Action: reassurance, the work continues past the break. Lighting: flat UI glow, one mint focal light on the surviving pane. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent, functional amber #F0883E only on a small 'running' status glyph; monospaced 'rmux' wordmark; multiplex-grid; matte; real plausible agent log lines; no people.
 - _CTA :_ cargo install rmux

### J4, dimanche 5 juil.
- **11:00 · linkedin** · Build-in-Public · 🤖 AUTO
 - Unpopular opinion for a terminal tool: compatibility is a feature, not a constraint.
 
 A lot of new dev tools ask you to relearn everything to get one new capability. That's why most of them die in a side project folder, the switching cost dwarfs the payoff.
 
 With rmux I made the opposite bet. All 90 tmux commands are implemented. Your .tmux.conf migrates on first run. Your muscle memory, your prefix key, your panes, untouched. The new capability (a typed SDK, structured snapshots, native Windows) rides on top of a workflow you already have.
 
 The cost of trying it is one cargo install. That, to me, is the whole difference between a tool people admire and a tool people actually adopt.
 
 github.com/Helvesec/rmux
 - _Visuel :_ {SYSTEM} Subject: a balance/scale motif rendered as two terminal panes, left pane 'switching cost' nearly empty (just 'cargo install'), right pane 'what you gain' listing SDK · snapshots · native Windows. Scene: cost vs payoff. Format: 4:5. Action: the payoff side clearly outweighing. Lighting: flat, one mint focal light on the payoff pane. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ cargo install rmux --locked

### J5, lundi 6 juil.
- **12:00 · twitter** · tmux, Upgraded · 🤖 AUTO
 - "Why not just use tmux control mode?"
 
 Because control mode hands you text to parse. When the output format shifts, your automation quietly breaks.
 
 rmux gives you a typed PaneSnapshot { cols, rows, content } and wait_for_text() instead, an interface that doesn't rot when a version bumps. Parsing is not an API.
 - _Visuel :_ {SYSTEM} Subject: a split card, left 'tmux control mode' shown as a fragile stream of text being regex-parsed with a red-ish crack, right 'rmux typed SDK' shown as a stable structured PaneSnapshot in mint. Scene: brittle parsing vs a typed contract. Format: 1:1. Action: the typed side holding firm. Lighting: even, low-noise, one mint focal light on the typed side. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent on the rmux side; monospaced 'rmux' wordmark; matte; plausible output; no people.
 - _CTA :_ cargo add rmux-sdk

### J6, mardi 7 juil.
- **10:00 · reddit** · Engineering / Build-in-Public · 🙋 MANUEL · ~35 min
 - Title: Why I gave the terminal a typed SDK instead of another control-mode wrapper
 
 Most ways to automate a terminal boil down to: spawn, write bytes, read bytes, regex the result. tmux control mode is a bit nicer but you're still parsing text that can change between versions. I wanted an interface that doesn't rot.
 
 So in rmux (a Rust, tmux-compatible multiplexer) the daemon exposes a typed SDK. ensure_session is idempotent (attach-or-create). send_text writes input. wait_for_text blocks until a string appears, no sleep-and-hope. snapshot() returns a structured PaneSnapshot { cols, rows, content } instead of a byte blob, so you assert on terminal state like it's a DOM node.
 
 The part I want to discuss with this sub: the trade-offs of a daemon-backed typed surface vs a text protocol. Latency, versioning, how you keep three clients (CLI, SDK, Ratatui widget) honest against one protocol, and where a typed snapshot is the wrong abstraction. It's a v0.3.1 preview, forbid-unsafe in the upper crates, no network at runtime. Repo: github.com/Helvesec/rmux, happy to go deep in the comments.
 - _À faire :_ Post to r/rust as a text discussion (lead with the engineering trade-off, link last). No hero image. Reply to every substantive comment for the first 2 hours, this is a thinking-out-loud thread, not an announcement. Screenshot a real snapshot() call if someone asks for specifics.
 - _CTA :_ engage in comments; repo link at the end

### J7, mercredi 8 juil.
- **17:00 · twitter** · Engineering · 🤖 AUTO
 - One daemon. Three surfaces.
 
 The CLI a human types, the rmux-sdk crate code calls, and the ratatui-rmux widget you embed, all speak the same local protocol. There's no "scripting API that lags behind the real thing," because there is no separate scripting API. It's the same runtime, three ways in.
 - _Visuel :_ {SYSTEM} Subject: the three-surfaces → one-daemon architecture, three labeled boxes (CLI / rmux-sdk / ratatui-rmux) connected by thin #1E2733 connectors to one central daemon node. Scene: authoritative schematic on canvas-black. Format: 16:9. Action: static, engineered symmetry, all three surfaces equal. Lighting: schematic flat, mint focal light on the daemon node only. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent on the daemon; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ github.com/Helvesec/rmux

### J8, jeudi 9 juil.
- **09:00 · linkedin** · Programmable Terminal · 🤖 AUTO
 - There's a pattern I keep seeing in agent infrastructure: teams reinvent a fragile terminal-control layer, in-house, every time.
 
 It always looks the same. A pty wrapper. A stdout reader. A pile of regexes to know when a command "finished." A retry loop for when the SSH tunnel drops. Six months later it's the least-loved, most-load-bearing 800 lines in the repo.
 
 rmux is an attempt to make that a library instead of a liability. connect_or_start() handles the daemon lifecycle. ensure_session() is idempotent. wait_for_text() replaces the sleep-and-hope. snapshot() gives you typed state instead of parsed bytes. And the session survives the dropped connection, because the daemon, not your process, owns it.
 
 If your team has written that layer twice, this is the third version you don't have to.
 
 github.com/Helvesec/rmux
 - _Visuel :_ {SYSTEM} Subject: a 'before / after' engineering frame, left a tangled grey pane labeled 'in-house pty + stdout scraping + retry loop', right a clean single pane labeled 'rmux-sdk' with four tidy method rows. Scene: a fragile home-grown layer replaced by a library. Format: 4:5. Action: complexity collapsing into a clean API. Lighting: even, one mint focal light on the rmux side. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent on the rmux side; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ cargo add rmux-sdk

### J9, vendredi 10 juil.
- **11:00 · twitter** · Agentic Terminal · 🤖 AUTO
 - Inspect what an agent typed, without attaching.
 
 With tmux you have to attach, scroll, and squint. With rmux the pane is a value you can read:
 
 ```rust
 let snap = pane.snapshot()?;
 println!("{}", snap.content);
 ```
 
 No eyes required. Which is exactly what you need when there are twenty agents and one of you.
 - _Visuel :_ {SYSTEM} Subject: a grid of small agent panes with one pane pulled forward and its content read out as a typed PaneSnapshot beside it, no human attach step shown. Scene: reading agent state programmatically at scale. Format: 16:9. Action: one pane inspected out of many, calm control. Lighting: flat UI glow, one mint focal light on the inspected pane. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; multiplex-grid; matte; real plausible agent log lines; no people.
 - _CTA :_ cargo add rmux-sdk

### J10, samedi 11 juil.
- **10:00 · devto** · Programmable Terminal · 🤖 AUTO
 - # ensure_session: the idempotent primitive terminal automation was missing
 
 Most terminal automation bugs come from one question: does this session already exist? You spawn a duplicate, or you attach to something that isn't there, and now your script is in an undefined state.
 
 rmux answers it with one call.
 
 ## The primitive
 ```rust
 use rmux_sdk::connect_or_start;
 
 let client = connect_or_start()?;      // start the daemon if needed, else attach
 let pane = client.ensure_session("build")?; // attach if it exists, create if it doesn't
 ```
 
 Both calls are idempotent. Run the script once or a hundred times, you converge on exactly one daemon and exactly one "build" session. No "session already exists" errors, no orphaned duplicates, no lifecycle bookkeeping in your own code.
 
 ## Why it matters for agents
 An agent supervisor that restarts should reattach to the work in progress, not spawn a second copy of it. Idempotent ensure_session makes "reattach or create" a single, safe line, the same property that makes Kubernetes reconcilers sane, applied to terminal sessions.
 
 ## Then drive it
 ```rust
 pane.send_text("cargo build --release\n")?;
 pane.wait_for_text("Compiling")?;      // synchronize, don't sleep
 let snap = pane.snapshot()?;         // typed PaneSnapshot { cols, rows, content }
 assert!(snap.content.contains("Compiling"));
 ```
 
 Same daemon, three surfaces (CLI, rmux-sdk, ratatui-rmux). It's a v0.3.1 preview, all 90 tmux commands work, bugs get fixed fast. Try it: github.com/Helvesec/rmux
 - _Visuel :_ {SYSTEM} Subject: a diagram of idempotency, a single 'ensure_session(\"build\")' call with two arrows ('exists → attach' / 'missing → create') both converging on ONE session node. Scene: converge-not-duplicate. Format: 16:9 blog cover. Action: two paths resolving to one stable node. Lighting: schematic flat, one mint focal light on the single resulting session. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; thin #1E2733 connectors; monospaced 'rmux' wordmark; plausible Rust; matte; no people.
 - _CTA :_ cargo add rmux-sdk

### J11, dimanche 12 juil.
- **13:00 · twitter** · Terminal Testing · 🤖 AUTO
 - We have Playwright for the DOM. What's the Playwright for the terminal?
 
 You need two things to test a TUI honestly: a way to wait for a real state, and a way to assert on it. rmux gives you both, wait_for_text() to synchronize, a typed PaneSnapshot to assert. No sleeps, no brittle byte-diffs. That's terminal testing, finally deterministic.
 - _Visuel :_ {SYSTEM} Subject: a 'Playwright-for-terminals' concept card, a browser-style test bar reimagined as a terminal test bar, one assertion row 'snapshot.content.contains(...)' resolving to a mint PASS. Scene: DOM-testing metaphor applied to a terminal. Format: 4:5. Action: an assertion locking to PASS. Lighting: even, one mint focal light on PASS. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, mint #36E2C4 on PASS only; monospace; matte; no people.
 - _CTA :_ github.com/Helvesec/rmux
- **16:00 · twitter** · Build-in-Public · 🙋 MANUEL · ~30 min
 - Community engagement sprint (no scheduled post text, this is a human ritual).
 - _À faire :_ 30-min engagement sprint, not a broadcast. Search X for 'tmux', 'terminal multiplexer', 'agent over SSH', 'Ratatui', and reply genuinely to 5-8 threads from the engineer-to-engineer voice, answer the actual question, drop a code line if it helps, mention rmux ONLY where it truly fits (never copy-paste pitch). Also reply to every comment on this block's earlier posts. Goal: relationships, not impressions.
 - _CTA :_ reply to 5-8 relevant threads; no link-spam

### J12, lundi 13 juil.
- **16:00 · linkedin** · Engineering · 🤖 AUTO
 - The safety posture I insisted on for rmux, and why it's not just Rust-flavored marketing.
 
 This is software you leave running for days, holding your sessions and increasingly your agents. So three properties were non-negotiable:
 
 1. #![forbid(unsafe_code)] in the upper crates. Unsafe is isolated to the OS/PTY boundary where it genuinely belongs, not sprinkled through business logic.
 2. No network at runtime, and it's enforced by a build guardrail, not a README promise. The multiplexer that runs your infra has no business phoning home.
 3. Locked, reproducible builds, dual MIT/Apache. What you compile is what ships.
 
 None of this is glamorous. It's the difference between a tool you demo and a tool you trust with a long-lived process. For infrastructure, boring guarantees are the feature.
 
 github.com/Helvesec/rmux
 - _Visuel :_ {SYSTEM} Subject: three clean status glyphs stacked on canvas-black, 'forbid(unsafe_code)', 'no network at runtime', 'locked reproducible builds', each a matte engineered badge. Scene: an infra-grade safety-posture card. Format: 1:1. Action: quiet authority, nothing flashy. Lighting: schematic flat, one mint rim-light on the forbid-unsafe glyph only. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ github.com/Helvesec/rmux

### J13, mardi 14 juil.
- **09:00 · twitter** · tmux, Upgraded · 🤖 AUTO
 - All 90 tmux commands. Rewritten in Rust. Now on Windows.
 
 Not a subset. Not "the common ones." The full command surface, so switching is muscle memory, not a rewrite. And it runs natively on Windows, ConPTY + per-user Named Pipes, no WSL shim, which tmux structurally can't do.
 - _Visuel :_ {SYSTEM} Subject: a tidy grid hinting at '90 commands' as small monospaced command tokens, with a single 'Windows · native ConPTY' badge glowing mint in the corner. Scene: full-surface parity plus native Windows. Format: 16:9. Action: completeness and reach. Lighting: flat, one mint focal light on the Windows badge. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ cargo install rmux --locked

### J14, mercredi 15 juil.
- **11:00 · reddit** · tmux, Upgraded · 🙋 MANUEL · ~35 min
 - Title: If you've ever wanted tmux to run natively on Windows, it does now (rmux), AMA about compatibility
 
 I kept hitting the same wall: tmux everywhere on Linux/macOS, then a WSL shim (or nothing) on Windows. rmux is a Rust, tmux-compatible multiplexer, all 90 commands implemented, that runs natively on Windows via ConPTY + per-user Named Pipes, no WSL. Your .tmux.conf migrates automatically on first run (one env var opts out), so the keybindings and workflow come with you.
 
 Rather than pitch, I'd like to stress-test the compatibility claim with people who actually push tmux hard. Ask me about: nested sessions, copy-mode, your specific prefix setup, pane splitting behavior, a weird line in your .tmux.conf, or exactly how the Windows PTY path behaves. I'll tell you honestly what's solid and what's still rough in this v0.3.1 preview.
 
 Repo: github.com/Helvesec/rmux
 - _À faire :_ Post to r/commandline as a text AMA. Have a real .tmux.conf and a native Windows terminal open to screenshot live answers. Reply to every top comment; be candid about preview rough edges. Value-first, never announcement-spam.
 - _CTA :_ answer questions live; repo link at the end

### J15, jeudi 16 juil.
- **10:00 · twitter** · Programmable Terminal · 🤖 AUTO
 - Same keybindings as tmux. Plus an SDK. Plus a Ratatui widget.
 
 That's the whole pitch in one line. You lose nothing you already type, and you gain a typed way to drive sessions from Rust and a way to embed a real live pane in your own TUI. Additive, not a migration.
 - _Visuel :_ {SYSTEM} Subject: three stacked labeled rows on canvas-black, 'same keybindings' (grey chrome), '+ typed SDK' (mint), '+ Ratatui widget' (mint), a clean additive stack. Scene: nothing lost, capabilities added. Format: 1:1. Action: additive layering. Lighting: flat, mint focal light on the two '+' rows. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ cargo add rmux-sdk
- **15:00 · linkedin** · Build-in-Public · 🤖 AUTO
 - Building in public means saying the honest thing: rmux is a v0.3.1 preview, and I'd rather tell you that than oversell it.
 
 What's real today: all 90 tmux-compatible commands are implemented, the feature matrix is verified across Linux, macOS, and Windows, and five working demos ship in the repo, from a multi-agent orchestrator to a Playwright-style TUI test harness. What's still preview-grade: rough edges you'll find, which is exactly why the issue tracker matters.
 
 Here's the deal I'm offering early adopters: try it, file a bug, and watch it get fixed fast. A preview with a responsive maintainer beats a "1.0" nobody answers. Your bug report genuinely shapes what v0.4 becomes.
 
 cargo install rmux --locked → github.com/Helvesec/rmux
 - _Visuel :_ {SYSTEM} Subject: an honest 'v0.3.1 preview' status card, a version badge, a small row 'all 90 commands · verified matrix · 5 demos', and an open issue-tracker glyph. Scene: transparent build-in-public status. Format: 4:5. Action: candid, grounded. Lighting: flat, one mint focal light on the version badge. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ cargo install rmux --locked · file an issue

### J16, vendredi 17 juil.
- **12:00 · twitter** · Agentic Terminal · 🤖 AUTO
 - Broadcast a command to many agent panes:
 
 ```rust
 for name in ["planner", "coder", "reviewer"] {
   client.ensure_session(name)?.send_text("status\n")?;
 }
 ```
 
 Then snapshot() each to see where they all landed. One human, many agents, one loop. This is what "tmux for agents" actually looks like in code.
 - _Visuel :_ {SYSTEM} Subject: three labeled agent panes (planner / coder / reviewer) receiving the same broadcast 'status' command echoed across all three, one mint cursor on the focused pane. Scene: fan-out broadcast to many. Format: 16:9. Action: coordinated control across panes. Lighting: flat UI glow, single mint focal light. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; multiplex-grid; matte; real plausible agent log lines; no people.
 - _CTA :_ cargo install rmux

### J17, samedi 18 juil.
- **10:00 · linkedin** · Programmable Terminal · 🤖 AUTO
 - "Anything the CLI can do, your code can do too" is easy to say. Here's why it's structurally true in rmux, not a slogan.
 
 Most tools have a product and, bolted on later, an automation API that trails behind it. Two code paths, two levels of support, one of them always stale.
 
 rmux has one local daemon and a single protocol. The CLI is a client of that protocol. rmux-sdk is a client of that protocol. The ratatui-rmux widget is a client of that protocol. There is no separate "scripting mode" that can drift, because the human path and the code path are the same path.
 
 The upshot: a session you started by hand at the prompt is byte-for-byte the same session your code attaches to, snapshots, and drives. That symmetry is the actual product, everything else is a consequence of it.
 
 github.com/Helvesec/rmux
 - _Visuel :_ {SYSTEM} Subject: a schematic showing ONE protocol line with three identical client taps hanging off it (CLI / SDK / widget), emphasizing they are peers not a hierarchy. Scene: one protocol, equal clients. Format: 16:9 OG. Action: symmetry made visible. Lighting: schematic flat, mint focal light on the shared protocol line. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent on the protocol line; thin #1E2733 connectors; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ Star github.com/Helvesec/rmux

### J18, dimanche 19 juil.
- **17:00 · twitter** · Engineering · 🤖 AUTO
 - Getting tmux onto Windows without a WSL shim meant doing the unglamorous PTY work: ConPTY for the console, per-user Named Pipes for the IPC.
 
 No Cygwin, no WSL translation layer, no "works if you squint." Native Windows sessions that speak the same daemon protocol as Linux and macOS. Cross-platform is a build target here, not an aspiration.
 - _Visuel :_ {SYSTEM} Subject: a small platform-parity schematic, three OS rows (Linux / macOS / Windows) each connecting to the same daemon node, the Windows row annotated 'ConPTY + Named Pipes'. Scene: true cross-platform, Windows native. Format: 1:1. Action: three platforms, one protocol. Lighting: schematic flat, one mint focal light on the Windows row. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent on the Windows row; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ github.com/Helvesec/rmux

### J19, lundi 20 juil.
- **09:00 · youtube** · Build-in-Public · 🙋 MANUEL · ~70 min
 - rmux, why a terminal needs an API (2-min founder walkthrough)
 
 A short, talking-to-camera-plus-terminal walkthrough of the one idea behind rmux: the terminal is the last major layer of the stack with no programmatic surface. I show a live session, start work, drop the SSH connection, reattach, and read the pane back with pane.snapshot(), and explain why that matters more now that agents, not just humans, live in the terminal.
 
 Honest v0.3.1 preview, open-source (MIT/Apache). cargo install rmux → github.com/Helvesec/rmux
 - _À faire :_ RECORD a 2-min founder video: you on camera (or voiceover) over a live rmux terminal. Beat sheet: (1) 'every layer of your stack has an API except the terminal' (2) live: start a task, drop SSH, reattach, snapshot() reads it back (3) 'now that agents live in the terminal, that gap matters'. Keep the brand terminal theme (canvas #0B0E14, mint cursor). This is a personal-voice asset, post as a short + pin under the LinkedIn/X week. Repurpose the clip across platforms.
 - _CTA :_ Watch the walkthrough · cargo install rmux

### J20, mardi 21 juil.
- **11:00 · twitter** · Terminal Testing · 🤖 AUTO
 - Flaky TUI tests almost always come from one sin: sleep(2) and hope it's ready.
 
 Replace the hope with a synchronization primitive:
 
 ```rust
 pane.wait_for_text("listening on")?;
 assert!(pane.snapshot()?.content.contains("0 errors"));
 ```
 
 wait_for_text blocks on a real state, snapshot asserts on it. Deterministic terminal tests, no timing roulette.
 - _Visuel :_ {SYSTEM} Subject: a before/after test frame, top 'sleep(2) // hope' shown crossed out in muted grey, bottom 'wait_for_text(...) → snapshot' resolving to a mint PASS. Scene: replacing timing hacks with synchronization. Format: 4:5. Action: the flaky approach struck out, the deterministic one passing. Lighting: even, one mint focal light on PASS. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, mint #36E2C4 on PASS only; monospace; plausible Rust; matte; no people.
 - _CTA :_ cargo add rmux-sdk

### J21, mercredi 22 juil.
- **10:00 · linkedin** · Build-in-Public · 🤖 AUTO
 - Three weeks of talking about rmux in public, and the objection I get most is the healthiest one: "zellij already exists."
 
 It does, and it's excellent, at being a human terminal UX. Batteries-included, beautiful defaults, great for a person at a keyboard. If that's your need, use it.
 
 rmux isn't competing for that seat. It's built for the case zellij and tmux structurally don't serve: driving and inspecting terminals, and agents, from code, with full tmux-CLI compatibility underneath. A typed SDK, structured pane snapshots, a Ratatui widget, native Windows. The overlap is the word "multiplexer"; the design center is completely different.
 
 Different job, different tool. Naming the boundary honestly is more useful than pretending there's a fight.
 
 github.com/Helvesec/rmux
 - _Visuel :_ {SYSTEM} Subject: a two-lane frame, left lane 'human terminal UX' (grey, calm), right lane 'programmable / agent-ready' (mint accent), with rmux clearly in the right lane and no head-to-head arrow. Scene: different jobs, not a fight. Format: 4:5. Action: clear boundary, honest positioning. Lighting: flat, one mint focal light on the right lane. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent on the right lane; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ github.com/Helvesec/rmux

### J22, jeudi 23 juil.
- **12:00 · twitter** · Programmable Terminal · 🤖 AUTO
 - Detachable. Scriptable. Inspectable.
 
 Those three words are the whole design center of rmux. Detach a session so it outlives the connection. Script it from a typed SDK instead of scraping stdout. Inspect its state as a structured snapshot. tmux gave you the first. rmux gives you all three.
 - _Visuel :_ {SYSTEM} Subject: three stacked word-rows on canvas-black, 'detachable', 'scriptable', 'inspectable', each paired with a tiny monospaced glyph (a broken-tether, a code bracket, a magnifier). Scene: the three-word design center. Format: 1:1. Action: a clean triad. Lighting: flat, mint focal light cycling to the third word 'inspectable'. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ cargo install rmux

### J23, vendredi 24 juil.
- **10:00 · devto** · tmux, Upgraded · 🤖 AUTO
 - # Embedding a live terminal pane in your Ratatui app with ratatui-rmux
 
 Ratatui is wonderful for building TUIs, but the moment you want a real shell inside your app, you're stuck: reimplement a terminal emulator, or shell out and lose control. ratatui-rmux gives you a third option, embed a real, live pane driven by the same daemon as your CLI.
 
 ## Add it
 ```bash
 cargo add ratatui-rmux
 ```
 
 ## Drop a pane into your layout
 ```rust
 use ratatui_rmux::TerminalPane;
 
 // inside your draw loop:
 let pane = TerminalPane::new(&session);  // a real pane, not a mock
 frame.render_widget(pane, area);     // renders live terminal output
 ```
 
 The pane you render is the same session your rmux CLI can attach to and your rmux-sdk code can snapshot(). It's not a screenshot and it's not a reimplemented emulator, it's the actual multiplexed pane, surfaced as a Ratatui widget.
 
 ## Why this is the interesting part
 Three surfaces, one daemon: the CLI a human types, the SDK your code calls, and this widget you embed all speak one protocol. So you can build a custom TUI dashboard that shows live agent panes, while a script drives those same panes and a human can attach to any of them from another terminal, all consistent, because it's one runtime.
 
 v0.3.1 preview, dual MIT/Apache. Repo + issues: github.com/Helvesec/rmux
 - _Visuel :_ {SYSTEM} Subject: a Ratatui dashboard window with grey widget chrome (a sidebar, a status bar) and ONE embedded live terminal pane glowing mint at its center. Scene: a real pane embedded inside a custom TUI. Format: 16:9 blog cover. Action: the embedded pane clearly live, not a mock. Lighting: even, one mint focal light on the embedded pane only. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent on the embedded pane; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ cargo add ratatui-rmux

### J24, samedi 25 juil.
- **11:00 · twitter** · Agentic Terminal · 🤖 AUTO
 - The agent-supervisor pattern nobody ships cleanly: reattach-or-start.
 
 When your orchestrator restarts, it should pick the in-progress agent sessions back up, not spawn duplicates. With rmux that's one idempotent line:
 
 ```rust
 let pane = client.ensure_session(agent_id)?; // attach if alive, create if not
 ```
 
 Crash-safe orchestration for free, because the daemon owns the sessions, not your process.
 - _Visuel :_ {SYSTEM} Subject: a supervisor node restarting (a small refresh glyph) and reconnecting via ensure_session to the SAME set of still-running agent panes, no duplicates spawned. Scene: crash-safe reattach. Format: 16:9. Action: the supervisor rejoining existing work cleanly. Lighting: flat UI glow, one mint focal light on the reattach connector. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent, amber #F0883E only on a small 'running' glyph; monospaced 'rmux' wordmark; multiplex-grid; matte; no people.
 - _CTA :_ cargo add rmux-sdk

### J25, dimanche 26 juil.
- **10:00 · linkedin** · Engineering / Build-in-Public · 🤖 AUTO
 - A small architecture decision I'm proud of: in rmux, the daemon owns the sessions, not your terminal, and not your script.
 
 It sounds obvious, but it's the hinge everything else swings on. Because a separate long-lived daemon holds the PTYs:
 - your SSH connection can drop and the work survives,
 - your orchestrator can crash and restart without losing state,
 - a human can attach from one place while code drives the same session from another,
 - and any of the three surfaces (CLI, SDK, widget) can come and go independently.
 
 Every headline feature, detach/reattach, agent orchestration, inspectable snapshots, the shared runtime, is downstream of that one ownership choice. Get the ownership boundary right and the capabilities almost fall out for free.
 
 That's the kind of decision that doesn't demo well but defines whether a tool is trustworthy. github.com/Helvesec/rmux
 - _Visuel :_ {SYSTEM} Subject: a central 'daemon' node clearly OWNING several session/PTY blocks, with transient clients (a terminal, a script, a widget) attaching and detaching around it via dashed connectors. Scene: the daemon owns state; clients are ephemeral. Format: 4:5. Action: stable core, coming-and-going clients. Lighting: schematic flat, one mint focal light on the daemon core. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent on the daemon; thin #1E2733 connectors; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ Star github.com/Helvesec/rmux

### J26, lundi 27 juil.
- **12:00 · twitter** · tmux, Upgraded · 🤖 AUTO
 - The multiplexer humans and agents share.
 
 tmux made one runtime for many human panes. rmux makes one runtime for humans and agents at once, same sessions, same keys, same daemon, but now readable and drivable from code. The terminal stops being a place only a person can operate.
 - _Visuel :_ {SYSTEM} Subject: a single multiplex grid where some panes are labeled 'human' and some 'agent', all sharing the same mint-cursor daemon underneath. Scene: one shared runtime for humans and agents. Format: 1:1. Action: coexistence, one runtime. Lighting: flat UI glow, one mint focal light on the shared daemon indicator. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; multiplex-grid; matte; no people.
 - _CTA :_ cargo install rmux
- **16:00 · reddit** · Programmable Terminal · 🙋 MANUEL · ~35 min
 - Title: Driving long-lived terminal jobs over SSH from code, typed snapshots instead of capture-pane scraping
 
 A lot of us end up automating remote terminal work the same fragile way: ssh in, run something, tail a log or scrape tmux capture-pane, and guess when it's "done." It works until an output format changes or a connection drops mid-run.
 
 I've been building rmux to make that layer boring. A local daemon owns the sessions, so the work survives a dropped SSH connection. From code you get connect_or_start(), an idempotent ensure_session(), send_text(), wait_for_text() to synchronize on a real state, and snapshot() returning a typed PaneSnapshot { cols, rows, content } instead of a byte blob. Full tmux-compatible CLI on the same daemon, plus native Windows.
 
 Genuinely curious how this sub handles remote terminal orchestration today, what's your current approach to (a) surviving SSH drops and (b) knowing a remote command finished? Trade-offs welcome; it's a v0.3.1 preview. Repo: github.com/Helvesec/rmux
 - _À faire :_ Post to r/devops as a value-first text post that ASKS a question (their orchestration approach), not just pitches. No hero image. Reply to every answer for the first 2 hours and compare approaches honestly. Never announcement-spam.
 - _CTA :_ engage in comments; repo link at the end

### J27, mardi 28 juil.
- **17:00 · twitter** · Terminal Testing · 🤖 AUTO
 - One of the five demos we ship is a full Playwright-style TUI test harness (~1,495 LOC).
 
 It drives a real terminal app, waits on real states with wait_for_text(), and asserts on typed snapshots, no sleeps, no byte-diff brittleness. If you maintain a CLI or TUI and "we don't really test the UI" sounds familiar, this is the pattern to steal.
 - _Visuel :_ {SYSTEM} Subject: a CI test-run panel driving a TUI-under-test, a column of terminal assertions (wait_for_text, snapshot.content.contains) each resolving to a mint PASS, one '~1,495 LOC harness' caption. Scene: Playwright-style terminal testing in CI. Format: 4:5. Action: a full green/mint run completing. Lighting: even, mint on PASS states only. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, mint #36E2C4 on PASS only; deterministic clean layout; monospace; matte; no people.
 - _CTA :_ github.com/Helvesec/rmux

### J28, mercredi 29 juil.
- **10:00 · linkedin** · Build-in-Public · 🤖 AUTO
 - A month of building rmux in public taught me the pitch in one sentence: keep the tmux you know; gain the SDK you wished it had.
 
 Everything else is detail underneath that. The full 90-command tmux CLI so switching is free. A daemon-backed typed SDK so you drive sessions from code instead of scraping stdout. Structured snapshots so you inspect and test terminals deterministically. A Ratatui widget so you embed a live pane. Native Windows so it runs where tmux can't. And a daemon that owns your sessions so agents survive dropped connections.
 
 One runtime. Three surfaces. Built for the moment the terminal stopped being a human-only place.
 
 If that resonates, the best thing you can do is try it and file the first bug you hit: cargo install rmux --locked → github.com/Helvesec/rmux
 - _Visuel :_ {SYSTEM} Subject: a clean summary card, the line 'keep the tmux you know; gain the SDK you wished it had' set in neo-grotesk, with six tiny feature glyphs beneath (CLI · SDK · snapshots · widget · Windows · daemon). Scene: the one-sentence pitch. Format: 1:1. Action: confident summary. Lighting: flat, one mint focal light on the 'SDK' half of the line. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ cargo install rmux --locked

### J29, jeudi 30 juil.
- **11:00 · twitter** · Programmable Terminal · 🤖 AUTO
 - The fastest way to understand rmux is to run four lines:
 
 ```rust
 let client = connect_or_start()?;
 let pane = client.ensure_session("demo")?;
 pane.send_text("echo hello\n")?;
 println!("{}", pane.wait_for_text("hello").and(pane.snapshot())?.content);
 ```
 
 Connect, ensure, send, wait+snapshot. That's the terminal-with-an-API in one breath. cargo add rmux-sdk.
 - _Visuel :_ {SYSTEM} Subject: a single tidy Rust code card, four lines (connect_or_start / ensure_session / send_text / wait_for_text+snapshot) with the final snapshot content glowing mint. Scene: the whole SDK loop in one breath. Format: 4:5. Action: the last line resolving to a mint result. Lighting: even, low-noise, one mint focal light on the resolved snapshot. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, mint #36E2C4 on the result only; monospace alignment; plausible Rust; matte; no people.
 - _CTA :_ cargo add rmux-sdk

### J30, vendredi 31 juil.
- **10:00 · linkedin** · Build-in-Public · 🤖 AUTO
 - Closing out month one of building rmux in public with the thesis, stated plainly.
 
 For forty years the terminal has been a place you operate by hand. That was fine when the only operator was a human. It isn't fine anymore: more and more of the work in a terminal is done by autonomous agents that need to be started, supervised, inspected, and orchestrated from code, over SSH, at scale, surviving dropped connections.
 
 A multiplexer built for that world has to be three things at once: fully tmux-compatible so humans lose nothing, programmable so code is a first-class operator, and inspectable so state is a value you can read, not pixels you squint at. That's the whole design of rmux, and it's why I think "programmable terminal multiplexer" is a category, not a feature.
 
 The terminal is getting an API. If you build or run agents, I'd love your eyes on it, and your first bug report. github.com/Helvesec/rmux
 - _Visuel :_ {SYSTEM} Subject: a quiet thesis card, 'the terminal is getting an API' set in neo-grotesk on generous canvas-black negative space, a single mint blinking cursor after the sentence, small monospaced 'rmux' wordmark and repo handle at the bottom. Scene: month-one thesis statement. Format: 1:1. Action: calm, declarative, forward-looking. Lighting: flat, one mint focal light on the cursor only. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent on the cursor; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ Star github.com/Helvesec/rmux

# ── Mois 2 (J31-J60) ──

### J31, samedi 1 août
- **09:00 · linkedin** · Build-in-Public · 🤖 AUTO
 - Month one of rmux in public taught me something about "proof."
 
 When you ship a preview, nobody wants the pitch, they want to see the thing under load. So this month I'm doing less positioning and more evidence: real case studies, the daemon protocol torn open, the objections answered honestly, and the four demos beyond orchestration.
 
 Start here, the one number that matters for a multiplexer you leave running for days: it holds the session when the connection doesn't. Detach, drop SSH, reattach, and pane.snapshot() reads exactly what the process did while you were gone. No scraping.
 
 The next 30 days are receipts, not claims.
 
 github.com/Helvesec/rmux
 - _Visuel :_ {SYSTEM} Subject: a calm 'Month 2, Proof & Depth' title card over a faint multiplex grid of panes, one active pane glowing mint. Scene: a quiet chapter divider. Format: 1:1 for LinkedIn. Action: measured confidence, engineered order. Lighting: flat, single mint focal light on the active pane. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark lower-left; matte; no people.
 - _CTA :_ Star github.com/Helvesec/rmux

### J32, dimanche 2 août
- **10:00 · twitter** · Agentic Terminal · 🤖 AUTO
 - Case study: wiring a Claude Code agent to survive a laptop-lid-close.
 
 Before: agent ran in a raw SSH shell → lid closes → connection dies → run gone.
 
 After, with rmux:
 
 ```rust
 let pane = client.ensure_session("agent")?;
 pane.send_text("claude -p 'refactor the auth module'\n")?;
 // close the lid, come back an hour later
 let snap = pane.snapshot()?;
 assert!(snap.content.contains("done"));
 ```
 
 The session outlives the connection. You read the result from code. This is the wedge.
 - _Visuel :_ {SYSTEM} Subject: a single agent pane running an autonomous task, an SSH link icon greyed/dropped in the corner while the pane keeps streaming log lines, a mint cursor still alive. Scene: a real agent surviving a disconnect. Format: 4:5. Action: continuity under failure; quiet reassurance. Lighting: flat UI glow, one mint focal light on the surviving pane. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; real plausible agent log lines; matte; no people.
 - _CTA :_ cargo install rmux

### J33, lundi 3 août
- **09:00 · linkedin** · Engineering · 🤖 AUTO
 - How rmux actually works, part 1: why there's a daemon at all.
 
 Every terminal multiplexer has to answer one question, where does the session live when nobody's attached? tmux answered it in 2007 with a server process. rmux keeps that shape but changes what sits on top.
 
 The daemon owns every PTY. Your CLI, your Rust code, and an embedded Ratatui widget are all just clients speaking the same local IPC protocol to it. That's the trick behind "anything the CLI can do, code can do": there's no separate scripting layer that drifts, there's one protocol, three front doors.
 
 It also means detach is free. The session was never tied to your terminal; it was always the daemon's. You just stopped watching.
 
 Next part: what a PaneSnapshot really contains, and why it's typed.
 
 github.com/Helvesec/rmux
 - _Visuel :_ {SYSTEM} Subject: a box-and-connector diagram, a central daemon node owning three stacked PTY blocks, with three client boxes (CLI / rmux-sdk / ratatui-rmux) connecting in over thin #1E2733 lines. Scene: 'where the session lives' schematic. Format: 16:9 OG. Action: static engineered clarity. Lighting: schematic flat, mint focal light on the daemon node only. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent on the daemon; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ Read the docs · github.com/Helvesec/rmux

### J34, mardi 4 août
- **11:00 · twitter** · tmux, Upgraded · 🤖 AUTO
 - "I'm not relearning tmux.", every tmux veteran, immediately.
 
 You won't. All 90 commands are implemented with the same verbs: split-window, select-pane, copy-mode. Your prefix key, your bindings, your workflow, identical.
 
 rmux reads your existing .tmux.conf on first run and migrates it. Opt out with one env var if you'd rather start clean.
 
 Same muscle memory. New surface area.
 - _Visuel :_ {SYSTEM} Subject: a .tmux.conf file panel on the left flowing through a thin mint connector into an identical rmux session on the right, key bindings shown carrying across unchanged. Scene: zero-cost migration. Format: 1:1. Action: effortless continuity. Lighting: flat, mint accent only on the migration arrow. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; comparison grid; matte; no people.
 - _CTA :_ cargo install rmux --locked

### J35, mercredi 5 août
- **11:00 · reddit** · Engineering / Programmable Terminal · 🙋 MANUEL · ~35 min
 - Title: Driving long-lived terminal jobs over SSH from code, typed snapshots instead of capture-pane scraping
 
 Most of us who automate remote terminal work end up in the same place: spawn the job, then poll with tmux capture-pane (or expect) and regex the screen to figure out what state it's in. It works until the output format shifts by one character and your parser silently lies to you.
 
 I've been building rmux, a Rust, tmux-compatible multiplexer, around a different primitive. The daemon that owns each PTY exposes a typed SDK, so instead of scraping you get:
 
 - ensure_session(name), idempotent attach-or-create, so your automation is re-runnable
 - send_text(...) / wait_for_text("ready"), synchronize on real state instead of sleeping
 - snapshot() -> PaneSnapshot { cols, rows, content }, a structured value you assert on
 
 The session lives in the daemon, not your SSH connection, so a dropped link doesn't kill the job, you reattach and snapshot() to see everything that happened while you were gone. Native Windows too (ConPTY + Named Pipes), which matters if your fleet is mixed. #![forbid(unsafe_code)] in the upper crates, no network at runtime.
 
 Curious how others here handle terminal-job state today, capture-pane? expect? a homegrown PTY wrapper? Happy to compare notes and go deep on the snapshot model. It's a v0.3.1 preview: github.com/Helvesec/rmux
 - _À faire :_ Post to r/devops as a value-first text discussion (lead with the shared pain, ask a real question, link last). No hero image. Reply to every comment for the first 2 hours; be honest about what's still rough in preview, never announcement-spam. Repurpose answers into future X tips.
 - _CTA :_ engage in comments; repo link at the end
- **16:00 · twitter** · Programmable Terminal · 🤖 AUTO
 - capture-pane gives you bytes. snapshot() gives you a value.
 
 ```rust
 let snap = pane.snapshot()?;
 // PaneSnapshot { cols: 120, rows: 40, content: String }
 for line in snap.content.lines().rev().take(5) { ... }
 ```
 
 cols and rows are typed fields, not something you parse out of an escape sequence. That's the whole difference.
 - _Visuel :_ {SYSTEM} Subject: a split card, left a raw byte/escape-sequence blob greyed out, right a clean typed PaneSnapshot with cols/rows/content fields resolving in mint. Scene: 'bytes vs a value'. Format: 4:5. Action: the messy left giving way to the structured right. Lighting: even, low-noise, mint only on the typed fields. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospace; plausible Rust; matte; no people.
 - _CTA :_ cargo add rmux-sdk

### J36, jeudi 6 août
- **12:00 · twitter** · Engineering · 🤖 AUTO
 - How rmux works, part 2: what's actually inside a PaneSnapshot.
 
 It's not a screenshot and it's not a byte dump. It's the pane's grid, resolved:
 
 ```rust
 pub struct PaneSnapshot {
   pub cols: u16,
   pub rows: u16,
   pub content: String, // the rendered grid, newline-joined
 }
 ```
 
 Escape sequences already applied. Cursor moves already resolved. You get what a human would see, as a value you can assert on. That's why terminal tests stop being flaky.
 - _Visuel :_ {SYSTEM} Subject: a Rust struct card for PaneSnapshot with cols/rows/content fields, beside a small terminal grid it maps to, one field highlighted mint. Scene: the anatomy of a snapshot. Format: 1:1. Action: precise, resolved, clean. Lighting: even, mint focal light on the content field. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospace; plausible Rust; matte; no people.
 - _CTA :_ Docs · rmux.io/docs

### J37, vendredi 7 août
- **17:00 · twitter** · Agentic Terminal · 🤖 AUTO
 - Demo #2 of 5 we ship: the Agent Broadcast Arena (~2,171 LOC).
 
 One command fans out to every agent pane at once, then each pane reports back independently. Think "send the same prompt to planner + coder + tester, then snapshot() where each one landed."
 
 It's the orchestration primitive that's painful to build on raw tmux + shell glue, here it's a for-loop. Recording coming this week.
 - _Visuel :_ {SYSTEM} Subject: a near-black terminal split into a grid of six agent panes, one broadcast command echoing across all of them simultaneously, each pane showing a slightly different response state, the active pane cursor mint. Scene: broadcast-to-many arena. Format: 16:9. Action: coordinated fan-out, controlled. Lighting: flat UI glow, single mint focal light. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; multiplex-grid; real plausible agent log lines; matte; no people.
 - _CTA :_ cargo install rmux

### J38, samedi 8 août
- **09:00 · linkedin** · Build-in-Public · 🤖 AUTO
 - Build-in-public receipt: this week's release notes, unedited.
 
 - Fixed a copy-mode selection off-by-one on wrapped lines (reported via issue #, thank you)
 - Hardened the daemon reconnect path so a killed client never orphans a session
 - Added wait_for_text timeout semantics (it now returns a typed error, not a hang)
 - Windows: resolved a Named Pipe permission edge case on locked-down corp machines
 
 None of that is glamorous. All of it is why you'd trust a multiplexer to hold a three-day agent run. Preview software earns trust one boring fix at a time, and every one of these came from someone filing an issue.
 
 Keep them coming: github.com/Helvesec/rmux/issues
 - _Visuel :_ {SYSTEM} Subject: a clean changelog panel, four terminal-styled bullet rows each prefixed with a small check glyph, one row highlighted mint. Scene: an honest release-notes card. Format: 1:1. Action: quiet, methodical progress. Lighting: flat, mint focal light on one fixed line. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ File an issue · github.com/Helvesec/rmux/issues

### J39, dimanche 9 août
- **11:00 · youtube** · Agentic Terminal · 🙋 MANUEL · ~75 min
 - rmux, the Agent Broadcast Arena, live (75s screencast)
 
 An un-cut recording of the shipped Broadcast Arena demo (~2,171 LOC): spin up six agent panes from one daemon, broadcast a single prompt to all of them at once, then snapshot() each pane to see where every agent landed, independently, no scraping. This is the orchestration pattern that's a pile of brittle glue on raw tmux, and a for-loop here.
 
 rmux is a Rust, tmux-compatible multiplexer with a daemon-backed typed SDK. cargo install rmux → github.com/Helvesec/rmux
 - _À faire :_ RECORD the Agent Broadcast Arena demo (the shipped ~2,171-LOC demo) as a 60-90s asciinema/screen capture: 6 agent panes → one broadcast command → each pane responds → snapshot() each. Keep the brand terminal theme (canvas #0B0E14, mint cursor, no rainbow highlight). HERO asset, repurpose into the D37/D40 X posts and future blog covers per the content-strategy repurposing engine.
 - _CTA :_ cargo install rmux · watch full demo

### J40, lundi 10 août
- **15:00 · twitter** · Agentic Terminal · 🤖 AUTO
 - New recording: broadcasting one prompt to six agents from a single rmux daemon, then reading each pane back with snapshot(). 👇
 
 No cuts, no stdout scraping. The panes survive; the daemon coordinates; your code inspects.
 
 [link to the broadcast screencast]
 - _À faire :_ No Higgsfield generation, attach the D39 Broadcast Arena screencast (recorded earlier) as the video/GIF; the recording IS the visual.
 - _CTA :_ Watch screencast; cargo install rmux

### J41, mardi 11 août
- **10:00 · twitter** · Programmable Terminal · 🤖 AUTO
 - Objection: "Why not just tmux control mode?"
 
 Fair question, control mode does let you drive tmux from a program. But it hands you a text stream you have to parse, and the format is a protocol you don't control.
 
 rmux gives you typed calls instead: wait_for_text("ready") returns Result, snapshot() returns a struct. When tmux changes its output, your control-mode parser breaks. When rmux changes internally, your types still compile.
 
 Parsing vs. calling. That's the difference.
 - _Visuel :_ {SYSTEM} Subject: a two-column contrast card, left 'control mode: parse a text stream' shown as a greyed escape-sequence blob, right 'rmux-sdk: typed calls' shown as clean Rust method calls resolving mint. Scene: parsing vs calling. Format: 4:5. Action: the fragile left vs the typed right. Lighting: even, mint only on the typed side. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospace; plausible Rust; matte; no people.
 - _CTA :_ cargo add rmux-sdk

### J42, mercredi 12 août
- **12:00 · twitter** · Terminal Testing · 🤖 AUTO
 - Demo #3 of 5: the Playwright-style testing harness we ship (~1,495 LOC).
 
 It drives a real TUI, waits on actual rendered state, and asserts on a typed snapshot, the same loop Playwright made normal for the browser, now for terminals:
 
 ```rust
 pane.send_text("./mytui\n")?;
 pane.wait_for_text("Loaded")?;
 assert!(pane.snapshot()?.content.contains("3 items"));
 ```
 
 No sleep(). No flaky byte diffs. CI-friendly by construction.
 - _Visuel :_ {SYSTEM} Subject: a test-runner panel driving a small TUI pane, a column of assertions (send_text / wait_for_text / snapshot.content.contains) each resolving to a mint PASS. Scene: Playwright-for-terminals harness. Format: 16:9. Action: a deterministic run completing. Lighting: even, mint only on the PASS states. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, mint #36E2C4 on PASS only; deterministic clean layout; monospace; plausible Rust; matte; no people.
 - _CTA :_ Docs · rmux.io/docs

### J43, jeudi 13 août
- **16:00 · devto** · Terminal Testing · 🤖 AUTO
 - # Deterministic terminal tests: a Playwright-style harness for your TUI
 
 Everyone who has tested a terminal app knows the two failure modes: sleep(2) and hope, or diff a raw byte capture that breaks the moment a color code moves. Both are why "we don't really test the TUI" is so common.
 
 rmux ships a testing harness (~1,495 LOC, one of five demos) built on two primitives the rmux-sdk gives you: synchronize on rendered text, then assert on a typed snapshot.
 
 ## The loop
 ```rust
 use rmux_sdk::connect_or_start;
 
 let client = connect_or_start()?;
 let pane = client.ensure_session("tui-test")?;
 
 pane.send_text("./target/debug/mytui\n")?;
 pane.wait_for_text("Ready")?;     // block on real state, no sleep
 
 pane.send_text("j j <Enter>")?;     // drive it like a user
 pane.wait_for_text("Details")?;
 
 let snap = pane.snapshot()?;      // typed PaneSnapshot
 assert_eq!(snap.cols, 120);
 assert!(snap.content.contains("Selected: item 3"));
 ```
 
 ## Why it's deterministic
 - wait_for_text blocks until the grid actually contains the text, then returns, replacing sleeps with real synchronization.
 - snapshot() returns the rendered grid, escape sequences already applied. You assert on what a human would see, not on bytes.
 - ensure_session is idempotent, so the same test is safe to re-run and safe in CI.
 
 ## In CI
 It runs headless in a container, the daemon owns the PTY, so there's no attached terminal to depend on. No network at runtime, locked builds, dual MIT/Apache.
 
 v0.3.1 preview, all 90 tmux commands work and bugs get fixed fast. Try it in your test suite and tell me where it's rough: github.com/Helvesec/rmux
 - _Visuel :_ {SYSTEM} Subject: a blog-cover split, left a Rust test file calling wait_for_text/snapshot, right a TUI pane under test with a mint PASS column resolving. Scene: deterministic terminal testing. Format: 16:9 blog cover. Action: the run turning green cleanly. Lighting: even, low-noise, mint only on PASS. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, mint #36E2C4 accent; monospaced 'rmux' wordmark; monospace code; matte; no people.
 - _CTA :_ cargo add rmux-sdk

### J44, vendredi 14 août
- **17:00 · twitter** · Engineering · 🤖 AUTO
 - Behind the scenes: the hardest part of rmux wasn't tmux compatibility. It was Windows.
 
 tmux doesn't run natively on Windows because there's no PTY the way Unix has one. Windows has ConPTY, a different beast, and no Unix domain sockets, so the daemon talks over per-user Named Pipes instead.
 
 Getting detach/reattach to behave identically on ConPTY + Named Pipes as it does on a Unix PTY + socket was weeks of work. The payoff: the same rmux session, the same keys, the same SDK, natively, no WSL shim.
 - _Visuel :_ {SYSTEM} Subject: a schematic contrasting two stacks, 'Unix: PTY + domain socket' and 'Windows: ConPTY + Named Pipe', both feeding one shared daemon node in the middle. Scene: cross-platform PTY architecture. Format: 16:9. Action: two paths converging to parity. Lighting: schematic flat, mint focal light on the shared daemon. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent on the daemon; monospaced 'rmux' wordmark; box-and-connector; matte; no people.
 - _CTA :_ github.com/Helvesec/rmux

### J45, samedi 15 août
- **13:00 · twitter** · Programmable Terminal · 🤖 AUTO
 - Demo #4 of 5: Mini-Zellij (~944 LOC), a full pane manager built on rmux-sdk.
 
 The point isn't that it's a zellij clone. It's that the entire layout engine, splits, focus, resize, is written against the same SDK your automation uses. Anything the CLI does, code does, including building a whole new front-end.
 
 Proof that "three surfaces, one daemon" isn't a slogan. It compiles.
 - _Visuel :_ {SYSTEM} Subject: a custom pane-manager TUI (mini-zellij) with several split panes, a focused pane glowing mint, grey widget chrome around it. Scene: a front-end built entirely on rmux-sdk. Format: 4:5. Action: a real layout engine, calm and ordered. Lighting: flat UI glow, one mint focal light on the focused pane. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; multiplex-grid; matte; no people.
 - _CTA :_ cargo add rmux-sdk

### J46, dimanche 16 août
- **11:00 · reddit** · Programmable Terminal / Rustacean · 🙋 MANUEL · ~30 min
 - Title: cargo add ratatui-rmux, drop a live terminal pane into your Ratatui app (feedback welcome)
 
 If you build TUIs with Ratatui, you've probably hit the wall where you want a real terminal inside your app, a shell, a build log, an agent, and the options are all bad: shell out and parse, or reimplement a PTY yourself.
 
 ratatui-rmux is a widget backed by the same rmux daemon that powers the CLI and SDK, so the pane you embed is a real, live terminal session, detachable and driveable from code, not a mock.
 
 ```rust
 use ratatui_rmux::TerminalPane;
 // in your draw loop:
 frame.render_widget(TerminalPane::new(&pane), area);
 ```
 
 Because it's the same daemon protocol, you can also snapshot() or wait_for_text() that same pane from your app's logic. I'd genuinely like Ratatui folks to kick the tires on the widget API, what feels off, what's missing, whether the dependency weight is acceptable. It's a v0.3.1 preview: github.com/Helvesec/rmux
 - _À faire :_ Post in the Ratatui Discord (showcase/feedback channel), value-first, ask for API critique, not upvotes. Have a running mini-zellij / embedded-pane screenshot ready to share live. Reply to every response; log the API feedback as GitHub issues. Never announcement-spam.
 - _CTA :_ engage in the thread; repo link at the end

### J47, lundi 17 août
- **09:00 · linkedin** · Engineering / Platform · 🤖 AUTO
 - For the platform and security folks vetting rmux: the posture, plainly.
 
 - #![forbid(unsafe_code)] in the upper crates. The OS/PTY boundary, where unsafe is unavoidable, is isolated in one place, not sprinkled through the codebase.
 - No network at runtime. Not "we don't phone home as a policy", it's enforced by a build guardrail that fails the build if a network call sneaks in.
 - Locked, reproducible builds. Dual MIT OR Apache-2.0. A CI release-validation gate on every cut.
 
 This is a tool you leave running on your fleet for days. The supply-chain story should be boring and checkable, so I made it both.
 
 github.com/Helvesec/rmux
 - _Visuel :_ {SYSTEM} Subject: three matte status glyphs on canvas-black, 'forbid(unsafe_code)', 'no network at runtime', 'locked builds', rendered as clean badges, one glowing mint. Scene: a security-posture card. Format: 1:1. Action: quiet authority, checkable claims. Lighting: schematic flat, one mint rim-light on the forbid-unsafe glyph. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ Read the security posture · github.com/Helvesec/rmux

### J48, mardi 18 août
- **12:00 · twitter** · tmux, Upgraded · 🤖 AUTO
 - Objection: "zellij already exists, why rmux?"
 
 Both true and fine. zellij is a genuinely great human UX, floating panes, WASM plugins, a lovely default experience. If you want the best keyboard-driven multiplexer for a person, use it.
 
 rmux is built for the case zellij doesn't target: driving and inspecting sessions (and agents) from typed code, with full tmux-CLI compatibility on top. Different center of gravity. zellij optimizes the human at the keyboard; rmux optimizes the code, and the agent, behind it.
 - _Visuel :_ {SYSTEM} Subject: a restrained two-lane diagram, one lane labeled 'human UX' (grey), one lane labeled 'code + agents' (mint), both terminating at a terminal grid. Scene: honest positioning, not a takedown. Format: 4:5. Action: clear differentiation, respectful. Lighting: flat, mint only on the code/agents lane. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ github.com/Helvesec/rmux

### J49, mercredi 19 août
- **17:00 · twitter** · Programmable Terminal · 🤖 AUTO
 - The one method that changed how I write terminal automation: ensure_session.
 
 ```rust
 let pane = client.ensure_session("deploy")?;
 ```
 
 It's idempotent. Session exists? You attach. Doesn't? It's created. Your script stops caring which run it's on, no "does it already exist" branching, no cleanup dance. Re-runnable automation, one call.
 
 Small primitive, huge ergonomics.
 - _Visuel :_ {SYSTEM} Subject: a single Rust code card for ensure_session with two mint arrows branching to 'attach' and 'create' converging back to one pane. Scene: idempotent session lifecycle. Format: 1:1. Action: two paths, one clean result. Lighting: even, mint focal light on the converged pane. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospace; plausible Rust; matte; no people.
 - _CTA :_ cargo add rmux-sdk

### J50, jeudi 20 août
- **09:00 · linkedin** · Build-in-Public · 🙋 MANUEL · ~20 min
 - What broke this week (a founder's build-in-public note).
 
 A user filed an issue: their agent's session showed a truncated snapshot() after a very large paste, thousands of lines at once. The daemon was flushing the paste in bursts, and the snapshot occasionally read the grid mid-flush.
 
 The wrong fix is to add a sleep before snapshot(). The right fix, and the one that shipped, was to make snapshot() consistent with respect to in-flight writes, so you always read a coherent grid, never a half-applied one.
 
 I'm posting this because "build in public" shouldn't only mean the wins. A multiplexer you trust with a three-day run is built out of exactly these unglamorous consistency bugs, found by real users, fixed in the open.
 
 If you're running long agent sessions and hit an edge like this, file it. That's the whole feedback loop. github.com/Helvesec/rmux/issues
 - _À faire :_ FOUNDER post, write/record in your own voice (optionally a short talking-head or a screen recording of the issue → fix → passing test). Keep it honest and specific; this is the human-engagement / behind-the-scenes beat. If video: brand terminal theme in any screen capture (canvas #0B0E14, mint cursor). Engage with commenters personally.
 - _CTA :_ File an issue · github.com/Helvesec/rmux/issues

### J51, vendredi 21 août
- **10:00 · twitter** · Agentic Terminal · 🤖 AUTO
 - Case study: inspecting an agent without interrupting it.
 
 The old way to check on a long agent run was to attach, and attaching changes terminal size, redraws, sometimes nudges the process. So you'd hesitate to look.
 
 With rmux you don't attach to look. snapshot() reads the pane's grid from the daemon, out of band. The agent never knows you checked.
 
 ```rust
 let snap = client.ensure_session("agent")?.snapshot()?;
 println!("{}", snap.content);
 ```
 
 Observe without disturbing. That's a real difference for autonomous runs.
 - _Visuel :_ {SYSTEM} Subject: an agent pane running undisturbed while a separate 'snapshot()' read arrow taps it out-of-band from the daemon, the agent pane cursor mint and uninterrupted. Scene: observe-without-disturbing. Format: 4:5. Action: a quiet side-channel read. Lighting: flat UI glow, one mint focal light on the running pane. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; real plausible agent log lines; matte; no people.
 - _CTA :_ cargo add rmux-sdk

### J52, samedi 22 août
- **11:00 · twitter** · Engineering · 🤖 AUTO
 - Demo #5 of 5: Terminal ↔ Browser Mirroring (~649 LOC).
 
 A live rmux pane, mirrored into a browser in real time, powered by the same snapshot stream your code reads. The terminal isn't screenshotted; the browser renders the actual grid the daemon owns.
 
 Why it matters: if a terminal's state is a typed value, you can send it anywhere, a test, a TUI widget, or a webpage. One inspectable source of truth, many surfaces.
 - _Visuel :_ {SYSTEM} Subject: a terminal pane on the left mirrored to a browser window on the right via a thin mint stream connector, both showing the identical grid. Scene: terminal-to-browser mirroring from one snapshot stream. Format: 16:9. Action: one source, two surfaces, in sync. Lighting: flat, mint only on the stream connector. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ github.com/Helvesec/rmux

### J53, dimanche 23 août
- **11:00 · youtube** · Programmable Terminal · 🙋 MANUEL · ~75 min
 - rmux, mirroring a live terminal into the browser (60s screencast)
 
 The shipped Terminal↔Browser Mirroring demo (~649 LOC), un-cut: a real rmux pane rendered live in a browser tab off the same snapshot stream your code reads. Type in the terminal, watch it appear in the browser, not a screenshot, the actual grid the daemon owns. Proof that once terminal state is a typed value, you can put it on any surface.
 
 rmux is a Rust, tmux-compatible multiplexer with a daemon-backed typed SDK. cargo install rmux → github.com/Helvesec/rmux
 - _À faire :_ RECORD the Terminal↔Browser Mirroring demo (shipped ~649-LOC demo) as a 60s screen capture: terminal on one side, browser on the other, typing mirrored live. Brand terminal theme (canvas #0B0E14, mint cursor). Repurpose into the D52 X post and a blog cover per the repurposing engine.
 - _CTA :_ cargo install rmux · watch full demo

### J54, lundi 24 août
- **16:00 · devto** · Agentic Terminal · 🤖 AUTO
 - # Orchestrating N agents from one daemon: a walkthrough
 
 The pitch is "tmux for agents." This post is the code behind it, how you actually run, coordinate, and inspect several long-lived agents from a single rmux daemon.
 
 ## The shape of the problem
 You have three agents, say a planner, a coder, and a tester, each a long-running process over SSH. You need to: start them, keep them alive past disconnects, send each one work, and know where each one is without babysitting a terminal.
 
 ## Start and address them by name
 ```rust
 use rmux_sdk::connect_or_start;
 
 let client = connect_or_start()?;
 for name in ["planner", "coder", "tester"] {
   let pane = client.ensure_session(name)?;  // idempotent
   pane.send_text(&format!("agent --role {name}\n"))?;
 }
 ```
 Each agent now lives in the daemon, addressable by a stable name. Your SSH connection is just a viewer.
 
 ## Coordinate with real state, not sleeps
 ```rust
 let planner = client.ensure_session("planner")?;
 planner.wait_for_text("plan ready")?;     // block on actual output
 let plan = planner.snapshot()?.content;     // typed grid
 
 let coder = client.ensure_session("coder")?;
 coder.send_text(&format!("implement: {}\n", first_task(&plan)))?;
 ```
 
 ## Inspect everyone at once
 ```rust
 for name in ["planner", "coder", "tester"] {
   let snap = client.ensure_session(name)?.snapshot()?;
   println!("[{name}] {} cols x {} rows", snap.cols, snap.rows);
   println!("{}", last_lines(&snap.content, 3));
 }
 ```
 No attaching, no capture-pane scraping, a structured read of every agent's terminal.
 
 ## Why the daemon matters here
 Because the sessions live in the daemon, not your connection, a dropped SSH link doesn't kill the run. Reattach later and every pane is exactly where you left it. This is the whole reason rmux exists, and it's the same primitives (ensure_session, send_text, wait_for_text, snapshot) the CLI and the Ratatui widget use.
 
 v0.3.1 preview, native Windows too, forbid-unsafe, no network at runtime. Try it and file issues: github.com/Helvesec/rmux
 - _Visuel :_ {SYSTEM} Subject: a blog cover, three named agent panes (planner/coder/tester) fed by one central daemon node, a mint snapshot() arrow reading each pane back. Scene: multi-agent orchestration walkthrough. Format: 16:9 blog cover. Action: coordinated, inspectable orchestration. Lighting: flat, mint focal light on the daemon and the active read. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; multiplex-grid + connectors; matte; no people.
 - _CTA :_ cargo install rmux --locked

### J55, mardi 25 août
- **12:00 · twitter** · Terminal Testing · 🤖 AUTO
 - Objection: "Does this actually run headless in CI?"
 
 Yes, and it's the whole design. The daemon owns the PTY, so there's no attached terminal for a test to depend on. In a container, ensure_session creates the session, wait_for_text synchronizes, snapshot() asserts. No TTY gymnastics, no xvfb-style shims.
 
 That's why the testing harness works the same on your laptop and in a headless runner. Deterministic isn't a feature you bolt on; it falls out of the daemon model.
 - _Visuel :_ {SYSTEM} Subject: a CI runner panel, a headless container icon feeding a daemon node that drives a test pane, assertions resolving to a mint PASS column. Scene: headless CI terminal testing. Format: 4:5. Action: a clean pass with no attached terminal. Lighting: even, mint only on the PASS states. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, mint #36E2C4 on PASS only; monospace; matte; no people.
 - _CTA :_ cargo add rmux-sdk

### J56, mercredi 26 août
- **09:00 · linkedin** · Build-in-Public / Engineering · 🤖 AUTO
 - A quiet milestone worth naming: every one of the 90 tmux-compatible commands is now verified across Linux, macOS, and Windows in the feature matrix, not "should work," but checked on each platform.
 
 Why make a fuss about a matrix? Because compatibility is a claim people are right to distrust. "tmux-compatible" usually means "the common 20 commands." For a veteran to switch, the long tail, copy-mode, nested sessions, the obscure binding buried in their .tmux.conf, has to work too.
 
 The matrix is how I keep myself honest about that, and it's public. If a command behaves differently than tmux for you, that's a bug, and I want the issue.
 
 github.com/Helvesec/rmux
 - _Visuel :_ {SYSTEM} Subject: a clean verification-matrix grid, rows of tmux command groups against three platform columns (Linux/macOS/Windows), cells marked with small check glyphs, the fully-verified row glowing mint. Scene: a public feature matrix. Format: 1:1. Action: methodical, checkable honesty. Lighting: schematic flat, mint focal light on one verified row. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ See the matrix · github.com/Helvesec/rmux

### J57, jeudi 27 août
- **11:00 · twitter** · Programmable Terminal · 🤖 AUTO
 - How rmux works, part 3: connect_or_start, or "why you never manage the daemon."
 
 ```rust
 let client = connect_or_start()?;
 ```
 
 One call. If the daemon is up, you connect. If it isn't, it's started, then you connect. Your program never has to check, spawn, or babysit a background process, the lifecycle is the library's problem, not yours.
 
 It's a small thing that removes an entire class of "is the server running?" boilerplate from every script.
 - _Visuel :_ {SYSTEM} Subject: a single Rust code card for connect_or_start with a mint branch to 'connect' vs 'start then connect', both landing on one daemon node. Scene: transparent daemon lifecycle. Format: 1:1. Action: one call, no babysitting. Lighting: even, mint focal light on the daemon node. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospace; plausible Rust; matte; no people.
 - _CTA :_ cargo add rmux-sdk

### J58, vendredi 28 août
- **17:00 · twitter** · tmux, Upgraded · 🤖 AUTO
 - Objection: "Is it actually compatible, or just 'inspired by' tmux?"
 
 Actually compatible. All 90 commands, the same verbs and arguments, same prefix-key model, same copy-mode. Your .tmux.conf is read and migrated on first run.
 
 "Inspired by" means you relearn things. Compatible means you type split-window -h and get a horizontal split, exactly like you already do. The proof is the public feature matrix, verified per-command, per-platform. If something diverges from tmux, file it as a bug.
 - _Visuel :_ {SYSTEM} Subject: a side-by-side of a tmux command and the identical rmux command producing the identical split, a small mint 'verified' check between them. Scene: real compatibility, not homage. Format: 4:5. Action: identical behavior, provable. Lighting: flat, mint only on the verified check. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; comparison grid; matte; no people.
 - _CTA :_ cargo install rmux --locked

### J59, samedi 29 août
- **12:00 · twitter** · Rustacean / Programmable Terminal · 🤖 AUTO
 - For the Rust crowd wondering about weight: rmux-sdk is a thin, typed client over a local IPC protocol, not a re-embed of a terminal emulator.
 
 You depend on the SDK crate; the daemon is a separate process it talks to. So your binary stays small, and the heavy PTY/multiplexing work lives out-of-process where a crash can't take your app with it.
 
 Type-safe API, small dependency, process isolation. The Rust defaults you'd want.
 - _Visuel :_ {SYSTEM} Subject: a schematic, a small 'your app + rmux-sdk' box connected by a thin mint IPC line to a separate, larger 'daemon (PTY/multiplex)' box. Scene: thin client, out-of-process engine. Format: 1:1. Action: light dependency, isolated heavy work. Lighting: schematic flat, mint only on the IPC connector. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; box-and-connector; matte; no people.
 - _CTA :_ cargo add rmux-sdk

### J60, dimanche 30 août
- **09:00 · linkedin** · Build-in-Public · 🤖 AUTO
 - 30 days of proof, in one post.
 
 This month I stopped pitching rmux and started showing it: the daemon protocol torn open, all four demos beyond orchestration (broadcast, mini-zellij, testing harness, browser mirroring), the objections answered in public, and the boring consistency bugs fixed in the open by real users' issues.
 
 What I keep coming back to: a multiplexer earns trust the same way infrastructure always has, not with a claim, but with a verifiable feature matrix, an isolated unsafe boundary, no network at runtime, and a fix log anyone can read.
 
 Next month I go from proof to reach: the tmux-migration guides, the SDK cookbook, and the comparisons the switchers actually search for.
 
 If you run long-lived terminal work, for yourself or for agents, this is the month to try it. cargo install rmux → github.com/Helvesec/rmux
 - _Visuel :_ {SYSTEM} Subject: a summary board, five small demo thumbnails (orchestration, broadcast, mini-zellij, testing, mirroring) arranged in a grid over canvas-black, one tile glowing mint. Scene: a month-2 recap card. Format: 1:1. Action: a body of evidence, calm and complete. Lighting: flat, one mint focal light on the highlighted tile. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; multiplex-grid; matte; no people.
 - _CTA :_ cargo install rmux · Star github.com/Helvesec/rmux

# ── Mois 3 (J61-J90) ──

### J61, lundi 31 août
- **09:00 · linkedin** · Build-in-Public · 🤖 AUTO
 - Two months ago rmux was a claim: give the terminal an API.
 
 Today it's a thing you can cargo install, drive from Rust, and leave running for days with agents inside it. Same 90 tmux commands you already type, plus a daemon-backed typed SDK tmux never had.
 
 The next stretch isn't about features. It's about adoption: more installs, more crates.io pulls on rmux-sdk, more people embedding ratatui-rmux, more issues filed by people who actually depend on it.
 
 If you run long-lived terminal work over SSH, this is the moment to try it and tell me where it hurts. Preview-honest, bugs get fixed fast.
 
 cargo install rmux --locked → github.com/Helvesec/rmux
 - _Visuel :_ {SYSTEM} Subject: a quiet 'month 2 → month 3' momentum card, a single terminal pane on canvas-black with a monospaced caption 'from claim to cargo install' and a mint cursor blinking at the prompt. Format: 1:1 for LinkedIn. Action: understated milestone, forward motion. Lighting: flat UI glow, one mint focal light on the cursor. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark lower-left; matte; no people; no charts, no fabricated numbers.
 - _CTA :_ cargo install rmux --locked · Star Helvesec/rmux
- **14:00 · twitter** · Agentic Terminal · 🤖 AUTO
 - The 30-second pitch for rmux:
 
 Your agent runs over SSH. The connection drops. With tmux the session survives, but you're blind until you reattach and squint.
 
 With rmux: pane.snapshot() reads exactly what it did, from code, without attaching.
 
 That's the whole difference. cargo install rmux.
 - _Visuel :_ {SYSTEM} Subject: one terminal pane running an agent, an SSH link glyph dropping in the corner, and a small code overlay calling pane.snapshot() that resolves to typed fields, session survives, code reads it. Format: 16:9. Action: continuity across a dropped connection; quiet reassurance. Lighting: flat UI glow, single mint focal light on the snapshot result. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; real plausible agent log lines; matte; no people.
 - _CTA :_ cargo install rmux

### J62, mardi 1 sept.
- **11:00 · twitter** · Programmable Terminal · 🤖 AUTO
 - Three commands to go from zero to driving a terminal from Rust:
 
 ```bash
 cargo install rmux --locked
 cargo add rmux-sdk
 ```
 ```rust
 let pane = connect_or_start()?.ensure_session("work")?;
 ```
 
 That's it. The daemon lifecycle is handled for you. Now send_text, wait_for_text, snapshot(), all typed.
 - _Visuel :_ {SYSTEM} Subject: a clean three-step onboarding card, two shell lines then one Rust line, arranged as a numbered vertical flow, the final ensure_session line glowing mint. Format: 4:5. Action: frictionless first-run, each step locking in. Lighting: even, low-noise, one mint focal light on the last step. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; plausible shell + Rust; matte; no people.
 - _CTA :_ cargo add rmux-sdk

### J63, mercredi 2 sept.
- **10:00 · reddit** · Build-in-Public · 🙋 MANUEL · ~35 min
 - Title: Two months of rmux (Rust tmux-compatible multiplexer with a typed SDK), what people actually use it for, and where it's still rough
 
 I shipped rmux as a v0.3.x preview a couple months ago: a tmux-compatible multiplexer (all 90 commands) with a daemon-backed typed SDK, ensure_session, send_text, wait_for_text, snapshot() returning a structured PaneSnapshot.
 
 Rather than pitch, here's the honest state from real usage and issues:
 
 What people reach for it for: keeping long-lived agents alive over SSH and inspecting them from code without scraping capture-pane; deterministic TUI/CLI testing (wait_for_text + snapshot instead of sleeps); native Windows sessions (ConPTY + Named Pipes, no WSL).
 
 What's still rough (preview): some copy-mode edge cases, a couple of nested-session quirks, and Windows corner cases that get filed and fixed fast.
 
 I'd genuinely like to hear: if you script or test terminals today, what's the one primitive you wish existed? Happy to go deep on the daemon protocol or the snapshot model. Repo: github.com/Helvesec/rmux
 - _À faire :_ Post to r/rust or r/commandline as a value-first build-in-public retro (lead with usage + honest rough edges, link last). Have real open issues / a snapshot code sample ready to paste in replies. Answer every top comment for the first 2 hours. Never announcement-spam. If any comment is quotable-positive, screenshot it (with permission) as a testimonial asset for later days.
 - _CTA :_ engage in comments; repo link at the end

### J64, jeudi 3 sept.
- **13:00 · twitter** · Terminal Testing · 🤖 AUTO
 - If your CLI/TUI test suite is full of sleep(500) and flaky string diffs, this is for you.
 
 ```rust
 pane.wait_for_text("listening on :8080")?;
 assert!(pane.snapshot()?.content.contains("0 errors"));
 ```
 
 Synchronize, then assert. Deterministic in CI. rmux-sdk ships this today.
 - _Visuel :_ {SYSTEM} Subject: a before/after test card, left a greyed-out flaky 'sleep(500)' assertion, right an rmux wait_for_text + snapshot assertion resolving to a mint PASS. Format: 16:9. Action: flaky replaced by deterministic. Lighting: even, one mint focal light on the PASS side. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, mint #36E2C4 on the PASS side only; monospace; plausible Rust; matte; no people.
 - _CTA :_ cargo add rmux-sdk
- **17:00 · twitter** · tmux, Upgraded · 🤖 AUTO
 - Switching from tmux costs you exactly one thing: `cargo install rmux --locked`.
 
 Your .tmux.conf migrates on first run. Your keybindings come with you. Your prefix key, your panes, your windows, identical.
 
 You lose nothing and gain an SDK, native Windows, and forbid-unsafe. Try it this week.
 - _Visuel :_ {SYSTEM} Subject: a .tmux.conf file flowing rightward into an identical rmux session, a small 'migrated automatically' tag and a native Windows badge in the corner. Format: 1:1. Action: effortless continuity, zero switching cost. Lighting: flat. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent on the rmux side only; monospaced 'rmux' wordmark; migration grid; matte; no people.
 - _CTA :_ cargo install rmux --locked

### J65, vendredi 4 sept.
- **09:00 · linkedin** · Programmable Terminal · 🤖 AUTO
 - A pattern I keep recommending to people wiring agents: stop treating the terminal as a black box you scrape, and start treating it as a service you call.
 
 With rmux-sdk, connect_or_start() hands you a client, ensure_session(name) is idempotent (attach or create), and every pane returns a typed PaneSnapshot { cols, rows, content }. You wait_for_text() to synchronize, then read structured state.
 
 The practical payoff: your orchestration code becomes deterministic and testable instead of a pile of regexes over stdout. A human can start the session by hand; your code drives the exact same one.
 
 If you're building agent infrastructure, this is worth an afternoon. cargo add rmux-sdk → github.com/Helvesec/rmux
 - _Visuel :_ {SYSTEM} Subject: a schematic, 'terminal as a service': a code node on the left calling into a daemon node, which returns a typed PaneSnapshot card on the right; thin #1E2733 connectors. Format: 4:5 for LinkedIn. Action: structured request/response, engineered clarity. Lighting: schematic, flat, one mint focal light on the returned snapshot. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ cargo add rmux-sdk

### J66, samedi 5 sept.
- **12:00 · twitter** · Community · 🤖 AUTO
 - Small ask, big signal: if rmux has been useful to you, a GitHub star genuinely helps other people find it.
 
 It's a solo-ish preview competing with 15-year-old incumbents, discoverability is the whole game right now.
 
 ⭐ github.com/Helvesec/rmux
 
 And if it's NOT useful yet, reply and tell me why. That helps more.
 - _Visuel :_ {SYSTEM} Subject: a restrained call-to-star card, a single mint star glyph beside a monospaced 'Helvesec/rmux', quiet grey chrome around it, no confetti. Format: 1:1. Action: sincere, understated ask. Lighting: flat, one mint focal light on the star. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent on the star; monospaced 'rmux' wordmark; matte; no people; no fabricated star count.
 - _CTA :_ Star github.com/Helvesec/rmux

### J67, dimanche 6 sept.
- **10:00 · twitter** · Agentic Terminal · 🤖 AUTO
 - Orchestrating a small agent swarm is three primitives:
 
 ```rust
 for a in ["planner","coder","tester"] {
   client.ensure_session(a)?.send_text("start\n")?;
 }
 // later, from anywhere:
 let done = client.ensure_session("tester")?
   .snapshot()?.content.contains("PASS");
 ```
 
 Start many, poll any, from one daemon. This is the wedge.
 - _Visuel :_ {SYSTEM} Subject: a near-black terminal split into three labeled agent panes (planner / coder / tester), a small code overlay polling one pane's snapshot, active pane cursor mint. Format: 16:9. Action: coordinated multi-agent control from one daemon. Lighting: flat UI glow, single mint focal light on the active pane. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; multiplex-grid; real plausible agent log lines; matte; no people.
 - _CTA :_ cargo install rmux
- **16:00 · devto** · Agentic Terminal · 🤖 AUTO
 - # Orchestrating a small agent swarm with rmux-sdk
 
 You have three agents, a planner, a coder, a tester, and you want them running over SSH, surviving disconnects, and inspectable from one place. Here's the whole loop with rmux.
 
 ## Setup
 ```bash
 cargo install rmux --locked
 cargo add rmux-sdk
 ```
 
 ## Start the swarm
 ```rust
 use rmux_sdk::connect_or_start;
 
 let client = connect_or_start()?;
 for name in ["planner", "coder", "tester"] {
   let pane = client.ensure_session(name)?;  // idempotent: attach or create
   pane.send_text("./run-agent.sh\n")?;
 }
 ```
 
 ensure_session is idempotent, so this same code is safe to re-run after a crash or a dropped SSH connection, it reattaches instead of duplicating.
 
 ## Wait for readiness, then inspect
 ```rust
 let planner = client.ensure_session("planner")?;
 planner.wait_for_text("plan ready")?;     // synchronize
 let snap = planner.snapshot()?;        // typed PaneSnapshot
 assert!(snap.content.contains("3 tasks"));
 ```
 
 ## Poll any agent from anywhere
 ```rust
 let tester = client.ensure_session("tester")?;
 if tester.snapshot()?.content.contains("PASS") {
   println!("tester green");
 }
 ```
 
 Because the daemon owns the sessions, your orchestration process can restart and pick everything back up. No lost terminals, no capture-pane scraping, structured content instead.
 
 ## Why rmux for this
 tmux keeps sessions alive but hands you text to parse; rmux gives you the same session as a typed, inspectable object across CLI, SDK, and a Ratatui widget, one daemon, three surfaces. It's a v0.3.x preview: all 90 tmux commands work and bugs get fixed fast. Try it and file an issue: github.com/Helvesec/rmux
 - _Visuel :_ {SYSTEM} Subject: left a Rust code panel looping ensure_session over three agent names, right a three-pane terminal with the agents running and one pane's snapshot resolving mint. Format: 16:9 blog cover. Action: code spinning up a swarm, one pane reporting PASS. Lighting: even, one mint focal light on the resolved pane. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, mint #36E2C4 only on the active result; monospace; plausible Rust + agent logs; matte; no people.
 - _CTA :_ cargo add rmux-sdk

### J68, lundi 7 sept.
- **11:00 · twitter** · Community · 🤖 AUTO
 - First-issue-friendly, honestly:
 
 rmux is a preview with real edges, copy-mode cases, a few Windows corners, docs that could be sharper. If you've ever wanted to contribute to a Rust project you actually use, this is a low-barrier one.
 
 The daemon protocol is small. The SDK is typed. PRs get reviewed fast.
 
 github.com/Helvesec/rmux
 - _Visuel :_ {SYSTEM} Subject: a quiet 'good first issue' card, a monospaced issue-title line with a small mint label tag, grey issue chrome beneath. Format: 1:1. Action: an open, welcoming invitation to contribute. Lighting: flat, one mint focal light on the label. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent on the label; monospaced 'rmux' wordmark; matte; no people; no fabricated counts.
 - _CTA :_ github.com/Helvesec/rmux · good first issues

### J69, mardi 8 sept.
- **09:00 · linkedin** · Terminal Testing · 🤖 AUTO
 - A quiet superpower for teams shipping CLIs and TUIs: make your terminal tests read like Playwright.
 
 Instead of spawning a process, sleeping, and diffing brittle strings, you wait_for_text() to synchronize on real output, then assert on a typed PaneSnapshot { cols, rows, content }. Deterministic. No flakes. Runs in CI on Linux, macOS, and Windows.
 
 One of the five demos we ship is exactly a Playwright-style harness driving and asserting on a TUI. If your team owns terminal software and your integration tests are flaky, this is a concrete fix you can adopt this sprint.
 
 cargo add rmux-sdk → github.com/Helvesec/rmux
 - _Visuel :_ {SYSTEM} Subject: a CI report panel, a column of terminal assertions (wait_for_text, snapshot.content.contains) each resolving to a mint PASS, a small 'Linux · macOS · Windows' row beneath. Format: 4:5 for LinkedIn. Action: a clean deterministic run completing across platforms. Lighting: even, mint only on the PASS states. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, mint #36E2C4 on PASS only; monospace; deterministic clean layout; matte; no people.
 - _CTA :_ cargo add rmux-sdk

### J70, mercredi 9 sept.
- **13:00 · twitter** · Programmable Terminal · 🤖 AUTO
 - The ratatui-rmux widget is the most underrated part of rmux.
 
 ```bash
 cargo add ratatui-rmux
 ```
 
 Drop a real, live terminal pane into your own TUI, driven by the same daemon as the CLI and SDK. Not an emulator mock. An actual pane. Dashboards, agent monitors, dev tools: build them.
 - _Visuel :_ {SYSTEM} Subject: a custom Ratatui dashboard TUI with one embedded live terminal pane glowing mint at its center, surrounded by grey widget panels (a sidebar, a status row). Format: 4:5. Action: a real pane living inside a bespoke TUI. Lighting: even, one mint focal light on the embedded pane. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent on the embedded pane; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ cargo add ratatui-rmux
- **17:00 · twitter** · tmux, Upgraded · 🤖 AUTO
 - "But I have a huge .tmux.conf."
 
 Good. rmux migrates it automatically on first run. Prefix key, binds, status line, options, carried over. Opt out with one env var if you'd rather start clean.
 
 The config is the biggest reason people don't try new multiplexers. rmux removes it.
 - _Visuel :_ {SYSTEM} Subject: a large .tmux.conf file rendered on the left, an arrow into an identical rmux status line + panes on the right, a small 'auto-migrated' tag. Format: 1:1. Action: a big config carried across with zero effort. Lighting: flat. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent on the rmux side; monospaced 'rmux' wordmark; migration grid; matte; no people.
 - _CTA :_ cargo install rmux --locked

### J71, jeudi 10 sept.
- **10:00 · twitter** · Testimonial · 🙋 MANUEL · ~8 min
 - From an early user (shared with permission):
 
 "<PASTE REAL QUOTE, e.g. 'Replaced ~200 lines of capture-pane parsing with three snapshot() calls. It just works.'>"
 
 This is exactly why rmux exists, kill the stdout-scraping, get typed state. If it's saved you code too, I'd love to hear it. cargo add rmux-sdk
 - _Visuel :_ {SYSTEM} Subject: a clean quote card, a short user testimonial in ink text on canvas-black, a small mint quote-mark glyph as the only accent, grey attribution line beneath. Format: 1:1. Action: a credible, understated dev endorsement. Lighting: flat, one mint focal light on the quote mark. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent on the quote mark; monospaced 'rmux' wordmark; matte; no people; no stock imagery.
 - _À faire :_ BEFORE publishing: replace <PASTE REAL QUOTE> with a genuine, permission-granted quote from an issue/DM/Reddit reply captured on day 63/68. Do NOT invent a testimonial (R-CITE). If no real quote exists yet, swap this post for a build-in-public tip instead.
 - _CTA :_ cargo add rmux-sdk

### J72, vendredi 11 sept.
- **09:00 · linkedin** · Engineering · 🤖 AUTO
 - Something I want more infra teams to demand from their tools: a supply-chain posture you can verify, not just a promise.
 
 rmux ships with #![forbid(unsafe_code)] in the upper crates, the OS/PTY boundary isolated, and no network at runtime, enforced by a build guardrail, not a README sentence. Builds are locked; the license is dual MIT/Apache.
 
 For something you leave running for days with long-lived agents inside it, that posture is the point. "It won't phone home" should be a compile-time fact, not a vibe.
 
 If that matters to your stack, rmux is worth an evaluation. github.com/Helvesec/rmux
 - _Visuel :_ {SYSTEM} Subject: a safety-posture card, three clean status glyphs on canvas-black: 'forbid(unsafe_code)', 'no network at runtime', 'locked builds', rendered as verified check rows. Format: 4:5 for LinkedIn. Action: quiet infra-grade authority. Lighting: schematic, flat, one mint rim-light on the forbid-unsafe glyph. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ github.com/Helvesec/rmux

### J73, samedi 12 sept.
- **12:00 · twitter** · Agentic Terminal · 🤖 AUTO
 - Reasons people install rmux this week (real ones):
 
 • An agent run over SSH they can't afford to lose
 • A CLI test suite that's flaky in CI
 • Wanting native tmux on Windows without WSL
 • Needing to drive a terminal from Rust, typed
 
 Pick your reason. cargo install rmux --locked.
 - _Visuel :_ {SYSTEM} Subject: a four-cell grid, each cell a small monospaced use-case label with a tiny terminal glyph, one cell (the agentic one) highlighted mint. Format: 16:9. Action: 'pick your reason', a menu of entry points. Lighting: flat, one mint focal light on the highlighted cell. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; clean grid; matte; no people.
 - _CTA :_ cargo install rmux --locked

### J74, dimanche 13 sept.
- **11:00 · reddit** · Programmable Terminal · 🙋 MANUEL · ~35 min
 - Title: Driving and inspecting long-lived terminal jobs from Rust, typed snapshots instead of scraping capture-pane
 
 A lot of automation over SSH still comes down to: spawn a process, read stdout, regex it, hope the format never changes. I got tired of that for long-lived jobs (builds, TUIs, agents), so rmux exposes the terminal as a typed API.
 
 The loop with rmux-sdk:
 - connect_or_start() -> client (daemon lifecycle handled)
 - client.ensure_session(name) -> pane (idempotent attach-or-create; survives your process restarting)
 - pane.send_text(...), pane.wait_for_text("ready") to synchronize
 - pane.snapshot() -> PaneSnapshot { cols, rows, content } to inspect deterministically
 
 Because the daemon owns the sessions, a dropped SSH connection or a crashed orchestrator doesn't lose state, you reattach and keep going. Same session is reachable from the CLI, the SDK, and a Ratatui widget.
 
 Curious how others here handle inspecting remote terminal state today, control mode? expect? something custom? And what would you want from a typed version. Repo (v0.3.x preview, MIT/Apache): github.com/Helvesec/rmux
 - _À faire :_ Post to r/devops (or r/commandline) as a value-first technical discussion, lead with the problem and the SDK loop, ask a real question, link last. Have a runnable snippet ready to paste. Answer every substantive comment for the first 2 hours; capture any positive quotes (with permission) as testimonial assets. Never announcement-spam.
 - _CTA :_ engage in comments; repo link at the end

### J75, lundi 14 sept.
- **09:00 · linkedin** · Build-in-Public · 🤖 AUTO
 - Halfway through the growth quarter, a note on how I'm thinking about "conversion" for an open-source dev tool.
 
 There's no checkout. So the funnel is: someone reads a post → runs cargo install rmux → keeps it → pulls rmux-sdk into a project → files an issue or opens a PR → tells one colleague. Each step is a real, honest signal, and none of them is a vanity metric.
 
 What I optimize for isn't installs I can't verify, it's the moment someone says "I replaced my scraping code with snapshot() and I'm not going back." That's the only conversion that compounds.
 
 If you've hit that moment with rmux, reply, I read every one. And if you haven't tried it: cargo install rmux --locked → github.com/Helvesec/rmux
 - _Visuel :_ {SYSTEM} Subject: an honest OSS 'funnel' as a horizontal box-and-connector flow, read → install → keep → adopt SDK → contribute, thin #1E2733 connectors, the 'adopt SDK' node highlighted mint. Format: 16:9. Action: a calm, non-salesy adoption path. Lighting: schematic, flat, one mint focal light on the adopt node. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; matte; no people; no fabricated numbers.
 - _CTA :_ cargo install rmux --locked
- **15:00 · twitter** · Programmable Terminal · 🤖 AUTO
 - The whole rmux-sdk API in one breath:
 
 connect_or_start() → ensure_session() → send_text() → wait_for_text() → snapshot()
 
 Daemon lifecycle, idempotent sessions, typed input, synchronization, structured state. Five calls, and your terminal has an API. cargo add rmux-sdk.
 - _Visuel :_ {SYSTEM} Subject: a five-step horizontal API flow, connect_or_start · ensure_session · send_text · wait_for_text · snapshot, each a monospaced node linked by thin connectors, the final snapshot node resolving mint. Format: 16:9. Action: a clean pipeline reading left to right. Lighting: even, one mint focal light on snapshot. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ cargo add rmux-sdk

### J76, mardi 15 sept.
- **12:00 · twitter** · Terminal Testing · 🤖 AUTO
 - Flaky terminal tests cost you more than red CI, they cost you trust in the suite.
 
 rmux fixes the root cause: no more sleep-then-diff. wait_for_text() blocks on real output; snapshot() gives typed content to assert on. The test passes because the thing is ready, not because you waited long enough.
 
 Deterministic by construction. cargo add rmux-sdk.
 - _Visuel :_ {SYSTEM} Subject: a CI panel showing a previously-flaky test row turning from an amber 'flaky' state to a stable mint PASS via a wait_for_text arrow. Format: 4:5. Action: instability resolving into determinism. Lighting: even; amber only on the 'flaky' status glyph, mint on the resolved PASS. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, mint #36E2C4 on PASS, functional amber #F0883E on the flaky glyph only; monospace; matte; no people.
 - _CTA :_ cargo add rmux-sdk

### J77, mercredi 16 sept.
- **10:00 · twitter** · tmux, Upgraded · 🤖 AUTO
 - Windows devs: you've never had native tmux. rmux gives it to you.
 
 All 90 commands, real ConPTY + per-user Named Pipes, no WSL shim, no cygwin gymnastics. Same session model as your Linux box, same keybindings, same config.
 
 If you've been stuck alt-tabbing between a WSL tmux and native Windows, try this. cargo install rmux.
 - _Visuel :_ {SYSTEM} Subject: one rmux session shown running natively with a 'Windows · ConPTY' badge, stacked above matching Linux and macOS rows, one session, three platforms, parity. Format: 16:9. Action: 'finally native, everywhere'. Lighting: flat. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent on the Windows badge; monospaced 'rmux' wordmark; platform grid; matte; no people.
 - _CTA :_ cargo install rmux --locked

### J78, jeudi 17 sept.
- **11:00 · youtube** · Programmable Terminal · 🙋 MANUEL · ~75 min
 - rmux, driving a terminal from Rust in 90 seconds (SDK walkthrough)
 
 A real, un-cut screencast: cargo add rmux-sdk, then connect_or_start → ensure_session → send_text → wait_for_text → snapshot(), with the typed PaneSnapshot printing structured content live. No stdout scraping, no edits.
 
 rmux is a Rust, tmux-compatible multiplexer with a daemon-backed typed SDK. cargo install rmux → github.com/Helvesec/rmux
 - _À faire :_ RECORD a 90s asciinema/screen capture of the rmux-sdk quickstart end to end (cargo add → the five SDK calls → snapshot printing typed fields). Keep the brand terminal theme (canvas #0B0E14, mint cursor). This is a HERO asset, repurpose per content-strategy: the X post on day 80, a LinkedIn embed, a blog cover, a README GIF refresh.
 - _CTA :_ cargo install rmux · watch the SDK walkthrough

### J79, vendredi 18 sept.
- **13:00 · twitter** · Community · 🤖 AUTO
 - If you use rmux and hit a rough edge, don't work around it silently. File the issue.
 
 This is a preview, and every reported bug on copy-mode, Windows, or the SDK makes the 1.0 better for the next person. The repo is small enough that your report actually gets read and fixed.
 
 github.com/Helvesec/rmux/issues
 - _Visuel :_ {SYSTEM} Subject: a clean issue-tracker card, a monospaced issue title with a small mint 'triaged' label, a subtle arrow to a 'fixed' state. Format: 1:1. Action: a report turning into a fix; a healthy feedback loop. Lighting: flat, one mint focal light on the label. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; matte; no people; no fabricated issue counts.
 - _CTA :_ github.com/Helvesec/rmux/issues

### J80, samedi 19 sept.
- **09:00 · linkedin** · Programmable Terminal · 🤖 AUTO
 - New: a 90-second screencast of driving a terminal from Rust with rmux, no stdout scraping, no edits.
 
 cargo add rmux-sdk, then connect_or_start → ensure_session → send_text → wait_for_text → snapshot(). Watch the typed PaneSnapshot print structured content live.
 
 If you've ever wanted your terminal to have a real API, this is the clearest 90 seconds I can give you. 👇
 
 github.com/Helvesec/rmux
 - _À faire :_ No Higgsfield generation, embed the day-78 SDK screencast; the recording IS the visual. Add the repo link and a one-line caption.
 - _CTA :_ Watch the walkthrough · cargo add rmux-sdk
- **15:00 · twitter** · Programmable Terminal · 🤖 AUTO
 - 90 seconds, zero cuts: driving a terminal from Rust with rmux-sdk.
 
 connect_or_start → ensure_session → send_text → wait_for_text → snapshot(). The typed PaneSnapshot prints live. No capture-pane scraping anywhere. 👇
 
 [screencast link]
 - _À faire :_ No Higgsfield generation, attach the day-78 SDK screencast as the video/GIF; the recording IS the visual.
 - _CTA :_ Watch; cargo add rmux-sdk

### J81, dimanche 20 sept.
- **12:00 · twitter** · Agentic Terminal · 🤖 AUTO
 - "tmux for agents" isn't a tagline, it's a spec:
 
 • survive SSH drops (detach/reattach) ✅
 • inspect from code, not by squinting (snapshot) ✅
 • synchronize on real output (wait_for_text) ✅
 • orchestrate N from one daemon (ensure_session) ✅
 
 All shipping in the preview. cargo install rmux.
 - _Visuel :_ {SYSTEM} Subject: a spec checklist card, four monospaced capability lines each with a mint check, on canvas-black, 'tmux for agents' as a quiet caption. Format: 4:5. Action: a spec being satisfied, line by line. Lighting: flat, mint only on the check marks. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, mint #36E2C4 on the checks only; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ cargo install rmux

### J82, lundi 21 sept.
- **10:00 · twitter** · Testimonial · 🤖 AUTO
 - A merged PR I want to shout out:
 
 <PASTE REAL PR / CONTRIBUTOR, e.g. '@user fixed a copy-mode edge case on Windows this week.'>
 
 This is how a preview becomes solid, real users fixing real edges. If you want to be in the next one of these, the good-first-issues are open. github.com/Helvesec/rmux
 - _Visuel :_ {SYSTEM} Subject: a merged-PR card, a monospaced PR title line with a mint 'merged' glyph, quiet grey diff chrome beneath. Format: 1:1. Action: a community contribution landing. Lighting: flat, one mint focal light on the merged glyph. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; matte; no people; no fabricated contributor counts.
 - _À faire :_ BEFORE publishing: replace <PASTE REAL PR / CONTRIBUTOR> with an actual merged PR / contributor handle (get consent to name them). If none exists yet, swap for a build-in-public engineering tip. Never fabricate a contribution (R-CITE).
 - _CTA :_ github.com/Helvesec/rmux · good first issues

### J83, mardi 22 sept.
- **09:00 · linkedin** · Programmable Terminal · 🤖 AUTO
 - A concrete adoption path if your team is evaluating rmux:
 
 1) One engineer runs cargo install rmux --locked and uses it as their daily driver for a week (the .tmux.conf migrates, so nothing breaks).
 2) Pick one flaky terminal integration test. Rewrite it with wait_for_text() + snapshot(). Watch it stop flaking.
 3) If you run agents or long jobs over SSH, wrap one orchestration script with rmux-sdk instead of capture-pane parsing.
 
 By the end of the week you'll know if it belongs in your stack, on evidence, not a pitch. It's a v0.3.x preview, dual MIT/Apache, no network at runtime.
 
 cargo install rmux --locked → github.com/Helvesec/rmux
 - _Visuel :_ {SYSTEM} Subject: a three-step evaluation checklist as a vertical numbered flow, 'daily driver' · 'rewrite one flaky test' · 'wrap one script', the final step highlighted mint. Format: 4:5 for LinkedIn. Action: a pragmatic, evidence-first trial plan. Lighting: schematic, flat, one mint focal light on step 3. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ cargo install rmux --locked

### J84, mercredi 23 sept.
- **12:00 · twitter** · tmux, Upgraded · 🤖 AUTO
 - The honest pitch, one line:
 
 rmux is the tmux you already know, minus the parts C and 2007 forced on it, plus the SDK, the snapshots, and the Windows support you always wanted.
 
 No relearning. cargo install rmux --locked.
 - _Visuel :_ {SYSTEM} Subject: a minimal equation-style card, 'tmux you know' + 'SDK + snapshots + Windows' rendered as monospaced terms, the added-value term highlighted mint. Format: 1:1. Action: a clean, honest value equation. Lighting: flat, one mint focal light on the plus term. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ cargo install rmux --locked
- **17:00 · twitter** · Agentic Terminal · 🤖 AUTO
 - You don't need a swarm to benefit from rmux.
 
 Even ONE long agent run over SSH, detached, reattachable, and readable via snapshot() from code, that's the whole value. Start there. Add panes later.
 
 cargo install rmux, ensure_session("agent"), and never lose a run to a dropped connection again.
 - _Visuel :_ {SYSTEM} Subject: a single terminal pane running one long agent, an SSH link dropping and reconnecting glyph, the mint cursor still blinking, one session, resilient. Format: 16:9. Action: 'start with one', quiet resilience. Lighting: flat UI glow, one mint focal light on the surviving pane. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; real plausible agent log lines; matte; no people.
 - _CTA :_ cargo install rmux

### J85, jeudi 24 sept.
- **11:00 · reddit** · Community · 🙋 MANUEL · ~35 min
 - Title: cargo add ratatui-rmux, embed a live terminal pane in your Ratatui app (feedback wanted)
 
 If you build TUIs with Ratatui, ratatui-rmux lets you drop a real, live terminal pane into your app, driven by the same rmux daemon that backs the CLI and the SDK. Not a mock emulator: an actual pane you can send input to and snapshot.
 
 Use cases people have tried: an agent-monitor dashboard, a dev-tool with an embedded shell, a test runner that shows the live TUI under test. One daemon, three surfaces (CLI / SDK / widget), so the pane you embed is the same session you can also drive from the CLI.
 
 I'd love feedback from this community specifically, the widget API, layout/resize behavior, and what you'd want to embed. It's a v0.3.x preview (MIT/Apache); rough edges get fixed fast. cargo add ratatui-rmux · github.com/Helvesec/rmux
 - _À faire :_ Post to the Ratatui Discord (and/or r/rust) as a value-first 'try it + feedback' thread. Have a tiny working example repo/gist ready to link. Engage with every reply on the widget API and resize behavior; capture positive feedback (with permission) as testimonial assets. Never announcement-spam.
 - _CTA :_ cargo add ratatui-rmux; feedback in-thread

### J86, vendredi 25 sept.
- **09:00 · linkedin** · Terminal Testing · 🤖 AUTO
 - If I could get one practice adopted across teams that ship terminal software, it'd be this: test your CLI/TUI the way you test a web app.
 
 You wouldn't sleep(2) and diff raw HTML in a Playwright test. You'd wait for an element, then assert on structured state. Terminals deserve the same. With rmux: wait_for_text() to synchronize, snapshot() for typed { cols, rows, content }, assert.
 
 We ship a full Playwright-style testing demo (~1,495 LOC) that does exactly this against a real TUI. Deterministic, cross-platform, CI-ready.
 
 If your terminal tests are the flaky ones in your suite, this is the fix. cargo add rmux-sdk → github.com/Helvesec/rmux
 - _Visuel :_ {SYSTEM} Subject: a split card, left a familiar web-test assertion (greyed), right an equivalent terminal assertion with rmux (wait_for_text + snapshot) resolving mint; a caption 'test terminals like the web'. Format: 4:5 for LinkedIn. Action: the analogy landing, terminal test turning deterministic. Lighting: even, mint on the terminal side only. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, mint #36E2C4 on the terminal PASS only; monospace; matte; no people.
 - _CTA :_ cargo add rmux-sdk

### J87, samedi 26 sept.
- **12:00 · twitter** · Programmable Terminal · 🤖 AUTO
 - One daemon. Three surfaces. Zero second-class citizens.
 
 The CLI you type, the rmux-sdk your code calls, the ratatui-rmux widget you embed, all the same protocol, all driving the same sessions. There's no "scripting mode that lags behind the real thing."
 
 That symmetry is the product. cargo install rmux.
 - _Visuel :_ {SYSTEM} Subject: the three-surfaces → one-daemon diagram, CLI / rmux-sdk / ratatui-rmux boxes linked by thin #1E2733 connectors to a central daemon node, all three equal in weight. Format: 16:9. Action: clean symmetry, engineered equality. Lighting: schematic, flat, one mint focal light on the daemon node. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent on the daemon; monospaced 'rmux' wordmark; matte; no people.
 - _CTA :_ cargo install rmux

### J88, dimanche 27 sept.
- **10:00 · twitter** · Community · 🤖 AUTO
 - Quiet momentum check-in:
 
 rmux went from an idea to a tool people run daily, test with, and file PRs against. No paid tier, no growth hacks, just cargo install and word of mouth.
 
 If it's earned a spot in your terminal, the best thanks is a star and one honest issue. github.com/Helvesec/rmux ⭐
 - _Visuel :_ {SYSTEM} Subject: a restrained momentum card, a single mint star glyph beside 'Helvesec/rmux' and a quiet monospaced caption 'idea → daily driver', grey chrome. Format: 1:1. Action: sincere, low-key gratitude. Lighting: flat, one mint focal light on the star. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent on the star; monospaced 'rmux' wordmark; matte; no people; no fabricated metrics.
 - _CTA :_ Star github.com/Helvesec/rmux

### J89, lundi 28 sept.
- **12:00 · twitter** · tmux, Upgraded · 🤖 AUTO
 - Still on tmux and curious but haven't switched? Here's the lowest-risk experiment:
 
 cargo install rmux --locked, use it for one day. Your config migrates, your keys work. If you don't feel the difference, `tmux` is still right there.
 
 But you'll feel it the first time you wait_for_text() something tmux would make you scrape.
 - _Visuel :_ {SYSTEM} Subject: a 'one-day trial' card, a .tmux.conf migrating into an rmux session with a small 'reversible · your tmux is still there' tag, one wait_for_text line highlighted mint. Format: 16:9. Action: a safe, reversible experiment. Lighting: flat, one mint focal light on the wait_for_text line. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; migration grid; matte; no people.
 - _CTA :_ cargo install rmux --locked

### J90, mardi 29 sept.
- **09:00 · linkedin** · Build-in-Public · 🤖 AUTO
 - 90 days of building rmux in public. A short recap, and where it goes next.
 
 The thesis held up: terminals built for humans aren't enough once agents live in them. rmux answers that with one daemon and three equal surfaces, the tmux-compatible CLI you already know, a typed rmux-sdk, and a ratatui-rmux widget, so a session you start by hand can be detached, reattached, inspected via snapshot(), and orchestrated from code, on Linux, macOS, and Windows.
 
 What these 90 days proved to me: the people who feel it hardest are the ones running long agents over SSH and the ones fighting flaky terminal tests. Every one of them stopped scraping capture-pane and started calling an API. That's the conversion that matters for an open-source tool, not a number, a habit change.
 
 What's next: harden the preview toward a solid 1.0 on your issues, deepen the SDK, and keep the compatibility promise absolute. It stays free, dual MIT/Apache, no network at runtime.
 
 If you've been meaning to try it, today's a good day: cargo install rmux --locked. And if you already run it, thank you, and please file the rough edges. github.com/Helvesec/rmux
 - _Visuel :_ {SYSTEM} Subject: a calm '90 days' recap card, the three-surfaces → one-daemon diagram small in the frame with a monospaced caption 'the thesis held: terminals need an API', a single mint cursor. Format: 4:5 for LinkedIn. Action: a confident, understated milestone and forward look. Lighting: schematic, flat, one mint focal light on the daemon node. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; matte; no people; no fabricated numbers.
 - _CTA :_ cargo install rmux --locked · Star Helvesec/rmux
- **14:00 · twitter** · Agentic Terminal · 🤖 AUTO
 - 90 days of rmux, one line: your terminal now has an API, and your agents survive the SSH drop.
 
 tmux-compatible CLI + typed SDK + Ratatui widget, one daemon, all three platforms. Free, MIT/Apache, preview honest.
 
 If you build with terminals, this is the day to install it. cargo install rmux --locked 👇
 - _Visuel :_ {SYSTEM} Subject: a flagship multiplex grid of four agent panes with a mint cursor, quiet monospaced caption 'your terminal has an API', clean and confident. Format: 16:9. Action: a closing hero shot, controlled power. Lighting: flat UI glow, single mint focal light on the active pane. Palette canvas #0B0E14 / panel #11161F / ink #E6EDF3 / muted #7D8590, ONE mint #36E2C4 accent; monospaced 'rmux' wordmark; multiplex-grid; real plausible agent log lines; matte; no people.
 - _CTA :_ cargo install rmux --locked
