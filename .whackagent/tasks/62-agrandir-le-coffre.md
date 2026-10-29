---
issue: 62
status: todo
wiki: [[scene]], [[animation]]
note: "idem le coffre est trop petit met le a gauche de l'abre en plus gros"
created: 2026-10-29
---

## Context / Decisions

> idem le coffre est trop petit met le a gauche de l'abre en plus gros

- Builds on #60 Agrandir le puits: reuses its left-of-Arbre slotting and the `‹ dézoom` hint.
- Coffre today: `src/animation/lore.rs` (`CHEST`, 5 × 2, right of the trunk, yellow `$` on the ground).
- **New drawing, 9 × 3**, `WOOD`, `$` in yellow:
  ```
   _______
  /_______\
  [___$___]
  ```
- **Placed left of the Arbre's foliage, between the Puits and the Arbre**, standing on the ground. Order left → right: Puits · Coffre · Arbre. Layout unchanged.
- The red `X` (`x_mark`, before the Coffre is unlocked) moves to the Coffre's new spot.
- **No room** → Coffre not drawn, grey `‹ dézoom` at the left edge (same hint as #60). Room for the Coffre but not the Puits → Coffre drawn, Puits hidden, hint shown.
- Key under the mailbox (`o-m`) unchanged.
- Out of scope: moving the house or the Arbre, Album, Lettres.

## Acceptance criteria

- [ ] `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` all green.
- [ ] Coffre unlocked, wide terminal (e.g. 200 × 40): 9 × 3 Coffre left of the Arbre's foliage, right of the Puits, no overlap.
- [ ] Coffre unlocked, terminal too narrow: no Coffre, grey `‹ dézoom` at the left edge.
- [ ] Coffre locked, `x_mark` unlocked: red `X` at the Coffre's spot.
- [ ] Coffre locked, `x_mark` locked: nothing there, no hint for it.
- [ ] Slot helper tested: Puits + Coffre fit / Coffre only / none.

## Implementation

## Review

## Verification

## Feedback
