# Animation

All moving things: sky (sun, moon, stars, clouds, birds, planes), precipitation, garden animals, story props (lore), seasonal + Fête effects. Static scenery in [[scene]].

## Shape

- `AnimationManager` (`src/animation_manager.rs:69`) owns one instance per system, exposes four layered entry points called from `App::run` (see [[architecture]]):
  - `render_background` (`:225`) — stars, moon, fireflies, fireworks, birds, sun, rainbow, clouds, planes, cloud bank, Notes du ciel.
  - `render_chimney_smoke` (`:359`) — wind bends it, cloud bank caps it.
  - `render_garden` (`:389`) — snow cover, spring, Halloween, lore, creepy, finds, owl, cat, dog, duck, frog, snail, snowman, squirrel, Facteur, kid, music notes, butterflies, New Year banner.
  - `render_foreground` (`:484`) — rain/thunder (screen flash), snow, fog, leaves, gusts.
- Each system = small struct with `update(...)` + `render(...)`, reads `&WeatherConditions`, usually `GardenLayout` (`src/animation/garden.rs:9`). `garden::draw` / `draw_solid` / `draw_clipped` = shared sprite helpers.
- Inputs pushed from app: `update_rain/snow/fog_intensity`, `update_wind`, `set_season`, `set_unlocks`, `set_mail_hour`, `set_mail_waiting`, `set_finds_remaining`.
- Outputs read back: `sightings()` (shooting star, Facteur delivering, snowman → Badges), `visible_finds()` and `take_object/take_bottle` (key `r`).

## Shooting stars — `src/animation/stars.rs`

`StarSystem::set_sky(ShootingStarSky)` picks one of three skies each night frame (`animation_manager.rs`):
- `Covered` — `!WeatherConditions::is_clear_sky()` (rain, storm, snow, fog, Nuageux, Couvert): no shooting star at all, the ones in flight dropped. Éclaircies count as a clear sky.
- `Clear` — one at a time, chance 0.005 per frame when the sky is empty.
- `MeteorShower` — `Holiday.meteor_shower` is `Some` (Perséides 9–15 Aug, Géminides 10–16 Dec): up to 16 at once, chance 0.3 per free slot, sky full in ~10 frames; going back trims the list.
Demo has `Perséides` and `Géminides` scenes. `StarGazing` (constellations) uses the same `is_clear_sky()`.

## Sun path

`sun_path::day_progress` (`src/animation/sun_path.rs:8`) computes sunrise/sunset from latitude, longitude, date. Sun travels arc across screen, turns orange at twilight. Polar day/night → `None` → centred fallback sprite. `on_sun_path` (`:15`) = the arc itself, for any sprite width and lowest row: x within `SUN_MARGIN` of edges, y from `CloudBank::TOP` down to lowest. Sun and Notes du ciel share it.

## Lore and unlocks — `src/animation/lore.rs`

- `Unlocks` (`src/animation/lore.rs:72`) = flat set of booleans, one per Apparition (`Thing`, `src/story/event.rs:31`) + `choices`. Set from the Timeline's Arrival beats on today's **Jour de jeu**, never from reading: bench on day 4, well on day 13, finds on day 45, Bouteilles with archive 3, creepy events on the n°13 neighbour's Lettre days. See [[mail]].
- `Choices` (`:38`) maps letter answers (keys `1`/`2`) to visual variants: bow colour, postman bell, owl silent, ducklings count…
- `App` derives unlocks with `unlocks_on(timeline, day, answers)` (`src/app.rs:53`): on Jour de jeu rollover (`refresh_timeline`) and when an answer changes. Answers only drive `Choices`. Never store unlocks; always derive.
- `Lore` draws gifted props (bench, well, treehouse, chest, balloon, bunting…). `Creepy` handles n°13 neighbour: wet footprints, silhouette, whispering well, "staring" state freezing cat, dog, owl.
- Puits sits left of Arbre's foliage: `GardenLayout::well_x` (`src/animation/garden.rs`) = `tree.x - (1 + WELL_WIDTH)`, `None` without room. Hidden while unlocked → dark grey `‹ dézoom` at column 0, 4 rows above Arbre top. `Creepy` footprints + whisper read same `well_x`. `WELL_WIDTH` tied to `WELL` drawing by compile-time assert. Props drawn `draw_solid` so mountains don't show through.
- Coffre sits 5 columns left of Puits: `GardenLayout::chest_x` = `well_x - (CHEST_GAP + CHEST_WIDTH)`, so order left → right Coffre · Puits · Arbre (Puits arrives first in story). Red `X` (`x_mark`) marks Coffre spot before it's unlocked. One `‹ dézoom` for both. `CHEST_WIDTH` tied to `CHEST` by compile-time assert.
- Cabane (16 × 4) centred on Arbre foliage by `treehouse_spot` (`src/animation/lore.rs`): highest row where all 4 rows fit inside round outline (`Tree::canopy_span`); foliage < 18 columns → no Cabane, no ladder. Winter fir is triangular: Cabane overlaps its narrow top (accepted).

