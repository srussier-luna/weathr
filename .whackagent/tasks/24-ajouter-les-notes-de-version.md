---
issue: 24
status: todo
wiki: [[updater]], [[render]], [[mail]]
note: "ajouter un encadrer avec une petite release note pour prevenir l'utilisateur de ce qui change dans la mise a jour qu'il vien d'installer"
created: 2026-10-26
---

## Context / Decisions

> ajouter un encadrer avec une petite release note pour prevenir l'utilisateur de ce qui change dans la mise a jour qu'il vien d'installer

- No blocker, no sprint.
- **Source = hand-written French file** at the repo root, `NOUVEAUTES.md`, embedded at build (`include_str!`): weathr is always built from its own checkout ([[updater]]).
  - One section per release, headed by its date: `## 2026-10-26`, newest first.
  - Bullets in the village voice, `tu`, no jargon (`{conventions}/wording.md`). Several releases the same day → one section.
  - This task writes the first section, describing what changed for the player since the previous install.
- **"New" = sections dated after the last one seen.** Last seen date saved in `progress.json` (new `#[serde(default)]` field).
  - Fresh install (no `progress.json` yet) → nothing shown, newest date marked seen.
  - Existing save without the field (players updating to this version) → show the newest section only, then mark it seen.
- **When**: first launch after the update, once the TUI is up, before anything else. A box over the scene, same style as the other overlays (`Overlay` enum, [[render]]).
  - Title: `Nouveautés`. Every unseen section, newest first, each under its date.
  - Longer than the box → `↑`/`↓` scroll.
  - `Esc` or `Enter` closes it and marks every shown section seen.
- **Shown once**: no key to reopen it. The file stays readable in the repo.
- `--silent`, `--demo`, forced modes: no box, nothing marked seen.
- Out of scope: auto-generated notes from commits, reopening from Aide, `--update` output.

## Acceptance criteria

- [ ] `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` all green.
- [ ] `NOUVEAUTES.md` exists with at least one dated section in French, `tu`.
- [ ] Last seen `2026-10-01`, file has sections `2026-10-26` and `2026-10-12`: launch shows a `Nouveautés` box with both, `2026-10-26` first.
- [ ] `Esc` closes it; next launch shows no box.
- [ ] Last seen = newest date: no box.
- [ ] No `progress.json` (fresh install): no box; next launch still no box.
- [ ] Old `progress.json` without the field: box with the newest section only.
- [ ] Notes taller than the box: `↓` scrolls to the last line.
- [ ] `weathr --demo` or `--silent`: no box, last seen date unchanged.
- [ ] Parsing tested: sections split on `## YYYY-MM-DD`, malformed headings ignored (test).

## Implementation

## Review

## Verification

## Feedback
