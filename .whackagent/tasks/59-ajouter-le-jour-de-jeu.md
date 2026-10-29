---
issue: 59
status: todo
wiki: [[render]], [[mail]]
note: "ajouter le numero du jours dans l'histoire dans la barre du haut v1.6"
created: 2026-10-29
---

## Context / Decisions

> ajouter le numero du jours dans l'histoire dans la barre du haut v1.6

- Sprint **Timeline histoire**, built on #12 (landed in the sprint): uses `StoryClock`, never the old `Progress::day_on`.
- Target release 1.6: ships when the sprint lands in develop.
- **Barre du haut** shows `Jour N` right after the clock: `14:32 | Jour 8 | Lettre ! [c] | Badges 3/40 [b] | Aide [h] | Weather: …`.
  - Same on the loading line: `14:32 | Jour 8 | Weather: Loading... /`.
- N = the Jour de jeu the Timeline uses (`story_today(&clock)` in `src/app.rs`): changes at `courrier.heure`, like the Lettres. #14 (`--day`) overriding that day → the bar shows the overridden day, nothing extra here.
- No save yet → `Jour 1`.
- Always shown with the bar, every mode (demo, simulate included). `--hide-hud` hides it with the rest.
- weathr left open past `courrier.heure` → the bar switches to the next day without restart.
- Lives in `src/app_state.rs` (bar text) + `src/app.rs` (feeds the day). No new file.
- Out of scope: redesign of the bar (#37).

## Acceptance criteria

- [ ] `cargo fmt --check`, `cargo test` green; `cargo clippy -- -D warnings` green except the pre-existing `double_must_use` (`src/weather/provider.rs:23`).
- [ ] Bar reads `HH:MM | Jour N | …` with `Jour N` right after the clock, before « Lettre ! [c] » and Badges (test on the bar text).
- [ ] Loading line reads `HH:MM | Jour N | Weather: Loading...` (test).
- [ ] Start 2026-10-22, `courrier.heure` = 9: at 2026-10-29 8:59 the bar shows `Jour 7`, at 9:00 `Jour 8` (test).
- [ ] No `progress.json` → `Jour 1`.
- [ ] weathr open across `courrier.heure` → bar goes from `Jour N` to `Jour N+1` without restart.
- [ ] `--hide-hud` → no bar, unchanged.
- [ ] Running `weathr` on a save started 2026-10-22, on 2026-10-29 after `courrier.heure`: the bar shows `Jour 8`.

## Implementation

## Review

## Verification

## Feedback
