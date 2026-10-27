---
issue: 41
status: todo
wiki: [[mail]], [[render]]
note: "Protéger la progression : si progress.json est abîmé, weathr repart de zéro sans prévenir et écrase le fichier ; garder une copie de secours et afficher un message"
created: 2026-10-27
---

## Context / Decisions

> Protéger la progression : si progress.json est abîmé, weathr repart de zéro sans prévenir et écrase le fichier ; garder une copie de secours et afficher un message

- No blocker, no sprint. Today `Progress::load` (`src/progress.rs`) turns an unreadable file into `Progress::default()`, then saves over it: the old save is lost, silently.
- **Unreadable `progress.json`** (exists, but can't be read or parsed):
  - Renamed, never overwritten: `progress.json.abime-YYYY-MM-DD-HHMM` next to it.
  - A new game starts (`Progress::default()` + start date), as for a fresh install.
  - Missing file stays the fresh-install case of #24, no message.
- **Safe write**: `Progress::save` writes `progress.json.tmp`, then renames it over `progress.json`. The file is never half written.
- **Message**: a box at launch, in the TUI, before the Nouveautés box.
  - Title `Sauvegarde`. Text: « Ta progression n'a pas pu être lue. Une copie est gardée dans <chemin>. Une nouvelle partie commence. »
  - `Esc` or `Enter` closes it. Shown once, on that launch only.
- **New frame `Frame::Floppy`** (`src/overlay.rs`), for save info: top `+-[#]-----…+`, sides `| … |`, bottom `+-----…+`, grey-blue colour.
- **Release notes**: the new game has no `notes_seen_on` → treated like an old save (#24), newest section shown after the Sauvegarde box. Not a fresh install.
- `--demo`, `--silent`, forced modes: the copy and the new game happen anyway (the file must not be overwritten); the box is shown too, since it says data moved.
- Wording: `{conventions}/wording.md`, error situation: say what happened and where the copy is.
- Out of scope: repairing the old file, merging it back, rotating several copies.

## Acceptance criteria

- [ ] `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` all green.
- [ ] `progress.json` containing `{oops`: launch → box `Sauvegarde` in the floppy frame with the copy's path; the copy holds `{oops` byte for byte.
- [ ] Same launch: new game started (Lettre 1), `progress.json` valid JSON.
- [ ] `Esc` closes the box, then the Nouveautés box shows.
- [ ] Next launch: no Sauvegarde box.
- [ ] No `progress.json`: no box, no copy (fresh install).
- [ ] `save` leaves no `progress.json.tmp` behind and never a truncated `progress.json` (test: write then read back).
- [ ] Copy name follows `progress.json.abime-YYYY-MM-DD-HHMM` (test with a fixed date).

## Implementation

## Review

## Verification

## Feedback
