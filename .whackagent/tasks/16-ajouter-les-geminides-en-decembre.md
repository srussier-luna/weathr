---
issue: 16
status: to-close
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

- `src/holiday.rs`: `MeteorShower { Perseids, Geminids }` with `MeteorShower::on(month, day)` (9–15 Aug, 10–16 Dec); `Holiday.meteor_shower: Option<MeteorShower>` replaces `is_perseids`; `Holiday::geminids()` (December setup: fir, winter river).
- `src/animation/stars.rs`: one `ShootingStarSky { Covered, Clear, MeteorShower }` + `set_sky`; #15's rain reused (up to 16 at once); `Covered` → no shooting star at all, every star in flight dropped.
- `src/weather/types.rs`: `WeatherConditions::is_clear_sky()` (+ test), shared with `StarGazing` (constellations).
- `src/animation_manager.rs`: night block → `set_sky(Covered | MeteorShower | Clear)`.
- `src/app.rs`: `StarWatch::observe(is_visible, Option<MeteorShower>)`; any shower stops the `Chasseur d'étoiles` count; picks `perseids` / `geminids` / `shooting_star`.
- `src/badges.rs`: `Géminides` Badge (Fêtes, 39), medal `"* *  "` `" \ \ "` `"  \ \"`.
- `src/demo.rs`: `Géminides` scene right after `Perséides`.
- Proof: build ✅ · fmt ✅ · clippy only the known provider.rs warning ✅ · `cargo test` 173 passed ✅.
- Behavior change for every shooting star (from the "hidden by rain, snow, fog, clouds" decision): none on rainy, snowy, foggy or cloudy nights, Éclaircies included. Twinkling stars unchanged.
- #15 landed (squash) → `develop` merged in, PR retargeted to `develop`; conflicts were only from the squash. `cargo test` 179 passed.

## Review

### Round validation

- 🟡 [structure] `src/animation_manager.rs` — clear-sky predicate copied from `StarGazing` → `WeatherConditions::is_clear_sky()` · fixed
- 🟡 [elegance] `src/animation/stars.rs` — two bools for three sky states → `ShootingStarSky` enum · fixed
- 🟡 [style] wiki `animation.md` — stale names, Géminides missing · fixed
- Round 2: code clean (4 lenses). Proof: fmt ✅ · clippy only the known provider.rs warning ✅ · `cargo test` 181 passed ✅.
- Changed since the user's test: Géminides are a rain like the Perséides (after #15 landed); internal refactor only otherwise.

## Verification

Agent drove the release binary in a pty (libfaketime, real Paris weather replayed, 225 s per run):

- ✅ 13 Dec 23:00 clear: a rain, median 9 shooting stars at once (max 14), more than 5 in 93 % of samples (after #15's rework).
- ✅ 17 Dec 23:00: normal rate; awards `Étoile filante` then `Chasseur d'étoiles`, not `Géminides`.
- ✅ 13 Dec daytime: 0 shooting stars.
- ✅ 13 Dec 23:00 snowing: 0 shooting stars (37 before the guard), no star Badge.
- ✅ 13 Dec night: `Géminides` awarded with its medal card, never `Étoile filante` / `Chasseur d'étoiles`.
- ✅ Badges window: `Badges 2/39`, `Fêtes (1/7)`, `[x] Géminides`.
- ✅ `weathr --demo`: `Géminides` scene right after `Perséides`, a rain (median 12 at once); no Badge.

## Feedback
