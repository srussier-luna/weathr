---
issue: 24
status: to-test
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

- `NOUVEAUTES.md` (new): first section `## 2026-10-26` — Perséides, Géminides, window frames, big Badge card, this box.
- `src/release_notes.rs` (new): `parse` (sections `## YYYY-MM-DD`, newest first, malformed headings skipped) and `unseen(sections, seen)`.
- `src/progress.rs`: `notes_seen_on` field (`#[serde(default)]`), `Progress::exists()` for fresh-install detection before `load`.
- `src/app.rs`: `Overlay::Notes(scroll)`, `notes_on_launch`, `mark_notes_seen_when_left`, `scroll`; no box under `--silent`, `--demo`, forced modes. `INDEX_MARGIN_ROWS` → `OVERLAY_MARGIN_ROWS`.
- `src/views.rs`: `notes_view` (Frame::Plain, title `Nouveautés`, dates in French), `notes_scroll_limit`.
- `src/main.rs`: `mod release_notes;`.
- Notes leave `seen` marked when the box is replaced by another overlay too (`h`, `l`…); `q` quits without marking.
- Build: `cargo build` ✅ · `cargo fmt --check` ✅ · `cargo test` ✅ (12 new tests).
- ⚠️ `cargo clippy -- -D warnings` ❌ on a pre-existing lint, untouched file: `clippy::double_must_use`, `src/weather/provider.rs:23` (`async_trait`, clippy 0.1.99). With that lint allowed, clippy is clean on this diff.

## Review

Round validation — LENSES: style ✓ · elegance ✓ · structure ✓ · correctness ✓ — clean after 3 autofix rounds.
- 🟠 [correctness] views.rs — scroll counted lines before wrap → wrapped at box width (`overlay::text_width`) — fixed
- 🟡 [correctness] app.rs — fresh install in suppressed mode showed box next launch → newest marked seen on fresh install — fixed
- 🟡 [elegance] app.rs — redundant `|| is_fresh_install` → removed — fixed
- 🟡 [elegance] app.rs — `.to_vec()` clone → `unseen` truncates the Vec — fixed
- 🟡 [elegance] release_notes.rs / progress.rs — duplicated `DATE_FORMAT` → shared + `parse_date` — fixed
- 🟡 [elegance] app.rs — two bools on `notes_on_launch` → `enum Launch` — fixed
- Not a finding: `clippy::double_must_use` at src/weather/provider.rs:23 predates this task.

## Verification

manual validation — not run by agent

## Feedback

Round 1 — asked: « le texte ne doit pas spoiler, dire qu'on a ajouté des pluies d'étoiles filantes saisonnières » · « Chaque fenêtre a maintenant son cadre… c'est pas très joli, il faut une phrase plus chouette et moins fonctionnalité » · « pour le Badge, enlève au milieu de l'écran » · « la dernière ligne est bien ».
- Triage: adjustment (copy only, inline micro-fix in NOUVEAUTES.md).
- Changed: Perséides + Géminides lines → one line without dates or Badge; frames line rewritten; « au milieu de l'écran » removed. Variant « complice » picked by the user. Badge line → « Les Badges s'annoncent maintenant en grande pompe. » (user wording).
- Code untouched → review still valid for code; tests green.
