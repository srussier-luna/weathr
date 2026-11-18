---
issue: 74
status: todo
wiki: [[mail]], [[architecture]]
note: "j'aimerai une recette tous les 10 jour avec une lettre qui demande les regimes alimentaire, vegetarien, vegan, sans legume, sans viande, sans poisson, sans porc, full proteine, possible d,en avoir plusierus adapter les recette en fonction de la reponse"
created: 2026-11-18
---

## Context / Decisions

> j'aimerai une recette tous les 10 jour avec une lettre qui demande les regimes alimentaire, vegetarien, vegan, sans legume, sans viande, sans poisson, sans porc, full proteine, possible d,en avoir plusierus adapter les recette en fonction de la reponse

- Aujourd'hui : 12 Recettes de Rose (`RECIPES`, `src/story/content/chronicles.rs`), une le 1er de chaque mois (`src/story/sources/recipes.rs`, `Entry::Recipe(month)`).
- **Régimes** (7, cumulables) : végétarien, vegan, sans légume, sans viande, sans poisson, sans porc, full protéine. Aucun coché = « je mange de tout ».
- **Lettre de Rose** qui demande les Régimes :
  - partie neuve → Jour de jeu 10 ;
  - partie déjà plus loin → au prochain Jour de jeu après la mise à jour, jour gardé dans la sauvegarde ;
  - le courrier déjà livré ne bouge jamais (les Lettres lues se décaleraient).
- **Réponse dans la Lettre** : `[1]`…`[7]` cochent/décochent (`✓`), `[Entrée]` envoie. Lettre relue → Régimes choisis.
- **Changer ensuite** : ligne « Régime » dans les Réglages, qui ouvre la même liste à cocher. Effet sur les Recettes suivantes seulement.
- **Cadence** : Recette du 1er du mois tant que la Lettre de Rose n'a pas de réponse. 1re Recette de la réserve le lendemain de la réponse, puis tous les 10 jours.
- **Réserve étiquetée** (pas de variantes par Régime) :
  - 36 Recettes dans la voix de Rose, dont les 12 actuelles, chacune étiquetée (viande, porc, poisson, légumes, produits animaux, protéinée) ;
  - au moins 12 compatibles avec chaque Régime ;
  - Recette de saison (Galette des rois en janvier, Crêpes de la Chandeleur en février…) livrée seulement dans son mois.
- **Choix** : la première Recette pas encore reçue, dans l'ordre de la réserve, qui respecte tous les Régimes et son mois. Recette livrée figée dans la sauvegarde.
- **Réserve épuisée** pour les Régimes choisis → plus de Recette. #137 en ajoute avant février 2027.
- **Étape 1 : textes** (décision utilisateur·rice). J'écris le tableau des 36 Recettes (nom, texte, étiquettes, mois), tu le valides avant le code, comme le tableau des Arcanes.
- Repères : une partie au Jour 35 qui répond le Jour 36 reçoit sa 1re Recette le Jour 37. Avec 12 compatibles, la dernière arrive le Jour 147, soit 4 mois. Avec les 36, le Jour 387, soit un an.

## Acceptance criteria

- Tableau des 36 Recettes validé avant le code ; au moins 12 compatibles par Régime.
- Partie neuve, Jour 10 → Lettre de Rose avec 7 cases ; `[1]` `[6]` puis `[Entrée]` → Régimes gardés, Lettre relue les montre.
- Jour 11 → Recette végétarienne sans porc ; Jours 21, 31… → une Recette compatible, jamais deux fois la même.
- Recette de saison livrée seulement dans son mois.
- Lettre sans réponse → Recette du 1er du mois, comme aujourd'hui.
- Sauvegarde au Jour 29 → Lettre de Rose au prochain Jour de jeu ; aucune Lettre déjà livrée ne bouge (test de sauvegarde ancienne).
- Réglages → Régime → changé → les Recettes suivantes suivent ; celles déjà reçues ne changent pas.
- Plus aucune Recette compatible → plus de Recette.
- Tests : filtre par Régime et mois, cadence, Recette figée, ancienne sauvegarde ; `cargo fmt --check`, `cargo test` verts.

## Implementation

## Review

## Verification

## Feedback
