---
issue: 109
status: to-close
wiki: [[mail]]
note: plus aucun nom de joueur dans le code public ; aussi pour le Maître du temps
created: 2026-11-26
---

## Context / Decisions

> plus aucun nom de joueur dans le code public

- Le dépôt est public : aucun nom ni identifiant git de joueur ne doit rester dans les fichiers actuels.
- **Décision** : retirer la vérification par nom des deux Badges secrets « Vigie du ciel » (`sky_lookout`, #108, #132) et « Maître du temps » (`time_master`, #133). Pas d'empreinte (hash) : plus aucune comparaison de nom.
- Plus personne ne gagne ces deux Badges. Qui les a déjà les garde : la sauvegarde n'est pas touchée, les ids restent connus.
- **Affichage** : chacun de ces deux Badges n'apparaît dans l'écran Badges que si la sauvegarde l'a déjà. Sinon rien, pas de case « ??? », et il ne compte pas dans le total des Secrets.
- **Gardé pour de futurs Badges contributeur** : lecture du `git config user.name` au lancement (`updater::git_user_name`), comparaison `is_listed`, branchement de « Maître du temps » sur `CONTRIBUTORS`, qui devient une liste vide. Ajouter un nom plus tard suffit.
- Ce code gardé porte un commentaire demandant aux agents de ne pas le supprimer (demande explicite, exception à la règle sans commentaire).
- « Vigie du ciel » : nom en dur, fonction et tests de nom supprimés.
- Tests : plus aucun nom réel ; les tests de comparaison utilisent un nom inventé.
- **Nettoyage** : noms et identifiants git de joueurs remplacés par « le joueur » / « le contributeur » dans les fichiers de tâches #108, #132, #133 et la page wiki [[mail]]. L'historique git les garde : pas de réécriture (push forcé sur develop et main).

## Acceptance criteria

- `git grep` sur `src/`, `.whackagent/wiki/` et les fichiers de tâches 108, 132, 133 : aucun nom ni identifiant git de joueur.
- Lancement avec n'importe quel `git config user.name` : aucune carte « Vigie du ciel » ni « Maître du temps ».
- Le code de lecture du nom git et la liste `CONTRIBUTORS` (vide) sont toujours là, avec le commentaire « ne pas supprimer ».
- Sauvegarde qui a déjà « Vigie du ciel » → écran Badges (`b`) : le Badge est toujours là, gagné, et compté dans Secrets.
- Même chose pour « Maître du temps ».
- Sauvegarde sans ces Badges → écran Badges : ni l'un ni l'autre, pas de « ??? » pour eux, total des Secrets réduit de 2.

## Implementation

- `src/badges.rs` : `SKY_LOOKOUT_NAME` et `is_sky_lookout` supprimés ; `CONTRIBUTORS: &[&str] = &[]` ; `is_listed` et `is_time_master` gardés. Liste privée `RETIRED = ["sky_lookout", "time_master"]` + `is_shown(badge, earned)` : affiché si gagné, sinon ni retiré ni Mauvais point (règle Tricheur reprise, un seul mécanisme). `collection(earned)` filtre avec `is_shown`.
- `src/overlay.rs` : `badge_lines` saute les Badges cachés et les familles vides ; titre « Badges x/y » selon la sauvegarde.
- `src/app.rs` : champ et ligne d'attribution `is_sky_lookout` supprimés ; lecture `git_user_name()` et `is_time_master` gardées.
- 2 commentaires « Kept on purpose for future contributor Badges: do not remove. » : au-dessus de `CONTRIBUTORS` et de la lecture du nom dans `App::new`.
- Tests : nom inventé pour `is_listed` ; `CONTRIBUTORS` vide → jamais Maître du temps ; Badges retirés cachés sans sauvegarde, affichés et comptés une fois gagnés. Plus aucun nom réel dans `src`.
- Si un nom revient dans `CONTRIBUTORS` : retirer `time_master` de `RETIRED`, sinon les autres joueurs ne le voient pas.
- Preuve : clippy (hors `double_must_use` déjà sur develop), build, fmt, tests verts.

## Review

Round validation — 2026-11-26, one wa-verifier, LENSES style ✓ elegance ✓ structure ✓ correctness ✓.
- 🟠 [correctness] identifiant git du joueur encore dans la tâche 133 → remplacé par `<identifiant du joueur>` / « le joueur ».
- 🟡 [elegance] collection des Badges parcourue deux fois (`App::refresh`, `badges_title`) → autofix : `badges::collection_progress(earned) -> (gagnés, total)`.
- Round 2 : clean. Tests 801 verts, fmt OK.
- Hors revue, à la demande : prénom retiré de la tâche 104 ; NOUVEAUTES.md laissé tel quel.

## Verification

round 1: manual validation — not run by agent

## Feedback
