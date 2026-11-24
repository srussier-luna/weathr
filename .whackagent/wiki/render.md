# Render

How cells reach terminal, how modal screens (Courrier, Lettre, Album, Aide, Badges, Bouteille, Réglages) drawn on top of scene.

## TerminalRenderer — `src/render/mod.rs`

- `TerminalRenderer` (`src/render/mod.rs:30`) keeps two `Vec<Cell>` buffers (char + colour): frame being drawn + last flushed frame.
- Per frame: `clear()` blanks buffer → layers call `render_char`, `render_line_colored`, `render_centered_colored` → `flush()` (`:196`) writes **only changed cells**, batching cursor moves + colour changes. Diffing keeps 30 fps full-screen redraw cheap.
- Offscreen writes silently dropped → sprites can be partly offscreen.
- `char_at` (`:179`) reads back current buffer; snow cover + spring flowers use it to land only on free cells. Depends on draw order.
- `flash_screen` recolours every cell white (lightning).
- `manual_resize` reallocates both buffers, clears screen; next flush redraws all.
- Needs TTY + min 70×20, else `TerminalError` with friendly message.
- `Drop` + panic hook in `main.rs` both restore terminal.

## Colour capabilities — `src/render/capabilities.rs`

`TerminalCapabilities::detect` reads `NO_COLOR`, `TERM`, `COLORTERM`.
`adjust_color` downgrades: no colour → `Reset`; basic → RGB becomes white. All writes go through it → code use RGB freely.

## Overlays — `src/overlay.rs`

