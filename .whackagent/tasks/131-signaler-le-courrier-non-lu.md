---
issue: 131
status: to-test
wiki: [[render]], [[mail]]
note: "ajoute un chevron animé a droite du courrier en cours de lecture s'il y a un courier non lu, vu que le numero concerne leslettre s'il y a une archive apres on ne sais pas qu'il y a encore du courrier non lu v1.8"
created: 2026-11-18
---

## Context / Decisions

> ajoute un chevron animé a droite du courrier en cours de lecture s'il y a un courier non lu, vu que le numero concerne leslettre s'il y a une archive apres on ne sais pas qu'il y a encore du courrier non lu

- Le compteur `x/y` du titre compte dans une série (Lettres, Archives, Recettes…) : après une Archive, rien ne dit qu'il reste du courrier.
- **Chevron** : un `»` jaune juste à droite du cadre de la Lettre ouverte, à mi-hauteur. Il avance d'une colonne et revient, en boucle (~1 s).
- Visible tant qu'il reste au moins un courrier non lu après celui ouvert (`available_letters()` > `letters_read`, `src/app.rs`), quel que soit le genre de courrier. Disparaît quand tout est lu.
- Ni nombre ni texte. `[c]` / `[>]` inchangés.
- Terminal trop étroit pour la colonne à droite du cadre → pas de chevron.

## Acceptance criteria

- 2 courriers non lus, `c` → 1er ouvert, `»` à droite du cadre qui va et vient.
- `c` → dernier non lu ouvert → plus de `»`.
- Une Archive puis une Lettre non lue → sur l'Archive, `»` visible.
- Relire un ancien courrier (`<`) alors que tout est lu → pas de `»`.
- `»` un espace plus à droite du cadre qu'avant (feedback 2026-11-19).
- Album : `«` à gauche du cadre sauf sur la 1re page, `»` à droite sauf sur la dernière, même animation, pour inciter à parcourir les pages.
- Test : chevron selon le nombre de non lus ; `cargo fmt --check`, `cargo test` verts.

## Implementation

Round 1 — /wa-autopilot 2026-11-19.

- `src/story/postman_note.rs`: `has_unread_after(open_letter, available, letters_read)` next to `next_mail`; 2 tests.
- `src/overlay.rs`: `render_box` returns its `BoxArea`; `render_letter(renderer, view, has_unread_mail)` draws the yellow `»` at `x + width` / `x + width + 1`, row `y + height / 2`, 0.5 s per position (blink idiom of `tree.rs`); no room → nothing; 2 tests.
- `src/app.rs`: the `Overlay::Letter` branch of `render_overlays` passes the flag. Keys, titles, footers, views unchanged.
- With today's layout the box always leaves 2 free columns: the « no room » guard is covered by a unit test only.
- Build: `cargo build`, `cargo test` (489) green · fmt OK · clippy: existing `double_must_use` only.

## Review

## Verification

Round 1 — runtime check by the agent (pty 100x40, fresh `HOME`, seeded save at Jour 29):

- ✅ 2 unread, `c` → yellow `»` at mid-height right of the frame, alternating 2 columns every ~0.5 s.
- ✅ `c` to the last unread → no `»`.
- ✅ Archive then Lettre unread → `»` on the Archive and on the next Lettre, gone on the last.
- ✅ Old mail reread (Left arrow) with all read → no `»`; footers unchanged.

## Feedback

Round 2 — 2026-11-19. Asked: « met le un espace plus à droite, il faudrait la même chose sur archive à droite et à gauche sauf premier et dernier pour inciter à aller voir les onglets » ; screen = Album pages.
- adjustment: chevron one column further right.
- adjustment: Album chevrons `«` / `»` except first / last page.
- Changes: `src/overlay.rs` `render_with_chevrons(view, has_previous, has_next)`, `chevron_cell`, `Side`, `CHEVRON_GAP = 1` (one empty column, then the back-and-forth); `src/app.rs` Letter and Album arms. Tests replaced for both sides.
- Under 70 columns the right chevron no longer fits (needs 3 free columns): nothing drawn.
- Review: deferred (→ /wa-validate). Verify: ✅ agent pty — mail `»` one column further; Album page 1 only `»`, pages 2–5 both, last only `«`.
