---
issue: 68
status: to-close
wiki: [[mail]], [[render]]
note: "ajouter un moyen de changer les choix anciennement faits"
created: 2026-11-18
---

## Context / Decisions

> revenir sur une réponse déjà donnée — ajouter un moyen de changer les choix anciennement faits

- Les 11 Questions [1]/[2] des Lettres du village (`CHOICES`, `src/story/content/letters.rs`). La réponse est gardée dans `Progress::answers`.
- **Où** : la page « Album : tes réponses » devient interactive. La Lettre relue ne change pas (« Ta réponse : … », sans [1]/[2]).
- **Rendu** : sélection `>` comme dans Lettres reçues. La ligne choisie montre les deux options, la réponse actuelle marquée `✓`, comme les Réglages ; les autres lignes gardent « question ? réponse ».
- **Touches**, sur cette page seulement : `↑` `↓` choisit, `[1]` `[2]` change aussitôt, sans confirmation ni toast. `[<]` `[>]` pages et `[a]`/`Echap` inchangés.
- Footer de la page : `[↑] [↓] choisir  [1] [2] changer  [<] [>] pages  [a] ou [Echap] fermer`. Autres pages de l'Album : footer actuel.
- **Limite** : aucune, autant de fois qu'on veut, à tout moment.
- Seules les Questions déjà répondues apparaissent (comme aujourd'hui) : on ne répond pas depuis l'Album.
- Suit la nouvelle réponse : Lettre relue, Écho relu, variante au jardin (`Choices::from_answers`, `refresh_unlocks`), sauvegarde.
- Réutiliser la sélection de `Overlay::Index` / `Overlay::Settings` (`app.rs`) et `answer_letter` pour l'écriture.
- Sélection remise sur la première ligne à chaque ouverture de la page.

## Acceptance criteria

- Album → page « tes réponses » : `>` sur la première ligne, ses deux options visibles avec `✓` sur la réponse actuelle ; footer avec `[↑] [↓] choisir  [1] [2] changer`.
- `↓` puis `↑` → la sélection se déplace, bornée à la première et à la dernière ligne.
- Réponse 1 sélectionnée, `[2]` → la ligne montre `✓` sur l'option 2 ; `progress.json` → `answers` mis à jour.
- Relire la Lettre de cette Question → « Ta réponse : <option 2> ».
- Relire sa Lettre d'Écho (déjà reçue) → nouvel Écho.
- Question du nœud passée à bleu → nœud bleu au jardin ; repassée à rouge → rouge.
- Changer trois fois de suite → la dernière réponse gardée.
- `[1]` `[2]` sur une autre page de l'Album → rien ; aucune Question répondue → page vide, `↑` `↓` sans effet.
- `cargo fmt --check`, `cargo test` verts ; tests : changement de réponse, bornes de la sélection.

## Implementation

Round 1 — /wa-autopilot 2026-11-19.

- `src/app.rs`: `Overlay::Album { page, selected }` (selection reset on open and page change); `set_answer` shared with `answer_letter`; `press_option` routes `[1]`/`[2]` to the Album or the Letter; `album_choice`, `album_after_scroll` testable.
- `src/views.rs`: « tes réponses » page = `views::ANSWERS_PAGE` (index 2); selected line « question ? [1] Rouge ✓ [2] Bleu » with non-breaking spaces; other lines keep `- `; footer with `[↑] [↓] choisir  [1] [2] changer`.
- `src/story/content/letters.rs`: `answered_choices` in Question order (shared by page and keys).
- No answer yet → « Rien pour l'instant. » with the normal Album footer (choice of the implementer, not in spec).
- `CHOICES` has 12 Questions (spec said 11).
- Build: `cargo build`, `cargo test` green · fmt OK · clippy: existing `double_must_use` only.

## Review

Round validation — 2026-11-19, one wa-verifier, LENSES style ✓ elegance ✓ structure ✓ correctness ✓.
- 2 minor findings fixed by autofix round 1: answer writes now go through one pure `answer_option` (replaces `answer_letter`/`set_answer`), tested; test pins the Album page order. Runtime re-run by the agent: all criteria ✅. Code changed since the user's test (app.rs, views.rs).

- Landing 2026-11-19: develop (with #131's Album chevrons) merged in (6cb233e), one conflict in the Album arm resolved keeping both; runtime re-run by the agent; resolution re-verified clean.

## Verification

Round 1 — runtime check by the agent (pty 120x50, fresh `HOME`, seeded save at Jour 40 with 3 answers; `--day` not used: it never saves):

- ✅ Page opens with `>` on line 1, both options and `✓`, new footer.
- ✅ ↑/↓ bounded; selection back to line 1 after a page change.
- ✅ `[2]` / `[1]` → line and `progress.json` updated; Lettre relue « Ta réponse : Bleu »; Écho relu follows; bow at the garden red ↔ blue.
- ✅ Three changes → last kept; second line changes only its Question.
- ✅ `[1]`/`[2]` on other Album pages → nothing; no answers → empty page, keys inert.

## Feedback
