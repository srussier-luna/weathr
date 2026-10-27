---
issue: 36
status: to-test
wiki: [[animation]], [[scene]]
note: "6 janvier : couronne sur le toit ou sur le chat"
created: 2026-10-27
---

## Context / Decisions

> 6 janvier : couronne sur le toit ou sur le chat

- No blocker, no sprint. Quick Fête: a flag on `Holiday` computed in `Holiday::on` (`src/holiday.rs`).
- **6 January only**, every year, whole day, day and night.
- **Golden paper crown always on the roof**, at its peak (`House::peak`, `src/scene/house.rs`), e.g. `\/\/` in yellow. Never on the cat.
- Snow on the roof: crown drawn over it.
- Small and large house both get it.
- No Badge, no `--demo` scene.
- Out of scope: galette in Lettres, crown on Habitants.

## Acceptance criteria

- [ ] `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` all green.
- [ ] `Holiday::on(6 January)` sets the flag; 5 and 7 January don't (test).
- [ ] 6 January: golden crown at the roof's peak, small and large house.
- [ ] 6 January, snowing: crown still visible on the roof.
- [ ] 7 January: no crown.

## Implementation

- `src/holiday.rs`: `is_epiphany` (6 January) + test (5/6/7 Jan, 6 June).
- `src/animation/epiphany.rs` (new): `crown_offset(&House)` (small (15,2), large (24,0)), `render_crown`; tests for both house sizes. Registered in `src/animation/mod.rs`.
- `src/animation_manager.rs`: crown drawn in `render_garden` after `lore.render_scenery` (so after snow cover), golden `Rgb(255,200,40)`, day and night.
- ⚠️ Crown sits on the weather vane's mast cell (lore, unlocked by Lettre 49); the vane's arrow stays visible above.
- Build ✅ · fmt ✅ · clippy ✅ (pre-existing double_must_use allowed) · tests ✅.

## Review

## Verification

autopilot: terminal app, not launched by the agent — build + tests only. Manual check yours.

## Feedback
