---
project: rmux
layer: 06-branding
produced_by: marketing-machine-upgrade (spec §2.1.10 governance loop · §6.2 feedback loop)
status: empty (no assets published yet, SETUP only)
role: The win/lose log. Post-publish metrics land here against the rubric; winners are PROMOTED into templates/ + prompt-library/, a pattern that fails twice is RETIRED.
---
# Pattern Ledger, rmux

> **Promote / retire rule (spec §6.2):** after publication, metrics return here against the **rubric**.
> A **winning pattern is PROMOTED** into `templates/` + `prompt-library/` (it becomes a reusable
> template) and triggers a **follow-up post within 48h**. A pattern that **fails twice is RETIRED**
> (do not regenerate it). The one-page brief (§2.1.10) cites the **2-3 top performers** from this ledger.

## Rubric (decision thresholds, spec §5.1 · §6.2)
| Metric | "Win" threshold | Notes |
|---|---|---|
| Hook rate | ≥ 75% (viewed-vs-swiped) | >30% drop in first 3s = failed hook → re-shoot the open |
| Completion | ≥ 70% TikTok · ≥ 65% Shorts <30s · ≥ 55% 30-60s | the viral gate |
| Sends-per-reach (IG) | north-star IG metric | DM shares weighted 3-5x likes |
| Swipe-completion (carousels) | > 60% | penalty if dead-stop after slide 1 |
| Reddit first-hour reply SLA | reply within 1h | first hour decides ranking |
| GitHub star spike (launch post) | net new stars in 48h window | tracked from upstream repo |

## Ledger (empty, SETUP, nothing published)

| Date | Asset / template | Pillar | Platform | Format | Key metric | Verdict | Action |
|---|---|---|---|---|---|---|---|
|, |, |, |, |, |, |, | *(no assets published, machine in setup)* |

### Verdict definitions
- **WIN** → promote the pattern into `templates/` / `prompt-library/` + schedule a follow-up within 48h.
- **NEUTRAL** → keep, re-test with one variable changed.
- **FAIL (1)** → adjust (hook, cover, pacing, code snippet) and re-test once.
- **FAIL (2)** → **RETIRED**: do not regenerate this pattern.

> First entries expected after the first GitHub content push and Reddit warm-up threads
> (`shotlist.md` calendar slots: `rmux-tmux-for-agents-thread`, `sdk-quickstart-tutorial`,
> `three-surfaces-one-daemon`). No fabricated data here (L2), the ledger fills only
> with real observed metrics.
