---
issue: 13
status: to-test
wiki: [[animation]], [[mail]]
note: "Split de #12 (sprint Timeline histoire)."
created: 2026-10-26
---

## Context / Decisions

- Apparitions (gifted props, creepy events, found objects, Bouteilles) arrive by **Jour de jeu**, no longer by Lettres read. Stardew-like: the garden changes on its own, the Lettre explains it.
- Builds on #12 (`src/story/`, `StoryClock`, `Timeline`) and #14 (`--day`, used by the acceptance criteria).
- `StoryEvent` gains `Arrival(Thing)`. Each Arrival beat is declared in the same source as the Lettre that explains it:
  - village source: Lettre N threshold → day N (bench 4, cat bow 6, footprints 8, well 13, bone 15, key 17, treehouse 22, owl friend 24, blue lantern 26, garland 31, kittens 33, x mark 35, balloon 40, ducklings 42, found objects 45, weather vane 49, bunting 58, chest 59).
  - creepy source: strange Lettre 3 / 5 / 8 / 11 → its day: wet footprints 13, silhouette 21, whispering well 33, staring 45.
  - archives source: Bouteilles with the 3rd archive → day 40.
- An Apparition shows from `courrier.heure` of its Jour de jeu, same instant as its Lettre. weathr left open → appears live at rollover, no restart.
- `Unlocks` stays the view `Lore`, `Creepy`, `AnimationManager` and the Aide screen read; it's now built from the Timeline + today's Jour de jeu (`Unlocks::after_reading` → derived from Arrival beats). Never stored. `Lore` drawing code untouched.
- `App` refreshes Unlocks on Jour de jeu rollover, not only after reading.
- Choices (`1`/`2` answers) stay tied to reading: default variant until answered.
- Badges stay as today (letter milestones count Lettres read).
- Players with unread Lettres: everything due by their Jour de jeu shows at once.
- Out of scope: Habitants (#4), Arbre growth (#5), new Apparitions.

## Acceptance criteria

- [ ] `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` all green.
- [ ] `weathr --day 3`: no bench. `weathr --day 4`: bench in the garden.
- [ ] `weathr --day 13`: well and wet footprints. `--day 22`: treehouse. `--day 59`: chest.
- [ ] Arrival ignores reading: a save on Jour de jeu 30 with only 2 Lettres read shows bench, well and treehouse.
- [ ] `--day 39`: no Bouteille ever drifts; `--day 40` onward: Bouteilles can drift and `r` shows in Aide.
- [ ] Rollover: start 2026-10-01, `courrier.heure` 9 → at 2026-10-04 8:59 no bench, at 9:00 bench appears without restart (test on Unlocks with injected clock).
- [ ] Test: for every Apparition, Unlocks on its day − 1 is off and on its day is on.
- [ ] Choice variant (e.g. bow colour) still shows the default until the Lettre's question is answered.

## Implementation

stacked on: wa/14-ajouter-le-flag-day @ 1f691c2

Autopilot 2026-10-26, one implementer.

- `story/event.rs`: `Thing` enum (23 variants, one per `Unlocks` bool except `choices`), `StoryEvent::Arrival(Thing)`, `Beat::arrival`. Arrival sorts after mail on the same day.
- `story/timeline.rs`: `mail()` mail-only; `arrived_by(day)`.
- Sources declare Arrivals next to their mail: `village.rs` `ARRIVALS` by Lettre number (only if that Lettre exists), `creepy.rs` via `creepy_day()`, `archives.rs` Bottles with archive 3 via `archive_day()`. No second copy of any schedule.
- `animation/lore.rs`: only `Unlocks::after_reading` removed; `Unlocks`, `Choices`, drawing untouched; animation doesn't import story.
- `app.rs`: `unlocks_on(&Timeline, day, &answers)` replaces `unlocks_for`; refreshed on Jour de jeu rollover (existing `refresh_timeline`), no longer on reading.
- Side effect by design: Album gifts page and unlock-gated animal behaviours follow the Jour de jeu too.
- Proof: fmt ✅ · clippy ✅ · `cargo test` ✅ 177 passed. #12 golden fixtures unchanged.
- Rebased 2026-10-27 onto rebased #14 (after #12 merged, PR #26). Conflicts in `app.rs`, `event.rs`, `creepy.rs`, `timeline.rs`: `Timeline` keeps `mail: Vec<(usize, Entry)>` + `first_mail` from #12's review and stores `arrivals: Vec<(usize, Thing)>` beside it, both built once in `new()`; `arrived_by` reads `arrivals`. develop's badge / StarWatch tests kept next to #13's. #13 doc comments dropped. fmt ✅ · clippy ✅ · test ✅.

## Review

## Verification

Pty harness + ANSI replay into a 100×30 grid, temp `HOME` (real save shasum unchanged):
- ✅ `--day 3` no bench · `--day 4` bench.
- ✅ `--day 12` vs `--day 13 --night`: well + wet footprints only on 13.
- ✅ `--day 21` vs `22`: treehouse 0 → 1 · `--day 58` vs `59`: chest 0 → 1.
- ✅ Aide: `--day 40` lists `[r] ramasser une bouteille`, `--day 39` doesn't.
- ✅ Unit: every Apparition off on day−1, on on its day · day 30 with 2 Lettres read → bench, well, treehouse · rollover 08:59/09:00 · choices default until answered.

## Feedback
