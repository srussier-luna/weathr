---
issue: 25
status: todo
wiki: [[animation]], [[weather]]
note: "etoile filante si eclaircie"
created: 2026-10-26
---

## Context / Decisions

- Éclaircies (`PartlyCloudy`) nights count as a clear sky: shooting stars (normal and meteor showers) and the owl's constellations show again.
- Still hidden: Nuageux (`Cloudy`), Couvert (`Overcast`), rain, storm, snow, fog.
- One rule for both: `WeatherConditions::is_clear_sky()` (shared by `StarSystem` and `StarGazing`).
- `is_cloudy` keeps its meaning elsewhere (cloud drawing): Éclaircies must be told apart without changing it (e.g. an `is_overcast` flag for `Cloudy | Overcast` set where `is_cloudy` is set).
- Badges follow: a shooting star seen on an Éclaircies night counts again (`Étoile filante`, `Chasseur d'étoiles`, `Perséides`, `Géminides`), real weather only.
- Fireflies rule unchanged.

## Acceptance criteria

- [ ] Clear-sky night, Éclaircies: shooting stars appear; the owl's constellation caption appears.
- [ ] Night, Nuageux, Couvert, rain, snow or fog: no shooting star, no constellation.
- [ ] Night, Ciel dégagé: unchanged (shooting stars and constellations).
- [ ] `weathr --demo`: the night Éclaircies scene shows shooting stars.

## Implementation

## Review

## Verification

## Feedback
