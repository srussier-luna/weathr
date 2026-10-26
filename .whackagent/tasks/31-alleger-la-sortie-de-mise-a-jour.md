---
issue: 31
status: todo
wiki: [[updater]]
note: "on garde « [weathr] Une nouvelle version est disponible : », « [weathr] Mettre à jour maintenant ? [O/n] o », « [weathr] Installation en cours » ; mettre un /|\\- qui tourne en mode loader à côté de Installation en cours avec une progress bar ; pas besoin de toutes ces lignes (git pull, fichiers, cargo)"
created: 2026-10-26
---

## Context / Decisions

> on garde « [weathr] Une nouvelle version est disponible : », « [weathr] Mettre à jour maintenant ? [O/n] o », « [weathr] Installation en cours » ; mettre un /|\- qui tourne en mode loader à côté de Installation en cours avec une progress bar ; pas besoin de toutes ces lignes

- No blocker, no sprint. All in `src/updater.rs` ([[updater]]).
- **Output of an accepted update, nothing else**:
  ```
  [weathr] Une nouvelle version est disponible.
  [weathr] Mettre à jour maintenant ? [O/n] o
  [weathr] Installation en cours /  [=========>          ] 45 %
  ```
  - The change list (`• <commit subject>`, `… et N autres`) is removed: the Nouveautés box (#24) tells what changed after restart.
  - `git pull` and `cargo build` output fully captured, never printed.
  - Last line rewritten in place (`\r`): spinner `/ | \ -` turning, bar and percentage.
  - No « Mise à jour terminée, redémarrage… » line: bar reaches 100 %, then weathr restarts.
- **Progress**:
  - `git pull --ff-only` done → 10 %.
  - `cargo build --release --message-format=json`: each `compiler-artifact` message moves 10 % → 95 %, total = package count of `Cargo.lock` (cap at 95 % if the count is off).
  - `install` done → 100 %.
  - Spinner turns on its own (timer), so it moves even while one crate compiles for long.
- **Failure** (git, cargo or install error):
  - Captured output written to `<config>/weathr/update.log` (overwritten each update).
  - Line ends, then `[weathr] Mise à jour impossible. Détails dans <path>`, wait for Enter, as today.
- `--update` flag (`update_now`): same display, no prompt, no restart. « weathr est à jour. » unchanged.
- stdout not a TTY → no spinner or `\r`: one line `Installation en cours…`, then nothing until done or error.
- Location lines (`Auto-detecting location…`) out of scope: printed on every launch, not by the updater.
- Wording: `{conventions}/wording.md` — short system messages, `[weathr] …`, factual.

## Acceptance criteria

- [ ] `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` all green.
- [ ] Update available, answer `o`: only the three lines above are shown; no git file list, no `Compiling…`, no change list.
- [ ] During the build: spinner turns and the percentage rises past 10 %; bar reaches 100 % before the restart.
- [ ] No « Mise à jour terminée » line before the restart.
- [ ] Failing build (e.g. broken source): `[weathr] Mise à jour impossible. Détails dans …/update.log`; that file holds cargo's error.
- [ ] `weathr --update`: same progress line, then exits.
- [ ] Output piped (`weathr --update | cat`): no `\r` frames, one plain line.
- [ ] Progress math tested: artifacts / total → percentage, capped at 95 % until install (test).

## Implementation

## Review

## Verification

## Feedback
