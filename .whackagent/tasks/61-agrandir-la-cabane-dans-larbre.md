---
issue: 61
status: to-close
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

stacked on: wa/62-agrandir-le-coffre @ 08831ba

- `src/animation/lore.rs`: `TREEHOUSE` 16 × 4, `TREEHOUSE_WIDTH = 16`, `TREEHOUSE_MIN_CANOPY_WIDTH = 18`, `treehouse_spot(&Tree) -> Option<(u16, u16)>`: None under 18 columns; x = `tree.x + (width - 16) / 2`; y = highest row where all 4 rows sit inside the round foliage outline (`Tree::canopy_span`). Replaces `canopy_height >= 6`. Paint choice and ladder unchanged.
- Tests: width 17 → None, width 24 centred and inside, every foliage height 8–15 gets a spot, drawing rows all 16 wide.
- ⚠️ Winter fir (triangular): the Cabane overlaps its narrow top rows — not in spec, left as is.
- Build ✅ · fmt ✅ · tests ✅ · clippy ❌ pre-existing on develop only (`src/weather/provider.rs:23`).

## Review

Round validation — LENSES: style ✓ · elegance ✓ · structure ✓ · correctness ✓ — 3 minor, fixed by autofix:
- 🟡 [elegance] test copied Tree's width formula → real trees via `Tree::fit(100, horizon_y)`, non-empty check.
- 🟡 [style] vacuous `y >= top_y` assert → each Cabane row checked inside `canopy_span`.
- 🟡 [elegance] `(*top..top + rows)` → `|&top|` + `(top..top + rows)`.
Final: clean. Build ✅ · fmt ✅ · clippy ✅ (except pre-existing provider.rs:23) · tests ✅.

## Verification

manual validation — not run by agent (interactive TUI, no headless driver). Proof = build + tests.

## Feedback
