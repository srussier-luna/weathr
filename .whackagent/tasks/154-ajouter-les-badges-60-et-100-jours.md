---
issue: 154
status: todo
wiki: [[mail]], [[animation]], [[render]]
note: ajouter un badge 60 jours et 100 jours, pour chacun l'utilisateur recois un recompense pour son jardin qu'il peut afficher ou non dans son jardin
created: 2026-11-24
---

## Context / Decisions

> ajouter un badge 60 jours et 100 jours, pour chacun l'utilisateur recois un recompense pour son jardin qu'il peut afficher ou non dans son jardin

- Deux Badges Fidélité de plus, suite de `streak_3` / `streak_7` / `streak_30` : « 60 jours d'affilée » (`streak_60`) et « 100 jours d'affilée » (`streak_100`). Même compteur de jours d'affilée, même attribution dans `App::collect_badges`, icône sur le modèle `|60 |` / `|100|`.
- Chaque Badge offre un Trophée au jardin : 60 jours → Nain de jardin, 100 jours → Statue (petite statue dorée).
- Le Trophée est posé au jardin dès le Badge gagné.
- Le joueur choisit d'afficher ou non chaque Trophée dans une nouvelle page de l'Album, « Album : tes trophées », placée juste après « Album : tes réponses ».
  - Même mécanique que « tes réponses » : `↑`/`↓` sélection, `[1]` au jardin, `[2]` rangé.
  - Un Trophée n'apparaît sur la page qu'une fois son Badge gagné.
  - Choix sauvegardé dans `Progress` (champ `#[serde(default)]`, vieilles sauvegardes OK).
- Placement : au sol, à côté de la niche de Biscuit (`Dog::doghouse`, `src/animation/dog.rs`). La niche arrive Jour 9, donc toujours là à 60 jours. Pas la place → Trophée pas dessiné.
- Hors scope : autres Trophées, déplacer un Trophée, Trophées pour d'autres Badges.

## Acceptance criteria

- Sauvegarde avec 59 jours d'affilée, lancement le jour suivant → carte « Nouveau badge ! » « 60 jours d'affilée », Nain de jardin visible à côté de la niche.
- Même chose à 100 jours → Badge « 100 jours d'affilée », Statue visible à côté de la niche.
- Écran Badges (`b`) : les deux Badges dans la colonne Fidélité, après « 30 jours d'affilée ».
- Album (`a`) : page « Album : tes trophées » après « tes réponses », une ligne par Trophée gagné, aucune ligne pour un Trophée pas gagné.
- Sur cette page, `[2]` sur le Nain de jardin → il disparaît du jardin ; `[1]` → il revient. Pareil pour la Statue.
- Choix gardé après relance du jeu.
- Ancienne sauvegarde sans le nouveau champ → se charge, Trophées gagnés affichés au jardin.

## Implementation

## Review

## Verification

## Feedback
