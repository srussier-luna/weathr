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

- 25 décembre, premier lancement : le traîneau traverse le ciel, le bébé renne apparaît au jardin, puis un message du Père Noël (merci d'avoir pris soin de Chocolutin, il espère qu'il n'a pas fait n'importe quoi, le renne est un cadeau de remerciement), puis la carte demande son nom.
- Nom saisi « Comète » → le renne s'appelle Comète partout où son nom apparaît.
- Nom laissé vide → Rudolf.
- Relancer le 25 : pas de nouveau passage.
- 26 décembre sans avoir ouvert le 25 : pas de renne.
- Le renne reste au jardin les jours et mois suivants.

## Implementation

stacked on: #203 (wa/203-ajouter-la-date-de-test, contient #196 et #197)

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

round 3: manual validation — not run by agent.

round 2: manual validation — not run by agent.

round 1: build + tests only — date réelle non simulable. À tester : run: yours.

## Feedback

### Round 2 — 2026-11-27

- Asked : « le jour où on reçoit le renne, le Père Noël nous laisse un message pour nous dire merci de nous être occupés de Chocolutin et qu'il espère qu'il n'a pas fait n'importe quoi, et du coup en remerciement on a le bébé renne ».
- Triage : ajustement (critère mis à jour).
- Changes (implementer) : `src/story/content/santa_note.rs` (nouveau : texte + signature « — Le Père Noël ») ; `Overlay::SantaNote` avant la carte du nom (cadre Parchment, « [Entrée] continuer », Entrée ou Echap → carte du nom) ; carte du nom sans « Le Père Noël vient de le déposer au jardin ». Relance le 25 avant d'avoir nommé → message puis carte ; dès le 26 → Rudolf.
- Review : deferred. Verify : manual, run: yours.
- Build : clippy OK, fmt OK, tests 189 + 655 + 10 + 7 + 3 + 5, 0 échec.

### Round 3 — 2026-11-27

- Asked : « ok, continue » — mettre tout le sprint Avent dans la branche de test.
- Triage : empilement sur #203 (conflits src/app.rs aux 2 arrêts, résolus par l'implementer).
- Changes : `santa_due_on_launch(is_real_date, …)` réutilise la garde de #196 ; `clock::now()` partout ; ordre au lancement le 25 : cartes de base, tour de Chocolutin (pas le 25), traîneau, renne, message du Père Noël, carte du nom.
- Review : deferred. Verify : manual, run: yours.
- Build : clippy OK (avec et sans dev-date), fmt OK ; tests sans 195 + 705 + 10 + 7 + 3 + 5, avec 199 + 710 + 10 + 7 + 3 + 5, 0 échec.

