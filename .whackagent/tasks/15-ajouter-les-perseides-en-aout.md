---
issue: 15
status: to-close
wiki: [[animation]], [[mail]]
note: "ajouter les perseide en aout pendant une semaine"
created: 2026-10-26
---

## Context / Decisions

- Perséides every year from 9 to 15 August, both included (real peak: night of 12 to 13 August).
- Date flag computed like the Fêtes (`Holiday::today`, `src/holiday.rs`).
- Look: a real rain of shooting stars — many at once (more than 5 together, up to 16), each one drawn like today's shooting stars. Outside the Perséides nothing changes: one at a time, same rate. (User scope change at review, replaces "about 3 times more often, one at a time".)
- When the Perséides end (midnight, 16 August), the extra stars vanish at once, so no rain leaks into the next night.
- Same sky rules as today's shooting stars: night only, hidden by rain, snow, fog, clouds.
- Badges:
  - Shooting stars seen during the Perséides never count for `Étoile filante` nor `Chasseur d'étoiles`.
  - New Badge `Perséides`, kind Fêtes: one shooting star seen during the Perséides. Total 37 → 38.
  - Real weather only: forced modes (`--demo`, `--simulate`, `--christmas`, `--fireworks`, `--night`) never award it, like every Badge.
  - Medal drawing for #7 (whichever lands second adds it): `"  * *"` `" / / "` `"/ /  "`.
- `--demo`: new scene `Perséides` (clear night, Perséides on), after the existing Fête scenes.

## Acceptance criteria

- [ ] 12 August, 23:00, clear sky: more than 5 shooting stars on screen at once most of the time; 20 August at the same hour: never more than one.
- [ ] 16 August, 23:00: back to the normal rate.
- [ ] 12 August, daytime: no shooting star.
- [ ] A shooting star seen on 12 August at night awards `Perséides`, never `Étoile filante`, and doesn't move the `Chasseur d'étoiles` count.
- [ ] Badges window: Fêtes shows `Perséides` and `(n/6)`; title shows `/38`.
- [ ] `weathr --demo`: the 1 s `Perséides` scene shows a rain, more than 5 shooting stars at once; no Badge awarded.

## Implementation

- `src/holiday.rs`: `is_perseids` (8th month, days 9..=15), `Holiday::perseids()` for the demo; boundary test (8, 9, 15, 16 August, two years).
- `src/animation/stars.rs`: `shooting_stars: Vec<ShootingStar>`; `set_meteor_shower(bool)`; shower → up to 16 stars, chance 0.3 per free slot (sky full in ~10 frames); outside → 1 star, chance 0.005, roll only when the sky is empty (as before); shower end truncates to 1. Seeded tests: sky full within half a second, > 5 at once in > 90 % of frames, one at most outside, shower end.
- `src/animation_manager.rs`: `star_system.set_meteor_shower(state.holiday.is_perseids)` in the night block.
- `src/app.rs`: pure `StarWatch::observe(is_visible, is_perseids)` replaces `stars_seen` / `was_star_visible`; during the Perséides a shooting star awards `perseids`, never `shooting_star`, never counts for `star_hunter`. Still inside the `!is_simulated` gate.
- `src/badges.rs`: `Perséides` Badge (Fêtes, total 38), medal `"  * *"` `" / / "` `"/ /  "`.
- `src/demo.rs`: `Perséides` scene (clear night) after Pâques, before Rainbow.
- `develop` (#7) merged in: Badges are announced by the #7 card, not a toast.
- Proof: build ✅ · fmt ✅ · clippy only the known provider.rs warning ✅ · `cargo test` all green ✅.
- Note for the user: the code has no rain / cloud / fog guard on shooting stars today (only night); none added. Stars keep their current sky behavior.

## Review

### Round validation

- 🟡 [correctness] `src/demo.rs` — 1 s demo scene showed one star at most · fixed by the user's scope change: a shower is a rain, sky full in ~10 frames
- 🟡 [structure] `src/animation/stars.rs` — calendar-named flag → `set_meteor_shower(bool)` · fixed
- 🟡 [correctness] task file — text still described ×3 one at a time · fixed
- 🟡 [elegance] `src/animation/stars.rs` — test `filter` closure with side effect → `for` loop · fixed
- 🟡 [correctness] task file — stale star count in Verification · fixed
- Round 3: code clean (4 lenses). Proof: fmt ✅ · clippy only the known provider.rs warning ✅ · `cargo test` all green ✅.
- Changed since the user's test: the Perséides are now a rain (up to 16 stars at once) instead of one star at a time ×3; extra stars vanish when the shower ends at midnight.

## Verification

Agent drove the release binary in a pty (libfaketime dates, real Open-Meteo weather replayed from cache):

- ✅ 12 August 23:00 clear: median 8 shooting stars at once, max 14, more than 5 in 90 % of samples (review round).
- ✅ 16 and 20 August 23:00: never more than one at once, normal rate; awards `Étoile filante` then `Chasseur d'étoiles`, not `Perséides`.
- ✅ 12 August daytime: 0 shooting stars in 228 s.
- ✅ 12 August night: `Perséides` awarded, never `Étoile filante` nor `Chasseur d'étoiles` during the shower (review-round run: badges `clear`, `perseids` only); card with its medal after the merge of #7.
- ✅ Badges window: `Badges 0/38`, `Fêtes (0/6)`, `Perséides` listed.
- ✅ `weathr --demo`: `Perséides` scene median 11 stars at once (max 14, > 5 in 98 % of samples); other night scenes at most 2; no Badge awarded.

## Feedback
