---
issue: 198
status: to-test
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

- Nouveaux : `src/animation/santa.rs` (`Sleigh`, traverse le ciel une fois, gauche → droite, 21×3), `src/animation/reindeer.rs` (`Reindeer`, 3×3, nez rouge), `src/progress/reindeer.rs` (`is_santa_due`, `welcome_reindeer`, `has_reindeer`, `awaits_reindeer_name`, `name_reindeer`, `reindeer_name`).
- Sauvegarde : `reindeer_arrived_on`, `reindeer_name` (`#[serde(default)]`, fin de `Progress`).
- `src/animation/trophies.rs` : `Trophies::free_spot(height, layout, river)` ; le renne se place dans la rangée des Trophées après le Nain et la Statue, avant la clôture, hors rivière ; pas de place → pas dessiné.
- `src/app.rs` : `Overlay::Reindeer`, garde date réelle (`santa_due_on_launch`), le traîneau part une fois les cartes de lancement fermées (Bienvenue, Noël, Badges). Arrivée enregistrée au départ du traîneau.
- Carte « Bébé renne » : « Le Père Noël vient de le déposer au jardin. » / « Comment veux-tu l'appeler ? », champ « > nom_ », 20 caractères max, `[Entrée] valider` ; Echap ne ferme pas. Vide → Rudolf.
- Ajout hors spec (petit, retirable) : toast « Bienvenue au jardin, <nom> » après validation — seul endroit où le nom réapparaît (aucun écran ne nomme les Habitants).
- Quitter avant de nommer : la carte revient le 25 ; dès le 26, Rudolf.
- Village dégarni : le renne est caché comme les autres Habitants.
- Build : clippy OK, build OK, fmt OK ; tests 189 + 651 + 10 + 7 + 3 + 5, 0 échec (19 nouveaux).

## Review

## Verification

round 1: build + tests only — date réelle non simulable. À tester : run: yours.

## Feedback
