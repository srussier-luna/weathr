---
issue: 157
status: todo
wiki: [[mail]], [[render]]
note: ajoute une section commun pour y mettre boussole, Voyageur, première énigme, 10 enigmes, facteur et Bonhomme de neige et Bouteille à la mer / ajoute une section epic pour toutes les enigme, ajoute un badge toutes les recette et met y aussi toutes les lettres / baroudeur va dans rare
created: 2026-11-24
---

## Context / Decisions

> ajoute une section commun pour y mettre boussole, Voyageur, première énigme, 10 enigmes, facteur et Bonhomme de neige et Bouteille à la mer
> ajoute une section epic pour toutes les enigme, ajoute un badge toutes les recette et met y aussi toutes les lettres
> baroudeur va dans rare

- Deux nouvelles rangées de Badges (`BadgeKind`) : **Commun** et **Épique**.
- Ordre des rangées dans l'écran Badges : Météo, Fêtes, Fidélité, Commun, Rares, Épique, Secrets (Mauvais points inchangé, après).
- Commun : Voyageur, Première énigme, Dix énigmes, Facteur, Bonhomme de neige, Bouteille à la mer. Boussole y arrive avec #155.
- Rares : Arc-en-ciel, Étoile filante, Baroudeur (vient de Fidélité).
- Épique : Toutes les lettres, Tous les objets (viennent de Fidélité), plus deux nouveaux Badges :
  - « Toutes les énigmes » : les 30 Énigmes (`RIDDLES`) résolues.
  - « Toutes les recettes » : toutes les Recettes compatibles avec les Régimes du joueur reçues (pas les 40, impossible avec un Régime). Si #137 ajoute des Recettes, le seuil suit ; Badge déjà gagné gardé.
- Fidélité garde les jours d'affilée (3, 7, 30, puis 60 et 100 avec #154).
- Seule la rangée change pour les Badges déplacés : même id, déjà gagnés gardés.
- Hors scope : renommer des Badges, changer leurs conditions.

## Acceptance criteria

- Écran Badges (`b`) : rangées dans l'ordre Météo, Fêtes, Fidélité, Commun, Rares, Épique, Secrets.
- Commun contient Voyageur, Première énigme, Dix énigmes, Facteur, Bonhomme de neige, Bouteille à la mer.
- Rares contient Arc-en-ciel, Étoile filante, Baroudeur.
- Épique contient Toutes les lettres, Tous les objets, Toutes les énigmes, Toutes les recettes.
- Fidélité ne contient plus que les jours d'affilée.
- Sauvegarde avec Baroudeur gagné → toujours gagné, affiché dans Rares.
- Sauvegarde avec 29 Énigmes résolues, 30e résolue → carte « Nouveau badge ! » « Toutes les énigmes ».
- Joueur avec un Régime, dernière Recette compatible reçue → carte « Nouveau badge ! » « Toutes les recettes ».

## Implementation

## Review

## Verification

## Feedback
