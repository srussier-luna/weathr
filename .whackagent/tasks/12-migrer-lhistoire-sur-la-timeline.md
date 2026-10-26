---
issue: 12
status: to-test
wiki: [[mail]], [[animation]], [[architecture]]
note: "Idée d'origine : séparer temps réel (décor) et temps du jeu (histoire), brancher les Lettres sur une timeline, journal du jour plus tard. Archi composable, tests obligatoires."
created: 2026-10-26
---

## Context / Decisions

> j'aimerais bien qu'on mette en place un vrai système de timeline et de gestion du temps. Deux choses : (1) météo réelle + temps réel → uniquement le décor (couleur, ciel…) ; (2) temps in game → l'histoire (lettres, apparition de personnages/objets/maison). Petit lore. Brancher les Lettres existantes dessus. Plus tard : journal du jour. Archi hyper clean, composable. Tests obligatoires (je ne code pas en Rust).

First task of sprint **Timeline histoire**. Pure refactor: the player sees no difference. Follow-ups: #13 (Apparitions by Jour de jeu), #14 (`--day` flag), #19 (story pauses when 7 Lettres wait). #5 builds on the clock, #4 on #13.

### Two clocks

- **Real time + real weather** → scenery only (sky, colours, seasons, Fêtes). Stays in `AppState` / `AnimationManager`, untouched here.
- **Jour de jeu** → story only. 1 Jour de jeu = 1 real day since `Progress` start, rolling over at `courrier.heure` (today's `mail_date` + `Progress::day_on`). Missed days catch up on return, as today (#19 later caps this).
- Every story event is scheduled by Jour de jeu (Stardew-like: the world changes on its own, the Lettre explains). Reading a Lettre never triggers an event. Only exception: answers `1`/`2` still pick variants (`Choices`).

### Module `src/story/` (group by feature)

```
src/story/
  mod.rs
  clock.rs       StoryClock — start date + courrier.heure → Jour de jeu; date_of(day)
  timeline.rs    Timeline — Vec<Beat>, sorted by (day, kind order); composed from sources
  event.rs       Beat { day, event } · StoryEvent::Mail(MailId)  (#13 adds Arrival)
  sources/       one file per source, each implements StorySource
                 village, sam, neighbours, marcel, archives, creepy, gazette, recipes, postcards
  content/       MailContent implementations + static texts (today letters.rs, chronicles.rs)
```

- `trait StorySource { fn beats(&self, clock: &StoryClock, today: usize) -> Vec<Beat> }`. Timeline = concatenation of registered sources. New mail kind = one source file + one content impl + one registration line.
- **Single Timeline** for mail and Apparitions: a Lettre and the thing it explains are declared side by side, never drift apart. `StoryEvent` holds only `Mail` here (YAGNI); #13 adds `Arrival(Thing)`.
- Calendar rhythms (Gazette every Monday, recipe each 1st of month) use `StoryClock::date_of(day)`, never `Local::now()`. Player-facing behaviour unchanged.
- **When vs what**: the Timeline says *when*; `trait MailContent { fn compose(&self, ctx: &ReadContext) -> View }` says *what*.
  - Static: `Written` (village Lettres incl. `lettres.txt` override, correspondents, archives, creepy, recipes).
  - Dynamic: `Gazette`, `Postcard` — read weather snapshots from `ReadContext`.
  - Night postscript = decorator over `Written`, computed at read time (22h–6h), as today. "Lecture nocturne" Badge unchanged.
  - `ReadContext` = everything dynamic content may read: reading hour, weather snapshots, answers, progress. Content never calls `Local::now()` or the weather client itself.
- Dynamic content is **frozen at delivery**: Gazette and Carte postale weather stay as saved in `progress.json` (`gazette_weather`, `postcard_weather`). Re-reading an old Gazette shows its own week.
- Out of the Timeline on purpose (real-time, no story progress): weather + Fête Badges, night postscript, live Carte postale / Gazette weather fetch.
- `letters.rs` and `chronicles.rs` move under `src/story/` (`content/`). `badges.rs`, `progress.rs` stay where they are.
- Save compatibility: `Progress::letters_read` stays a prefix length into the Timeline. Past-day beats never move. No `progress.json` field renamed or removed.
- `App` keeps orchestration only: builds `StoryClock` + `Timeline`, hands `ReadContext` to views. `unlocks_for` keeps its current read-count logic in this task (#13 switches it).
- Tests: unit tests next to the code (`#[cfg(test)]`, as `letters.rs` does today). Before refactoring, capture the current `letters::timeline` output as a golden fixture and compare after.
- Out of scope: daily newspaper (future source), new story content / lore, Apparitions by day (#13), `--day` (#14).

## Acceptance criteria

- [ ] `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` all green.
- [ ] Golden test: for starts 2026-01-05 (Monday), 2026-03-31, 2026-12-15 and today = start, +6, +40, +120 days, the Timeline lists exactly the same `(day, entry)` pairs, same order, as the pre-refactor `letters::timeline`.
- [ ] Clock test: start 2026-10-01, `courrier.heure` = 9 → 2026-10-08 8:59 is Jour de jeu 7, 2026-10-08 9:00 is Jour de jeu 8.
- [ ] Gazette beats fall on Mondays and recipe beats on the 1st of the month, computed through `StoryClock::date_of` (test).
- [ ] A `progress.json` with `letters_read: 37` opens on the same Lettre and the Courrier shows the same list as before the refactor (test on a fixture save).
- [ ] Night postscript: a village Lettre composed with reading hour 23 has its P.-S.; at hour 14 it doesn't (test).
- [ ] Frozen weather: a Gazette composed after the current weather changed still shows its saved week's weather (test).
- [ ] No file under `src/story/` calls `Local::now()`/`Utc::now()` or the weather client (`grep` clean).
- [ ] Adding a source = one new file in `src/story/sources/` + one registration line; shown by a test-only dummy source added to a Timeline in a test.
- [ ] Running `weathr`: Courrier, a village Lettre, a Gazette and a Carte postale look exactly as before.

## Implementation

Autopilot 2026-10-26, one implementer, 4 bricks.

- `src/story/`: `clock.rs` (`StoryClock`), `event.rs` (`Entry`, `Beat`, `StoryEvent::Mail`), `timeline.rs` (`Timeline::new(sources, clock, today)`, `mail()`, `delivered_by`, `mail_at`).
- `src/story/sources/`: `village`, `correspondents` (Sam, Nadia/Claire, Marcel share one `Correspondent` type), `archives`, `creepy`, `gazettes`, `recipes`, `postcards`. Schedule constants moved here. Gazette/recipe days via `clock.date_of`.
- `src/story/content/`: `MailContent` + `ReadContext` (`letters`, `progress`, `clock`, `reading_hour`, `riddle_input`), `Written`, `VillageLetter`, `NightPostscript` (wraps `VillageLetter`, not `Written`: P.-S. sits before signature + question, `overlay::Line` not Clone), `Gazette`, `Postcard`. `letters.rs`, `chronicles.rs` moved here (`git mv`).
- `views.rs` keeps index, help, album, bottle. `entry_view` + Courrier label moved to `story::content`.
- `progress.rs`: `day_on` removed (now `StoryClock`); save format unchanged.
- Fixtures `src/story/testdata/timeline_golden.txt`, `save_37_golden.txt` captured from pre-refactor code.
- Edges: rollover on local wall-clock (differs only inside DST hour); out-of-range Lettre index draws nothing instead of panicking.
- Proof: `cargo fmt -- --check` ✅ · `cargo clippy -- -D warnings` ✅ · `cargo test` ✅ 165 passed · `grep -rn "Local::now\|Utc::now" src/story` empty.

## Review

## Verification

- Smoke run in a pty harness, `HOME` = temp dir (real save untouched, shasum identical): `--simulate clear` renders, `q` exits 0, no panic.
- Baseline binary (pre-refactor) vs new, same save (`letters_read: 37`), keys `l`, Esc, `c`, 50× →: same overlay titles and text (Courrier, Lettre 38/101, Gazettes with Monday dates, Cartes postales, Archives).
- Manual validation of the real terminal look — yours.

## Feedback
