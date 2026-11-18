---
issue: 39
status: to-test
wiki: [[architecture]], [[mail]], [[updater]]
note: "weathr --export / --import pour changer de Mac sans perdre Lettres et Badges"
created: 2026-10-26
---

## Context / Decisions

> weathr --export / --import pour changer de Mac sans perdre Lettres et Badges

- No blocker, no sprint. Multiple save slots are #40 (draft): #39 stays single-slot; #40 will adapt import later (import into a free slot).
- **Content**: `progress.json` + `config.toml` (village and city found again). Not `lettres.txt` (regenerated with defaults), not the cache dir (weather/location rebuilt on their own).
- **CLI** (`Cli`, `src/main.rs`), path required:
  - `weathr --export <FICHIER>` → writes the backup, exits.
  - `weathr --import <FICHIER>` → restores it, exits.
  - Handled before the TUI, like `--update` (no raw mode, no alternate screen).
- **Where**: new module `src/backup.rs` (bundle build/read, version compare); `main.rs` only wires the flags.
- **Backup file** = one JSON object:
  - `version`: weathr version that wrote it (`CARGO_PKG_VERSION`, e.g. `1.5.0`).
  - `progress`: the `progress.json` content as a JSON object.
  - `config`: `config.toml` raw text (comments kept). Missing on disk → field absent / empty, import skips it.
- **Export**: existing file at `<FICHIER>` → overwritten. Then `[weathr] Sauvegarde écrite dans <chemin>.`
- **Import**:
  - File unreadable or not a backup (bad JSON, no `progress`) → `[weathr] Sauvegarde illisible : <chemin>`, nothing written, exit code 1.
  - `version` ≠ running version → warn only, import goes on: `[weathr] Sauvegarde faite avec weathr <v>, tu as la <current> : des données peuvent manquer.`
  - `progress.json` already exists on this Mac → `[weathr] Ta progression et tes réglages actuels seront remplacés. Continuer ? [o/N] ` — empty / anything but `o`/`oui`/`y`/`yes` → nothing written, exit. No question when no `progress.json` yet.
  - Writes `progress.json` (and `config.toml` when present in the backup), then `[weathr] Progression importée.`
  - No detection of a running weathr; README says to close it first (a running instance re-saves its own progress).
- Importing awards nothing and keeps `started_on` as saved (Jour de jeu count follows the backup).
- README: `--export` / `--import` in usage, one line in « Fichiers créés » or a « Changer de Mac » note.
- Wording: `{conventions}/wording.md` — system messages `[weathr] …`, short, factual.

## Acceptance criteria

- [ ] `cargo fmt --check`, `cargo test` green.
- [ ] `weathr --export /tmp/s.json` → file holds `version`, `progress` (same data as `progress.json`) and `config` (same text as `config.toml`); prints `[weathr] Sauvegarde écrite dans /tmp/s.json.`; re-running overwrites it.
- [ ] Fresh config dir (no `progress.json`), `weathr --import /tmp/s.json` → no question; `progress.json` and `config.toml` restored; prints `[weathr] Progression importée.`; launching weathr shows the same Lettres read and Badges.
- [ ] Existing progress, `--import` → asks `Continuer ? [o/N]`; Enter or `n` → nothing changed; `o` → replaced.
- [ ] Backup with another `version` → warning line, import still done.
- [ ] Bad file (missing, not JSON, no `progress`) → `[weathr] Sauvegarde illisible : <chemin>`, exit code 1, nothing written.
- [ ] Unit tests: export → import round trip keeps progress and config; version mismatch detected; invalid backup rejected.

## Implementation

Round 1 — /wa-autopilot 2026-11-18 (branch merged with origin/develop first).

- `src/backup.rs` (new): Sauvegarde bundle (`version`, `progress`, optional `config`), export, read + validation, version check, import with `[o/N]` question (empty = no), unit tests on temp dirs.
- `src/main.rs`: `mod backup;`, `--export <FICHIER>` / `--import <FICHIER>` (clap `conflicts_with`), early exits after `--update`, before config load and TUI.
- `src/config/mod.rs`: `get_config_path` now `pub(crate)`, reused by the backup.
- `README.md`: « Changer de Mac » section (what the Sauvegarde carries, close weathr before `--import`).
- Messages use `\u{202F}` before `:` and `?` (wording.md). Success on stdout, warning and errors on stderr.
- A backup without `version` is « illisible » (spec only named `progress`).
- Imported `progress.json` is JSON-equal to the saved one, keys in alphabetical order; weathr rewrites it in its own order at next save.
- Absent or empty `config` in the backup → `config.toml` left untouched.
- `accepts` and `PREFIX` duplicated locally (updater's `accepts` treats empty as yes).
- Open: `--export` with no `progress.json` prints the raw OS error (`[weathr] No such file or directory (os error 2)`, exit 1). No wording in the spec → to decide at test.
- Follow-up: #127 reworks the import.
- Build: `cargo build` no warning · `cargo test` green (bin 340, 9 new `backup::` tests) · `cargo fmt --check` OK · clippy: only the existing `double_must_use`.

## Review

## Verification

Round 1 — runtime check by the agent (`target/debug/weathr`, `HOME` = fresh temp dir each time):

- ✅ Export: message, exit 0; file holds `version`, `progress` (JSON-equal to progress.json), `config` (text-equal, comments kept).
- ✅ Overwrite of an existing file at the target path.
- ✅ Import into an empty HOME: no question, `Progression importée.`, progress and config restored.
- ✅ Existing progress: `Continuer ? [o/N]`; Enter, `n`, `non` → both files unchanged; `o` → replaced.
- ✅ Backup from 1.5.0: warning line, import still done.
- ✅ Missing / not JSON / no `progress`: `Sauvegarde illisible : <chemin>`, exit 1, nothing written.
- ⚠️ « Same Lettres and Badges at launch » checked by comparing progress.json, not by driving the TUI.

## Feedback
