---
issue: 160
status: to-close
wiki:
note: Demande initiale « il faut fix la baseline du linter ». Aucune baseline n'existait, aucun linter actif.
created: 2026-11-24
---

## Context / Decisions

- Le projet n'avait aucun linter (`lint.tool: none`) et les notes de config disaient « no clippy ». Décision : activer clippy strict.
- Pas de baseline : clippy n'a pas d'outil de baseline, et un essai avec `-D clippy::all` n'a trouvé qu'une violation dans la lib (`src/animation/raindrops.rs:200`, `if` guard qui renvoie un booléen littéral). Les tests et le binaire restaient à vérifier.
- Toutes les violations sont corrigées dans cette task.
- Config : `lint.tool: clippy`, `fix_command: ""`. `build.command` préfixé : `cargo clippy --all-targets && cargo build` (**lint → Setup** 2).
- `Cargo.toml` : `[lints.clippy] all = "deny"`.
- Notes de config : retirer « no clippy » de la ligne Rust.
- Dépend de #158 (blocs `lint:` / `archtest:` dans la config). Non mergé → partir de sa branche ou attendre son merge.
- Version : v1.8.1.

## Acceptance criteria

- [ ] `cargo clippy --all-targets` passe sans erreur ni warning clippy.
- [ ] `Cargo.toml` a `[lints.clippy] all = "deny"`.
- [ ] `.whackagent/config.md` : `lint.tool: clippy`, `build.command: "cargo clippy --all-targets && cargo build"`, « no clippy » retiré des notes.
- [ ] `cargo test` passe.
- [ ] Une violation clippy ajoutée exprès fait échouer `build.command`.

## Implementation

- `Cargo.toml` : `[lints.clippy] all = "deny"`.
- `src/animation/raindrops.rs` l.199 : `clippy::needless_bool`, le `if … { return false; } true` devient `!(…)`. Même comportement, NaN compris. Seule violation : bin, tests, examples propres.
- `.whackagent/config.md` : `lint.tool: clippy`, `build.command: "cargo clippy --all-targets && cargo build"`, note projet sans « no clippy ».
- Aucun `#[allow]`.
- Proof : `cargo clippy --all-targets && cargo build` ✅ sans warning clippy · `cargo test` ✅ (721 tests) · gate : une violation ajoutée exprès → build en erreur (exit 101), retirée ensuite.
- Reste le warning cargo `package.homepage` redondant (`cargo::redundant_homepage`), déjà là avant, hors clippy.

## Review

### Round validation · 2026-11-24

- style ✓ · elegance ✓ · structure ✓ · correctness ✓
- Clean. `cargo clippy --all-targets` relancé : 0 erreur, 0 warning clippy. Fix raindrops équivalent, NaN compris.
- Autofix : rien.

## Verification

- manual validation — not run by agent

## Feedback
