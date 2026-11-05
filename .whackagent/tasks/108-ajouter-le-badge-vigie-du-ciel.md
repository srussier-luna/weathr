---
issue: 108
status: to-test
wiki: [[mail]]
note: badge secret « Vigie du ciel » pour Denis (merci pour le retour #104) : vérifier si le joueur est drichard-luna et lui donner le badge à la 1.7.
created: 2026-11-05
---

## Context / Decisions

- Remerciement à Denis pour son retour sur l'arrivée des nuages (#104).
- **Identité** : `git config user.name` lancé dans `SOURCE_DIR` (le dossier source que l'updater utilise déjà) vaut exactement `drichard-luna` → badge gagné.
  - git absent, dossier sans dépôt, ou nom différent → pas de badge, aucune erreur.
  - Pas de réseau, pas d'email, pas de `gh`.
- **Badge** : nouveau secret dans `BADGES` (`src/badges.rs`), id `sky_lookout`, nom « Vigie du ciel ».
  - Indice une fois gagné : « a repéré les nuages pressés ».
  - Icône longue-vue : `" .-. "`, `"(___)"`, `"  \\o "`.
- **Visibilité** : case cachée comme les autres secrets, pour tous ; compte dans le total (Badges x/43).
- Gagné via la liste des jalons de `collect_badges` (`src/app.rs`), annonce de badge existante, aussi en mode simulé.
- Hors périmètre : mention dans `NOUVEAUTES.md` (aucun spoiler), autre joueur, autre moyen d'identification.

## Acceptance criteria

- `git config user.name` = `drichard-luna` dans le dossier source → au lancement, le badge « Vigie du ciel » est annoncé puis coché dans `b` avec son indice.
- Autre nom, ou git indisponible → badge non gagné, affiché en case cachée, aucune erreur.
- Écran Badges : total à 43 pour tout le monde.
- Test unitaire : la règle d'identité renvoie vrai pour `drichard-luna`, faux pour un autre nom et pour une absence de nom.

## Implementation

- `src/badges.rs` : secret `sky_lookout` « Vigie du ciel » (43 badges), const `SKY_LOOKOUT_LOGIN`, `is_sky_lookout(Option<&str>)` (trim + égalité exacte), 4 tests.
- `src/updater/mod.rs` : `git_user_name()` → `git config user.name` dans `SOURCE_DIR` via `git_output`.
- `src/app.rs` : champ `is_sky_lookout` calculé une fois dans `App::new`, jalon `sky_lookout` dans `collect_badges`.
- `git config` hors dépôt lit quand même la config globale ; git absent ou nom non défini → pas de badge.
- Sous `--day N`, pas de badge (règle `awards_badges` existante).
- `cargo build` vert · `cargo test` vert · `cargo fmt -- --check` propre.

## Review

## Verification

manual validation — not run by agent

## Feedback
