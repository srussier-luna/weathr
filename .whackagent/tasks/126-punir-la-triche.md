---
issue: 126
status: todo
wiki: [[mail]], [[animation]], [[scene]], [[render]]
note: "Découpé de #124 (parapluie « Ajouter la détection de triche »), sprint « Anti-triche ». Lit le compteur de triches de #125."
created: 2026-11-17
---

## Context / Decisions

> fair une detection de triche, modification du fichier de sauvegarde et si le jouer est un tricheur le remarquer et proposer une punition,, soit plus de courrier pensant plusieur jours, soit un anti-badge pour qu'il soit puni ou alors une modification sur son jeu: les habitant disparaisse, les arbres perde leur feuille, si plusieurs triche plusieurs punition

- Bloqué par #125 : lit son compteur de triches.
- **Escalade**, chaque nouvelle triche ajoute la suivante :
  - 1re triche → anti-Badge **Tricheur**, gardé pour toujours, dans sa propre famille de Badges ;
  - 2e → **Village dégarni** pendant 7 Jours de jeu : Habitants absents du jardin, Arbre sans feuilles, puis tout revient ;
  - 3e et suivantes → **Jours sans courrier** : 3 jours à la 3e triche, + 3 par triche (6, 9…).
- **Jours sans courrier** :
  - Jour de jeu figé comme pour la Boîte pleine : rien n'est perdu, l'histoire reprend ensuite ;
  - aucun courrier livré, pas de tournée du Facteur ;
  - le village est dégarni pendant ces jours.
- **Mot des voisins** : fenêtre à l'ouverture du jeu, signée par les voisins, dans un cadre à elle.
  - Annonce la triche et sa punition.
  - Pendant les Jours sans courrier, revient à chaque ouverture avec une phrase de voisins déçus (« tu casses le fun », ils trouvent le joueur nul…), tirée d'une liste.
- **Textes** (annonce par niveau, phrases déçues, condition du Badge Tricheur, nom de sa famille) : proposés au `/wa-code`, validés par l'utilisateur·rice avant d'être figés.
- `--day N` → aucune punition affichée.
- Arcanes (sprint « Le secret », v2) pas encore dans `develop` : à leur arrivée, les Jours sans courrier arrêtent aussi l'Arcano-Facteur.
- Hors périmètre : repérer la triche (#125), racheter une punition.

## Acceptance criteria

- Compteur de triches à 1 au lancement → Mot des voisins (annonce) ; le Badge Tricheur apparaît dans les Badges, dans sa famille, et y reste après relance.
- Compteur à 2 → Mot des voisins ; pendant 7 Jours de jeu, aucun Habitant au jardin et l'Arbre sans feuilles ; le 8e, tout revient.
- Compteur à 3 → Mot des voisins ; 3 jours sans courrier : Jour de jeu figé, pas de Facteur, Courrier inchangé, village dégarni ; chaque ouverture pendant ces jours → Mot des voisins avec une phrase déçue ; ensuite l'histoire reprend où elle en était.
- Compteur à 4 → 6 jours sans courrier ; à 5 → 9 jours.
- `--day 43` → ni Mot des voisins ni punition.
- Tests : niveau de punition selon le compteur, durées, reprise de l'histoire.
- `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings -A clippy::double_must_use`, `cargo test` verts.

## Implementation

## Review

## Verification

## Feedback
