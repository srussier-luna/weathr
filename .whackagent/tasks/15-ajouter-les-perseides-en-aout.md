---
issue: 15
status: todo
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

## Review

## Verification

## Feedback
