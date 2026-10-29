---
issue: 61
status: todo
wiki: [[scene]], [[animation]]
note: "la cabane dans les arbre est trop petite elargie la"
created: 2026-10-29
---

## Context / Decisions

> la cabane dans les arbre est trop petite elargie la

- Cabane today: `src/animation/lore.rs` (`TREEHOUSE`, 4 × 2, at `center_x() + 3`, shown when `canopy_height >= 6`), ladder `H` down the trunk.
- **New drawing, 16 × 4** (4× wider), keeps the `treehouse_blue` choice colour (blue / red):
  ```
     _/------\_
   /____________\
  |  []  ||  []  |
  |______||______|
  ```
- **Centred on the foliage**, in its upper part, fully inside the canopy.
- **Shown only when the foliage is ≥ 18 columns wide**; smaller Arbre → no Cabane. No `‹ dézoom` hint here (#60's hint is for objects left of the Arbre).
- Ladder unchanged: from the canopy bottom to the ground, along the trunk.
- Out of scope: Puits, Coffre, Lettres.

## Acceptance criteria

- [ ] `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` all green.
- [ ] Cabane unlocked, big Arbre (foliage ≥ 18): 16 × 4 Cabane centred on the foliage, inside it.
- [ ] Cabane unlocked, foliage < 18 columns: no Cabane, no ladder.
- [ ] `treehouse_blue` choice still paints it blue, else red.
- [ ] Ladder still runs from the canopy bottom to the ground.
- [ ] Placement helper tested: centred x, fits / doesn't fit by foliage width.

## Implementation

## Review

## Verification

## Feedback
