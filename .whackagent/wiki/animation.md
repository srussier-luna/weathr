# Animation

All moving things: sky (sun, moon, stars, clouds, birds, planes), precipitation, garden animals, story props (lore), seasonal + Fête effects. Static scenery in [[scene]].

## Shape

- `AnimationManager` (`src/animation_manager.rs:69`) owns one instance per system, exposes four layered entry points called from `App::run` (see [[architecture]]):
  - `render_background` (`:225`) — stars, moon, fireflies, fireworks, birds, sun, rainbow, clouds, planes, cloud bank.
  - `render_chimney_smoke` (`:359`) — wind bends it, cloud bank caps it.
  - `render_garden` (`:389`) — snow cover, spring, Halloween, lore, creepy, finds, owl, cat, dog, duck, frog, snail, snowman, squirrel, Facteur, kid, butterflies, New Year banner.
  - `render_foreground` (`:484`) — rain/thunder (screen flash), snow, fog, leaves, gusts.
- Each system = small struct with `update(...)` + `render(...)`, reads `&WeatherConditions`, usually `GardenLayout` (`src/animation/garden.rs:9`). `garden::draw` / `draw_solid` / `draw_clipped` = shared sprite helpers.
- Inputs pushed from app: `update_rain/snow/fog_intensity`, `update_wind`, `set_season`, `set_unlocks`, `set_mail_hour`, `set_mail_waiting`, `set_finds_remaining`.
- Outputs read back: `sightings()` (shooting star, Facteur delivering, snowman → Badges), `visible_finds()` and `take_object/take_bottle` (key `r`).

## Shooting stars — `src/animation/stars.rs`

One shooting star at a time at night (chance 0.005 per frame when the sky is empty). During a meteor shower (`set_meteor_shower`, fed by `Holiday.is_perseids`, 9–15 August) the sky keeps up to 16 at once, chance 0.3 per free slot, full in ~10 frames; the shower end trims back to one. Demo has a `Perséides` scene.

## Sun path

`sun_path::day_progress` (`src/animation/sun_path.rs:6`) computes sunrise/sunset from latitude, longitude, date. Sun travels arc across screen, turns orange at twilight. Polar day/night → `None` → centred fallback sprite.

## Lore and unlocks — `src/animation/lore.rs`

- `Unlocks` (`src/animation/lore.rs:72`) = flat set of booleans derived **only** from count of Lettres read per kind + player answers: `Unlocks::after_reading` (`:100`). E.g. bench after 4 village Lettres, well after 13, finds after 45, Bouteilles after 3 archives, creepy events after N strange letters.
- `Choices` (`:38`) maps letter answers (keys `1`/`2`) to visual variants: bow colour, postman bell, owl silent, ducklings count…
- `App` recomputes unlocks when reading progress or answers change (`unlocks_for`, `src/app.rs:55`). Never store unlocks; always derive.
- `Lore` draws gifted props (bench, well, treehouse, chest, balloon, bunting…). `Creepy` handles n°13 neighbour: wet footprints, silhouette, whispering well, "staring" state freezing cat, dog, owl.

## Seasons and Fêtes — `src/holiday.rs`

- `Season::on` (`src/holiday.rs:16`) uses astronomical boundaries (20–22 of month).
- `Holiday` (`src/holiday.rs:27`) = bag of flags computed daily by `Holiday::today`: Christmas (24–25 Dec), 14 juillet (13–14 Jul), Halloween (all October), New Year (±7 days), Easter (±7 days, computed), spring, winter.
- CLI flags + demo inject fixed `Holiday` values (`christmas()`, `national_day()`…).
- Consumers: fireworks (national day, New Year, night only), pumpkins/spirits, spring flowers + lilies, Easter eggs, autumn falling leaves, seasonal constellations, gust leaf colours, fir tree. Fêtes also award Badges (see [[mail]]).

## Facteur — `src/animation/postman.rs`

Rides in to **arrive at mailbox exactly at `courrier.heure`**: departure time back-computed from distance + speed (`due_round`). Shows mailbox flag when Lettre unread. Rings bell if player chose.

## Demo — `src/demo.rs`

`--demo` cycles every weather condition by day and night, then Fête scenes, 1 s each, label in HUD.

## Gotchas

- Speeds/waits in **frames** (30 fps), e.g. `Finds` waits 18 000–54 000 frames.
- `update_wind` arg named `speed_kmh`, but live data normalized to m/s (see [[weather]]). Check units before tuning wind effects.
- Background systems each use own `!is_raining && !is_snowing…` guards; no central "what is visible" table. New sky system → add same guards.