---
project: rmux
layer: visual-identity/higgsfield
status: filled
---
# Shot List, rmux

> Each planned image/video generation, mapped to a content pillar + calendar.json slot. No soul-id
> (product/scene-led). Generate with system-prompt.md + the matching preprompt.md pillar preset.
> NO live generation performed in setup (R-VISUAL-ID), this is the production queue.

| # | Pillar | Platform | Format | Prompt ref (preprompt preset) | Soul? | Calendar slot |
|---|---|---|---|---|---|---|
| 1 | Agentic Terminal | X / OG | 16:9 | Pillar 1, 4-pane orchestration hero, mint cursor | no | `rmux-tmux-for-agents-thread` (OG card) |
| 2 | Agentic Terminal | Instagram / X | 1:1 | Pillar 1, single active pane, detach/reattach | no | `lost-agent-ssh-story` |
| 3 | Programmable Terminal | X / dev.to | 16:9 | Pillar 2, code panel + live pane, `snapshot()` | no | `sdk-quickstart-tutorial` |
| 4 | Programmable Terminal | LinkedIn | 4:5 | Pillar 2, `wait_for_text("ready")` resolving mint | no | `stop-scraping-capture-pane` |
| 5 | tmux, Upgraded | Reddit / X | 1:1 | Pillar 3, rmux vs tmux split, mint on rmux side | no | `rmux-vs-tmux-comparison` |
| 6 | tmux, Upgraded | X | 16:9 | Pillar 3, `.tmux.conf` migrating + Windows badge | no | `tmux-on-windows-finally` |
| 7 | Terminal Testing | dev.to / X | 16:9 | Pillar 4, assertion panel, mint "PASS" | no | `playwright-for-terminals` |
| 8 | Engineering | OG / LinkedIn | 16:9 | Pillar 5, three-surfaces → one-daemon diagram | no | `three-surfaces-one-daemon` |
| 9 | Engineering | X | 1:1 | Pillar 5, forbid-unsafe / no-network badge motif | no | `forbid-unsafe-no-network` |
| 10 | Agentic Terminal | YouTube/Shorts | 9:16 | Pillar 1, vertical orchestration screencap end-card | no | `orchestration-demo-short` |
| 11 | Programmable Terminal | YouTube | 16:9 | Pillar 2, SDK screencast thumbnail | no | `ratatui-rmux-widget-showcase` |

## Production notes
- **Priority:** #1 and #8 first (launch OG card + architecture explainer, highest reuse).
- **Real assets beat renders:** where possible, use actual high-DPI screencaps / asciinema frames of the
 shipped demos (orchestration, broadcast, mirroring, Playwright) instead of synthetic generation, they're
 more credible to the ICP and fully on-DA. Reserve Higgsfield for stylized OG/social cards and the diagram motif.
- **Aspect discipline:** never crop the `rmux` wordmark or the focal pane; one mint accent per frame.
- **Output naming:** `rmux-<calendar-slot>-<ratio>.png` → drop into `04-publishing/` and reference from calendar.json `media`.
