---
issue: 184
status: to-close
wiki: [[updater]]
note: "Aujourd'hui le joueur choisit s'il veut la mise à jour. Pour certaines versions, une mise à jour forcée, sans demander l'autorisation."
created: 2026-11-26
---

## Context / Decisions

> ajouter un force update — aujourd'hui les user choisissent s'ils veulent la mise à jour, j'aimerais pour certaines versions ajouter un force upgrade, qui se fait sans demander l'autorisation

- **Marqueur** : version minimale dans `Cargo.toml`, `[package.metadata.weathr] min_version = "x.y.z"`. Montée à la main, à la release concernée.
- **Lecture** : après le `git fetch` de `offer_update`, lire `min_version` dans le `Cargo.toml` distant (`git show @{u}:Cargo.toml`). Version locale (`CARGO_PKG_VERSION`) plus vieille → mise à jour forcée. Champ absent ou illisible → comportement actuel.
- **Forcée** = pas de question `[O/n]` : bannière avec le message actuel « Une nouvelle version est arrivée ! », puis « Installation en cours » et redémarrage, comme une mise à jour acceptée.
- **Garde-fous gardés** : dépôt local modifié → pas de mise à jour ; `--silent` → pas de mise à jour.
- **Sans clavier** (stdin pas un terminal : script, tâche planifiée) : la mise à jour forcée s'installe quand même. La mise à jour proposée reste ignorée dans ce cas.
- **Échec** : comme aujourd'hui (`[weathr] Mise à jour impossible. Détails dans …`, journal `update.log`), puis l'ancienne version se lance. Sans clavier, pas d'attente d'Entrée. Nouvel essai au lancement suivant.
- `--update` inchangé.
- Vérification à chaque lancement, comme aujourd'hui (la limite d'une fois par jour n'existe plus depuis 11d6d67 ; le wiki updater est à corriger).
- Hors périmètre : pas de serveur, pas de tag git, pas de liste des changements.

Glossaire à ajouter :

| Terme | Évite | Code | Sens |
|---|---|---|---|
| Mise à jour forcée | force update | `min_version` | mise à jour installée sans question quand la version locale est plus vieille que la version minimale du dépôt → [[updater]] |

## Acceptance criteria

- Dépôt distant avec `min_version` > version locale, commits en attente → au lancement : bannière « Une nouvelle version est arrivée ! », pas de `[O/n]`, « Installation en cours », redémarrage sur la nouvelle version.
- `min_version` ≤ version locale ou absent → question `[O/n]` comme aujourd'hui.
- `min_version` illisible (« abc ») → question `[O/n]` comme aujourd'hui, pas d'erreur.
- Dépôt local modifié → rien, même si forcée.
- `--silent` → rien, même si forcée.
- Lancé sans clavier (`weathr < /dev/null`) : forcée → installe ; pas forcée → rien.
- Build en échec pendant une forcée → message « Mise à jour impossible », journal écrit, puis l'ancienne version se lance (sans attendre Entrée quand il n'y a pas de clavier).

## Implementation

- `src/updater/forced.rs` (nouveau) : `is_forced(local, manifest)` ; `min_version` lu avec `toml::Table` dans `[package.metadata.weathr]` ; version = exactement 3 nombres, sinon (suffixe, autre longueur, pas une chaîne, TOML invalide) → pas forcée. 12 tests.
- `src/updater/mod.rs` : `is_update_forced(source)` = `git show @{u}:Cargo.toml` + `forced::is_forced(CARGO_PKG_VERSION, …)`. `offer_update` : dépôt, propre, fetch (une fois), commits en attente, puis forcée → bannière, pas de question, installation ; pas forcée → seulement avec un terminal. Erreur → message, Entrée seulement avec un terminal, l'ancienne version se lance.
- Effet de bord accepté : sans terminal, le lancement fait maintenant `git status` + `fetch` (5 s max).
- Inchangés : `main.rs`, `--update`, `Cargo.toml` (pas de `min_version`).
- Glossaire : Mise à jour forcée. Wiki updater : flux forcé, vérification à chaque lancement (ligne « une fois par jour » périmée corrigée).
- Preuve : clippy, build, tests, fmt verts.

## Review

Round validation — 2026-11-26, one wa-verifier, LENSES style ✓ elegance ✓ structure ✓ correctness ✓.
- 🟡 [style] wiki updater : bannière « (prompt only) » → « (proposed or forced update) ». Corrigé.
- 🟡 [style] wiki updater : numéros de ligne `mod.rs` périmés (16, 21, 51, 149, 291). Corrigé.
- Code : aucune remarque. Tests verts (dont 12 `updater::forced`).
- Test joueur : mise à jour forcée installée depuis une source locale (`test/184-upstream`, `min_version = "1.8.1"`).

## Verification

round 1: manual validation — not run by agent

## Feedback
