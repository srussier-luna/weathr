---
issue: 43
status: todo
wiki: [[mail]], [[weather]]
note: "Prévisions dans la Gazette au lieu de la météo passée"
created: 2026-10-29
---

## Context / Decisions

> Prévisions dans la Gazette au lieu de la météo passée
> prevision resume plus beau jours, jours de pluie : lundi jeudi — si present jours de neige, jours d'orage, etc attention au vent preciser les alerte meteo alerte orage , alerte canicule etc

- Sprint **Timeline histoire** (target 1.6): the Gazette lives in `src/story/content/gazette.rs` from #12, not yet in develop.
- Today: each Monday Gazette shows « Ta semaine : N jours de pluie, maximum X °C, minimum Y °C. », the 7 past days (`week_recap`, `src/remote_weather.rs`), frozen in `progress.json` (`gazette_weather`).
- New: the Gazette shows the week **from its Monday to its Sunday**, whatever day it is fetched: days already past = real weather, others = forecast (Open-Meteo forecast API, `start_date` / `end_date` = the Gazette's Monday → Sunday, via `StoryClock::date_of`). Frozen at delivery, as today.
- Text, one line each, empty lines skipped:
  - `Prévisions de la semaine : 18 °C au plus, 2 °C au moins.`
  - `Plus beau jour : jeudi.`
  - `Pluie : lundi, mardi.` — rainy day = precipitation ≥ `RAINY_DAY_MM` (existing 1 mm).
  - `Neige : samedi.` — only if a snow day (snowfall > 0 or snow weather code).
  - `⚠ Alerte orage : vendredi.` — thunderstorm weather code (95, 96, 99).
  - `⚠ Alerte canicule : mardi (36 °C).` — max ≥ 35 °C.
  - `⚠ Alerte grand froid : mercredi (−12 °C).` — min ≤ −10 °C.
  - `⚠ Alerte vent fort : mercredi (rafales 85 km/h).` — gusts ≥ 70 km/h.
  - Several days → `lundi, jeudi`; alert value = the worst of those days.
- **Plus beau jour**: least precipitation, then warmest max, then earliest; never an alert day. All days on alert → no line.
- Alerts are computed from the forecast, not Météo-France vigilances (not provided by Open-Meteo).
- Day names lowercase, French (`lundi` … `dimanche`).
- New strings: non-breaking `\u{202F}` before `:` per wording.md. Existing Gazette strings untouched.
- Gazettes already saved keep their « Ta semaine : … » text. No migration, no `progress.json` field change.
- Not fetched yet (offline): « Les prévisions ne sont pas encore arrivées. » instead of today's « Ta semaine : les relevés ne sont pas encore arrivés. ».
- Lives in `src/remote_weather.rs` (request + text, `WeekRecap` → week forecast with its Monday date), `src/app.rs` (request built with the Gazette's Monday), `src/story/content/gazette.rs` (multi-line text, fallback). No new file.
- Out of scope: real Météo-France vigilances, the rain alert (umbrella) in the bar, the Gazette's other sections.

## Acceptance criteria

- [ ] `cargo fmt --check`, `cargo test` green; `cargo clippy -- -D warnings` green except the pre-existing `double_must_use` (`src/weather/provider.rs:23`).
- [ ] Text test on a fixed week (Mon → Sun): max/min line, `Plus beau jour`, `Pluie : lundi, mardi.`, no `Neige`/alert line when none.
- [ ] Text test: snow day → `Neige : samedi.`; thunderstorm code → `⚠ Alerte orage : vendredi.`; 36 °C → `⚠ Alerte canicule : mardi (36 °C).`; −12 °C → grand froid line; gusts 85 km/h → vent fort line; thresholds exactly at 35 / −10 / 70 trigger, just below don't.
- [ ] Best-day test: driest wins; tie → warmest; tie → earliest; an alert day never wins; all days on alert → no `Plus beau jour` line.
- [ ] Request test: a Gazette of Monday 2026-11-02 asks Open-Meteo for `start_date=2026-11-02&end_date=2026-11-08` with max/min temperature, precipitation, snowfall, weather code, gusts.
- [ ] Old save with `gazette_weather` « Ta semaine : … » → that Gazette still shows it, unchanged.
- [ ] Offline, Gazette not fetched → « Les prévisions ne sont pas encore arrivées. »
- [ ] New strings use `\u{202F}` before `:`.
- [ ] Running `weathr`: this week's Gazette shows the forecast lines inside its Newspaper frame, borders aligned (with ⚠).

## Implementation

## Review

## Verification

## Feedback
