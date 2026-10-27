# Updater and CLI

weathr self-update from own git checkout: no release channel, no download.
Assumes binary built from local clone of this fork.

## How it works — `src/updater/`

`mod.rs` (flow) · `progress.rs` (loader) · `banner.rs` (banner).

- `SOURCE_DIR = env!("CARGO_MANIFEST_DIR")` (`mod.rs:15`) — repo path **baked in at compile time**. Move or delete clone → updates silently disabled.
- `offer_update` (`mod.rs:22`), called from `main` before config load and before raw mode
  (see [[architecture]]):
  1. Skip if stdin not TTY or check already done today
     (`<config>/weathr/update_check` holds date; written before check).
  2. Skip unless checkout exists and clean (untracked files ignored).
  3. `git fetch --quiet` with 5 s timeout, no prompts (`non_interactive_git_env`:
     `GIT_TERMINAL_PROMPT=0`, SSH `BatchMode=yes` unless user set `GIT_SSH_COMMAND`,
     `GIT_SSH` or `core.sshCommand`) so never blocks startup.
  4. Pending commits (`git rev-list --count HEAD..@{u}` > 0) → banner + `[O/n]` (empty = yes). No change list: Nouveautés box tells after restart.
  5. `update` (`mod.rs:143`): `git pull --ff-only` (same git env) → `cargo build --release --message-format=json` → `install`. All output captured, never printed.
  6. `restart` re-`exec`s current executable with same args. No « terminée » line.
- **Banner** (`banner::show`): « WEATHR » ANSI Shadow logo, sunset gradient per column, outline `╗╔╝╚═║` ×0.45, postman below. `Some(NEW_VERSION_MESSAGE)` adds « Une nouvelle version est arrivée ! » (prompt only). Skipped when stdout not TTY.
- **Loader** (`InstallProgress`, `progress.rs:55`): one line `[weathr] Installation en cours / [====>   ] 45 %`, redrawn with `\r` every 100 ms. Pull → 10 %, each cargo `compiler-artifact` → up to 95 % (total = `[[package]]` count in `Cargo.lock`), install → 100 %. Stops in `Drop`. stdout not TTY → one plain `Installation en cours…` line.
- **Failure**: captured output → `<config>/weathr/update.log` (overwritten), `[weathr] Mise à jour impossible. Détails dans <path>`, wait for Enter.
- `install` (`mod.rs:285`) copies built binary next to running one as `.new`, then
  renames over it (atomic on same filesystem). No-op if already running from
  `target/release`.
- `update_now` (`mod.rs:49`) = `--update` flag: banner without message, same pipeline, no prompt, no restart. Returns `false` on error (`[weathr] …` printed), `main` exits 1. Never checks a version exists first.

## CLI — `src/main.rs`

`Cli` (`src/main.rs:62`), parsed with clap:

| Flag | Effect |
|---|---|
| `-s, --simulate <cond>` | fixed weather, no network (missing value prints list) |
| `-n, --night` | night for simulated scenes |
| `-l, --leaves` | falling leaves outside autumn |
| `--christmas`, `--fireworks` | inject Fête (`--fireworks` also forces night) |
| `--demo` | cycle every scene, see [[animation]] |
| `--auto-location`, `--hide-location`, `--hide-hud` | override config |
| `--imperial` / `--metric` | units (mutually exclusive) |
| `--silent` | no info output, **and no update prompt** |
| `--completions <shell>` | print completions, exit |
| `--update` | update now, exit |

Any simulation flag marks session simulated: weather/Fête Badges and rain
alerts off (see [[mail]], [[weather]]).

## Release notes — `src/release_notes.rs`

- `NOUVEAUTES.md` at repo root, French, `tu`, one `## YYYY-MM-DD` section per release, newest first. Embedded with `include_str!` (binary always built from checkout). Written by hand at each release; U+202F before `: ; ! ?` (wrap keeps it glued).
- `notes_seen_on` in `progress.json` = last section shown. First launch after update → `Nouveautés` box (`Overlay::Notes`) with every newer section; `Esc`/`Enter` closes and marks seen. Shown once, no key to reopen.
- `Launch` (`src/app.rs`): `Fresh` (no `progress.json` before load) → newest marked seen, nothing shown, whatever mode · `Suppressed` (`--silent`, `--demo`, forced modes) → nothing shown or marked · `Normal`. Old save without the field → newest section only. Accueil pending (see [[mail]]) → Nouveautés box waits until it ends.
- Lines wrapped at box width (`overlay::text_width`) before scroll limit; `↑`/`↓` scroll when taller than the box.

## Gotchas

- Unix-only: uses `std::os::unix::process::CommandExt::exec`.
- Needs `git` and `cargo` on `PATH`; errors shown in French (details in `update.log`), wait for Enter.
- `--ff-only`: local commits on branch → pull fails, no merge.
- Prompt writes to plain stdout; all output here happens before TUI
  takes alternate screen.