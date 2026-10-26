---
issue: 25
status: to-close
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

- `src/weather/types.rs`: `WeatherCondition::is_overcast()` (`Cloudy | Overcast`), `WeatherConditions.is_overcast` flag; `is_clear_sky()` checks `!is_overcast` instead of `!is_cloudy`. Test built from real conditions: PartlyCloudy clear; Cloudy, Overcast, rain, storm, snow, fog not.
- `src/app_state.rs`: `update_weather` sets `is_overcast` next to `is_cloudy` (single path for real, offline, simulated and demo weather).
- `is_cloudy` and its users (cloud drawing, cat) untouched; fireflies untouched.
- Proof: build ✅ · fmt ✅ · clippy only the known provider.rs warning ✅ · `cargo test` all green ✅.

## Review

### Round validation

- ✅ review clean (4 lenses), nothing changed since the user's test.

## Verification

Agent drove the release binary in a pty (real Berlin weather replayed, 80 s per scenario):

- ✅ Éclaircies night: shooting stars from 1.7 s, caption `Hou ! Là-haut : Pégase.`, `Étoile filante` + `Chasseur d'étoiles` cards.
- ✅ Nuageux, Couvert, rain, snow, fog nights: 0 shooting stars, 0 constellations, no star Badge.
- ✅ Ciel dégagé night: unchanged (shooting stars, `Cassiopée`, star Badges).
- ✅ `weathr --demo`: `DEMO 16/36 Night` (Partly Cloudy) shows shooting stars; scenes 17–28 show none.

## Feedback
