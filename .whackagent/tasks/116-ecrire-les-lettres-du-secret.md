---
issue: 116
status: todo
wiki: [[mail]]
note: "Découpé de #113 (parapluie « Ajouter un secret »), sprint « Le secret ». Arcanes livrées par l'Arcano-Facteur de #114."
created: 2026-11-16
---

## Context / Decisions

> Spoiler : ce fichier décrit le secret. Ticket, PR et commits restent vagues (« Lettres du secret »).

- Blocked by #114 (livraison, Courrier, numérotation, cadre Nuit étoilée). Ici : le texte, ses choix, ses illustrations.
- **Brief de l'utilisateur·rice (16/11)** : deux saisons cachées. Cette tâche = **saison 1** ; la saison 2 et sa deuxième piste ont leurs tickets.
- **Saison 1 — « Le Syndicat des Dieux au chômage »**, sous-titre « Les divinités ont perdu leur vocation ».
  - Pitch : les dieux ne sont pas morts, plus personne ne leur demande rien (le dieu des moissons est devenu comptable, la déesse des carrefours tient une auberge, le dieu des rêves travaille dans une fabrique de nuages). Ils ont rejoint le Syndicat des Divinités Déclassées, qui enquête sur pourquoi ses membres perdent leur raison d'exister. L'Arcano-Facteur apporte les 21 lettres de l'enquête.
  - **Arcano-Facteur** = ancien dieu des passages, des seuils et des correspondances, presque sans pouvoirs, dernier messager encore officiellement en activité. Narrateur d'introduction seulement : déblocage (scène #117), courte introduction avant l'Arcane n° 1, éventuellement la conclusion. Ancien, fatigué mais curieux, poli à l'ancienne, solennel devant l'absurde, en sait plus qu'il ne dit, ne ment jamais, n'explique jamais tout. Interventions courtes.
  - Arcanes (ordre et inspiration imposés) : 1 Bateleur, dieu des commencements · 2 Papesse, gardienne des secrets divins · 3 Impératrice, déesse de la fertilité des mondes · 4 Empereur, ancien souverain des constellations · 5 Pape, dieu des croyances oubliées · 6 Amoureux, divinité des liens et séparations · 7 Chariot, dieu des voyages célestes · 8 Justice, juge des divinités · 9 Hermite, dernier allumeur d'étoiles · 10 Roue de Fortune, déesse de la chance · 11 Force, gardienne des créatures primordiales · 12 Pendu, dieu qui a refusé de redescendre · 13 Arcane sans nom, ancien dieu des fins · 14 Tempérance, alchimiste des éléments · 15 Diable, dieu des petites tentations · 16 Maison Dieu, architecte des cieux · 17 Étoile, déesse des souhaits · 18 Lune, gardienne des rêves · 19 Soleil, ancien dieu de la lumière · 20 Jugement, responsable des convocations · 21 Monde, dernier dieu encore en fonction.
  - Chaque Arcane : un personnage à voix propre, un problème concret, un **événement** (pas une présentation), un lien discret avec l'intrigue, au moins un détail étrange, une information nouvelle.
  - **Retournement** : les dieux n'ont pas été oubliés, ils ont **accompli leur mission** ; le monde fonctionne sans eux. Le dernier dieu en fonction est celui des passages : il attend quelqu'un à qui transmettre son dernier passage. Révélations majeures réservées aux Arcanes 18 à 20 ; indices dès les premières.
  - **Fin A — La Nouvelle Mythologie** : le joueur rétablit les dieux, le monde retrouve une dimension divine, l'Arcano-Facteur retrouve une part de ses pouvoirs. **Fin B — La Retraite des Dieux** : le joueur accepte leur libération, ils choisissent leur existence, l'Arcano-Facteur garde son balai et devient voyageur. Aucune n'est « la bonne ».
- **Ton** : mystère d'abord, touche de SF absurde et de fantasy cosmique (inspirations d'ambiance seulement, rien de copié) ; humour discret, pince-sans-rire, jamais expliqué ; images poétiques simples ; détails concrets plutôt que grandes explications ; le joueur n'est pas un élu.
- **Forme** : 6 à 10 lignes courtes par Arcane (hors titre et choix), lisibles en 80 colonnes, `tu`, typographie du projet ; pas de jargon SF, ni marque ni personne réelle.
- **Choix** : [1] / [2] sur chaque Arcane, dilemmes non moraux (conserver / transmettre, comprendre / accepter, protéger / libérer, observer / participer, se souvenir / laisser partir). Chaque choix change une phrase de l'Arcane suivante (variante naturelle) et une **variable cachée** cumulée ; la variable choisit la fin à l'Arcane n° 21. Pas de branches. Réponses dans `Progress` (`#[serde(default)]`). L'Arcane suivante n'arrive qu'une fois le choix fait.
- **Illustrations ASCII** compactes par Arcane (saison 1 : symboles divins, astres, couronnes, objets anciens), lisibilité d'abord.
- **Méthode** (validée à chaque étape par l'utilisateur·rice, qui relit tout, fin comprise) :
  1. architecture : synopsis, retournement, indices, deux fins ;
  2. tableau des 21 Arcanes : événement, personnage, fonction, indice, choix, deux variantes de la phrase suivante ;
  3. écriture par lots (1–7, 8–14, 15–21 + fins) ;
  4. intégration dans `src/story/content/` (ex. `arcanes.rs`), sans nouvelle mécanique inutile ;
  5. tests : deux parcours, variantes, fins.
- Hors périmètre : saison 2 et sa piste, piste 1 (#115), scène (#117), Badge.

## Acceptance criteria

- Étapes 1 et 2 (architecture, tableau des 21 Arcanes) validées par l'utilisateur·rice avant l'écriture.
- `weathr --arcane` (secret débloqué), soirs successifs → introduction de l'Arcano-Facteur, puis Arcanes n° 1 à 21 dans l'ordre, chacune avec son illustration, sa carte et son choix [1] / [2].
- Arcane sans réponse → la suivante n'arrive pas ; après la réponse → elle arrive au passage suivant.
- Même Arcane répondue [1] puis [2] (deux parties) → une phrase de l'Arcane suivante diffère.
- Deux jeux de réponses → Fin A « La Nouvelle Mythologie » ou Fin B « La Retraite des Dieux » à l'Arcane n° 21.
- Chaque Arcane : 6 à 10 lignes hors titre et choix, aucune ligne au-delà de 80 colonnes.
- Toutes les Arcanes et les deux fins relues et validées avant la fusion.
- Ancienne sauvegarde → se charge, aucune réponse d'Arcane.
- Tests unitaires : 21 Arcanes, variante selon la réponse précédente, fin selon la variable cachée, largeur des lignes.
- `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings -A clippy::double_must_use`, `cargo test --bin weathr` verts.

## Implementation

## Review

## Verification

## Feedback
