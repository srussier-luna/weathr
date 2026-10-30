---
issue: 43
status: to-test
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
  - No rainy day → `Semaine ensoleillée, sans pluie.` when more than half of the days with a weather code are Clear / PartlyCloudy, else `Semaine nuageuse, sans pluie.` (feedback round 1). No weather code at all → no line.
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
- [ ] Dry-week test: sunny dry week → `Semaine ensoleillée, sans pluie.`; grey → `Semaine nuageuse, sans pluie.`; exactly half sunny → nuageuse; no weather code → no line; any rainy day → `Pluie : …` and no recap line.
- [ ] Best-day test: driest wins; tie → warmest; tie → earliest; an alert day never wins; all days on alert → no `Plus beau jour` line.
- [ ] Request test: a Gazette of Monday 2026-11-02 asks Open-Meteo for `start_date=2026-11-02&end_date=2026-11-08` with max/min temperature, precipitation, snowfall, weather code, gusts.
- [ ] Old save with `gazette_weather` « Ta semaine : … » → that Gazette still shows it, unchanged.
- [ ] Offline, Gazette not fetched → « Les prévisions ne sont pas encore arrivées. »
- [ ] New strings use `\u{202F}` before `:`.
- [ ] Running `weathr`: this week's Gazette shows the forecast lines inside its Newspaper frame, borders aligned (with ⚠).

## Implementation

Autopilot 2026-10-29, one implementer, one brick.

- `src/remote_weather.rs`: `Request::WeekRecap(usize)` → `Request::WeekForecast { week, monday }` (Copy + Ord). `week_forecast` + pure `week_forecast_url` (`start_date={monday}&end_date={monday+6}`, daily max/min temperature, precipitation, snowfall, weather code, gusts). `Daily` + `snowfall_sum`, `weather_code`, `wind_gusts_10m_max`. `describe_week` builds the multi-line text ('\n'-joined, empty lines skipped); helpers `at`, `days_where`, `worst`, `days_line`, `degrees`. Constants `HEAT_WAVE_CELSIUS` 35, `SEVERE_COLD_CELSIUS` −10, `STRONG_GUSTS_KMH` 70. Snow / thunderstorm via `WeatherNormalizer::wmo_code_to_condition` (thunderstorm = 95/96/99).
- `src/app.rs`: request loop passes `monday: self.clock.date_of(day)`; receive matches `WeekForecast { week, .. }`.
- `src/story/content/gazette.rs`: stored text split with `.lines()`, fallback « Les prévisions ne sont pas encore arrivées. ».
- `src/story/content/mod.rs`: `pub const WEEKDAYS` (lundi … dimanche) next to `MONTHS`.
- `src/story/testdata/save_37_golden.txt`: 12 old fallback lines → new fallback; old « Ta semaine : … » entries unchanged.
- Tests (9): calm week, snow, thunderstorm, heat wave / severe cold / strong wind incl. thresholds at and just below, best-day rules, forecast URL for 2026-11-02, Gazette compose before/after fetch.
- Notes: alerts compare raw values while the text shows rounded ones (−9.7 shows « −10 °C au moins » with no grand froid alert; 34.6 shows « 35 °C » with no canicule). Temperatures use U+2212, `f64::round`, −0 → « 0 °C ». A day missing values is never best day nor alert. Failed fetch (e.g. Monday out of API range) stores nothing, fallback stays, no retry in the same session.
- Proof: `cargo build` ✅ · fmt ✅ · clippy ✅ with `--all-targets -A clippy::double_must_use` (pre-existing `src/weather/provider.rs:23`) · `cargo test` ✅ 279 passed.

- Round 1 hunks (feedback): `src/remote_weather.rs` — `skies` / `sunny_days` from `wmo_code_to_condition` (`Clear | PartlyCloudy`), `dry_week` line when no rainy day and at least one code, takes the `Pluie` line's place (`days_line("Pluie", …).or(dry_week)`); test `test_dry_week_recap`.

## Review

## Verification

Runtime, pty 120x40, temp `HOME`, debug binary, save `started_on` 2026-10-19 (week 0 = old text), `c` then `<`, Esc, `q`:
- ✅ Gazette of lundi 26 octobre, fetched live (Utqiagvik): max/min line, « Plus beau jour : lundi. », Pluie, Neige, « ⚠ Alerte vent fort : jeudi (rafales 80 km/h). »
- ✅ Every day on alert (Yakutsk): no « Plus beau jour », long « ⚠ Alerte grand froid : lundi, …, dimanche (−24 °C). » wraps.
- ✅ Newspaper frame borders aligned in all runs, ⚠ lines included (pyte emulator, 1 column per ⚠ / −; real Terminal.app not checked).
- ✅ Old save: lundi 19 octobre still shows « Ta semaine : 2 jours de pluie, maximum 18 °C, minimum 2 °C. »
- ✅ Offline (dead proxy): « Les prévisions ne sont pas encore arrivées. », nothing stored.
- ✅ Stored text: U+202F before every colon.

round 1: pty check by agent — Paris 26 octobre « Semaine nuageuse, sans pluie. », Aswan « Semaine ensoleillée, sans pluie. », borders aligned · rest manual validation · run: yours

## Feedback

### Round 1 — 2026-10-30

- Asked: « si pas de pluie annoncer ajouter une ligne, semaine ensoleiller sans pluie » · « selon le ciel, semaine ensoleillee, ou semaine nuageuse ,sans pluie, un petit recap de la meteo quoi »
- Triage: adjustment → criteria updated (dry-week recap line).
- Changes: implementer — `src/remote_weather.rs` (recap line + test).
- Build: fmt ✅ · clippy `--all-targets -A clippy::double_must_use` ✅ · `cargo test` ✅ 280 passed.
- Review: deferred → /wa-close.
- Verify: pty check by agent (Paris nuageuse, Aswan ensoleillée, borders aligned).
- Notes: Open-Meteo's daily weather code is the day's worst (often 3 = overcast), so real dry weeks will mostly read « nuageuse ». An unknown code maps to `Clear` in the reused normalizer.
- Rule captured: none.

