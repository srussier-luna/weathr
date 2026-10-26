---
issue: 16
status: to-test
wiki: [[animation]], [[mail]]
note: "Géminides 13–14 décembre, pendant d'hiver des Perséides, réutilise le code de #15"
created: 2026-10-26
---

## Context / Decisions

- Winter twin of the Perséides (#15). Blocked by #15: reuses its rate boost and Badge exclusion, no second mechanism.
- Géminides every year from 10 to 16 December, both included (real peak: night of 13 to 14 December).
- Date flag computed like the Fêtes and the Perséides (`Holiday::today`, `src/holiday.rs`).
- Look: same as the Perséides — current shooting stars, about 3 times more often. No radiant, no several at once.
- Same sky rules as today's shooting stars: night only, hidden by rain, snow, fog, clouds.
- Badges:
  - Shooting stars seen during the Géminides never count for `Étoile filante` nor `Chasseur d'étoiles`.
  - New Badge `Géminides`, kind Fêtes: one shooting star seen during the Géminides. Total 38 → 39.
  - Real weather only: forced modes (`--demo`, `--simulate`, `--christmas`, `--fireworks`, `--night`) never award it, like every Badge.
  - Medal drawing for #7 (whichever lands second adds it), mirror of the Perséides: `"* *  "` `" \ \ "` `"  \ \"`.
- `--demo`: new scene `Géminides` (clear night, Géminides on), right after the `Perséides` scene.
- Glossary: `Géminides` row added.

## Acceptance criteria

- [ ] 13 December, 23:00, clear sky: shooting stars come about 3 times more often than on 20 December at the same hour.
- [ ] 17 December, 23:00: back to the normal rate.
- [ ] 13 December, daytime: no shooting star.
- [ ] 13 December, 23:00, snowing: no shooting star.
- [ ] A shooting star seen on 13 December at night awards `Géminides`, never `Étoile filante`, and doesn't move the `Chasseur d'étoiles` count.
- [ ] Badges window: Fêtes shows `Géminides` and `(n/7)`; title shows `/39`.
- [ ] `weathr --demo`: a `Géminides` scene, after `Perséides`, shows frequent shooting stars; no Badge awarded.

## Implementation

stacked on: wa/15-ajouter-les-perseides-en-aout @ da863c5

- `src/holiday.rs`: `MeteorShower { Perseids, Geminids }` with `MeteorShower::on(month, day)` (9–15 Aug, 10–16 Dec); `Holiday.meteor_shower: Option<MeteorShower>` replaces `is_perseids`; `Holiday::geminids()` (December setup: fir, winter river).
- `src/animation/stars.rs`: shared `set_meteor_shower(bool)` + `METEOR_SHOWER_SHOOTING_STAR_CHANCE` (0.05); new `set_sky_clear(bool)` — covered sky → no shooting star, the one in flight dropped.
- `src/animation_manager.rs`: night block sets the shower flag and `set_sky_clear(!raining && !thunderstorm && !snowing && !foggy && !cloudy)`.
- `src/app.rs`: `StarWatch::observe(is_visible, Option<MeteorShower>)`; any shower stops the `Chasseur d'étoiles` count; picks `perseids` / `geminids` / `shooting_star`.
- `src/badges.rs`: `Géminides` Badge (Fêtes, 39), medal `"* *  "` `" \ \ "` `"  \ \"`.
- `src/demo.rs`: `Géminides` scene right after `Perséides`.
- Proof: build ✅ · fmt ✅ · clippy only the known provider.rs warning ✅ · `cargo test` 173 passed ✅.
- Behavior change for every shooting star (from the "hidden by rain, snow, fog, clouds" decision): none on rainy, snowy, foggy or cloudy nights, Éclaircies included. Twinkling stars unchanged.
- Merge note: #15 is being reworked (many stars at once, flag rename); merging it here will conflict in `stars.rs` (`set_sky_clear` must clear every star) and `holiday.rs`.

## Review

## Verification

Agent drove the release binary in a pty (libfaketime, real Paris weather replayed, 225 s per run):

- ✅ 13 Dec 23:00 clear: 96 shooting stars vs 33 on 20 Dec (×2.9 count, ×3.0 visible time).
- ✅ 17 Dec 23:00: normal rate; awards `Étoile filante` then `Chasseur d'étoiles`, not `Géminides`.
- ✅ 13 Dec daytime: 0 shooting stars.
- ✅ 13 Dec 23:00 snowing: 0 shooting stars (37 before the guard), no star Badge.
- ✅ 13 Dec night: `Géminides` awarded with its medal card, never `Étoile filante` / `Chasseur d'étoiles`.
- ✅ Badges window: `Badges 2/39`, `Fêtes (1/7)`, `[x] Géminides`.
- ✅ `weathr --demo`: `Perséides` then `Géminides` scenes, frequent shooting stars, no Badge.

## Feedback
