---
issue: 158
status: to-close
wiki:
note: Suivi de /wa-update (whackagent 0.40.1)
created: 2026-11-24
---

## Context / Decisions

- `/wa-update` vers whackagent 0.40.1 : migrations 0.30, 0.31, 0.32 + sync config.
- `lint.tool: none` : notes projet = pas de clippy.
- `archtest.tool: none` : aucun outil pour Rust.
- `pr_screenshots.replay: off` : TUI Rust, pas d'app iOS.
- `commit.steps: on` : clé ajoutée avec la valeur par défaut du template.
- `.DS_Store` ajouté au `.gitignore` : fichiers macOS jamais suivis.
- Version : v1.8.1, release en préparation.

## Acceptance criteria

- [ ] `.whackagent/config.md` a les blocs `lint:` et `archtest:` (tool `none`), `pr_screenshots.replay: off`, `commit.steps: on`.
- [ ] Aucune autre valeur de config changée.
- [ ] `git status` ne liste plus aucun `.DS_Store`.
- [ ] `cargo build` et `cargo test` passent.

## Implementation

- `.whackagent/config.md` : blocs `lint:` et `archtest:` (`tool: none`), `pr_screenshots.replay: off`, `commit.steps: on`. Commentaires repris du template 0.40.1.
- `.gitignore` : `.DS_Store`.
- Proof : `cargo build` ✅ · `cargo test` ✅ (706 tests).

## Review

### Round validation · 2026-11-24

- style ✓ · elegance ✓ · structure ✓ · correctness ✓
- Clean. Clés identiques au template 0.40.1, aucune autre valeur touchée, YAML valide.
- Autofix : rien.

## Verification

## Feedback
