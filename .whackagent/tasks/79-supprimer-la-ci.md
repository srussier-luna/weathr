---
issue: 79
status: to-close
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
- **Décision (validation) : preuve = `cargo build` + `cargo test`**, plus de fmt ni de clippy. `.whackagent/config.md` mis à jour (`test_command`, note du projet).
- Hors périmètre, par choix :
  - Correctif clippy annulé (`async-trait` 0.1.92 le corrige, testé) : `cargo clippy -- -D warnings` reste rouge en local sur ce lint.
  - `release.yml`, `release-dry-run.yml`, `homebrew.yml`, `homebrew-dry-run.yml` restent tels quels.
  - Dépendances signalées par OSV (rustls, tokio, h2…) : pas de mise à jour.

## Acceptance criteria

- `.github/workflows/ci.yml` supprimé ; les 4 autres workflows inchangés.
- Seul autre fichier modifié : `.whackagent/config.md` (`test_command: "cargo test"`, note « Proof »).
- `cargo build` et `cargo test` verts.
- Après la prochaine release sur `main` : plus aucune exécution « CI » (`gh run list --workflow ci.yml`).

## Implementation

- `.github/workflows/ci.yml` supprimé ; aucun autre fichier touché.
- `cargo build` vert.
- `cargo fmt -- --check` vert.
- `cargo clippy -- -D warnings` rouge, seule erreur `clippy::double_must_use` (`src/weather/provider.rs:23`), hors périmètre.
- `cargo test` vert.
- Validation : `.whackagent/config.md` → `test_command: "cargo test"`, note « Proof = `cargo build` + `cargo test` — no CI, no fmt, no clippy ». Build + test verts.

## Review

Round validation — 4 lenses ✓.
- 🟡 `config.md:172` — note « Proof = CI gate » périmée → fixed.
- 🟡 `config.md:84` — `test_command` bloqué par clippy rouge, `cargo test` jamais lancé → fixed (`cargo test`).
- Re-verify : ✅ review clean.

## Verification

manual validation — not run by agent

## Feedback
