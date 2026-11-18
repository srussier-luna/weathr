---
issue: 133
status: todo
wiki: [[mail]]
note: il faudrait ajouter un badge comme vigie du ciel mais pour les gens qui aide a l'amelioration du projet en participant au code, que propose tu
created: 2026-11-18
---

## Context / Decisions

- Badge secret pour qui a contribué au code du projet, sur le même modèle que « Vigie du ciel » (#108).
- **Identité** : `updater::git_user_name()` (`git config user.name` dans `SOURCE_DIR`, déjà lu une fois dans `App::new`) présent dans une liste écrite dans le code.
  - Liste de départ : `srussier-luna`, `bpisano` (noms d'auteur des commits du fork).
  - Chaque nouveau contributeur → une tâche dédiée l'ajoute à la liste, pour la version suivante.
  - Pas de détection automatique par l'historique git, pas d'API GitHub.
- **Badge** : nouveau secret dans `BADGES` (`src/badges.rs`), id `time_master`, nom « Maître du temps » (clin d'œil à la Timeline de bpisano).
  - Indice une fois gagné : « a mis le village à l'heure ».
  - Icône sablier : `"\\___/"`, `" )_( "`, `"/___\\"`.
- **Visibilité** : comme Vigie du ciel — case cachée pour tous, compte dans le total (Badges x/44).
- Réutiliser le mécanisme de Vigie du ciel : un seul appel git au lancement, partagé par les deux badges ; jalon dans `collect_badges`.
- Hors périmètre : mention dans `NOUVEAUTES.md` (aucun spoiler), badge par personne.

## Acceptance criteria

- `git config user.name` = `srussier-luna` ou `bpisano` → au lancement, « Maître du temps » annoncé puis coché dans `b` avec son indice.
- Autre nom, ou git indisponible → badge non gagné, case cachée, aucune erreur.
- `drichard-luna` garde Vigie du ciel et n'a pas Maître du temps.
- Écran Badges : total à 44.
- `git` lancé une seule fois au démarrage pour les deux badges.
- Tests unitaires : vrai pour chaque nom de la liste (espaces autour ignorés), faux pour un autre nom, vide ou absent.

## Implementation

## Review

## Verification

## Feedback
