---
issue: 42
status: to-test
wiki: [[updater]], [[weather]], [[architecture]]
note: "weathr --diagnostic : vérifie git, cargo, le clone, le réseau, les droits et dit quoi corriger"
created: 2026-10-27
---

## Context / Decisions

> weathr --diagnostic : vérifie git, cargo, le clone, le réseau, les droits et dit quoi corriger

- Blocked by #31 (moves the updater to `src/updater/`, adds `config_path`), no sprint. New CLI flag `--diagnostic` (`src/main.rs`, clap, like `--update`): runs the checks, prints, exits. No TUI, no update prompt.
- **Checks**, in this order:
  - Tools: `git` and `cargo` on `PATH`, version shown.
  - Source clone: the compile-time source dir (`SOURCE_DIR`, [[updater]]) exists, is a git repo, clean (untracked ignored), has an upstream branch, up to date or N commits behind.
  - Network: Open-Meteo API reachable; `git fetch` of the clone. 5 s timeout each, no prompt (same env as the updater fetch).
  - Rights: config dir writable, `progress.json` readable and valid (or absent), running binary's folder writable (needed by auto-update `install`).
- **Output**: one line per check, `✓ <check> <detail>` or `✗ <check> <problem> → <what to do>`.
  - Ends with a summary line: `Tout est en ordre.` or `<n> problème(s) à corriger.`
  - Exit code 0 when all ✓, 1 if any ✗.
  - A failed check never stops the next ones.
- Fix hints in French, `tu`, short and actionable (e.g. `cargo introuvable → installe Rust : https://rustup.rs`). `{conventions}/wording.md`, error situation.
- Reuse the updater's helpers (`is_git_checkout`, `is_clean`, fetch with timeout, `config_path`) instead of copying them; expose them `pub(crate)` if needed.
- New module by feature: `src/diagnostic.rs` (or `src/diagnostic/` if it grows), checks as pure functions returning a result struct, printing separate.
- Out of scope: auto-fixing anything, checking location/geolocation, `--diagnostic` inside the TUI.

## Acceptance criteria

- [ ] `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` all green.
- [ ] `weathr --diagnostic` on a healthy install: every line `✓`, `Tout est en ordre.`, exit code 0.
- [ ] `PATH` without cargo: `✗ cargo …` with the rustup hint, other checks still run, exit code 1.
- [ ] Dirty source clone: `✗` on the clone check with a hint to commit or discard local changes.
- [ ] Offline: both network checks `✗` within ~10 s total, hint to check the connection.
- [ ] `progress.json` containing `{oops`: `✗` on that check, hint pointing to the file.
- [ ] `weathr --help` lists `--diagnostic`.
- [ ] Result formatting tested: ✓ / ✗ lines and summary from a list of results (test).

## Implementation

- Nouveau module `src/diagnostic/` : `mod.rs` (`run()`, sondes), `checks.rs` (`Check` / `Status`, fonctions pures + tests), `report.rs` (`render` ✓/✗ + résumé, `problem_count`, tests).
- `src/main.rs` : flag `--diagnostic`, traité avant `--update`, code de sortie 0 / 1.
- Helpers de l'updater passés en `pub(crate)` (SOURCE_DIR, is_git_checkout, git_output, is_clean, fetch) ; nouveaux `pending_count` et `config_dir` réutilisés par l'existant. `progress_path` et `FORECAST_URL` en `pub(crate)`.
- Choix : clone en retard sur l'upstream = ✓ avec « N commit(s) de retard, lance weathr --update », pas ✗.
- Tests d'écriture : fichier `.weathr-diagnostic` créé puis supprimé ; dossier de config absent → parent existant le plus proche.
- git absent → clone « impossible à vérifier sans git », fetch ✗.
- Build : `cargo build`, `cargo fmt --check`, `cargo test` verts. Clippy : seule l'erreur connue #79 (propre avec `-A clippy::double_must_use`).

## Review

## Verification

Lancé avec un HOME jetable (jamais le vrai) :
- ✅ `--help` liste `--diagnostic`.
- ✅ 8 lignes dans l'ordre, résumé, exit 1 (clone ✗ attendu : worktree non commité).
- ✅ PATH sans cargo → `✗ cargo introuvable → installe Rust : https://rustup.rs`, les autres continuent, exit 1.
- ✅ progress.json `{oops` → ✗ avec indication du fichier.
- ⚠️ Hors ligne simulé par proxy : Open-Meteo ✗ en 5 s ; git fetch (ssh) non affecté, couvert par test unitaire.
- ⚠️ Installation saine « Tout est en ordre. », exit 0 : non vue de bout en bout (worktree sale), couverte par test.

## Feedback
