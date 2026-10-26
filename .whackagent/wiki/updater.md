# Updater and CLI

weathr self-update from own git checkout: no release channel, no download.
Assumes binary built from local clone of this fork.

## How it works — `src/updater.rs`

- `SOURCE_DIR = env!("CARGO_MANIFEST_DIR")` (`src/updater.rs:10`) — repo path **baked in at compile time**. Move or delete clone → updates silently disabled.
- `offer_update` (`:16`), called from `main` before config load and before raw mode
  (see [[architecture]]):
  1. Skip if stdin not TTY or check already done today
     (`<config>/weathr/update_check` holds date; written before check).
  2. Skip unless checkout exists and clean (untracked files ignored).
  3. `git fetch --quiet` with 5 s timeout, no prompts (`GIT_TERMINAL_PROMPT=0`,
     SSH `BatchMode=yes`) so never blocks startup.
  4. List up to 5 subjects from `git log HEAD..@{u}`, ask `[O/n]` (empty = yes).
  5. `update` (`:136`): `git pull --ff-only` → `cargo build --release` → `install`.
  6. `restart` re-`exec`s current executable with same args.
- `install` (`:166`) copies built binary next to running one as `.new`, then
  renames over it (atomic on same filesystem). No-op if already running from
  `target/release`.
- `update_now` (`:43`) = `--update` flag: same pipeline, no prompt, no restart.

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

- `NOUVEAUTES.md` at repo root, French, `tu`, one `## YYYY-MM-DD` section per release, newest first. Embedded with `include_str!` (binary always built from checkout). Written by hand at each release; avoid `: ; ! ?` (wrap splits on U+202F).
- `notes_seen_on` in `progress.json` = last section shown. First launch after update → `Nouveautés` box (`Overlay::Notes`) with every newer section; `Esc`/`Enter` closes and marks seen. Shown once, no key to reopen.
- `Launch` (`src/app.rs`): `Fresh` (no `progress.json` before load) → newest marked seen, nothing shown, whatever mode · `Suppressed` (`--silent`, `--demo`, forced modes) → nothing shown or marked · `Normal`. Old save without the field → newest section only.
- Lines wrapped at box width (`overlay::text_width`) before scroll limit; `↑`/`↓` scroll when taller than the box.

## Gotchas

- Unix-only: uses `std::os::unix::process::CommandExt::exec`.
- Needs `git` and `cargo` on `PATH`; errors shown in French, wait for Enter.
- `--ff-only`: local commits on branch → pull fails, no merge.
- Prompt writes to plain stdout; all output here happens before TUI
  takes alternate screen.