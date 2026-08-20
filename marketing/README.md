# Marketing Machine, rmux

> OmegaOS per-project marketing machine. SSOT for this project's strategy, copy,
> visual identity (DA + Higgsfield), and zernio publishing. Generated 2026-06-30.
> Governing rules: R-MARKETING · R-VISUAL-ID · R-CITE · L0/L1.

**zernio profile slug:** `rmux` · **publish CLI:** `omega-zernio post rmux …`

## Status board
_Filled 2026-06-30, grounded in README.md / README.fr.md / RULES.md / docs/ / spec/feature-inventory-v1.yaml._

| Layer | File | Status |
|---|---|---|
| Context | 00-context/product-marketing.md | ☑ filled |
| Context | 00-context/market-research.md | ☑ filled |
| Context | 00-context/competitors.md | ☑ filled |
| Context | 00-context/audience-personas.md | ☑ filled |
| Strategy | 01-strategy/gtm-strategy.md | ☑ filled |
| Strategy | 01-strategy/content-strategy.md | ☑ filled |
| Strategy | 01-strategy/launch-strategy.md | ☑ filled |
| Copy | 02-copy/copywriting.md | ☑ filled |
| Copy | 02-copy/ad-creative.md | ☑ filled |
| Copy | 02-copy/social-content.md | ☑ filled |
| Copy | 02-copy/cold-email.md | ☑ filled |
| Visual | 03-visual-identity/DA.md | ☑ filled (palette #0B0E14/#36E2C4 mint accent) |
| Visual | 03-visual-identity/higgsfield/system-prompt.md | ☑ filled (brand-locked, no soul-id) |
| Visual | 03-visual-identity/higgsfield/preprompt.md | ☑ filled (5 pillar presets) |
| Visual | 03-visual-identity/higgsfield/avatar-soul.md | ☑ filled (decision: NO avatar, product/scene-led) |
| Visual | 03-visual-identity/higgsfield/shotlist.md | ☑ filled (11 shots, no live gen) |
| Publish | 04-publishing/zernio.md | ☑ filled (X/LinkedIn/Reddit/YouTube; HN manual) |
| Publish | 04-publishing/calendar.json | ☑ filled (12 stubs, scheduledFor=null) |

**Identity highlights:** Category = programmable/agent-ready terminal multiplexer · One-liner = "tmux for the agentic era" · ICP = AI-agent infra engineers (primary) + Rustaceans + tmux refugees · Moat = typed daemon-backed SDK + inspectable snapshots + native Windows + tmux-CLI compatibility · Free OSS (MIT/Apache) → community/content-led GTM · Visual = dark terminal canvas + ONE mint accent (#36E2C4), no avatar.
**Honest-scope:** this is the `agentik-os/rmux` fork; rmux.io / crates.io / GitHub releases belong to upstream `Helvesec/rmux`, market the product, don't claim those channels.

## Pipeline (marketing machine dependency order)
1. product-marketing-context → 00-context/product-marketing.md (positioning, ICP, read by all)
2. market-research / ads-competitors → 00-context/{market-research,competitors,audience-personas}.md
3. marketing-strategist + content-strategy + launch-strategy → 01-strategy/*
4. mk-copywriting / ad-creative / social-content / cold-email → 02-copy/*
5. brand-identity + art-director → 03-visual-identity/DA.md, then higgsfield-soul-id + higgsfield-generate → higgsfield/*
6. content-strategy calendar → 04-publishing/calendar.json → `omega-zernio post rmux`

## Golden path to autonomous publishing
validate content → `omega-zernio connect rmux <platform>` (OAuth, operator) → dry-run → `omega-zernio post rmux …` / bulk-upload.
