# Scene

`src/scene/` draw **static landscape**: mountains, ground, river, house, trees, fence, mailbox. No animation state; all recomputed from terminal size each frame. Moving things in [[animation]].

## Shape

- `WorldScene` (`src/scene/mod.rs:13`) — owns `House`, renders in order: mountains → ground → river → house → decorations. Called once per frame from `App::run`, between sky and garden layers (see [[architecture]]).
- `WorldScene::GROUND_HEIGHT = 7` (`src/scene/mod.rs:22`) — horizon is `height - 7`. Every layer (sun, rain spawn, garden) derives positions from it.
- `update_size` rebuilds house on resize; no other mutable state.

## Pieces

- **House** (`src/scene/house.rs:111`) — `Small` or `Large` ASCII by terminal size (≥170×32 for large). `x_on` centres it, then shifts left to leave room for tree. Exposes anchor points used by other modules: `chimney_offset`, `roof_perch`, `big_window`, `peak`, `porch_*`, `door_x`. Animals and lore position off these. Window lights switch off one by one at night, seeded by row/column (`window_color`, `src/scene/house.rs:298`).
- **Mountains** (`src/scene/mountains.rs:23`) — deterministic ridge from width + horizon (no RNG). Snow caps when snowing or winter. River reuses `ridge_tops`.
- **River** (`src/scene/river.rs:43`) — flows from summit near pond down to horizon, as list of `Stretch`es. Freezes (`=`) in winter. `is_water` and `stretch(i)` let duck, frog, drifting Bouteilles follow it.
- **Tree** (`src/scene/tree.rs:35`) — `Tree::fit` picks tallest tree fitting left of house, or none. Seasonal foliage; fir in December/winter, lights + presents at Christmas.
- **Decorations** (`src/scene/decorations.rs:52`) — fence right of house, mailbox left of tree, extra pine on wide terminals.
- **GardenPlan** (`src/scene/decorations.rs:20`) — tree + mailbox placement. Embedded in `GardenLayout` (`src/animation/garden.rs:9`) so Facteur stops at mailbox drawn here.

## Invariants

- Layout = pure function of `(width, height)`. Scene and garden must compute same `House`/`GardenPlan`; both call `House::for_terminal` + `x_on` independently, so placement changes must stay consistent in both.
- Solid house rows (walls, base) paint spaces between first and last glyph (`src/scene/house.rs:250`). This hides mountains behind house. Draw order alone not enough, since spaces normally transparent.
- `Holiday` (see [[animation]]) only changes looks here: snow caps, frozen river, fir.

## Gotchas

- Most `draw` helpers skip `' '`, so blank cells see-through. Use `draw_solid` or explicit spaces when something must occlude what's behind.
- Small terminals: `Tree::fit` and `mailbox_x` can be `None`. Readers (Facteur, lore) must handle missing case.