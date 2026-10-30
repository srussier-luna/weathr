# Mail and story

Village story: one Lettre/day, delivered by Facteur, plus Cartes postales, Gazette with Énigme, Bouteilles on river, found objects, Badges. Reading drives what appears in garden (see [[animation]]).

## Timeline — `src/story/`

Two clocks: real time + real weather drive scenery only ([[animation]]); **Jour de jeu** drives the story. Nothing under `src/story/` reads `Local::now()` or the weather client.

- `StoryClock` (`src/story/clock.rs:4`) = `Progress` start date + `courrier.heure`. `day_on(now)` → Jour de jeu (1 = start day, rolls over at mail hour, never below 1); `date_of(day)` → calendar date. New day's mail appears at that hour, synced with Facteur arrival.
- `--day N` (`StoryClock::frozen`, `src/story/clock.rs:20`): start = today − (N−1), `day_on` always N. Game on `Progress::in_memory` (`save()` no-op, real save never loaded), `letters::read()` (never writes `lettres.txt`), `Launch::Suppressed`, Badges never awarded, Facteur never rides. Barre du haut shows `Jour N` like any day.
- `Entry` (`src/story/event.rs:2`) = every mail kind: `Main` (village Lettre), `Sam`, `Neighbours`, `Marcel` (recurring correspondents), `Archive`, `Gazette`, `Recipe`, `Postcard`, `Creepy` (n°13 neighbour). `Beat { day, event }`, `StoryEvent::Mail(Entry)` or `StoryEvent::Arrival(Thing)` (Apparition, sorts after mail on the same day).
- Sources (`src/story/sources/`): one file per mail kind, each `impl StorySource { fn beats(clock, today) }`. `sources::registered` (`mod.rs:16`) lists them. **New mail kind = one source file + one registration line** (+ content). Sources declare Apparitions next to the Lettre that explains them (`village.rs` `ARRIVALS`, creepy, archives Bouteilles). Gazette every Monday, recipe each 1st of month — via `clock.date_of`. Carte postale every 7 days from day 40.
- `Timeline::new(sources, clock, today)` (`src/story/timeline.rs:10`) concatenates beats, sorts by `(day, kind order)`, keeps `mail: Vec<(usize, Entry)>` and `arrivals: Vec<(usize, Thing)>`. `first_mail(n)` borrows the read prefix; `delivered_by(day)`, `mail_at(index)`, `arrived_by(day)`.
- `Progress::letters_read` = **prefix length** into the Timeline. Works because past-day beats never move as days advance. No inserting beats on past days.
- `App` builds `StoryClock` + `Timeline` (`build_timeline`, `src/app.rs:191`), rebuilt on day rollover. Unlocks follow the Jour de jeu: `unlocks_on` reads `Timeline::arrived_by(day)` ([[animation]]).

### When vs what — `src/story/content/`

- Timeline says **when**; `trait MailContent { fn compose(&ReadContext) -> View }` says **what** (`content/mod.rs:52`). `entry_view` picks the content per `Entry`; `label` gives the Courrier line.
- `ReadContext` (`content/mod.rs:44`) = everything content may read: Lettres, `Progress`, clock, reading hour, riddle input. Content never fetches.
- Static: `Written` (correspondents, archives, creepy, recipes), `VillageLetter`. Dynamic: `Gazette`, `Postcard` read weather **frozen at delivery** from `progress.json` (`gazette_weather`, `postcard_weather`): an old Gazette keeps its own week.
- Village Lettre texts in `letters.rs` (`DEFAULT_LETTERS`), overridable by `<config>/weathr/lettres.txt` (`load`, `:482`): `=== N | Sender` blocks replace letter N or append N+1.
- `CHOICES` (`letters.rs:314`): some Lettres ask question (keys `1`/`2`); later Lettre echoes answer (`echo_for`). Answers also feed `Choices` in lore.
- `NightPostscript` (`night_postscript.rs`) wraps `VillageLetter`: opened at night (22h–6h) → creepy `NIGHT_POSTSCRIPTS` line before signature, computed at read time. "Lecture nocturne" Badge unchanged.
- `MONTHS` (`content/mod.rs`) = French month names, shared with Nouveautés.
- Tests: golden fixtures in `src/story/testdata/` captured from pre-refactor code (`timeline_golden.txt`, `save_37_golden.txt`). Changing story order or texts → recapture them on purpose.

