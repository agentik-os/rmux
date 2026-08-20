---
project: rmux
layer: strategy
produced_by: marketing-strategist (/omg-marketing-strategist)
status: filled
---
# GTM Strategy, rmux

> Reads from 00-context/product-marketing.md. rmux is **free, open-source** (MIT/Apache), this is a
> developer-adoption GTM, not a revenue funnel. Honest-scope note: public distribution (rmux.io,
> crates.io) is upstream `Helvesec/rmux`; this fork builds from source. Owned-channel goals below
> are framed around the rmux *product/ecosystem*, not claiming control of upstream channels.

## Strategic objective (12 mo) + North-star metric
- **Objective:** establish rmux as *the* programmable, agent-ready terminal multiplexer, the default answer to "how do I run and inspect a long-lived agent in a terminal."
- **North-star metric:** **Weekly active SDK adopters**, proxied by `rmux-sdk` + `ratatui-rmux` crates.io download trend (signals real integration, not just CLI-curiosity stars).
- **Supporting metrics:** GitHub stars trend, `cargo install` / installer runs, Discussions + issues opened (engagement), external blog posts / demos using rmux (ecosystem pull), contributors merged.

## Motion (PLG / sales-led / community / content)
- **Community-led + content-led, developer self-serve.** No sales motion, no paid tier.
- The product *is* the funnel: README → `cargo install` / `cargo add` → first session/SDK call → orchestration demo → contributor.
- **Wedge sequencing:** lead with the **agent-orchestration** narrative (least-served, highest-pain ICP), then convert tmux-veterans on compatibility, then pull in Rustaceans via the SDK + Ratatui widget.

## Channel strategy & priority
1. **GitHub (home base)**, README is the #1 conversion asset; topics, Discussions, good-first-issues, demo repos. (P0)
2. **Hacker News**, "Show HN" + technical deep-dives; single highest-leverage spike channel. (P0)
3. **Reddit**, r/rust, r/commandline, r/devops; value-first posts, not announcements. (P1)
4. **This Week in Rust + lobste.rs**, Rust-audience credibility and steady trickle. (P1)
5. **X/Twitter**, ongoing build-in-public, demo GIFs/asciinema, agent-infra + Rust circles. (P1)
6. **Long-form (dev.to / personal blog / docs)**, SEO for "tmux SDK", "drive terminal from Rust", "terminal testing", "tmux Windows alternative". (P2)
7. **YouTube / asciinema**, screencasts of the five demos. (P2)
8. **Ratatui community (Discord)**, direct placement for the `ratatui-rmux` wedge. (P1)

## Funnel architecture (TOFU/MOFU/BOFU)
- **TOFU (awareness):** HN/Reddit posts, demo GIFs, "tmux for agents" hook, This Week in Rust. → Goal: land on GitHub/README.
- **MOFU (consideration):** README "Why", architecture diagram, demos, SDK quickstart, comparison content (vs tmux/zellij), docs. → Goal: `cargo install` or `cargo add`.
- **BOFU (activation):** copy-paste CLI quickstart works in <60s; SDK quickstart runs; `.tmux.conf` migrates; first orchestration demo cloned. → Goal: a real session + a star + an issue/Discussion.
- **Loop (advocacy):** contributors, external blog posts, demos built on rmux, "Show HN: I built X on rmux".

## Budget posture & expected CAC/LTV
- **Near-zero paid budget.** This is OSS; spend is the maintainer's time + tokens, not ad dollars. Optional micro-experiments (≤ small X-promotion test) only after organic baseline is known. (R-BUDGET: no runaway spend.)
- CAC/LTV in dollar terms is **not the right frame**; the economic unit is *attention-hour → install → contributor*. Optimize for low time-to-first-success (TTFS < 60s) and high README→install rate.

## 90-day roadmap
- **Days 0-30, Foundation & proof.** Tighten README hook ("tmux for agents"); publish comparison page (vs tmux/zellij); record 2-3 demo asciinema/GIFs; seed 5-8 good-first-issues; set up Discussions; ship SDK quickstart blog post. Soft-share in Ratatui Discord + r/rust.
- **Days 31-60, Launch spike.** Coordinated **Show HN** + r/rust + This Week in Rust on a meaningful release; build-in-public thread on X; respond to every comment/issue within hours. Target: first big star/download spike + first external write-ups.
- **Days 61-90, Compound.** Convert spike traffic into retention: weekly "rmux recipe" content (SDK patterns, orchestration, terminal testing), onboard first external contributors, publish a deeper "how rmux orchestrates N agents" piece, line up a second launch moment (e.g. `ratatui-rmux` showcase or a new minor release).
