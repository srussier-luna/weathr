---
issue: 109
status: todo
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
- Code qui ne sert plus (lecture du `git config user.name` au lancement, listes de noms, tests associés) → supprimé.
- **Nettoyage** : noms et identifiants git de joueurs remplacés par « le joueur » / « le contributeur » dans les fichiers de tâches #108, #132, #133 et la page wiki [[mail]]. L'historique git les garde : pas de réécriture (push forcé sur develop et main).

## Acceptance criteria

- `git grep` sur `src/`, `.whackagent/wiki/` et les fichiers de tâches 108, 132, 133 : aucun nom ni identifiant git de joueur.
- Lancement avec n'importe quel `git config user.name` : aucune carte « Vigie du ciel » ni « Maître du temps ».
- Sauvegarde qui a déjà « Vigie du ciel » → écran Badges (`b`) : le Badge est toujours là, gagné, et compté dans Secrets.
- Même chose pour « Maître du temps ».
- Sauvegarde sans ces Badges → écran Badges : ni l'un ni l'autre, pas de « ??? » pour eux, total des Secrets réduit de 2.

## Implementation

## Review

## Verification

## Feedback