## Content — `src/story/content/chronicles.rs`

Static French content: correspondents' letters + days, `CITIES` (40 Cartes postales with coordinates), `ARCHIVES`, `RUMORS`, `ADS`, `RIDDLES` (30), `RECIPES`, `DRAWINGS`, `BOTTLE_MESSAGES` (20), `OBJECTS` (25), `CHEST_SOUVENIRS`.
`is_right_answer` (`:731`) folds case, accents, punctuation, leading article.

- **Carte postale**: from Sam travelling; `remote_weather` adds today's weather there.
- **Gazette**: weekly — home weather of its Monday → Sunday (past days real, rest forecast), rumour, ad, week's Énigme, last week's answer. Weather lines: max/min, Plus beau jour (sunny day, else snow day, else least rain; then warmest, earliest; never an alert day), Pluie (rain + showers ≥ 1 mm, snow excluded) or « Semaine ensoleillée / nuageuse, sans pluie. », Neige, ⚠ Alerte météo (orage, canicule ≥ 35 °C, grand froid ≤ −10 °C, vent fort ≥ 70 km/h; raw values vs rounded text). Old saves keep « Ta semaine : … ». Not fetched → « Les prévisions ne sont pas encore arrivées. ». Key `e` opens text input; solved weeks go in `riddles_solved`.
- **Bouteille**: drifts down river once 3 archives read; `r` picks up + opens. **Objet**: appears in garden by day once unlocked.

## Badges — `src/badges.rs`

`BADGES` (`src/badges.rs:42`): 39 entries, five `BadgeKind`s — Météo, Fêtes, Rares, Fidélité, Secrets. Awarding runs every frame in `App::collect_badges` (`src/app.rs:341`): milestones (streak, letters, riddles, objects, hours) always; weather, Fête, sightings only when **not** simulated and not offline. Shooting stars go through `StarWatch`: during a meteor shower they award `Perséides` or `Géminides` and never count for `Étoile filante` / `Chasseur d'étoiles`. `award` inserts ids into `progress.badges`, queues a Badge card (see [[render]]).

## Progress — `src/progress.rs`

`Progress` (`src/progress.rs:10`) = whole save, JSON in `<config>/weathr/progress.json`: start date, `letters_read`, badges, answers, streak, riddles, found objects, bottles, cached postcard/gazette weather, `notes_seen_on` (release notes, see [[updater]]). All fields `#[serde(default)]` so old saves keep loading — keep so when adding fields. Saved eagerly after each change; write errors ignored.

## Accueil — `src/welcome.rs`

Once per player, fresh **and** existing saves: `welcome_seen` in `progress.json` (`#[serde(default)]`, false for old saves).

- Shown on `Launch::Fresh` / `Normal` when not seen; `Suppressed` (`--silent`, `--demo`, forced modes on an existing save) → nothing, not marked. Fresh launch in those modes still shows it.
- `Overlay::Welcome(Step)`: `Arriving` (Facteur rides in, see [[animation]]) → `Greeting` (arrival bubble) → `Unfolding` (note grows) → `Reading` (« Bienvenue au village », keys, scroll `↑`/`↓`) → `Farewell` (departure bubble). `Entrée` next, `Échap` ends at once.
- Before `Farewell`: mailbox flag and HUD « Lettre ! [c] » hidden, story keys ignored, Badge cards wait. `q` quits, not marked → replays next launch.
- Marked + saved at the last `Entrée` or `Échap`; then Nouveautés box if pending, then Badge cards.
- Not a timeline entry, not in the Courrier, no replay key.

## Screens and keys

`c` next Lettre (Lettre) · `←/→` browse · `l` Courrier · `a` Album · `b` Badges ·
`h` Aide · `r` pick up · `e` Énigme · `1`/`2` answer · `Esc` close.
Rendering: [[render]]. Overlay state: `Overlay` enum in `src/app.rs:167`.

## Gotchas

- Indices like `CITIES[i % len]` and `RIDDLES[week % len]` wrap: content repeats when exhausted, no panic.
- Apparitions arrive on their **Jour de jeu**, read or not: a Lettre explains what is already in the garden. `--day N` shows the garden of day N.