---
issue: 16
status: todo
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

## Review

## Verification

## Feedback
