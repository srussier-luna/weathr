---
issue: 60
status: to-test
wiki: [[scene]], [[animation]]
note: "on ne vois pas le puit met le a gauche de l'arbre en plus plus large"
created: 2026-10-29
---

## Context / Decisions

> on ne vois pas le puit met le a gauche de l'arbre en plus plus large

- Puits today: `src/animation/lore.rs` (`WELL_ROOF`/`WELL_POSTS`/`WELL_RIM`, drawn at `house_x + porch_x() + 28`), 5 × 3, sits on the house → hard to see.
- **New drawing, 9 × 4**, roof keeps the `well_green` choice colour, posts `WOOD`, rim grey:
  ```
    _/^^^\_
   /_______\
    |  o  |
   (_______)
  ```
- **Placed left of the Arbre's foliage** (`x < tree.x`), standing on the ground. Layout unchanged: house and Arbre don't move.
- Slots from right to left: canopy's left edge → 1-column gap → Coffre slot (9 wide, filled by #62) → 1-column gap → Puits. The Coffre slot stays empty in this task.
- **No room** (narrow terminal, or no Arbre) → Puits not drawn, and a grey `‹ dézoom` hint on the screen's left edge, ground level. Hint shown only while an unlocked object is hidden. Reusable by #62.
- One source of truth for the Puits position: `src/animation/creepy.rs` (`WELL_OFFSET`, wet footprints door → Puits, whisper at the Puits) reads the same position; Puits hidden → no whisper, footprints stop at the screen's left edge.
- Glossary rows Puits, Coffre, Cabane added with this task.
- Out of scope: moving the house or the Arbre, new Lettres.

## Acceptance criteria

- [ ] `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` all green.
- [ ] Puits unlocked, wide terminal (e.g. 200 × 40): 9 × 4 Puits fully left of the Arbre's foliage, on the ground, not over the house.
- [ ] Puits unlocked, terminal too narrow (e.g. 120 × 30): no Puits, grey `‹ dézoom` at the left edge.
- [ ] Puits locked: no Puits, no hint.
- [ ] `well_green` choice still turns the roof green.
- [ ] Night, wet footprints unlocked: footprints run from the door to the Puits.
- [ ] Position helper tested: fits / doesn't fit for given terminal sizes.

## Implementation

- `src/animation/garden.rs`: `GardenLayout::well_x() -> Option<u16>` — `tree.x - (1 + 9 + 1 + 9)` (gap · Coffre slot · gap · Puits), `None` without Arbre or room. Tests: 200 × 40 fits, 120 × 30 hidden.
- `src/animation/lore.rs`: `WELL: [&str; 4]` (9 × 4, shared indent dropped: `" _/^^^\_"`, `"/_______\"`, `" |  o  |"`, `"(_______)"`), roof in `well_green` colour, posts `WOOD`, rim grey. Hidden → grey `‹ dézoom` at column 0, row ground_y-1. `+ 28` removed.
- `src/animation/creepy.rs`: `WELL_OFFSET` removed, reads `well_x()`. Wet footprints door → Puits (leftwards), to column 0 when hidden. No whisper when hidden.
- Coffre slot left empty for #62. House and Arbre layout untouched.
- Build ✅ · fmt ✅ · tests ✅ · clippy ❌ pre-existing on develop (`src/weather/provider.rs:23`, `double_must_use` from the newer toolchain, file not in this diff).

## Review

## Verification

manual validation — not run by agent (interactive TUI, no headless driver). Proof = build + tests.

## Feedback
