# Architecture

weathr = single-binary async TUI (tokio + crossterm). Draws ASCII village scene driven by live weather, layers slow-burn story on top: Facteur delivers Lettre each day, plus Cartes postales, Gazette, Bouteilles, Badges.

## Entry and startup — `src/main.rs`

Order matters (`src/main.rs:146`):

1. Panic hook restores terminal (raw mode, alt screen) before printing panic.
2. `clap` parses `Cli` (`src/main.rs:62`); `--completions` and `--update` exit early.
3. `updater::offer_update()` runs **before** raw mode, on plain stdout — see [[updater]].
4. `Config::load()` (`src/config/mod.rs:141`); broken config prints help, falls back to defaults.
5. CLI flags override config; `WEATHR_LATITUDE/LONGITUDE` env vars override file.
6. Optional auto-location + reverse geocoding (see [[weather]]).
7. `TerminalRenderer::new/init` (see [[render]]), then `App::new` and `App::run`, raced against `ctrl_c` in `tokio::select!`.

## The app — `src/app.rs`, `src/app_state.rs`

- `App` (`src/app.rs:176`) owns everything: `AppState`, `AnimationManager`, `WorldScene`, `Progress` save, `StoryClock` + `Timeline` (`src/story/`), open overlay, toasts, async channels.
- `AppState` (`src/app_state.rs:11`) = *weather + HUD* state: current `WeatherData`, derived `WeatherConditions` flags, `Holiday`, rainbow timer, cached status line.
- `Overlay` (`src/app.rs:332`) = one modal at a time: Lettre, Badges, Album, Bouteille, Aide, Courrier (index), Réglages (`Settings(row)`, see below), Nouveautés (release notes, see [[updater]]), Accueil (`Welcome`, see [[mail]]). Drawing in [[render]], content in [[mail]].

## Main loop — `App::run` (`src/app.rs:781`)

One iteration per ~33 ms (`FRAME_DURATION`, 30 fps input poll). Each frame:

1. Advance demo / compute today's `Holiday`, set season (see [[animation]]).
2. `refresh_progress` — streak, timeline rollover, remote weather for postcards/gazette, finds hint, mail flag, badge counts.
3. Drain `weather_receiver` (non-blocking). Error with no data yet → random offline weather + `OFFLINE` in HUD.
4. Draw in strict z-order: `render_background` (sky) → `WorldScene::render` (static scene) → chimney smoke → `render_garden` (animals, lore) → `render_foreground` (rain, snow, fog, leaves) → HUD → overlays. Later layers overwrite earlier cells.
5. `collect_badges`, toasts, rain alert, HUD text cache.
6. `renderer.flush()` diffs against previous frame; then poll one key event.

**Gotcha:** animations frame-counted, not time-based — speeds and waits assume 30 fps loop. Changing `INPUT_POLL_FPS` changes every animation's pace.

## Weather → scene data flow

```
tokio task (REFRESH_INTERVAL 300s) → WeatherClient → mpsc(1) → App::run
  → AppState::update_weather → WeatherConditions (is_raining, is_day…)
  → AnimationManager::update_*_intensity / update_wind
  → every render_* call reads &WeatherConditions
```

`--simulate`, `--night`, `--christmas`, `--fireworks`, `--music`, `--demo` bypass network entirely (`src/app.rs:236`), also disable weather/fête Badges and rain alerts.

## Config and Réglages — `src/config/`, `src/settings.rs`

`config.toml` = **only source** for settings. Read once at startup; Réglages screen (key `s`, #38) writes back changed value only. No settings copy in `progress.json`, one exception: pending Heure du Facteur (see [[mail]]).

- `Settings` (`src/settings.rs:33`) = running values, taken from `App`, not file → launch flags (`--imperial`, `--hide-hud`) shown as current. `ROWS` (`:24`): Température, Vent, Pluie, Heure du Facteur, Barre du haut.
- `Settings::stepped(row, step)` (`:40`) → one `Setting` (`src/config/mod.rs:101`) = next value; choices loop both ways, hour wraps 0–23. `apply` updates snapshot.
- `App::change_setting` (`src/app.rs:1177`): units → `AppState::set_units` (see [[weather]]), Barre du haut → `App.hide_hud`. Both live, no restart. Then `Config::save(setting)` (`src/config/mod.rs:133`). Write fails → kept for session anyway, toast « Réglage non enregistré : vérifie config.toml » (once, see [[render]]).
- Heure du Facteur → `App::change_mail_hour` (`src/app.rs:1193`) instead: clock changes only if `progress.json` **and** `config.toml` both saved, else rollback + same toast. Never under `--day`. Next-day rule in [[mail]].
- Not in Réglages: city (no search by name yet), `silent` (startup output only), Facteur bell (story choice, see [[animation]]).

### Writer — `src/config/writer.rs`

- `write_value(path, keys, value)` (`:8`) edits a `toml_edit::DocumentMut`, no serde round trip: `toml` serializer drops comments + reorders keys. Hand-written file stays as player left it.
- Only target value replaced, its decor (inline comment, spacing) kept. Missing key → inserted; missing table → appended at end (`[courrier]`). Dotted (`courrier.heure = 9`) and inline (`units = { … }`) tables edited in place.
- No file or folder yet → created on first change. Key path through a non-table (`units = "metric"`) → `ConfigError::NotATable`, file untouched.
- `Setting::keys` = TOML path; value through serde `ValueSerializer` → same spelling `Deserialize` reads (`"fahrenheit"`, `"kmh"`, `"inch"`).
- File re-read at each write → hand edits made while game runs survive, but apply only at relaunch.
- `toml_edit 0.24`, not latest: shares `toml_datetime`, `winnow`, `toml_parser`, `toml_writer` with `toml 0.9` → no duplicate crates.

## Module map

| Area | Files | Page |
|---|---|---|
| Static scenery | `src/scene/` | [[scene]] |
| Moving things, lore, seasons, fêtes | `src/animation/`, `animation_manager.rs`, `holiday.rs` | [[animation]] |
| Weather data, location, caches | `src/weather/`, `geolocation.rs`, `cache.rs`, `remote_weather.rs`, `rain_alert.rs` | [[weather]] |
| Terminal output, modal boxes | `src/render/`, `overlay.rs`, `views.rs` | [[render]] |
| Story Timeline, content, progress | `src/story/` (clock, timeline, sources, content), `badges.rs`, `progress.rs` | [[mail]] |
| Self-update, CLI | `updater.rs`, `main.rs` | [[updater]] |
| Config file, Réglages | `src/config/`, `settings.rs` | here; screen in [[render]] |
| Demo cycle | `demo.rs` | [[animation]] |

## Files on disk

Under `dirs::config_dir()/weathr/` (macOS: `~/Library/Application Support/weathr/`):

- `config.toml` — `[location]`, `units`, `hide_hud`, `silent`, `[courrier] heure` (`src/config/mod.rs:25`). Written back by Réglages, comments kept (see above).
- `progress.json` — all player progress (`src/progress.rs:21`), see [[mail]].
- `lettres.txt` — editable Lettre texts, written with defaults on first run.
- `update_check` — date of last update prompt.

Under `dirs::cache_dir()/weathr/`: `weather.json` (5 min), `location.json`, `geocode.json` (24 h). See [[weather]].