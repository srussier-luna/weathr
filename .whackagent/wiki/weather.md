# Weather

Get weather + location data, turn into flags scene uses.
All network calls go to Open-Meteo (weather), ipinfo.io (IP location), Nominatim
(city names). Every call best-effort: failure never stops app.

## Current weather — `src/weather/`

- `WeatherProvider` trait (`src/weather/provider.rs:22`) returns raw
  `WeatherProviderResponse`. Only impl: `OpenMeteoProvider`
  (`src/weather/open_meteo.rs:13`).
- `WeatherNormalizer::normalize` (`src/weather/normalizer.rs:7`) maps WMO code to
  `WeatherCondition` (`src/weather/types.rs:5`), builds `WeatherData` (`:153`).
- `WeatherClient` (`src/weather/client.rs:11`) adds two cache levels: in-memory
  (`RwLock`), then disk (`cache.rs`), then provider.
- `App::new` spawns loop calling client every `REFRESH_INTERVAL` (300 s), sends
  `Result` over `mpsc(1)` channel drained by main loop (see [[architecture]]).
- `AppState::update_weather` (`src/app_state.rs:66`) derives `WeatherConditions`
  (`src/weather/types.rs:210`): `is_raining`, `is_snowing`, `is_thunderstorm`,
  `is_cloudy`, `is_overcast`, `is_foggy`, `is_day`. `is_cloudy` (Éclaircies, Nuageux, Couvert) drives cloud drawing; `is_overcast` (Nuageux, Couvert) drives `is_clear_sky()` for shooting stars and constellations. Thunderstorm beats rain. Rain stop
  (not into snow) → 30 min rainbow.

### Units invariant

Requests ask Open-Meteo for user units, then `normalize_*`
(`src/weather/units.rs:66`) converts back so **`WeatherData` always °C, m/s, mm**.
`format_*` converts again only for HUD. Never store user units in `WeatherData`.

## Location — `src/geolocation.rs`

`detect_location` (`src/geolocation.rs:26`), only when `location.auto`:

1. `CoreLocationCLI` (macOS device location, 10 s timeout) if installed.
2. Cached `location.json` (24 h).
3. ipinfo.io, 3 retries + backoff; result cached.

`reverse_geocode` (`:158`) asks Nominatim for city/town/village name when
`location.display` is `city` or `mixed`; cached per coordinates + language.

## Disk cache — `src/cache.rs`

`dirs::cache_dir()/weathr/`: `weather.json` (300 s, keyed by lat/lon rounded to 2
decimals), `location.json` + `geocode.json` (24 h). Writes fire-and-forget
`tokio::spawn`s; read errors = "no cache".

## Side fetches

- `rain_alert.rs` — own task every 10 min (`src/rain_alert.rs:43`): first 15-min slot
  with ≥0.1 mm in next hour → `RainAlert::At("HH:MM")`, else ≥50 % hourly
  probability → `Soon`. HUD shows umbrella hint, hidden when already wet.
  Disabled in simulation.
- `remote_weather.rs` — one-shot fetches for story content (`src/remote_weather.rs:46`):
  `Request::Postcard(i)` gets current weather in Carte postale's city;
  `Request::WeekRecap(week)` gets past 7 days at home for Gazette. Replies land
  in `Progress`, saved once, so each fetched once. See [[mail]].

## Offline and simulation

- First fetch fails → `generate_offline_weather` (`src/app.rs:77`) picks random
  plausible condition, HUD shows `OFFLINE`. Later failures keep last data.
- `--simulate <condition>` / `--demo` skip client entirely. Condition names parse
  via `FromStr` on `WeatherCondition`.

## Gotchas

- `moon_phase` hard-coded 0.5 — Open-Meteo `current` doesn't provide it.
- Disk cache ignores units; fine only because data normalized.
- Story fetches (`remote_weather`) always metric + French text.