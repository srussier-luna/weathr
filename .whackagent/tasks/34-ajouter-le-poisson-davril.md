---
issue: 34
status: to-test
wiki: [[animation]], [[scene]]
note: "1er avril : poisson en papier accroché dans le dos du facteur"
created: 2026-10-27
---

## Context / Decisions

> 1er avril : poisson en papier accroché dans le dos du facteur

- No blocker, no sprint. Quick Fête, same pattern as the others: a flag on `Holiday` computed in `Holiday::on` (`src/holiday.rs`).
- **1 April only**, every year, whole day.
- **Two paper fish**, small, pink, e.g. `><>`:
  - on the Facteur's back, during his trip that day (`src/animation/postman.rs`);
  - stuck on the mailbox, all day (mailbox from `GardenPlan`).
- No mailbox drawn (small terminal) → no fish there, Facteur's fish still shown.
- No Badge, no `--demo` scene.
- Out of scope: other April jokes, fish in Lettres.

## Acceptance criteria

- [ ] `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` all green.
- [ ] `Holiday::on(1 April)` sets the flag; 31 March and 2 April don't (test).
- [ ] 1 April: paper fish on the mailbox all day.
- [ ] 1 April, Facteur's trip: fish on his back while he rides.
- [ ] 2 April: no fish anywhere.
- [ ] Narrow terminal without mailbox: no crash, Facteur's fish still drawn.

## Implementation

- `src/holiday.rs`: `is_april_fools` (1 April) + test.
- `src/animation/postman.rs`: `set_april_fools`, pink `><>` (`PAPER_FISH`, `PAPER_PINK`) on the Facteur's back (left, he rides left→right) on every trip phase but Waiting; on the mailbox front at `mailbox_x + 2`, flag at +6 stays visible. Helpers `back_fish`, `mailbox_fish` tested; no mailbox → no fish there.
- `src/animation_manager.rs`: `render_garden` passes `holiday.is_april_fools` to the Facteur.
- Build ✅ · fmt ✅ · clippy ✅ (pre-existing double_must_use allowed) · tests ✅ (3 new).

## Review

## Verification

autopilot: terminal app, not launched by the agent — build + tests only. Manual check yours.

## Feedback
