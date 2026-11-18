---
issue: 99
status: todo
wiki: [[animation]], [[mail]]
note: "ok, ajoute un draft poue verifier toutes les traces de pas de la saison 1, avjoute le label 1.8"
created: 2026-11-18
---

## Context / Decisions

> ok, ajoute un draft poue verifier toutes les traces de pas de la saison 1, avjoute le label 1.8

- Aujourd'hui (#97) : Traces de boue au Jour de jeu 8 seulement (`FOOTPRINT_DAYS`), Traces mouillées les nuits 13, 14 et 30 (`WET_FOOTPRINT_DAYS`, `src/app.rs`).
- Courriers de la saison 1 qui parlent de traces (à compléter par un relevé exhaustif) :
  - Lettres du village (`src/story/content/letters.rs`) : « Cette nuit, quelqu'un a marché dans ton jardin. Regarde le sol », « Ces traces sont petites », Léo « Pardon pour les traces de boue » ;
  - voisin n°13 : « Les traces vont du puits à ta porte », « de la boue sur ton paillasson » ;
  - ronde (`src/story/content/chronicles.rs`) : « de la boue près du puits… des pas qui vont vers ta maison ».
- **Règle** (décision utilisateur·rice) : les traces dont parle un courrier sont visibles le Jour de jeu où il arrive.
  - Traces de boue : toute la journée.
  - Traces mouillées : restent nocturnes, la nuit de ce jour.
- **Sortie** : un test liste chaque courrier qui parle de traces avec son Jour de jeu et vérifie les bonnes traces ce jour-là. Les jours faux sont corrigés dans `FOOTPRINT_DAYS` / `WET_FOOTPRINT_DAYS`.
- Pas de nouvelle trace, pas de texte modifié.

## Acceptance criteria

- Relevé de tous les courriers de la saison 1 qui parlent de traces, de boue ou de pas, avec leur Jour de jeu, dans `## Implementation`.
- Pour chacun, `--day N` (et `--day N --night` pour les mouillées) → les traces décrites sont visibles.
- Aucun jour de traces sans courrier qui l'explique.
- Test qui relie chaque courrier à son jour de traces ; `cargo fmt --check`, `cargo test` verts.

## Implementation

## Review

## Verification

## Feedback
