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
- **Régimes** (8, cumulables) : végétarien, vegan, sans légume, sans viande, sans poisson, sans porc, full protéine, sans fruit (ajouté le 2026-11-19). Aucun coché = « je mange de tout ».
- **Lettre de Rose** qui demande les Régimes :
  - partie neuve → Jour de jeu 10 ;
  - partie déjà plus loin → au prochain Jour de jeu après la mise à jour, jour gardé dans la sauvegarde ;
  - le courrier déjà livré ne bouge jamais (les Lettres lues se décaleraient).
- **Réponse dans la Lettre** : `[1]`…`[8]` cochent/décochent (`✓`), `[Entrée]` envoie. Lettre relue → Régimes choisis.
- **Changer ensuite** : ligne « Régime » dans les Réglages, qui ouvre la même liste à cocher. Effet sur les Recettes suivantes seulement.
- **Cadence** : Recette du 1er du mois tant que la Lettre de Rose n'a pas de réponse. 1re Recette de la réserve le lendemain de la réponse, puis tous les 10 jours.
- **Réserve étiquetée** (pas de variantes par Régime) :
  - 40 Recettes dans la voix de Rose, dont les 12 actuelles, chacune étiquetée (viande, porc, poisson, légumes, produits animaux, protéinée) ;
  - au moins 12 compatibles avec chaque Régime ;
  - Recette de saison (Galette des rois en janvier, Crêpes de la Chandeleur en février…) livrée seulement dans son mois.
- **Retours sur le tableau** (2026-11-19) :
  - pomme de terre ≠ légume ;
  - pâtes brisée et feuilletée peuvent être faites à l'huile : compatibles vegan ;
  - alternatives vegan dans le même texte, entre parenthèses (crème → crème soja ou avoine, lait → lait végétal, œufs → fécule…) : la Recette compte comme vegan ;
  - versions vegan dans le texte pour Galette des rois, Crêpes, Pain perdu, Clafoutis aux cerises ;
  - desserts protéinés, ex. cookies dattes + avoine (vegan aussi) ;
  - soupe aux orties retirée, remplacée par une autre soupe ;
  - Chili sin carne aux protéines de soja texturées.
- **Choix** : la première Recette pas encore reçue, dans l'ordre de la réserve, qui respecte tous les Régimes et son mois. Recette livrée figée dans la sauvegarde.
- **Réserve épuisée** pour les Régimes choisis → plus de Recette. #137 en ajoute avant février 2027.
- **Étape 1 : textes** (décision utilisateur·rice). J'écris le tableau des 40 Recettes (nom, texte, étiquettes, mois), tu le valides avant le code, comme le tableau des Arcanes.
- Repères : une partie au Jour 35 qui répond le Jour 36 reçoit sa 1re Recette le Jour 37. Avec 12 compatibles, la dernière arrive le Jour 147, soit 4 mois. Avec les 40, le Jour 427, soit plus d'un an.

## Acceptance criteria

- Tableau des 40 Recettes validé avant le code ; au moins 12 compatibles par Régime.
- Partie neuve, Jour 10 → Lettre de Rose avec 8 cases ; `[1]` `[6]` puis `[Entrée]` → Régimes gardés, Lettre relue les montre.
- Jour 11 → Recette végétarienne sans porc ; Jours 21, 31… → une Recette compatible, jamais deux fois la même.
- Recette de saison livrée seulement dans son mois.
- Lettre sans réponse → Recette du 1er du mois, comme aujourd'hui.
- Sauvegarde au Jour 29 → Lettre de Rose au prochain Jour de jeu ; aucune Lettre déjà livrée ne bouge (test de sauvegarde ancienne).
- Réglages → Régime → changé → les Recettes suivantes suivent ; celles déjà reçues ne changent pas.
- Plus aucune Recette compatible → plus de Recette.
- Tests : filtre par Régime et mois, cadence, Recette figée, ancienne sauvegarde ; `cargo fmt --check`, `cargo test` verts.

## Implementation

Round 1 — /wa-autopilot 2026-11-19. **blocked** — step 1 done, no code.

- Open question: validate the table of the 36 Recettes (kept outside the repo, `recettes-tableau.md`) before the code.
- Compatible Recettes per Régime (total / without month): végétarien 28/18 · vegan 15/12 · sans légume 25/18 · sans viande 31/21 · sans poisson 33/23 · sans porc 34/24 · full protéine 15/15.
- Choices: pomme de terre = légume; legumes (lentilles…) = protéinée, not légume; pastry = produits animaux; bread, yeast, wine = vegan; egg desserts not protéinée.
- Order: 1–12 existing Recettes unchanged, 13–15 new seasonal (Orties mars, Gaspacho juillet, Châtaignes novembre), 16–36 sweet and savoury alternated.
- Doubts to settle: « bouillon de légumes » in the Potiron text; Clafoutis stays in May; Poires pochées au vin (alcohol); Houmous, Tofu, Chili sin carne less « village » but needed for vegan + protéinée.

Round 1 (cont.) — table v2 after the user's feedback (2026-11-19), still blocked on validation.

- Counts (total / without month): végétarien 28/18 · vegan 21/13 · sans légume 27/20 · sans viande 31/21 · sans poisson 33/23 · sans porc 34/24 · full protéine 19/19 (4 desserts) · sans fruit 29/22.
- « produits animaux » tag = an animal product without an alternative in the text.
- « fruits » = fresh/dried fruit, citrus, juice, compote, jam; not olive, tomato, pepper, avocado, pumpkin, nuts.
- Removed to make room (to validate): Compote de pommes, Fougasse aux olives, Poires pochées au vin, Taboulé. Soupe aux orties → Soupe poireaux-pommes de terre (mars).
- 8 existing texts retouched (vegan alternatives in parentheses), also read by players who never answer Rose.

- Table v3: the 4 removed Recettes are back (never remove content unless asked) → 40 Recettes. Counts (total / without month): végétarien 32/22 · vegan 25/17 · sans légume 30/23 · sans viande 35/25 · sans poisson 37/27 · sans porc 38/28 · full protéine 19/19 · sans fruit 30/23. Chili: soja texturé + haricots rouges. Retouched texts OK.

## Review

## Verification

## Feedback
