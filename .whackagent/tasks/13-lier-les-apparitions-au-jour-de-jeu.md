---
issue: 13
status: todo
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

## Review

## Verification

## Feedback
