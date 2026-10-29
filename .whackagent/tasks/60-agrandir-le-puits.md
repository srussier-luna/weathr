---
issue: 60
status: todo
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

## Review

## Verification

## Feedback
