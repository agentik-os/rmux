---
project: rmux
layer: 06-branding
produced_by: marketing-machine-upgrade (spec §2.3 avatar-persona.md)
status: filled
decision: scene-led / product-led, NO recurring human avatar, NO soul trained now
source_of_truth: marketing/03-visual-identity/higgsfield/avatar-soul.md
---
# Avatar / Persona, rmux

## Decision: **scene-led, no recurring human spokesperson**

`hasRecurringAvatar: false` · `soul_id: null` · `heygen_avatar_id: null` → **generate without `--soul-id`.**

rmux is developer infrastructure, a Rust terminal multiplexer (`README.md`). Its audience is
engineers and AI-agent builders who trust *artifacts*, not personalities: real terminal output, typed
snapshots, architecture diagrams, and the multiplex-grid motif (`avatar-soul.md`). A human avatar
would:
- read as generic "AI startup" marketing and clash with the engineer-to-engineer voice (`product-marketing.md`);
- violate DA.md's DON'T list ("no stock photos of people, no hand-on-keyboard clichés, no mascots");
- add a face-consistency maintenance burden (Soul training = paid Higgsfield CLI opt-in, R-VISUAL-ID) for zero credibility gain with this ICP.

The brand's "character" is the **terminal itself**: canvas `#0B0E14`, the mint cursor `#36E2C4`,
the split-pane grid, and the lowercase monospaced `rmux` wordmark. That is the recurring identity
anchor, no soul-id required (DA.md §Avatar / Soul-ID decision).

---

## The rmux "persona" = a RECURRING ENVIRONMENT

Since no face repeats, what repeats, and must stay **locked** for series coherence (spec §2.2
series discipline), is the **scene**. Treat these settings as the persona sheet would be for
a human character.

**"Subject" recurring anchor:** the **multiplex pane grid** (thin `#1E2733` dividers, rectangular
pane blocks) + the **mint `#36E2C4` cursor** + **real rmux terminal output** (plausible commands
and typed `PaneSnapshot` data). These three elements repeat across every OG card, carousel cover,
and demo screencap, that *is* the face of rmux.

**5 recurring settings** (the "environments", equivalent to the persona's recurring scenes):

1. **Multi-agent orchestration view**, the 4-pane terminal canvas: one pane per agent, mint
  cursor active in one, amber `#F0883E` "agent running" glyph in another, thin `#1E2733`
  dividers, near-black `#0B0E14` field. The hero frame. Source: `shotlist.md` #1.
2. **SDK code panel**, a dark code editor / terminal split: `ensure_session` / `send_text` /
  `snapshot()` calls in Geist Mono, mint highlighting on the `await` resolve. Source: `shotlist.md` #3.
3. **rmux vs tmux comparison split**, a clean 50/50 split: left = tmux (grey, manual), right =
  rmux (mint cursor active, SDK call resolving). Source: `shotlist.md` #5.
4. **Architecture diagram**, three-surfaces → one-daemon box-and-connector: CLI / `rmux-sdk` /
  `ratatui-rmux` boxes in `#1E2733` chrome, thin connectors, daemon at center, mint accent on the
  active connection. Source: `shotlist.md` #8.
5. **Terminal testing assertion panel**, `pane.snapshot()` → `assert_contains("ready")`, a
  mint "PASS" badge, dark IDE / test runner background. Source: `shotlist.md` #7.

**"Wardrobe" = scene palette:** pulled from `tokens.color`, `bg #0B0E14`, `surface #1E2733`,
`text #E6EDF3`, **one** `accent #36E2C4`, and `amber #F0883E` strictly as a live/alert cue.
No saturated color, no decorative gradients.

**Voice / tone (linked to `product-marketing.md` §Voice & tone):**
Engineer-to-engineer. Precise, technical, no marketing fluff. Show the command, show the snapshot,
show the diff. Confident but honest, name the fork/upstream reality, name "preview status."
Rust-native register: "typed", "locked", "forbid-unsafe" are credible; hype adjectives without a
code artifact are not.

**Scene do/don'ts:**
- Show **real** plausible rmux commands: `rmux new-session`, `ensure_session("name")`, `snapshot()`, `wait_for_text("ready")`.
- Mint cursor **blinks** only in "live / active" context, not decoratively.
- Amber `#F0883E` appears **only** as an "agent running" or "alert" status glyph, never as ambient color.
- **Never** Rust-orange branding (DA.md: rmux owns mint, deliberately distinct from Rust-orange tool crowd).
- **Never** lorem-ipsum, invented flags, or fabricated benchmark numbers (L2 / R-CITE).

## Realism defaults (spec §2.3 · §4.2)

Since there is **no Soul**, consistency is enforced by the brand-locked system prompt
(`03-visual-identity/higgsfield/system-prompt.md`) + DA.md, not a trained face. No lensed shots
of people exist in the planned shot queue (`shotlist.md`: all shots are product/scene-led). If
any physical hardware shot is ever added, it receives the **anti-generic block** from
`prompt-library/kill-list.md §3` (candid, not posed, natural light, Kodak Gold 200 grain on the
environment only, screen stays crisp).

UI / terminal output: **always crisp and real** (high-DPI screencaps of shipped demos). Grain, if
ever used, lives on the physical environment only, never on the terminal screen.

---

## Conditional brief, IF a spokesperson were ever needed (NOT trained now)

*(Nothing to train until this brief is explicitly activated by the operator, R-VISUAL-ID.)*

- **Who:** a developer / open-source maintainer, 25-40, precise manner, credible to the agent-infra
 ICP, **never** "influencer" energy. If anyone, the real project maintainer (honesty L2), not a
 synthetic face.
- **If trained then:** `higgsfield-soul-id` on ≥20 photos ≥960px, varied angles/expressions, no
 sunglasses (~5 min, ~25 credits) → fill `soul_id` here + in `tokens.json.avatar`;
 `heygen_avatar_id` for talking-heads. `prompt-library/avatar-heygen.md` details the HeyGen wiring.
- **When to reconsider:** only if a founder-explainer content series is explicitly approved and
 the agent-infra ICP has moved toward video-trust signals. Not before.

> **Default, today and until explicit activation: generate WITHOUT `--soul-id`.** The identity = the terminal + the pane grid + the mint cursor.
