# Weather

Get weather + location data, turn into flags scene uses.
All network calls go to Open-Meteo (weather), ipinfo.io (IP location), Nominatim
(city names, city search). Every call best-effort: failure never stops app.

## Current weather — `src/weather/`

- `WeatherProvider` trait (`src/weather/provider.rs:22`) returns raw
  `WeatherProviderResponse`. Only impl: `OpenMeteoProvider`
  (`src/weather/open_meteo.rs:13`).
- `WeatherNormalizer::normalize` (`src/weather/normalizer.rs:7`) maps WMO code to
  `WeatherCondition` (`src/weather/types.rs:5`), builds `WeatherData` (`:153`).
- `WeatherClient` (`src/weather/client.rs:11`) adds two cache levels: in-memory
  (`RwLock`), then disk (`cache.rs`), then provider.
- `spawn_weather` (`src/app.rs:242`) loops on client every `REFRESH_INTERVAL` (300 s),
  sends `Result` over `mpsc(1)` channel drained by main loop (see [[architecture]]).
  Started by `App::new`, again on each Ville change (#82, see below): new channel
  replaces old one.
- Old loop stops when its channel closes: sleep raced against `sender.closed()` in
  `select!` → ends at once, not 300 s later. Why: else one live loop per past Ville.
  Rain alert loop same (`src/rain_alert.rs:43`).
- `AppState::update_weather` (`src/app_state.rs:66`) derives `WeatherConditions`
  (`src/weather/types.rs:210`): `is_raining`, `is_snowing`, `is_thunderstorm`,
  `is_cloudy`, `is_overcast`, `is_foggy`, `is_day`. `is_cloudy` (Éclaircies, Nuageux, Couvert) drives cloud drawing; `is_overcast` (Nuageux, Couvert) drives `is_clear_sky()` for shooting stars and constellations. Thunderstorm beats rain. Rain stop
  (not into snow) → 30 min rainbow.

### Units invariant

Requests ask Open-Meteo for user units, then `normalize_*`
(`src/weather/units.rs:92`) converts back so **`WeatherData` always °C, m/s, mm**.
`format_*` converts again only for Barre du haut. Never store user units in `WeatherData`.

- Réglages (#38) change units live: `AppState::set_units` (`src/app_state.rs:147`) sets `weather_info_needs_update` → Barre du haut rebuilt next frame in new units, no refetch ([[render]]).
- Fetch task keeps units it was spawned with: launch units, or current ones after a Ville change. Harmless: requests + `normalize_*` use same units, data stays metric.
- `*_symbol` (`src/weather/units.rs:43`) = unit labels (`°F`, `km/h`, `in`…), shared by `format_*` and Réglages rows.

## Location — `src/geolocation.rs`

`detect_location` (`src/geolocation.rs:34`), at launch when `location.auto`, in game
when « Position auto » → oui:

1. `CoreLocationCLI` (macOS device location, 10 s timeout) if installed.
2. Cached `location.json` (24 h).
3. ipinfo.io, 3 retries + backoff; result cached.

`reverse_geocode` (`:170`) asks Nominatim for city/town/village name when
`location.display` is `city` or `mixed`; cached per coordinates + language.

### Nominatim

- `nominatim_get` (`:198`) = one door for reverse geocode + search: same
  User-Agent `weathr/<version>`, `Accept-Language`, timeouts as `Duration` (5 s,
  connect 3 s).
- Shared ≥ 1 s gap between requests (`wait_for_nominatim`, one static `Instant`):
  each call books next free slot, sleeps till then. Why: Nominatim policy = 1 req/s
  max, repeated `Entrée` broke it (#82 review). Search + reverse geocode share it.
- `search_cities(query, language)` (`:255`): `/search`, `featureType=city`,
  `limit=5`. Pure `places_from` (tested on a Lyon sample) → `Place { label, location }`:
  bad coordinates or empty name skipped, same label twice kept once (Lyon city +
  Lyon municipality), 5 max.
- `place_label` = « Nom, région, pays »; région = `state_district`, else `county`,
  else `state`, dropped when equal to name (« Paris, France »).
- Network, HTTP status or JSON error → `Err` → « Pas de connexion »; empty list →
  « Aucune ville trouvée ». Kept apart: retry later vs other spelling.

### Ville from Réglages — #82

Rows « Ville » + « Position auto »: screen in [[render]], `config.toml` writes in
[[architecture]].

- Network work off main loop: `tokio::spawn` + `oneshot`, polled each frame with
  `try_recv` (`receive_city_search`, `receive_location_detection`) → frame never waits.
- Both wrapped in `settings::send_unless_cancelled` (`src/settings/mod.rs:94`):
  receiver dropped (`Échap` in search box, « Position auto » pressed again, Ville
  chosen meanwhile) → `sender.closed()` wins (`biased`), work future dropped →
  request cancelled, `CoreLocationCLI` killed (`kill_on_drop`). No late answer
  overwrites a newer choice.
- Ville chosen → `move_to` (`src/app.rs:1361`): new weather loop + new rain alert
  task on new coordinates (old umbrella hint cleared), `AppState::set_location`.
  Weather refetched at once, no restart. Simulation / demo → name + position only,
  no fetch (`fetches_weather`).
- « Position auto » → oui: `detect_location_with_city` (`src/geolocation.rs:233`) = `detect_location`,
  then `reverse_geocode` when no name, whatever `display`. Row shows `…` meanwhile.
  Success → saved with `auto = true`, then `move_to`. Fail → toast « Pas de
  connexion », nothing changes.
- « Position auto » → non (or pressed during `…`): detection dropped, current
  position saved with `auto = false`, no refetch: same place.
- Relaunch with `auto = false` → `config.toml` coordinates, no detection → chosen
  Ville stays.

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
  `Request::WeekForecast { week, monday }` gets the Gazette's Monday → Sunday at home (`start_date`/`end_date`, daily max/min, rain + showers, snowfall, weather code, gusts) and turns it into the Gazette text (`describe_week`). Replies land
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
- New Ville + Open-Meteo failing → new name at once, old Ville's weather kept
  (later failures keep last data) until a fetch succeeds.