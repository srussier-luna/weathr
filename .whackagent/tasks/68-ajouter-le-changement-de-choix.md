---
issue: 68
status: todo
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

## Review

## Verification

## Feedback
