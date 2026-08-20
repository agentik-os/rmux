---
project: rmux
layer: copy
produced_by: cold-email (/omg-cold-email)
status: filled
---
# Cold Email / Outbound, rmux

> rmux is free OSS, "outbound" here is **developer relations / ecosystem outreach**, not sales prospecting.
> No selling, no pitch deck: offer value (a tool that solves their stated problem), make it trivial to try,
> ask nothing but a look. Keep it short, technical, and honest about preview status. Respect anti-spam norms.

## ICP segments (outreach targets)
1. **Agent-framework / agent-infra maintainers**, projects that orchestrate terminal agents (the integration wedge). Goal: an `rmux-sdk` integration or a mention.
2. **Ratatui ecosystem maintainers & TUI app authors**, candidates for `ratatui-rmux`. Goal: try the widget, give feedback.
3. **Rust / dev-tooling newsletter & content creators** (This Week in Rust curators, dev YouTubers, blog authors), Goal: coverage of a launch.
4. **Terminal-testing / CLI-testing tool authors**, Goal: explore snapshot/`wait_for_text` as a testing primitive.

---

## Sequence A, Agent-framework maintainer (3 touches)
**T1, Subject: keeping your agents alive over SSH**
> Hi {first}, I maintain rmux, a Rust, tmux-compatible multiplexer with a daemon-backed typed SDK. I noticed {project} runs agents in terminals over SSH.
>
> rmux lets you detach those sessions, reattach later, and read what the agent did via a typed `snapshot()` / `wait_for_text()`, no stdout scraping. It might remove some of the glue code in {specific_file/area}.
>
> No ask, just thought it might fit. Repo: github.com/Helvesec/rmux. Happy to answer anything.

**T2 (+5 days), Subject: re: keeping your agents alive over SSH**
> Quick follow-up, here's a 30-line example driving an agent session from `rmux-sdk` end to end: {gist link}. If detaching/inspecting is something {project} users hit, I'm glad to help wire a proof-of-concept.

**T3 (+7 days), Subject: last one, rmux for {project}**
> I'll stop here so I'm not noise. If a tmux-compatible multiplexer with a real SDK is ever useful for {project}, the door's open, and feedback (even "not for us") would genuinely help the project. Thanks for building {project}.

## Sequence B, Ratatui / TUI app author (3 touches)
**T1, Subject: a live terminal pane for {project}**
> Hi {first}, love {project}. I built `ratatui-rmux`: a Ratatui widget that renders a *real*, live terminal pane (backed by the rmux daemon + typed SDK). `cargo add ratatui-rmux` and you get a working terminal inside your TUI.
>
> If embedding a terminal is ever on your roadmap, it might save you the PTY/IPC plumbing. Repo: github.com/Helvesec/rmux, feedback very welcome (it's a fresh preview).

**T2 (+6 days), Subject: re: a live terminal pane**
> Here's the minimal render path: `PaneState::from_snapshot(snapshot)` → `PaneWidget::new(&state).render(...)`. ~6 lines. Happy to pair on an integration if useful.

**T3 (+8 days), Subject: closing the loop**
> Not going to keep pinging, if `ratatui-rmux` is ever handy for {project}, grab it anytime, and I'd love to hear what breaks. Thanks for the great work on {project}.

## Sequence C, Rust newsletter / content creator (2 touches)
**T1, Subject: rmux, tmux for agents (might fit {newsletter})**
> Hi {first}, I just shipped a public preview of rmux: a Rust, tmux-compatible multiplexer (all 90 commands) with a typed, daemon-backed SDK and a Ratatui widget. The angle that's resonating with Rust folks is "tmux for agents", drive and inspect terminals from code. If it's a fit for {newsletter}, the repo + a short demo are here: {links}. No pressure either way.

**T2 (+7 days), Subject: re: rmux**
> Following up once, here's a 60-second asciinema of the multi-agent orchestration demo: {link}. Thanks for considering it, and either way, keep up {newsletter}.

## Subject lines (swipe bank)
- keeping your agents alive over SSH
- your terminal, now with an SDK
- a live terminal pane for {project}
- tmux for agents, might fit {newsletter}
- typed snapshots instead of `capture-pane` scraping
- {project} + rmux, a quick idea
- all 90 tmux commands, plus an API
