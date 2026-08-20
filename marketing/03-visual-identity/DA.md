---
project: rmux
layer: visual-identity
produced_by: brand-identity (/omg-brand-identity) + art-director-content-engine
inputs: [README.md (rmux.io SVG headers dark/light, terminal-demo GIF, architecture diagram), no brand-book found]
status: filled
---
# Direction Artistique (DA), rmux

> The visual law every Higgsfield/AI generation obeys. **No `brand-book/` exists in the repo**, the only
> existing visual cues are the README's dark/light SVG headers, the terminal-demo GIF, and the architecture
> diagram on rmux.io. This DA establishes a coherent system reconcilable with those (dark-first, terminal-native,
> monospace). One signature accent, disciplined chrome (per the operator's TUI design taste: grey chrome + ONE accent).

## Visual concept (1 line)
**The programmable terminal made visible**, a precise, dark, monospaced grid where panes split, agents run, and structured state is rendered as first-class typed data, not screen-scrape noise.

## Mood / emotion
Engineered · precise · calm-under-load · powerful-but-quiet. The feeling of a senior engineer's perfectly-tuned terminal at 2am: fast, controlled, zero clutter. Confidence through restraint, never loud, never "playful SaaS."

## Color palette (hex + roles)
| Role | Hex | Use |
| :--- | :--- | :--- |
| **Canvas / terminal black** | `#0B0E14` | Primary background (dark-first, matches README dark header) |
| **Panel / pane surface** | `#11161F` | Cards, pane chrome, code blocks |
| **Border / grid line** | `#1E2733` | Pane dividers, the multiplex grid motif |
| **Ink / foreground** | `#E6EDF3` | Primary text, code |
| **Muted / comment** | `#7D8590` | Secondary text, labels, captions |
| **★ Signature accent, "rmux mint"** | `#36E2C4` | THE one accent: logo mark, prompt cursor, key highlight, live/active pane, CTAs. Use sparingly for maximum signal. |
| **Functional amber (alert/agent-live)** | `#F0883E` | Sparingly, for "agent running" / warning states only, never as a second brand color |
| **Light-mode canvas** | `#FBFCFD` | For the rare light surfaces (README light header parity) |
| **Light-mode ink** | `#0B0E14` | Text on light |

Discipline: **one** brand accent (mint `#36E2C4`). Amber is a *functional* status color, not a co-brand. No gradients-as-decoration, no rainbow syntax-highlight as hero art.

## Typography (display / body)
- **Display / wordmark:** a precise geometric mono or grotesk, **Berkeley Mono**, **Geist Mono**, or **JetBrains Mono** (lowercase "rmux", tight tracking). The wordmark is monospaced on purpose, it *is* a terminal tool.
- **Headings (marketing surfaces):** a clean neo-grotesk, **Geist**, **Inter**, or **Söhne**, medium/semibold, tight leading.
- **Body:** Inter / Geist regular, generous line-height for readability.
- **Code / terminal:** JetBrains Mono or Berkeley Mono, the same family as the wordmark for coherence.

## Composition & layout principles
- **The multiplex grid is the hero motif:** split panes, clean dividers (`#1E2733`), the blinking mint cursor. Compose around rectangles snapping into a grid.
- **Generous negative space** on the canvas-black; let one pane or one line of code carry the frame.
- **Left-aligned, terminal-anchored:** prompts, `$`/`❯` glyphs, monospace alignment. Real-looking shell output, never lorem-ipsum.
- **Asymmetry with order:** offset a single mint element against grey chrome for focus.
- **Diagram language matches rmux.io architecture diagram:** three surfaces (CLI / SDK / widget) → one daemon, boxes + thin connectors.

## Photography / illustration style
- **No stock photography. No human-hand-on-keyboard clichés.** Visual identity is **product/scene-led**: real terminal screencaps, asciinema frames, code on canvas-black, schematic diagrams.
- Where 3D/render is used: matte, low-noise, dark studio, a single mint rim-light on geometric "pane" blocks, never glossy, never neon-overload.
- Screenshots are first-class assets: crisp, high-DPI, real rmux output (the orchestration/broadcast/mirroring demos).

## Motion language (if video)
- **Typed, mechanical, snappy.** Text appears as if typed; panes split with a fast, eased snap (~150-200ms); the cursor blinks at a real terminal cadence.
- Transitions = grid reflows (a pane divides), never slide-fade SaaS animations.
- Easing: quick ease-out for entrances; respect "calm-under-load", motion is purposeful, never decorative (matches the operator's motion taste).

## Reference board (links / descriptions)
- rmux.io README dark/light SVG headers + terminal-demo GIF (existing brand cues, reconcile, don't contradict).
- rmux.io architecture diagram (three-surfaces-one-daemon visual grammar).
- Aesthetic north stars: Linear (dark precision), Vercel/Geist (mono + grotesk discipline), Warp (terminal-as-product), but **less neon**, more matte/engineered. tmux/zellij screenshots as the "before" foil.

## DO / DON'T (anti-generic guardrails)
**DO**
- Dark canvas (`#0B0E14`) by default; monospace everywhere it's load-bearing.
- Exactly one mint accent per frame; let it mean "active / important".
- Show real terminal output, real code, real pane grids.
- Keep chrome grey and quiet; earn attention with restraint.
- Respect aspect ratios per platform (see preprompt.md): 1:1 / 4:5 social, 16:9 demos/OG, 9:16 shorts.

**DON'T**
- ❌ No generic "AI startup" gradients, blobs, glassmorphism, or 3D mascots.
- ❌ No rainbow palettes or multicolor syntax-highlight as hero decoration (one accent only).
- ❌ No stock photos of people, no hand-on-keyboard, no abstract "tech" swooshes.
- ❌ No lorem-ipsum or fake commands, every shell line must be plausible rmux usage.
- ❌ Don't co-brand amber as a second identity color; it's a status signal only.
- ❌ Don't imitate Rust-orange branding, rmux owns mint, deliberately distinct from the Rust-orange tool crowd.
