# Architecture

weathr = single-binary async TUI (tokio + crossterm). Draws ASCII village scene driven by live weather, layers slow-burn story on top: Facteur delivers Lettre each day, plus Cartes postales, Gazette, Bouteilles, Badges.

## Entry and startup — `src/main.rs`

Order matters (`src/main.rs:146`):

1. Panic hook restores terminal (raw mode, alt screen) before printing panic.
2. `clap` parses `Cli` (`src/main.rs:62`); `--completions` and `--update` exit early.
3. `updater::offer_update()` runs **before** raw mode, on plain stdout — see [[updater]].
4. `Config::load()` (`src/config.rs:98`); broken config prints help, falls back to defaults.
5. CLI flags override config; `WEATHR_LATITUDE/LONGITUDE` env vars override file.
6. Optional auto-location + reverse geocoding (see [[weather]]).
7. `TerminalRenderer::new/init` (see [[render]]), then `App::new` and `App::run`, raced against `ctrl_c` in `tokio::select!`.

## The app — `src/app.rs`, `src/app_state.rs`

- `App` (`src/app.rs:176`) owns everything: `AppState`, `AnimationManager`, `WorldScene`, `Progress` save, Lettre timeline, open overlay, toasts, async channels.
- `AppState` (`src/app_state.rs:11`) = *weather + HUD* state: current `WeatherData`, derived `WeatherConditions` flags, `Holiday`, rainbow timer, cached status line.
- `Overlay` (`src/app.rs:167`) = one modal at a time: Lettre, Badges, Album, Bouteille, Aide, Courrier (index). Drawing in [[render]], content in [[mail]].

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

`--simulate`, `--night`, `--christmas`, `--fireworks`, `--demo` bypass network entirely (`src/app.rs:236`), also disable weather/fête Badges and rain alerts.

## Module map

| Area | Files | Page |
|---|---|---|
| Static scenery | `src/scene/` | [[scene]] |
| Moving things, lore, seasons, fêtes | `src/animation/`, `animation_manager.rs`, `holiday.rs` | [[animation]] |
| Weather data, location, caches | `src/weather/`, `geolocation.rs`, `cache.rs`, `remote_weather.rs`, `rain_alert.rs` | [[weather]] |
| Terminal output, modal boxes | `src/render/`, `overlay.rs`, `views.rs` | [[render]] |
| Story content, progress | `letters.rs`, `chronicles.rs`, `badges.rs`, `progress.rs` | [[mail]] |
| Self-update, CLI | `updater.rs`, `main.rs` | [[updater]] |
| Demo cycle | `demo.rs` | [[animation]] |

## Files on disk

Under `dirs::config_dir()/weathr/` (macOS: `~/Library/Application Support/weathr/`):

- `config.toml` — `[location]`, `units`, `hide_hud`, `silent`, `[courrier] heure` (`src/config.rs:22`).
- `progress.json` — all player progress (`src/progress.rs:10`), see [[mail]].
- `lettres.txt` — editable Lettre texts, written with defaults on first run.
- `update_check` — date of last update prompt.

Under `dirs::cache_dir()/weathr/`: `weather.json` (5 min), `location.json`, `geocode.json` (24 h). See [[weather]].