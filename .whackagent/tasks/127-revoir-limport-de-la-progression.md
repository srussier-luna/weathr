---
issue: 127
status: todo
wiki: [[architecture]]
note: "Suite de #125 pour l'export/import de #39, sprint « Anti-triche »."
created: 2026-11-18
---

## Context / Decisions

> à reprendre une fois #125 fait — Suite de #125 pour l'import de #39.

- Détails gardés hors du dépôt.
- Un export puis un import ne coûtent jamais rien au joueur.
- #39 ne fusionne dans `develop` qu'avec ou après cette tâche.
- Après #39 et #125.

## Acceptance criteria

- Export puis import sur un HOME neuf → même partie ; relance → aucun Mot des voisins.
- Export puis import sur un HOME qui a déjà une partie (`o`) → partie importée ; relance → aucun Mot des voisins.
- `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings -A clippy::double_must_use`, `cargo test` verts.

## Implementation

## Review

## Verification

## Feedback
