---
issue: 131
status: todo
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
- Test : chevron selon le nombre de non lus ; `cargo fmt --check`, `cargo test` verts.

## Implementation

## Review

## Verification

## Feedback
