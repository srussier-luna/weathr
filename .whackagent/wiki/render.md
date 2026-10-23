# Render

How cells reach terminal, how modal screens (Courrier, Lettre, Album, Aide, Badges, Bouteille) drawn on top of scene.

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

- `View { title, lines, footer }` (`src/overlay.rs:20`) = single data shape for every modal. `Line` is `Text` (word-wrapped), `Preformatted` (truncated) or `Blank`.
- `render_view` draws centred `+--+` box, max 66 columns; `render_badges` = only special layout (columns grouped by `BadgeKind`, secret badges show `???`).
- Toasts (new Badge, found object, riddle result) drawn by `App::render_overlays` on row 0, one at a time, 4 s.

## Views — `src/views.rs`

Pure functions: story data → `View`. No I/O, no state.

| Screen | Function | Notes |
|---|---|---|
| Lettre | `entry_view` (`src/views.rs:56`) | dispatch on `Entry` kind; footer changes with Choice / Énigme / input |
| Courrier | `index_view` (`:114`) | scrolling list of read entries, date + sender |
| Album | `album_pages` | summary, gifts, answers, objects, chest, Bouteilles, Cartes postales, recipes, drawings |
| Aide | `help_view` (`:292`) | keys shown only once unlocked (`[e]`, `[r]`) |
| Bouteille | `bottle_view` (`:284`) | shown on pickup |

`MailContext` (`src/views.rs:36`) bundles view needs: Lettres, `Progress`, start date, night flag, current Énigme input. Content details in [[mail]].

## HUD

Status line built in `AppState::update_cached_info` (`src/app_state.rs:166`), rebuilt only when setter flags `weather_info_needs_update` or minute changes. Mixes French hints (`Lettre ! [c]`, `Badges x/y [b]`, `Aide [h]`) with English weather labels from upstream.

## Gotchas

- One `char` = one cell. Wide glyphs (emoji, CJK) misalign diff + boxes.
- UI strings French, live in `views.rs`, `overlay.rs`, `app.rs`; follow glossary (Lettre, never Message).