---
issue: 132
status: todo
wiki: [[mail]]
note: "le joueur vgie n'as pas fonctionné"
created: 2026-11-23
---

## Context / Decisions

> le joueur vgie n'as pas fonctionné

- Cause (retour du joueur) : son `git config user.name` vaut « Denis Richard » ; le jeu compare à `drichard-luna` (`SKY_LOOKOUT_LOGIN`, `src/badges.rs`), donc le badge secret « Vigie du ciel » ne se déclenche jamais.
- **Décision** : comparer au nom réel, en clair : `SKY_LOOKOUT_LOGIN` = « Denis Richard ». Même règle qu'aujourd'hui (`is_listed` : espaces autour ignorés, égalité exacte).
- #109 (retirer le nom du code public) reste à part.

## Acceptance criteria

- `git config user.name` = `Denis Richard` → au lancement, carte « Vigie du ciel », case cochée dans `b`.
- `drichard-luna`, autre nom, nom vide ou git absent → pas de Vigie du ciel.
- Tests unitaires mis à jour ; `cargo fmt --check`, `cargo test` verts.

## Implementation

## Review

## Verification

## Feedback
