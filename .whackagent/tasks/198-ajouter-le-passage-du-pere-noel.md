---
issue: 198
status: todo
wiki: [[animation]]
note: "split de #196 (calendrier de l'Avent)"
created: 2026-11-27
---

## Context / Decisions

- Le **25 décembre** (date réelle, jamais en `--day`/démo), au premier lancement du jour, le **Père Noël** passe en **traîneau** dans le ciel.
- Il dépose un **bébé renne** au jardin, puis une carte l'annonce.
- Seulement pour qui lance le jeu le 25. Pas de rattrapage le 26.
- La carte demande au joueur de nommer le renne (champ texte). Nom vide → **Rudolf**.
- Le bébé renne devient un **Habitant pour toujours**, toute l'année, comme Biscuit ou Minouche.
- Placement dans le jardin sans chevaucher les autres Habitants ni les Trophées ; pas de place → pas dessiné.
- Indépendante de #196 : peut être codée en parallèle.
- Lignes de glossaire au close : Père Noël, Traîneau, Bébé renne.

## Acceptance criteria

- 25 décembre, premier lancement : le traîneau traverse le ciel, le bébé renne apparaît au jardin, puis la carte demande son nom.
- Nom saisi « Comète » → le renne s'appelle Comète partout où son nom apparaît.
- Nom laissé vide → Rudolf.
- Relancer le 25 : pas de nouveau passage.
- 26 décembre sans avoir ouvert le 25 : pas de renne.
- Le renne reste au jardin les jours et mois suivants.

## Implementation

## Review

## Verification

## Feedback
