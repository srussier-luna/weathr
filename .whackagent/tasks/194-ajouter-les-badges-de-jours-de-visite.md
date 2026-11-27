---
issue: 194
status: todo
wiki: [[mail]], [[animation]]
note: "ajouter des badges ouvert x fois, pas forcement d'affilé, compte le nom de jours ou l'app est ouverte mais pas grave si on saute des jours, mettre ouvert 100 jours et ouvert 365 jours dans fidelité — pour la 1.9"
created: 2026-11-27
---

## Context / Decisions

- Compter les jours où le jeu est ouvert, pas forcément d'affilée : sauter des jours ne remet rien à zéro.
- Nouveau champ de sauvegarde : nombre de jours de visite, incrémenté une fois par jour dans `Progress::record_visit`, à côté de `streak`.
- Sauvegarde ancienne (champ absent) : le compteur démarre aux jours écoulés depuis `started_on`, jour de début et aujourd'hui inclus (choix du joueur, quitte à surestimer). Calculé une seule fois, puis incrémenté normalement.
- Champ en `#[serde(default)]` (ou `Option`) pour que les vieilles sauvegardes chargent toujours.
- Deux Badges dans la ligne Fidélité (`BadgeKind::Loyalty`), rangés après les séries d'affilée :
  - « Habitué » à 150 jours de visite.
  - « Pilier du village » à 365 jours de visite.
- Attribution dans `App::collect_badges`, comme les `streak_*`.
- Pas de Trophée de jardin pour ces deux Badges.
- Écran des Badges : la ligne du haut devient « Série en cours : N jours d'affilée · M jours de visite » (pluriel géré sur les deux).
- Test `test_loyalty_keeps_only_the_streaks` à adapter : la Fidélité n'a plus que des séries.
- Lignes de glossaire à écrire au close : Jours de visite, Habitué, Pilier du village.
- Sprint 1.9 (branche `sprint/1-9`).

## Acceptance criteria

- Nouvelle partie : le premier lancement affiche « 1 jour de visite » sur l'écran des Badges.
- Ouvrir le jeu un jour, sauter un jour, l'ouvrir le surlendemain : la série repart à 1, les jours de visite passent à 2.
- Rouvrir le jeu le même jour ne change pas le compteur.
- Ancienne sauvegarde sans le champ, `started_on` il y a 10 jours : au lancement, 11 jours de visite.
- 150 jours de visite → Badge « Habitué » gagné, carte de Badge affichée, rangé en Fidélité après « 365 jours d'affilée ».
- 365 jours de visite → Badge « Pilier du village » gagné, rangé après « Habitué ».
- Aucun nouveau Trophée dans le jardin.
- Les anciennes sauvegardes chargent sans erreur.

## Implementation

## Review

## Verification

## Feedback
