---
issue: 157
status: to-test
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

- `BadgeKind::Common` (« Commun ») et `BadgeKind::Epic` (« Épique ») dans `src/badges.rs` ; `KINDS` dans l'ordre de la spec ; Badges déplacés gardent leur id, seule la rangée change. `BADGES` 50 → 52.
- « Toutes les énigmes » (`all_riddles`) : `chronicles::has_solved_every_riddle`, compte les Énigmes distinctes (`week % RIDDLES.len()`).
- « Toutes les recettes » (`all_recipes`) : `recipes::has_every_recipe(diets, received)` = chaque Recette qui `suits` les Régimes reçue ; `App::has_received_every_recipe` lit les Recettes livrées à ce jour, exige un Régime répondu. Régime sans case cochée → les 40 Recettes.
- Icônes des 2 nouveaux Badges : choix de l'agent, à regarder.
- Écran Badges plus haut de 4 lignes : tient en 45 lignes, déborde déjà avant en ~24.
- Preuve : clippy sans nouveau warning, build OK, `cargo test` tout vert.

## Review

## Verification

Autopilot, app pilotée en pty (`HOME` à part, `-s clear`, sauvegardes préparées) :
- ✅ Ordre Météo, Fêtes, Fidélité, Commun, Rares, Épique, Secrets.
- ✅ Commun (6), Rares (3), Épique (4) avec les bons Badges.
- ✅ Fidélité : jours d'affilée seulement.
- ✅ Baroudeur déjà gagné → toujours gagné, dans Rares.
- ✅ 29 Énigmes + la 30e → carte « Toutes les énigmes ».
- ✅ Dernière Recette du Régime reçue → carte « Toutes les recettes » ; la veille, pas de carte.

## Feedback
