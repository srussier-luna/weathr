---
issue: 31
status: to-test
wiki: [[updater]]
note: "on garde « [weathr] Une nouvelle version est disponible : », « [weathr] Mettre à jour maintenant ? [O/n] o », « [weathr] Installation en cours » ; mettre un /|\\- qui tourne en mode loader à côté de Installation en cours avec une progress bar ; pas besoin de toutes ces lignes (git pull, fichiers, cargo)"
created: 2026-10-26
---

## Context / Decisions

> on garde « [weathr] Une nouvelle version est disponible : », « [weathr] Mettre à jour maintenant ? [O/n] o », « [weathr] Installation en cours » ; mettre un /|\- qui tourne en mode loader à côté de Installation en cours avec une progress bar ; pas besoin de toutes ces lignes

- No blocker, no sprint. All in `src/updater.rs` ([[updater]]).
- **Output of an accepted update, nothing else**:
  ```
  ██╗    ██╗███████╗ █████╗ ████████╗██╗  ██╗██████╗
  ██║    ██║██╔════╝██╔══██╗╚══██╔══╝██║  ██║██╔══██╗
  ██║ █╗ ██║█████╗  ███████║   ██║   ███████║██████╔╝
  ██║███╗██║██╔══╝  ██╔══██║   ██║   ██╔══██║██╔══██╗
  ╚███╔███╔╝███████╗██║  ██║   ██║   ██║  ██║██║  ██║
   ╚══╝╚══╝ ╚══════╝╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝

                    __o       .--.
                  _`\<,_     |____|>
                 (*)/ (*)      ||
               ~~~~~~~~~~~~~~~~~~~~~

  Une nouvelle version est arrivée !
  [weathr] Mettre à jour maintenant ? [O/n] o
  [weathr] Installation en cours /  [=========>          ] 45 %
  ```
  - Banner (feedback round 1) replaces « [weathr] Une nouvelle version est disponible. » before the prompt (`offer_update`); `--update` shows it too, before the progress line (feedback round 2).
  - Logo « WEATHR » in ANSI Shadow, bold, sunset gradient per column (stops `255,200,60` → `255,110,80` → `220,60,140` → `140,70,200`, 24-bit colour); outline chars `╗╔╝╚═║` in the same colour ×0.45.
  - Postman centered under the logo: two yellow rows `255,210,70`, wheels grey `150,150,165`, ground `~` green `110,200,110`.
  - « Une nouvelle version est arrivée ! » in the gradient colour at 0.3.
  - Reference rendering: scratchpad `logo_preview4.py`, variant N.
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

- [ ] `cargo fmt --check`, `cargo test` all green (clippy ignored for this task, feedback round 2).
- [ ] Update available, answer `o`: only the banner, the prompt and the progress line above are shown; no git file list, no `Compiling…`, no change list.
- [ ] Banner: « WEATHR » block logo in sunset gradient with darker outline, postman centered below, then « Une nouvelle version est arrivée ! », then the prompt.
- [ ] `weathr --update`: banner, then the progress line, then « weathr est à jour. ».
- [ ] During the build: spinner turns and the percentage rises past 10 %; bar reaches 100 % before the restart.
- [ ] No « Mise à jour terminée » line before the restart.
- [ ] Failing build (e.g. broken source): `[weathr] Mise à jour impossible. Détails dans …/update.log`; that file holds cargo's error.
- [ ] `weathr --update`: same progress line, then exits.
- [ ] Output piped (`weathr --update | cat`): no `\r` frames, one plain line.
- [ ] Progress math tested: artifacts / total → percentage, capped at 95 % until install (test).

## Implementation

- `src/updater.rs` → `src/updater/mod.rs` (moved) + `src/updater/progress.rs` (new): `build_percent` (10 % + 85 × artifacts/total, cap 95), `[[package]]` count from `Cargo.lock`, bar + spinner line, ticking thread (100 ms, `\r`), `finish()`.
- `update()`: git and `cargo build --release --message-format=json` output captured; `compiler-artifact` counted; `compiler-message` rendered text + stderr kept for the log; percentage never goes down (`fetch_max`).
- Failure → `<config>/weathr/update.log` (overwritten), `[weathr] Mise à jour impossible. Détails dans <path>`, Enter. Shared `config_path(file)` also used by `check_path`.
- Removed: change list (`pending_changes`, `print_changes`, `MAX_LISTED_CHANGES`) and « Mise à jour terminée, redémarrage… ». Changes detected with `git rev-list --count HEAD..@{u}`.
- Not a TTY → one plain line `[weathr] Installation en cours…`.
- Build: `cargo build` ✅ · fmt ✅ · clippy ✅ (pre-existing `double_must_use` allowed) · `cargo test` ✅.
- Not checked at runtime: `\r` rendering, piped output, failing-build log.
- Round 1 hunks (feedback): `src/updater/banner.rs` (new: `LOGO`, `SUNSET`, `OUTLINE` ×0.45, `POSTMAN`, `gradient`, `darker`, `draw`/`show` via crossterm `queue!`, 3 tests) · `src/updater/mod.rs` (`mod banner;`, `banner::show()` replaces the « nouvelle version disponible » line in `offer_update`).
- Round 2 hunks (feedback, inline): `src/updater/mod.rs` — `banner::show()` at the start of `update_now`, before `update`.

## Review

## Verification

manual validation — not run by agent

round 2: manual validation — not run by agent · run: yours

round 3: manual validation — not run by agent · run: yours

## Feedback

### Round 1 — 2026-10-26

- Asked: « Remplacer la ligne « [weathr] Une nouvelle version est disponible. » par une bannière façon oh-my-zsh » — variant N: « WEATHR » block logo, sunset gradient, postman below, « Une nouvelle version est arrivée ! », then the prompt.
- Triage: adjustment → criteria updated (banner, no banner for `--update`).
- Changes: implementer — `src/updater/banner.rs` (new), `src/updater/mod.rs`. Banner rendering matches the Python preview cell by cell, except the narrow no-break space before `!` (wording.md).
- Build: `cargo build` ✅ · fmt ✅ · `cargo test` ✅ · clippy ❌ only on pre-existing `double_must_use` in `src/weather/provider.rs` (unchanged from develop).
- Review: deferred → /wa-validate.
- Verify: manual validation — not run by agent.
- Rule captured: none.

### Round 2 — 2026-10-26

- Asked: « weathr --update doit afficher la bannière » · « on ignore clippy » (#31 seulement).
- Triage: adjustment → criteria updated (`--update` shows the banner) · adjustment → clippy dropped from this task's criteria only, project `test_command` unchanged.
- Changes: inline — `src/updater/mod.rs` (`banner::show()` in `update_now`). Tag: `inline`.
- Build: `cargo build` ✅ · fmt ✅ · `cargo test` ✅ · clippy not run (ignored for this task).
- Review: deferred → /wa-validate.
- Verify: manual validation — not run by agent.
- Rule captured: none.