## Seasons and Fêtes — `src/holiday.rs`

- `Season::on` (`src/holiday.rs:16`) uses astronomical boundaries (20–22 of month).
- `Holiday` (`src/holiday.rs:27`) = bag of flags computed daily by `Holiday::today`: Christmas (24–25 Dec), 14 juillet (13–14 Jul), Halloween (all October), New Year (±7 days), Easter (±7 days, computed), Fête de la musique (21 Jun, `is_music_day`, see below), spring, winter.
- 6 January (`Holiday::is_epiphany`): golden paper crown at the roof peak (`src/animation/epiphany.rs`), drawn in the garden layer after snow cover; sits on the weather vane's mast.
- CLI flags + demo inject fixed `Holiday` values (`christmas()`, `national_day()`, `music_day()`…).
- Consumers: fireworks (national day, New Year, night only), pumpkins/spirits, spring flowers + lilies, Easter eggs, autumn falling leaves, seasonal constellations, gust leaf colours, fir tree. Fêtes also award Badges (see [[mail]]).

## Fête de la musique — `src/animation/music.rs`

21 June only, all day, day and night, **any weather**: music plays indoors (#17). Off any other day: notes cleared, usual sun and moon. Preview: `--music` (any date), demo scene « Fête de la musique ». Badge « Fête de la musique » in [[mail]].

- **Window notes** (`MusicNotes`): ♪ ♫ rise from a random window, wobble sideways, several at once (16 max). Windows read from the sprite on screen (`House::windows`, see [[scene]]) → small and big house both play.
- Each note carries its own vanishing row. Lower windows: just under the eave (`House::eave_row`). Windows touching the eave would vanish after one frame → rise `RISE_ROWS` (4) rows above instead (`rise_top`), like heads.
- **Singers**: Habitants visibles (`cat`, `dog`, `owl`, `duck`, `frog`, `snail`, `squirrel`), Léo, Facteur while riding (`Postman::rider_head`, not the Accueil `head`). Now and then a note above the head, rises `RISE_ROWS`. Cat and dog sing even sheltered (big window, doghouse); other `head` accessors → `None` when hidden. Kittens, ducklings, owl's friend stay silent.
- Notes drawn in `render_garden` after Léo → over Habitants. Same `PALETTE` for windows, singers, sky: pink, gold, cyan, light green.
- **Notes du ciel** (`SkyNotes`, 12 × 27): croche + two doubles croches, straight beams, heads aligned (straight version chosen over diagonal, #17). Replaces sun by day, on sun's course (`on_sun_path`, own real size); no course (polar) → centred at top. Replaces moon by night at `MoonSystem::position()`, clamped to screen. Sun / moon not drawn.
- Drawn after cloud bank, all weathers, `garden::draw` → see-through; rain still falls in front (foreground layer). Colour steps through `PALETTE` every `COLOR_STEP_FRAMES` = 90 frames (~3 s).
- Sky shorter than 12 rows (`SkyNotes::shows`: horizon < `CloudBank::TOP` + 12, e.g. 20-row terminal, the game's minimum) → usual sun / moon.

## Facteur — `src/animation/postman.rs`

Rides in to **arrive at mailbox exactly at `courrier.heure`**: departure time back-computed from distance + speed (`due_round`). Shows mailbox flag when Lettre unread. Rings bell if player chose. 1 April (`Holiday::is_april_fools`): pink paper fish `><>` on his back while out, and on the mailbox all day (hidden while he stands in front).

Accueil visit (see [[mail]]): `visit` → `Arriving` (2.5 s ride) → `Staying` at mailbox, no labels, daily round can't start. `leave` rides off when the Accueil ends (last `Entrée` or `Échap`); still off-screen → just stays hidden. `head` = bubble anchor. No mailbox (narrow terminal) → never visible.

## Demo — `src/demo.rs`

`--demo` cycles every weather condition by day and night, then Fête scenes, 1 s each, label in HUD.

## Gotchas

- Speeds/waits in **frames** (30 fps), e.g. `Finds` waits 18 000–54 000 frames.
- Animals keeping absolute `x`/`y` across frames must re-place on resize: layout moves, they don't. `Squirrel` stores `placed_on: Option<(width, horizon_y)>` and resets at Arbre foot when it changes.
- `update_wind` arg named `speed_kmh`, but live data normalized to m/s (see [[weather]]). Check units before tuning wind effects.
- Background systems each use own `!is_raining && !is_snowing…` guards; no central "what is visible" table. New sky system → add same guards. Notes du ciel skip them on purpose (all weathers).