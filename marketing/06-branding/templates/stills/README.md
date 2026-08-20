---
project: rmux
layer: 06-branding/templates/stills
produced_by: marketing-machine-upgrade (spec §2.5 · §5.2 · 6 Remotion <Still> archetypes)
status: filled
role: >
 The 6 mandatory Remotion <Still> archetypes described for rmux, Cover/Hook, Big-Stat, Step,
 Quote, Chart, CTA. Rendered via renderStill / `npx remotion still --props` for data-driven
 batches (fallback: Playwright-screenshot of HyperFrames-style HTML at 1080x1350, R-TEST). All 6
 share the locked layers (logo, safe-zone, grid, color roles from tokens.json) and expose only
 editable slots (headline, terminal output / diagram, stat, CTA). Change a token → propagates to
 all 6. Anti-generic: type is the hero, oversized headline + real code artifact, no decoration.
---
# Remotion <Still> archetypes, rmux

> Master **1080x1920**, feed/carousel **1080x1350 (4:5)**, cover **1080x1080 center-safe**, demo OG **1920x1080 (16:9)**.
> Recompose, never crop.
> **Locked layers (all archetypes):** `rmux` wordmark (corner), safe-zone 900x1400, 4-column grid, color roles `tokens.json`, pane-grid dividers `#1E2733`.
> **Editable slots:** headline · terminal output / screenshot / diagram · stat · CTA line.
> Terminal output is always real and plausible rmux commands. Crisp, no grain on screen content.
> One mint `#36E2C4` accent per frame, maximum.

---

## 1. Cover / Hook (`<CoverStill>`)
- **Purpose:** the 2-second audition (carousel / reel). Curiosity gap + specific proof point, ≤12 words.
- **Composition:** display headline in Geist 600 on `bg #0B0E14`, left-aligned (terminal anchor), oversized, tracking -0.03em. ONE word or phrase in `accent #36E2C4`. The pane-grid border motif as a thin framing element at the edges. Generous whitespace. Optional swipe cue (→) bottom-right.
- **Slots:** `{headline}` (≤12 words), `{accentWord}` (the one mint word), `{swipeCue}`.
- **rmux examples:**
 - "Your agents survive SSH drops." (accent: "survive")
 - "tmux you already know. SDK you wished it had." (accent: "SDK")
 - "One runtime. Three surfaces. Zero scraping." (accent: "Zero")
 - "90 tmux commands. Now in Rust." (accent: "90")

## 2. Big-Stat (`<BigStatStill>`)
- **Purpose:** a single verified number that lands (proof / differentiator).
- **Composition:** one **large `tabular-nums` figure** left-aligned in `text #E6EDF3`, the number itself in `accent #36E2C4` as the sole accent, a label below in `textMuted #7D8590`. `bg #0B0E14`, massive whitespace. The `rmux` wordmark small in the corner.
- **Slots:** `{stat}` (e.g., "90", "5", "3", "v0.3.1"), `{label}` (e.g., "tmux-compatible commands", "shipped demos", "platforms (Linux, macOS, Windows)", "current preview").
- **Rule:** one accent · numbers are real and grounded in shipped artifacts only (product-marketing.md, R-CITE / L2, no fabrication).

## 3. Step (`<StepStill>`)
- **Purpose:** one step of an SDK tutorial, a demo flow, or the rmux install sequence.
- **Composition:** a surface panel `#11161F` with hairline `#1E2733` border (the pane-chrome look, NO blur) carrying a **real code snippet** or terminal output in Geist Mono. Step badge (e.g., "2 / 5") in `tabular-nums` `#7D8590`. Headline above in Geist 600. The `snapshot()` return is the "confirm moment", hold at least one step on it.
- **Slots:** `{stepIndex}` (e.g., "2 / 5"), `{stepTitle}` (e.g., "Call `snapshot()`"), `{codeSnippet}` (real Rust SDK call).
- **Examples:** "1 · Start a session" → "2 · Send a command" → "3 · Call `snapshot()`" → "4 · Assert the output" → "5 · `cargo install rmux`".
- All code is valid, real rmux API surface (`feature-inventory-v1.yaml`, R-CITE).

## 4. Quote (`<QuoteStill>`)
- **Purpose:** a brand principle or engineering philosophy, not a fake testimonial (no social proof that doesn't exist yet, L2).
- **Composition:** Geist 500, tracking -0.025em, on `bg #0B0E14`, left-aligned, generous whitespace. A thin hairline `#1E2733` rule as separator. Attribution in `#7D8590` (brand / pillar name, never an invented customer name).
- **Slots:** `{quote}`, `{attribution}`.
- **rmux examples:**
 - *"Your terminal, now with an SDK."*, rmux
 - *"Anything the CLI can do, your code can do too."*, rmux
 - *"Typed. Not scraped."*, rmux SDK
 - *"One session. Three surfaces. Zero `capture-pane`."*, rmux
 - *"`#![forbid(unsafe_code)]`, and we mean it."*, rmux

## 5. Chart (`<ChartStill>`)
- **Purpose:** a minimal data-viz (rmux vs tmux comparison, platform matrix, SDK surface comparison).
- **Composition:** a clean table or bar chart on a surface panel `#11161F`, hairline `#1E2733` borders, `text #E6EDF3`; the rmux column / row marked with the **single** `accent #36E2C4`; `tabular-nums`. For comparisons: show **one dimension where tmux wins** (radical honesty about the fork/preview status, product-marketing.md §Honest scope note).
- **Slots:** `{chartData}`, `{caption}`.
- **rmux examples:** platform matrix (Linux ✓ macOS ✓ Windows ✓ for rmux vs. "no native Windows" for tmux); SDK surface (CLI ✓ SDK ✓ Ratatui widget ✓ vs. tmux CLI-only); feature verification grid from `spec/feature-inventory-v1.yaml`.

## 6. CTA (`<CtaStill>`)
- **Purpose:** closing slide of a carousel, or a standalone CTA card.
- **Composition:** a clean action line in Geist 600 on `bg #0B0E14`, the install command in `accent #36E2C4` (or in a surface `#11161F` code-block); `rmux` wordmark lockup; the repo / install URL. One CTA only.
- **Slots:** `{ctaLine}`, `{installCmd}`, `{url}`.
- **rmux examples:**
 - "`cargo install rmux` → github.com/Helvesec/rmux"
 - "Star on GitHub. It's MIT. It's yours."
 - "Give your agents a runtime. `cargo install rmux`"
 - "Try the SDK. Build something that was impossible with tmux."

---

## Production (spec §2.5 · §5.2)
- Render: `npx remotion still --props='{...}'` per post → slides 4:5 + center-safe cover 1:1 + OG 16:9 → IG carousel / PDF assembly for LinkedIn (7-10 slides). Fallback: Playwright-screenshot of HyperFrames-style HTML at 1080x1350 (R-TEST: Playwright CLI via Bash, never a MCP browser).
- **Locked vs editable** enforced by the component, not convention. Change a token in `tokens.json` → propagates to all 6 archetypes.
- Run Pass-1 gate (safe-zones, WCAG AA contrast, caption reserve, token conformance, kill-list scan) before export.
- Carousel continuity: seamless pane-grid bleed right edge (the divider `#1E2733` continues across slides) / progress markers (3/10); mini proof-point at slide 2-3, big payoff at slide 8+; always end on CTA + "save this" mid-deck micro-CTA.
- LinkedIn PDF: 7-10 slides, 1080x1350, post from a person not the company page; winning angles = SDK tutorials, comparison data, honest engineering takes, original screencaps of the demos.
