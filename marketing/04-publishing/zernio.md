---
project: rmux
layer: publishing
tool: omega-zernio (tools/zernio/cli.ts) → https://zernio.com/api/v1
profile_slug: rmux
status: filled
---
# Publishing, rmux (zernio)

> One zernio profile per project. Accounts connect via OAuth (operator step). **NO auto-connect /
> auto-publish in setup**, this file only documents the target platforms, connect commands, and cadence.
> HN and GitHub are **manual-only** (never automate Show HN, comments, or Discussions, anti-spam + ToS).

## Target platforms (developer-fit, in priority order)
**Primary (automate via zernio):**
- **twitter (X)**, core dev/Rust/agent-infra audience; demo GIFs, threads, build-in-public. P0.
- **linkedin**, dev-leadership + infra framing; 1-2×/week. P1.
- **reddit**, r/rust, r/commandline, r/devops; value-first, ≤1×/week, hand-reviewed. P1 (use carefully, subreddit rules).
- **youtube**, asciinema/screencast demos; 2×/month. P2.

**Manual-only (do NOT route through zernio):**
- **Hacker News**, Show HN + comments, always by hand.
- **GitHub** (Discussions, README, Releases), native, not a social post.
- **This Week in Rust / lobste.rs**, manual submission.
- **Ratatui / Rust Discords**, manual, community-native.

**Out of scope (low ICP fit):** facebook · instagram (occasional reel only) · tiktok (dev-creator partnership only) · threads · pinterest · snapchat · whatsapp · googlebusiness · bluesky (optional, dev presence growing).

## Connect commands (run by operator, opens hosted authUrl)
```bash
omega-zernio connect rmux twitter
omega-zernio connect rmux linkedin
omega-zernio connect rmux reddit
omega-zernio connect rmux youtube
omega-zernio accounts rmux    # verify isActive before any post
```

## Publish (after content validated, dry-run FIRST)
```bash
# Always dry-run before a real post:
omega-zernio post rmux --text "…" --platforms twitter --media ./rmux-<slot>-16x9.png --dry-run

# Schedule (operator sets the real datetime; calendar.json scheduledFor stays null until then):
omega-zernio post rmux --text "…" --platforms twitter,linkedin --schedule 2026-07-XXT09:00:00Z

# Bulk from the calendar once content + media are validated:
omega-zernio bulk-upload rmux --file ./calendar.json --dry-run
```

## Cadence (from content-strategy.md)
| Platform | Cadence | Pillars | Notes |
| :--- | :--- | :--- | :--- |
| X (twitter) | 3-4×/week | Agentic, Programmable, Build-in-Public | demo GIF/asciinema, threads |
| LinkedIn | 1-2×/week | Build-in-Public, Programmable | infra/leadership framing |
| Reddit | ≤1×/week | tmux-Upgraded, Programmable, Agentic | value-first, per-subreddit rules, hand-reviewed |
| YouTube | 2×/month | Agentic, Terminal Testing | screencast demos |
| HN (manual) | per launch moment | Agentic (flagship) | never automated |

## Guardrails
- Validate copy (02-copy/*) + visuals (03-visual-identity/shotlist.md) → `--dry-run` → operator approves → schedule.
- Honest-scope: rmux.io / crates.io / GitHub releases are **upstream `Helvesec/rmux`**; social posts market the product and link the repo, but don't claim ownership of upstream channels.
- Never auto-post to HN/Reddit comment threads or GitHub. Respect each platform's self-promotion ratio.
