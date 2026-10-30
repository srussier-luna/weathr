---
issue: 60
status: to-close
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
- Slots from right to left: canopy's left edge → 1-column gap → Puits. The Coffre (#62) goes further left: the Puits arrives first in the story.
- **No room** (narrow terminal, or no Arbre) → Puits not drawn, and a dark grey `‹ dézoom` hint on the screen's left edge, 4 rows above the Arbre's top. Hint shown only while an unlocked object is hidden. Reusable by #62.
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

- `src/animation/garden.rs`: `GardenLayout::well_x() -> Option<u16>` — `tree.x - (1 + 9)` (gap · Puits), `None` without Arbre or room. Tests: 200 × 40 fits, 120 × 30 hidden.
- `src/animation/lore.rs`: `WELL: [&str; 4]` (9 × 4, shared indent dropped: `" _/^^^\_"`, `"/_______\"`, `" |  o  |"`, `"(_______)"`), roof in `well_green` colour, posts `WOOD`, rim grey. Hidden → dark grey `‹ dézoom` at column 0, 4 rows above `tree.top_y` (no Arbre → `ground_y / 2`). `+ 28` removed.
- `src/animation/creepy.rs`: `WELL_OFFSET` removed, reads `well_x()`. Wet footprints door → Puits (leftwards), to column 0 when hidden. No whisper when hidden.
- House and Arbre layout untouched.
- Build ✅ · fmt ✅ · tests ✅ · clippy ❌ pre-existing on develop (`src/weather/provider.rs:23`, `double_must_use` from the newer toolchain, file not in this diff).

## Review

Round validation — LENSES: style ✓ · elegance ✓ · structure ✓ · correctness ✓ — 3 + 1 minor, all fixed by autofix:
- 🟡 [correctness] wet footprints ran under the Puits → end at its right edge (`well_x + WELL_WIDTH`).
- 🟡 [elegance] squirrel `is_placed` + `(0, 0)` sentinel → `placed_on: Option<(u16, u16)>`.
- 🟡 [structure] `WELL_WIDTH` untied to the drawing → compile-time assert next to `WELL`.
- 🟡 [style] `pub(crate)` alone in src → `pub const WELL_WIDTH`.
Final: clean. Build ✅ · fmt ✅ · clippy ✅ (except pre-existing provider.rs:23) · tests ✅.

## Verification

manual validation — not run by agent (interactive TUI, no headless driver). Proof = build + tests.

## Feedback

### Round 1 — 2026-10-29

- Asked: « met un peu plus d'espace entre le coffre et le puits genre 5 colonnes et échange-les de place, on reçoit d'abord le puits et après le coffre »
- Triage: adjustment.
- Asked: « met le dézoom au milieu de l'écran, pas au niveau de l'herbe, entre 3 et 5 lignes au-dessus de l'arbre, en gris pour qu'il soit plus discret »
- Changed: hint 4 rows above the Arbre's top, `Color::DarkGrey`, still column 0.
- Changed: Puits right against the Arbre (`tree.x - (1 + 9)`), no reserved Coffre slot; the 5-column gap and the Coffre are #62's.

### Round 2 — 2026-10-29

- Asked: « on voit un bout de montagne dans le puits » → « tout doit passer en plein sauf exception, sinon on voit la montagne »
- Triage: defect.
- Changed: Puits posts row and `‹ dézoom` drawn with `draw_solid` (inner spaces painted); every Puits row now opaque.
- Asked: « quand je dézoome et que l'écureuil est dans l'arbre il se retrouve à des positions bizarres, genre dans le vide au-dessus de l'arbre » → fix here (user's call, pre-existing on develop).
- Changed: `src/animation/squirrel.rs` remembers the layout size it was placed on; size changes → back at the foot of the Arbre. Test: resize 120 × 30 → 240 × 50.
