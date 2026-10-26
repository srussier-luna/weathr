# Mail and story

Village story: one Lettre/day, delivered by Facteur, plus Cartes postales, Gazette with Énigme, Bouteilles on river, found objects, Badges. Reading drives what appears in garden (see [[animation]]).

## Timeline — `src/letters.rs`

- `Entry` (`src/letters.rs:297`) = every mail kind: `Main` (village Lettre),
  `Sam`, `Neighbours`, `Marcel` (recurring correspondents), `Archive`, `Gazette`,
  `Recipe`, `Postcard`, `Creepy` (n°13 neighbour).
- `timeline(main_count, start, today)` (`:333`) lays out `(day, Entry)` from player start date: one `Main`/day, others on fixed day schedules (`chronicles.rs` constants), Gazette every Monday, recipe each 1st of month, Carte postale every 7 days from day 40. Sorted by `(day, kind order)`.
- **Mail day** = local time minus `courrier.heure` hours (`mail_date`, `src/app.rs:68`). New day's mail appears at that hour, synced with Facteur arrival.
- `Progress::letters_read` = **prefix length** into timeline. Works because past-day entries never move as days advance. No inserting entries on past days.
- Village Lettre texts from `DEFAULT_LETTERS`, overridable by `<config>/weathr/lettres.txt` (`load`, `:601`): `=== N | Sender` blocks replace letter N or append N+1.
- `CHOICES` (`:443`): some Lettres ask question (keys `1`/`2`); later Lettre echoes answer (`echo_for`). Answers also feed `Choices` in lore.
- Village Lettre opened at night (22h–6h) gets creepy `NIGHT_POSTSCRIPTS` line before signature, awards secret "Lecture nocturne" Badge.

## Content — `src/chronicles.rs`

Static French content: correspondents' letters + days, `CITIES` (40 Cartes postales with coordinates), `ARCHIVES`, `RUMORS`, `ADS`, `RIDDLES` (30), `RECIPES`, `DRAWINGS`, `BOTTLE_MESSAGES` (20), `OBJECTS` (25), `CHEST_SOUVENIRS`.
`is_right_answer` (`:740`) folds case, accents, punctuation, leading article.

- **Carte postale**: from Sam travelling; `remote_weather` adds today's weather there.
- **Gazette**: weekly recap — home weather, rumour, ad, week's Énigme, last week's answer. Key `e` opens text input; solved weeks go in `riddles_solved`.
- **Bouteille**: drifts down river once 3 archives read; `r` picks up + opens. **Objet**: appears in garden by day once unlocked.

## Badges — `src/badges.rs`

`BADGES` (`src/badges.rs:42`): 38 entries, five `BadgeKind`s — Météo, Fêtes, Rares, Fidélité, Secrets. Awarding runs every frame in `App::collect_badges` (`src/app.rs:341`): milestones (streak, letters, riddles, objects, hours) always; weather, Fête, sightings only when **not** simulated and not offline. Shooting stars go through `StarWatch`: during the Perséides they award `Perséides` and never count for `Étoile filante` / `Chasseur d'étoiles`. `award` inserts ids into `progress.badges`, queues a Badge card (see [[render]]).

## Progress — `src/progress.rs`

`Progress` (`src/progress.rs:10`) = whole save, JSON in `<config>/weathr/progress.json`: start date, `letters_read`, badges, answers, streak, riddles, found objects, bottles, cached postcard/gazette weather. All fields `#[serde(default)]` so old saves keep loading — keep so when adding fields. Saved eagerly after each change; write errors ignored.

## Screens and keys

`c` next Lettre (Lettre) · `←/→` browse · `l` Courrier · `a` Album · `b` Badges ·
`h` Aide · `r` pick up · `e` Énigme · `1`/`2` answer · `Esc` close.
Rendering: [[render]]. Overlay state: `Overlay` enum in `src/app.rs:167`.

## Gotchas

- Indices like `CITIES[i % len]` and `RIDDLES[week % len]` wrap: content repeats when exhausted, no panic.
- `Unlocks` thresholds count read **village** Lettres, not all entries.