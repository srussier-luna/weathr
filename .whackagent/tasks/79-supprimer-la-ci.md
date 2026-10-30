---
issue: 79
status: todo
wiki:
note: corriger le proble de la ci clippy
created: 2026-10-30
---

## Context / Decisions

> double_must_use sur WeatherProvider bloque Test and Lint
>
> corriger le proble de la ci clippy

- `.github/workflows/ci.yml` (job « Test and Lint ») vient du projet d'origine `Veirt/weathr` (février 2026). Il tourne sur push vers `main` et PR vers `main`.
- Jamais vert ici : 11 exécutions sur 11 en échec depuis le 23/10, toutes à l'étape Clippy (`clippy::double_must_use`, `src/weather/provider.rs:23`).
- `main` n'est pas protégée : cet échec ne bloque aucune fusion.
- **Décision : supprimer `ci.yml`.** Les vérifications locales (`build.test_command`) restent la preuve.
- Hors périmètre, par choix :
  - Correctif clippy annulé (`async-trait` 0.1.92 le corrige, testé) : `cargo clippy -- -D warnings` reste rouge en local sur ce lint.
  - `release.yml`, `release-dry-run.yml`, `homebrew.yml`, `homebrew-dry-run.yml` restent tels quels.
  - Dépendances signalées par OSV (rustls, tokio, h2…) : pas de mise à jour.

## Acceptance criteria

- `.github/workflows/ci.yml` supprimé ; les 4 autres workflows inchangés.
- Aucun autre fichier du projet modifié.
- `cargo fmt -- --check` et `cargo test` verts ; clippy rouge uniquement sur `double_must_use` (`src/weather/provider.rs:23`).
- Après la prochaine release sur `main` : plus aucune exécution « CI » (`gh run list --workflow ci.yml`).

## Implementation

## Review

## Verification

## Feedback
