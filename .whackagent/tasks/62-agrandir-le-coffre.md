---
issue: 62
status: to-test
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
- **Placed left of the Puits, 5 columns apart**, standing on the ground. Order left → right: Coffre · Puits · Arbre (the Puits arrives first in the story). Layout unchanged.
- The red `X` (`x_mark`, before the Coffre is unlocked) moves to the Coffre's new spot.
- **No room** → Coffre not drawn, grey `‹ dézoom` at the left edge (same hint as #60). Room for the Puits but not the Coffre → Puits drawn, Coffre hidden, hint shown.
- Key under the mailbox (`o-m`) unchanged.
- Out of scope: moving the house or the Arbre, Album, Lettres.

## Acceptance criteria

- [ ] `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` all green.
- [ ] Coffre unlocked, wide terminal (e.g. 200 × 40): 9 × 3 Coffre left of the Puits, 5 columns between them.
- [ ] Coffre unlocked, terminal too narrow: no Coffre, grey `‹ dézoom` at the left edge.
- [ ] Coffre locked, `x_mark` unlocked: red `X` at the Coffre's spot.
- [ ] Coffre locked, `x_mark` locked: nothing there, no hint for it.
- [ ] Slot helper tested: Puits + Coffre fit / Puits only / none.

## Implementation

stacked on: wa/60-agrandir-le-puits @ 4ea04b2

- `src/animation/garden.rs`: `GardenLayout::chest_x()` = `well_x() - (5 + 9)` (`CHEST_GAP = 5`), order Coffre · Puits · Arbre. Tests: 240 × 40 Coffre 5 columns left of the Puits, 120 × 30 none, a width where only the Puits fits.
- `src/animation/lore.rs`: `CHEST` 9 × 3 (`" _______"`, `"/_______\\"`, `"[___$___]"`), `WOOD`, on the ground (rows ground_y-3..ground_y-1), `$` re-coloured yellow; old ground `$` removed. Red `X` at the Coffre's centre on ground_y. Coffre/X moved out of the trunk block. dark grey `‹ dézoom` drawn once, 4 rows above the Arbre's top: Puits or Coffre unlocked and hidden.
- Not covered: `x_mark` unlocked, Coffre locked, no room → no X, no hint.
- Build ✅ · fmt ✅ · tests ✅ · clippy ❌ pre-existing on develop only (`src/weather/provider.rs:23`).

## Review

## Verification

manual validation — not run by agent (interactive TUI, no headless driver). Proof = build + tests.

## Feedback

### Round 1 — 2026-10-29

- Asked: « met un peu plus d'espace entre le coffre et le puits genre 5 colonnes et échange-les de place, on reçoit d'abord le puits et après le coffre » + « met le dézoom … entre 3 et 5 lignes au-dessus de l'arbre et en gris pour qu'il soit plus discret »
- Triage: adjustment.
- Changed: order Coffre · Puits · Arbre, 5 columns between Coffre and Puits; hint 4 rows above the Arbre's top, dark grey.
