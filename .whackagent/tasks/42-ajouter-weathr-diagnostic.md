---
issue: 42
status: todo
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

## Review

## Verification

## Feedback
