---
project: rmux
layer: visual-identity/higgsfield
produced_by: higgsfield-soul-id (trains the identity anchor ONCE)
status: filled
note: NOVA_SOUL_ID already exists in secrets for the Nova persona; per-project soul trained separately if the brand needs its own face/character.
---
# Avatar / Soul-ID, rmux

## Does this project need a consistent avatar/character? [ ] yes [x] **NO**

**Decision: NO recurring avatar/character. Identity is product/scene-led.**

## If no, visual identity is product/scene-led (no recurring face); document why.
rmux is a developer infrastructure tool, a Rust terminal multiplexer (`README.md`). Its audience is
engineers and AI-agent builders who trust *artifacts*, not personalities: real terminal output, code,
architecture diagrams, and the multiplex-grid motif. A human spokes-avatar would:
- read as generic "AI startup" marketing and clash with the engineer-to-engineer voice (product-marketing.md);
- violate the DA's DON'T list (no stock people, no mascots);
- add a face-consistency maintenance burden for zero credibility gain with this ICP.

The brand's "character" is the **terminal itself**: the dark canvas (`#0B0E14`), the mint cursor (`#36E2C4`),
the split-pane grid, and the lowercase monospaced `rmux` wordmark. That is the recurring identity anchor,
no soul-id required.

**Consequence for generation:** all Higgsfield/AI generations run **without `--soul-id`** (see
`system-prompt.md` → Soul-id binding = N/A). Consistency is enforced by the brand-locked system prompt +
DA, not by a trained face.

## If yes, soul identity brief
*(Not applicable. Retained only as a future hook.)*
- If rmux ever adopts a brand mascot (e.g. a stylized terminal/robot motif, NOT a human), it would be a
 **non-human geometric character** trained once via `higgsfield-soul-id` and recorded below. Requires
 operator approval + a paid Higgsfield plan (R-VISUAL-ID). No generation performed in this setup phase.

## Soul-id (filled after training)
- soul_id: `N/A, not trained (decision: NO avatar)`
- trained_on: `N/A`
