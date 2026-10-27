---
issue: 36
status: todo
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

## Review

## Verification

## Feedback
