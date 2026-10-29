---
issue: 59
status: to-test
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

Autopilot 2026-10-29, one implementer, one brick.

- `src/app_state.rs`: `story_day` field (starts at 1), `set_story_day(day)` (store, compare, `weather_info_needs_update`), both bar lines print `{clock} | Jour {story_day} | …`.
- `src/app.rs`: `refresh_progress` calls `set_story_day(self.timeline_day)` right after `refresh_timeline()` (every frame, so the day switches without restart, demo included). Clock setup moved from `App::new` into `fn story_clock(progress, mail_hour)` (same 4 lines) so the fresh-save test runs the real path.
- Tests (5): bar and loading line start with `<clock> | Jour 8 | `, day change redraws, rollover 2026-10-29 08:59 → `Jour 7` / 09:00 → `Jour 8` (start 2026-10-22, hour 9), fresh progress → day 1.
- Notes: `Jour N | ` adds 9 columns, `Location: …` falls off sooner on narrow terminals (bar redesign = #37). `--day` (#14) not in code yet: the bar reads `timeline_day`, so an override there shows with no extra work.
- Proof: `cargo build` ✅ · fmt ✅ · clippy ✅ with `--all-targets -A clippy::double_must_use` (pre-existing `src/weather/provider.rs:23`) · `cargo test` ✅ 255 passed.

## Review

## Verification

Runtime, pty 120x40, `HOME` = temp dir (real save untouched), debug binary, quit with `q`:
- ✅ No save, `--simulate clear` → `09:31 | Jour 1 | Badges 0/39 [b] | Aide [h] | Weather: Clear | …`
- ✅ Save started 2026-10-22, `courrier.heure` 9, run 2026-10-29 09:31 → `09:31 | Jour 8 | Lettre ! [c] | …`
- ✅ Open across `courrier.heure` (`faketime` 08:59:50) → `08:59 | Jour 7 | …` then `09:00 | Jour 8 | …`, same process
- ✅ `--hide-hud` → no bar
- ✅ `--demo` → `09:32 | Jour 8 | Lettre ! [c] | … | DEMO 4/36 | …`

## Feedback