- `View { title, lines, footer, frame }` = single data shape for every modal. `Line` is `Text` (word-wrapped), `Preformatted` (truncated), `Centered`, `Rule` or `Blank`.
- `Frame` = one look per window, so each kind reads at a glance: parchment (Lettre, `Nouvel habitant`), wet paper (Lettre glissée sous la porte, `Frame::Wet`: light blue, `~` stain top-left, bottom-right corner cut over 4 rows, drops under the box), newspaper (Gazette), stamp (Carte postale), folder tab (Archives), punched card (Recette), waves (Bouteille), photo corners (Album), stars (Badges + Badge card), plain (Aide, Courrier, Réglages). Each frame has its own color.
- `render_view` draws the frame centred, max 66 columns; `frame_rows` is the pure layout part (tested). Footers or titles wider than the frame wrap between key groups. A single word or key group wider than the text area is never split and spills onto the right edge (boxes under 23 columns, terminals under 27).
- `render_badges` = only special layout (columns grouped by `BadgeKind`, secret badges show `???`).
- New Badge → centred card (`Overlay::NewBadge`): medal with the Badge's own 3×5 drawing (`Badge.icon`), name, condition for secrets. Queue `pending_badges`, one card per Badge, opens only when no window is open; a card replaced by another window comes back first.
- Toasts (found object, riddle result, unsaved Réglage, failed « Position auto » detection) one at a time (#162, `src/hud/toast.rs`): centred in a plain `+---+` frame, bottom just above the Barre du bas, yellow text typed 30 ms a letter in a frame already at its final width, then 4 s and the next one. Long text wraps (`overlay::wrap`) and the frame grows upward. Drawn by `App::render_overlays` even with the Barres masked, under any open window. `App.toast_started` times it, `toast::is_over` pops it. Réglages toasts go through `push_toast` (`src/app.rs:379`): same text already queued → skipped, so failing saves give one « Réglage non enregistré : vérifie config.toml » (`toast_unsaved_setting`), not one per key; same for « Pas de connexion ».
- Speech bubble (`Frame::Bubble`, `render_bubble`): Facteur's Accueil lines, ≤ 40 cols, tail toward his `head`, clamped to screen edges; no mailbox → centred near the bottom, no tail. Layout shared with `render_view` via `lay_out` / `draw_rows`.
- `wrap` splits on ASCII whitespace only: U+202F before `: ; ! ?` stays glued, French punctuation never starts a line.

## Views — `src/views.rs`

Pure functions: story data → `View`. No I/O, no state.

| Screen | Function | Notes |
|---|---|---|
| Lettre | `entry_view` (`src/views.rs:56`) | dispatch on `Entry` kind; footer changes with Choice / Énigme / input |
| Courrier | `index_view` (`:114`) | scrolling list of read entries, date + sender |
| Album | `album_pages` | summary, gifts, answers, objects, chest, Bouteilles, Cartes postales, recipes, drawings |
| Aide | `help_view` (`:292`) | keys shown only once unlocked (`[e]`, `[r]`); `[s] réglages` always |
| Bouteille | `bottle_view` (`:284`) | shown on pickup |
| Réglages | `settings_view` (`:201`) | key `s`; one `Preformatted` row per `settings::ROWS`, selected one `> label < value >`. `↑`/`↓` row, `←`/`→` or `Entrée` change, `s`/`Échap` close. « Nouvelle heure du Facteur dès demain. » while new hour waits ([[mail]]). « Ville » = name, else `lat, lon` (2 decimals); change key → search box below. « Position auto » = `non` / `…` (detecting) / `oui` from `AutoLocation { Off, Locating, On }` (one enum, not two bools); change key toggles. Flow + save: [[architecture]] |
| Réglages › Ville | `city_search_view` (`:264`) | drawn instead of Réglages while `App.city_search` set (#82): title « Ville », plain frame. `> texte_` input (60 chars max), `Entrée` → « Recherche… » → up to 5 lines « Lyon, Rhône, France », `>` on picked one; `↑`/`↓` move, `Entrée` choose. « Aucune ville trouvée » / « Pas de connexion » under input, text kept for retry; typing again clears list. `Échap` → Réglages, Ville unchanged. Every key goes to input (`q`, `s` typed, no quit or close), Ctrl combos excepted. State `CitySearch` + `SearchStatus` (`src/settings/city.rs`). Flow: [[weather]] |

`MailContext` (`src/views.rs:36`) bundles view needs: Lettres, `Progress`, start date, night flag, current Énigme input. Content details in [[mail]].

## Barres — `src/hud/`

Two bars split the screen (#37): weather on top, keys at the bottom. Weather is the point of the app, keys are secondary → top bar in colour, bottom bar grey.

- **Barre du haut** (`weather_bar.rs`, `WeatherBar`): left block, one info per line — `Éclaircies 12 °C`, `Vent 8 km/h`, `Pluie 1,2 mm`, `Pluie vers 16:00` / `Pluie dans l'heure`, `Hors ligne`, `--demo` label last. A value rounding to 0 at its display precision → its line is gone, not `0 mm`. No weather yet → `Météo en route…`. Right block: `HH:MM`, then `Ville • Jour N`.
- Colour per info: temperature by 7 steps (≤0 / 1-5 / 6-11 / 12-17 / 18-24 / 25-29 / ≥30 °C, always judged in °C so °F reads the same), rain blue, alerts yellow, rest grey (`MUTED`, shared by both bars). No bold: the renderer only knows colours.
- Rebuilt only when `AppState` flags it (`take_weather_info_update`: weather, units, Ville, minute change), cached in `App.weather_bar`. Units come from `AppState::set_units` ([[weather]]).
- N = Jour de jeu (`AppState::set_story_day`), fed every frame by `refresh_progress` from `timeline_day` ([[mail]]): rolls over at `courrier.heure` without restart. New Ville (#82) → `AppState::set_location` flags rebuild → name shown next frame, before new weather lands; chosen Ville also → `set_location_display(City)` so the name shows even if the bar was in coordinates.
- **Barre du bas** (`toolbar.rs`): last row, right-aligned, grey: `[c] Courrier   [b] Badges x/y   [h] Aide   [q] Quitter`. Waiting Lettre / Objet → `[c] Lettre !` / `Objet !` first, pink, blinking on `overlay::blink_tick` (500 ms). Narrow terminal → compact labels, then cut from the right with `…`: alerts listed first, so they survive.
- `hud::lay_out` places both: left block from row 1 after a 2-col margin, right block truncated so it never overlaps the left one, Barre du bas on the last row. Pure, tested.
- Réglages « Barres » → « masquée », or `--hide-hud`, hides both bars (`App.hide_hud`, live).

## Gotchas

- One `char` = one cell. Wide glyphs (emoji, CJK) misalign diff + boxes.
- UI strings French, live in `views.rs`, `overlay.rs`, `app.rs`; follow glossary (Lettre, never Message).

## Chevrons (#131)

- `overlay::render_with_chevrons(view, has_previous, has_next)` draws a yellow `«` left / `»` right of the box, mid-height, one empty column (`CHEVRON_GAP`) then a back-and-forth every 500 ms; no room → nothing (under 70 columns the right one does not fit).
- Lettre: `»` while unread mail remains after the open one (`postman_note::has_unread_after`). Album: `«` except on page 1, `»` except on the last page.

## Encadré de Fête (#94)

- `Overlay::HolidayBox` (`holiday_box_view`, `Frame::Plain`, `[Echap] fermer`), shown at every launch on a Fête day after the other launch windows and before Badge cards. Data: `HOLIDAY_BOXES` in `src/holiday.rs` (13: Fêtes, Chandeleur, 1er mai, Perséides, Géminides); one box per launch, a one-day Fête wins over a longer one. Not in `--demo`.
