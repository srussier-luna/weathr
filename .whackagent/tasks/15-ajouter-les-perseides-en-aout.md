---
issue: 15
status: to-test
wiki: [[animation]], [[mail]]
note: "ajouter les perseide en aout pendant une semaine"
created: 2026-10-26
---

## Context / Decisions

- Perséides every year from 9 to 15 August, both included (real peak: night of 12 to 13 August).
- Date flag computed like the Fêtes (`Holiday::today`, `src/holiday.rs`).
- Look: the current shooting stars, about 3 times more often. No radiant, no several at once.
- Same sky rules as today's shooting stars: night only, hidden by rain, snow, fog, clouds.
- Badges:
  - Shooting stars seen during the Perséides never count for `Étoile filante` nor `Chasseur d'étoiles`.
  - New Badge `Perséides`, kind Fêtes: one shooting star seen during the Perséides. Total 37 → 38.
  - Real weather only: forced modes (`--demo`, `--simulate`, `--christmas`, `--fireworks`, `--night`) never award it, like every Badge.
  - Medal drawing for #7 (whichever lands second adds it): `"  * *"` `" / / "` `"/ /  "`.
- `--demo`: new scene `Perséides` (clear night, Perséides on), after the existing Fête scenes.

## Acceptance criteria

- [ ] 12 August, 23:00, clear sky: shooting stars come about 3 times more often than on 20 August at the same hour.
- [ ] 16 August, 23:00: back to the normal rate.
- [ ] 12 August, daytime: no shooting star.
- [ ] A shooting star seen on 12 August at night awards `Perséides`, never `Étoile filante`, and doesn't move the `Chasseur d'étoiles` count.
- [ ] Badges window: Fêtes shows `Perséides` and `(n/6)`; title shows `/38`.
- [ ] `weathr --demo`: a `Perséides` scene shows frequent shooting stars; no Badge awarded.

## Implementation

- `src/holiday.rs`: `is_perseids` (8th month, days 9..=15), `Holiday::perseids()` for the demo; boundary test (8, 9, 15, 16 August, two years).
- `src/animation/stars.rs`: `set_perseids(bool)`; spawn chance 0.005 → 0.05 during the Perséides. Only one shooting star at a time and each stays ~85 frames, so ×10 spawn gives ~×2.8 visible at 120x45 (×3.6 at 80x24, ×2.5 at 160x50). Seeded test checks the ratio stays in 2.5..=3.5.
- `src/animation_manager.rs`: `star_system.set_perseids(state.holiday.is_perseids)` in the night block.
- `src/app.rs`: pure `StarWatch::observe(is_visible, is_perseids)` replaces `stars_seen` / `was_star_visible`; during the Perséides a shooting star awards `perseids`, never `shooting_star`, never counts for `star_hunter`. Still inside the `!is_simulated` gate.
- `src/badges.rs`: `Perséides` Badge (Fêtes, total 38), medal `"  * *"` `" / / "` `"/ /  "`.
- `src/demo.rs`: `Perséides` scene (clear night) after Pâques, before Rainbow.
- `develop` (#7) merged in: Badges are announced by the #7 card, not a toast.
- Proof: build ✅ · fmt ✅ · clippy only the known provider.rs warning ✅ · `cargo test` all green ✅.
- Note for the user: the code has no rain / cloud / fog guard on shooting stars today (only night); none added. Stars keep their current sky behavior.

## Review

## Verification

Agent drove the release binary in a pty (libfaketime dates, real Open-Meteo weather replayed from cache):

- ✅ 12 August 23:00 clear: 62 shooting stars in 228 s vs 17 on 20 August (×3.6 that run; ×2.8 vs both normal nights).
- ✅ 16 August 23:00: normal rate; awards `Étoile filante` then `Chasseur d'étoiles`, not `Perséides`.
- ✅ 12 August daytime: 0 shooting stars in 228 s.
- ✅ 12 August night: `Perséides` awarded, never `Étoile filante` nor `Chasseur d'étoiles` over 62 stars; card with its medal after the merge of #7.
- ✅ Badges window: `Badges 0/38`, `Fêtes (0/6)`, `Perséides` listed.
- ✅ `weathr --demo`: `DEMO 34/35 Perséides` scene with frequent shooting stars; no Badge awarded.

## Feedback
