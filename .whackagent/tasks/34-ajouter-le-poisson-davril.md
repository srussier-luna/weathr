---
issue: 34
status: to-close
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
- [ ] Narrow terminal without mailbox: no crash, no mailbox fish (the Facteur doesn't ride without a mailbox).

## Implementation

- `src/holiday.rs`: `is_april_fools` (1 April) + test.
- `src/animation/postman.rs` (after review: fish drawn in `render`, `back_fish` removed): `set_april_fools`, pink `><>` (`PAPER_FISH`, `PAPER_PINK`) on the Facteur's back (left, he rides left→right) on every trip phase but Waiting; on the mailbox front at `mailbox_x + 2`, flag at +6 stays visible. Helpers `back_fish`, `mailbox_fish` tested; no mailbox → no fish there.
- `src/animation_manager.rs`: `render_garden` passes `holiday.is_april_fools` to the Facteur.
- Build ✅ · fmt ✅ · clippy ✅ (pre-existing double_must_use allowed) · tests ✅ (3 new).

## Review

Round validation — LENSES: style ✓ · elegance ✓ · structure ✓ · correctness ✓ — clean after 3 rounds.
- 🟡 [correctness] narrow-terminal test claimed a visible back fish (Facteur never rides without mailbox) → test renamed, criterion reworded — fixed
- 🟡 [correctness] Facteur covers the mailbox fish while stopped → accepted (he stands in front of it)
- 🟡 [elegance] `back_fish` duplicated render's position math → removed, drawn in render — fixed
- 🟡 [elegance] `filter(|_| …)` on ignored value → `mailbox_x?` + `then_some` — fixed
- 🟡 [style] dead `visit()` in a test → removed — fixed
- Note: Facteur's fish has no test, manual check on 1 April.

## Verification

autopilot: terminal app, not launched by the agent — build + tests only. Manual check yours.

## Feedback
