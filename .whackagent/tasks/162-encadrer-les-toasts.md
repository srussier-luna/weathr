---
issue: 162
status: to-close
wiki: [[render]]
note: découpée de #37 au grill du 2026-11-24
created: 2026-11-24
---

## Context / Decisions

- Aujourd'hui le toast s'écrit seul sur la ligne 0, collé à la barre du haut. Le but : qu'il ne soit plus au niveau de la météo et qu'il se remarque.
- proto: [[protos/changer-la-barre-du-haut-separation]] · https://claude.ai/artifact/U5Kc6zxWSthjRk6ru3W5eC (v10 validé, réglage « Toast »)
- Le toast est centré horizontalement, au-dessus de la Barre du bas (#37), dans un cadre `+---+` / `|` comme `Frame::Plain`.
- Le texte s'écrit lettre par lettre, à 30 ms par lettre. Le cadre a sa taille finale dès le début et ne grandit jamais.
- Une fois écrit, il reste 4 s (`TOAST_DURATION`). Un toast à la fois, la file `push_toast` ne change pas.
- `--hide-hud` / Barres « masquée » ne cache pas les toasts.
- Où vit le code : `src/hud/toast.rs`, à côté de `weather_bar` et `toolbar` (#37). C'est une fonction pure : texte + temps écoulé → lignes à dessiner. `App::render_overlays` l'appelle.
- Bloquée par #37 : la position dépend de la Barre du bas.
- Version : v1.9.

## Acceptance criteria

- Un toast (par ex. objet trouvé) s'affiche centré, dans un cadre `+---+`, juste au-dessus de la dernière ligne. Plus rien n'est écrit sur la ligne 0.
- Le texte apparaît lettre par lettre, environ 30 ms par lettre, et le cadre garde la même largeur du début à la fin.
- Le toast reste 4 s après la dernière lettre, puis disparaît. S'il y en a un autre dans la file, il prend la suite.
- À 70 colonnes, un toast long reste dans l'écran, sans déborder du cadre.
- Avec les Barres masquées, le toast s'affiche quand même.

## Implementation

- autopilot 2026-11-24, wa-implementer, round 1.
- `src/hud/toast.rs` (new): `lay_out(message, elapsed, w, h) -> Vec<Placed>`, `is_over(message, elapsed, w)`; private `DURATION` 4 s replaces `TOAST_DURATION`.
- `app.rs`: `toast_until` → `toast_started: Option<Instant>`; `update_toasts(term_width)` pops once `is_over`; `render_overlays` draws via `render_placed`, nothing on row 0.
- Frame: Plain look, `Color::Reset`; text Yellow; padding 2 spaces each side; max width = w - 4; long text wraps with `overlay::wrap`, frame grows upward, bottom on row h-2; unrevealed cells filled with spaces.
- Toast still drawn before windows in `render_overlays` (same order as before).
- Proof: `cargo build` ok · `cargo test` 548 + 166 + 10 + 7 + 3 + 5 passed.

## Review

### validation — 2026-11-24 · clean

Rebased onto `sprint/barres` after #164 merged (no conflict). Round 1, 2 findings (🟡), autofixed:
- [elegance] `toast.rs` own `SCREEN_MARGIN` → shares `hud::inner_width`
- [elegance] frame side width written twice, mixed piece types → one `CHROME_WIDTH`, `CORNER`/`SIDE`/`RULE` all `&str`

Round 2: all fixed, nothing new. Build (clippy strict) ✅ · tests ✅.

## Verification

- manual validation — not run by agent (TUI). Criteria 1–4 covered by `hud::toast` tests; 5 (Barres masquées) = wiring, `render_overlays` outside the `hide_hud` branch.

## Feedback
