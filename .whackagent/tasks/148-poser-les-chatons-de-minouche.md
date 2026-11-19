---
issue: 148
status: todo
wiki: [[animation]], [[scene]]
note: "les chaton de minouche sont dans le vide"
created: 2026-11-19
---

## Context / Decisions

> les chaton de minouche sont dans le vide

- Les chatons `=^.^= =^.^=` (`src/animation/cat.rs`, `unlocks.kittens`) sont dessinés à `roof_perch + 8`, une ligne sous Minouche. Le toit descend à cet endroit : ils flottent au-dessus.
- **Décision** : chaque chaton est posé sur la ligne du toit sous lui, juste à côté de Minouche. Même couleur (blancs ou noirs selon la réponse), même moment (Minouche sur le toit).
- Petite et grande maison (`HouseSize`), toutes largeurs de terminal.
- Minouche, le nœud et la fenêtre ne bougent pas.

## Acceptance criteria

- Chatons arrivés (Jour de jeu 33+), Minouche sur le toit, petite maison puis grande (terminal ≥ 170×32) → chaque chaton touche le toit, aucun vide sous lui.
- Chatons blancs / noirs selon la réponse, comme avant.
- Pluie ou nuit (Minouche à la fenêtre) → pas de chatons sur le toit, comme avant.
- Test unitaire : la ligne de chaque chaton est juste au-dessus du toit, pour les deux tailles ; `cargo fmt --check`, `cargo test` verts.

## Implementation

## Review

## Verification

## Feedback
