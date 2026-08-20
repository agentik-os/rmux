---
project: rmux
layer: strategy
produced_by: launch-strategy (/omg-launch-strategy)
status: filled
---
# Launch Strategy, rmux

> rmux is at **v0.3.1 public preview** ("bugs expected", `README.md`). The right "launch" is a
> **technical credibility launch to developers**, not a consumer/Product-Hunt splash. Honest-scope:
> the public rmux.io/crates.io distribution is upstream `Helvesec/rmux`; coordinate any launch that
> points at those channels with upstream, or launch around the fork's own build-from-source story.

## Launch moment(s) & timeline
- **M1, "Show HN" technical launch** *(primary)*, tied to a stable, well-tested release. Headline: *"Show HN: rmux, tmux for agents (Rust multiplexer with a typed SDK)."* Target: HN front page + r/rust + This Week in Rust same week.
- **M2, `ratatui-rmux` ecosystem launch**, showcase post in the Ratatui community + r/rust: "embed a live terminal pane in your TUI." Wedge into the Rust-TUI audience.
- **M3, Terminal Testing launch**, "Playwright for your terminal" deep-dive, riding the shipped Playwright-testing demo, aimed at QA/testing communities.
- **Cadence:** roughly one launch moment per ~30 days; never launch on a release with known regressions (L1: runtime is the only truth, run the verification suite first).

## Channels (Product Hunt / waitlist / email / social)
- **Primary:** Hacker News (Show HN), Reddit (r/rust, r/commandline, r/devops), This Week in Rust, lobste.rs.
- **Amplify:** X build-in-public thread + demo GIF/asciinema; LinkedIn dev-leadership post; Ratatui Discord.
- **Owned:** GitHub Release notes + README hero; rmux.io (upstream, coordinate, don't assume control).
- **Product Hunt:** *optional/low-priority*, developer-infra tools underperform there vs HN; only if a polished landing + GIF is ready. No consumer waitlist (free OSS, instant install).
- **Email:** no list today; capture interest via GitHub Watch/Star + Discussions instead.

## Pre-launch → launch-day → post-launch sequence
**T-14 → T-2 (pre-launch):**
- Freeze a release candidate; run full verification (`cargo fmt/clippy/test`, `scripts/*`), green or no launch.
- Finalize README hook, comparison page (vs tmux/zellij), 2-3 demo GIFs/asciinema, SDK quickstart.
- Seed 6-8 good-first-issues + Discussions categories.
- Draft the Show HN post + first comment (author context, honest preview framing, "file issues"); pre-write X thread + Reddit posts.
- Line up 5-10 friendly devs to look (no vote-manipulation, HN-safe; just genuine early eyes).

**Launch day (T-0):**
- Post Show HN early in the US morning; author available all day to answer every comment fast and technically.
- Publish X build-in-public thread + pinned demo GIF; post to r/rust (value-first, not "please upvote").
- Submit to This Week in Rust; share in Ratatui Discord.
- Monitor issues in real time; triage and label; thank bug-filers.

**Post-launch (T+1 → T+30):**
- Ship a fast patch addressing top launch-day bugs (signals momentum + responsiveness).
- Publish a "what we learned / what's next" follow-up; convert active commenters into contributors.
- Repurpose the launch demo into the content calendar (see content-strategy.md repurposing engine).
- Set up the next launch moment (M2/M3).

## Assets checklist
- [ ] Green verification run (fmt/clippy/test + scripts) on the launch commit (L0/L1).
- [ ] README hero + tightened "tmux for agents" hook.
- [ ] Comparison page: rmux vs tmux vs zellij.
- [ ] 2-3 demo GIFs/asciinema (orchestration is the hero).
- [ ] SDK quickstart blog post + runnable example.
- [ ] Show HN post + author first-comment drafted.
- [ ] X thread + Reddit posts drafted (see 02-copy/social-content.md).
- [ ] Good-first-issues + Discussions live.
- [ ] GitHub Release notes for the launch version.
- [ ] Visual assets per 03-visual-identity/shotlist.md (OG/social cards).

## Success criteria
- **M1:** HN front page (top 10) for ≥2 hours; +X GitHub stars in launch week; ≥N `rmux-sdk` downloads added; ≥10 quality Discussions/issues; ≥2 external mentions/write-ups.
- **Health:** every launch-day issue triaged < 24h; a patch shipped within the week.
- **Compounding:** at least one external developer builds/blogs something on rmux within 30 days.
- (Set concrete N targets from the real pre-launch baseline; do not fabricate numbers, R-CITE.)
