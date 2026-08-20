---
project: rmux
layer: strategy
produced_by: content-strategy (/omg-content-strategy)
status: filled
---
# Content Strategy, rmux

> Feeds 04-publishing/calendar.json. Audience = developers; voice = engineer-to-engineer (see
> product-marketing.md). Distribution skews to GitHub/HN/Reddit/X + Rust newsletters, not consumer social.

## Content pillars (3-5)
1. **Agentic Terminal** *(shareable, flagship)*, running, detaching, reattaching, orchestrating, and inspecting long-lived agents over SSH. The "tmux for agents" narrative. This is the wedge.
2. **Programmable Terminal** *(searchable + shareable)*, the SDK: drive sessions from Rust, typed snapshots, `wait_for_text`, embedding `ratatui-rmux`. Tutorials + recipes.
3. **tmux, Upgraded** *(searchable)*, compatibility, `.tmux.conf` migration, Windows support, Rust safety; conversion content for tmux veterans. Comparison/SEO.
4. **Terminal Testing** *(searchable)*, assert on terminals like a DOM; snapshot-based TUI/CLI testing (Playwright-for-terminal angle).
5. **Build-in-Public / Engineering** *(shareable)*, architecture (daemon + 3 surfaces), Rust craft, release notes, forbid-unsafe / no-network posture, honest preview-stage status.

## Topic clusters (searchable + shareable)
- **Agentic Terminal:** "keep an AI agent alive over SSH", "orchestrate N agents from one daemon", "inspect what an agent typed without attaching", "broadcast a command to many agent panes", multi-agent orchestration demo walkthrough.
- **Programmable Terminal:** "drive a terminal from Rust" (SDK quickstart), "`ensure_session` patterns", "typed `PaneSnapshot` vs scraping `capture-pane`", "embed a live terminal pane in Ratatui", "`connect_or_start` daemon lifecycle".
- **tmux, Upgraded:** "rmux vs tmux", "rmux vs zellij", "migrate your `.tmux.conf`", "tmux on Windows (finally)", "all 90 tmux commands in Rust".
- **Terminal Testing:** "Playwright-style testing for TUIs", "`wait_for_text` + snapshot assertions", "deterministic terminal tests in CI".
- **Engineering:** "why three surfaces share one daemon protocol", "forbid-unsafe in a terminal multiplexer", "ConPTY + Named Pipes on Windows", graphics passthrough (Kitty/SIXEL), release deep-dives.

## Channel mix & cadence
> zernio platforms used: **twitter (X)**, **linkedin**, **reddit**, **youtube** (asciinema/screencasts), plus GitHub-native (Discussions/README, not via zernio) and HN (manual, never automated).

| Channel | Cadence | Primary pillar(s) | Format |
| :--- | :--- | :--- | :--- |
| X / Twitter | 3-4×/week | Agentic, Programmable, Build-in-Public | demo GIF/asciinema, threads, tips |
| Reddit (r/rust, r/commandline, r/devops) | 1×/week (value-first) | tmux-Upgraded, Programmable, Agentic | text post + demo, AMA-style |
| LinkedIn | 1-2×/week | Build-in-Public, Programmable | dev-leadership framed posts |
| YouTube / asciinema | 2×/month | Agentic, Terminal Testing | screencast demo |
| Dev blog / dev.to | 1×/week | Programmable, tmux-Upgraded, Testing | long-form tutorial (SEO) |
| HN | per launch moment (manual) | Agentic (flagship) | Show HN / deep-dive |

## Editorial calendar (→ 04-publishing/calendar.json)
- ≥10 seeded post stubs live in `04-publishing/calendar.json`, each tagged with pillar + draftHook + platform, `scheduledFor: null` (operator sets dates; nothing auto-publishes).
- Repurpose the 5 shipped demos (orchestration, broadcast, mini-zellij, mirroring, Playwright) into the calendar as hero assets.

## Repurposing engine (1 hero → N derivatives)
**Hero:** the *Multi-Agent Orchestration* demo (asciinema screencast + blog post).
→ X thread (step-by-step) → 60s vertical screen-capture (YouTube Short / X video) → LinkedIn "why we built this" post → Reddit r/rust value post → dev.to tutorial → README demo GIF refresh → 3 standalone tips/quote-cards. One hero = ~8 derivatives per cycle.

## KPIs per pillar
- **Agentic Terminal:** HN points, X impressions/reposts, GitHub stars added during pushes.
- **Programmable Terminal:** `rmux-sdk` + `ratatui-rmux` crates.io downloads (north-star proxy), tutorial dwell time.
- **tmux, Upgraded:** organic search rank for "rmux vs tmux" / "tmux SDK"; installer runs.
- **Terminal Testing:** referral traffic from testing communities; demo repo clones.
- **Build-in-Public:** follower growth, contributors merged, Discussions opened.
