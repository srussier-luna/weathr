---
issue: 19
status: to-close
wiki: [[mail]], [[animation]]
note: "bloquer la boite au lettre a 7 lettre maximum, si on pars en vacances trop longtemps on a pas 21 lettres a lire au retour, c'est trop"
created: 2026-10-26
---

## Context / Decisions

> bloquer la boite au lettre a 7 lettre maximum, si on pars en vacances trop longtemps on a pas 21 lettres a lire au retour, c'est trop

- Sprint **Timeline histoire**. Blocked by #12: adds a pause to its `StoryClock`. Replaces #12's "missed days catch up on return".
- **Box full** = 7 unread village Lettres (`Entry::Main`). Gazette, Cartes postales, correspondents, archives, recipes don't count.
- Box full → the **story pauses**: the Jour de jeu stops advancing. Nothing is lost.
  - Reading a village Lettre frees a slot → the Jour de jeu moves again from the next rollover at `courrier.heure`, one day at a time.
  - Never more than 7 unread village Lettres, however long the absence.
- **Whole Timeline freezes** with the Jour de jeu: no Gazette, Carte postale, correspondent, archive, recipe or Apparition during the pause.
  - Calendar beats (Gazette on Mondays, recipe on the 1st) skip the paused weeks/months. The next one falls on a real Monday / 1st after the pause.
  - Past beats never move: `Progress::letters_read` stays a valid prefix (save rule from #12).
- Paused real days persist in `progress.json` (new `#[serde(default)]` field). Old saves load with no pause.
- Out of the pause (real time, as today): weather, Fêtes, Perséides/Géminides, streak and other Fidélité Badges.
- No Facteur visit while the box is full (nothing to deliver).
- **Facteur note**: delivered like a Lettre when the box becomes full — mailbox flag, `c` opens it first, own double-line frame (`Frame::Notice`). Once opened for that pause, it stays only on top of the Courrier until a slot frees. Every new pause (box full again) re-delivers it, even while catching up day by day (user's call). Doesn't count in the 7.
- **Secret Badge « Grandes vacances »** (`summer_holidays`, hint « boîte aux lettres pleine », suitcase icon): first time the Facteur note is opened; its card shows on top of the note.
- **Note text** (role play, nothing is kept at the post office — the village pauses): « Pendant ton absence, tout le village en a profité pour prendre des vacances ! La rentrée, c'est le <tomorrow, e.g. 31 octobre>. » + signature.
  ```
  Ta boîte est pleine.
  Je garde le reste à la poste
  et je te l'apporte dès qu'il y a de la place.

  — Le Facteur
  ```
- Forced modes (`--demo`, `--simulate`, …) unchanged.
- Tests: unit tests next to the code (`#[cfg(test)]`), clock pause driven by fixed dates, never `Local::now()`.

## Acceptance criteria

- [ ] `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` all green.
- [ ] Start day 1, nothing read, 21 real days later: exactly 7 village Lettres unread, Jour de jeu = 7 (test).
- [ ] Same save: no Gazette, Carte postale or other mail dated after Jour de jeu 7 in the Courrier (test).
- [ ] Read 1 Lettre, next `courrier.heure`: Jour de jeu 8, Lettre 8 delivered, still 7 unread (test).
- [ ] After a pause, the next Gazette falls on a real Monday; skipped weeks have none (test).
- [ ] A save with `letters_read: 37` and no pause field opens on the same Lettre as before (test on a fixture save).
- [ ] Box full: mailbox flag up, `c` opens the Facteur note first, in a double-line frame (╔═╗), before any Lettre.
- [ ] Note opened: `c` goes to the Lettres; the note stays only on top of the Courrier (`l`) until a slot frees.
- [ ] Box full again (new pause): the note is delivered again.
- [ ] First note opened: secret Badge « Grandes vacances » card shows on top of the note, once; closing it returns to the note.
- [ ] Note text: « Pendant ton absence, tout le village en a profité pour prendre des vacances ! La rentrée, c'est le <next courrier.heure date>. — Le Facteur » (« 1er » on the 1st).
- [ ] Box full: the Facteur doesn't come to the mailbox.
- [ ] 6 unread village Lettres + 3 Gazettes: no pause, no Facteur note.

## Implementation

- Branch brought up to date with `sprint/timeline-histoire` (merge) before coding: #12, #13, #14 landed there.
- Facteur note decision (asked during /wa-code): shown while the box is full (≥ 7 unread village Lettres), gone as soon as one is read; the Jour de jeu still moves only at the next `courrier.heure`.
- `src/story/pause.rs` (new): `FULL_BOX = 7`, `is_box_full`, `hold_full_box` — pure walk judging each new real day once; box full on the eve → `clock.pause(day)`.
- `src/story/clock.rs`: `StoryClock` gains `pauses: BTreeMap<usize, u32>`; `date_of(day)` = start + (day-1) + pauses before `day` (past beats never move); `day_of(date)`, `mail_date(now)`, `with_pauses`, `is_frozen`.
- `src/story/timeline.rs`: `unread_village(day, letters_read)` counts `Entry::Main` only.
- `src/progress.rs`: `paused_days` + `pause_checked_on` (`#[serde(default)]`). Old or fresh save → checked from today, no retroactive pause.
- `src/app.rs`: walk runs at launch and each refresh, persists pauses; Facteur gate = frozen clock || box full; Courrier shows `full_box_index_view` when full.
- `src/views.rs`: `full_box_index_view` (note on top of the Courrier), `POSTMAN_SIGNATURE` shared with `src/welcome.rs`.
- Forced modes `--demo` / `--simulate` / `--night` also run the walk (same real save and clock); only `--day` skips it.
- Proof: `cargo fmt -- --check && cargo clippy -- -D warnings -A clippy::double_must_use && cargo test` green (287 tests). Golden fixture tests unchanged and green.

- Round 1 (feedback): `Overlay::PostmanNote` + `next_mail` / `is_postman_note_waiting` (`src/app.rs`), `postman_note_view` + shared `full_box_note` (`src/views.rs`), `Frame::Notice` double-line + char-width fix (`src/overlay.rs`), Badge `summer_holidays` (`src/badges.rs`), `postman_note_day` `#[serde(default)]` (`src/progress.rs`). Proof: fmt ✅ · clippy ✅ (except provider.rs:23) · 291 tests ✅.

- Round 2 (feedback): `Overlay::NoteBadge` + `closed_overlay` / `badge_card` (`src/app.rs`): Badge card over the note, back to the note on close. New FULL_BOX_NOTE text, `day_and_month` (« 1er »), views take `today`. Proof: fmt ✅ · clippy ✅ (except provider.rs:23) · 293 tests ✅.

## Review

Round validation — LENSES: style ✓ · elegance ✓ · structure ✓ · correctness ✓ — 6 + 1 minor, all fixed by autofix:
- 🟡 [correctness] rentrée date from midnight → `StoryClock::next_rollover` (next `courrier.heure`).
- 🟡 [correctness] Courrier note height counted unwrapped → measured with `overlay::lay_out` at `INDEX_FRAME` width.
- 🟡 [correctness] old-save test couldn't fail → pure `pause::hold_since` tested on `letters_read: 37` after 3 weeks away.
- 🟡 [style] three `is_box_full` → param `box_full`, free fn `is_live_box_full`.
- 🟡 [elegance] `SIGNATURE` alias in welcome.rs removed.
- 🟡 [structure] note flow (`MailStep`, `next_mail`, `is_postman_note_waiting`) moved to `src/story/postman_note.rs`.
- 🟡 [elegance] `hold_since -> bool` → `Option<NaiveDate>` (date to mark checked).
Final: clean. fmt ✅ · clippy ✅ (except pre-existing provider.rs:23) · 307 tests ✅.

## Verification

round 1: manual validation — not run by agent.
round 2: manual validation — not run by agent.

manual validation — not run by agent

## Feedback

### Round 1 — 2026-10-30

- Asked: « pas vu : le Courrier (l) affiche en haut la note du Facteur. Il faut que la note soit dans la boîte aux lettres, on ne va pas souvent voir l et on ne regarde pas souvent la barre du haut non plus. La note doit avoir un cadre différent des lettres. » + Badge secret « Grandes vacances ».
- Triage: adjustment (note delivery, frame, once per pause) + Badge added at the user's request.
- Changed: note delivered via `c` first, double-line frame, re-delivered each new pause (asked: kept even while catching up), Badge on first opening. Implementer.
- Review: deferred → /wa-validate. Verify: manual, not run by agent.

### Round 2 — 2026-10-30

- Asked: « le badge s'affiche après le courrier : note → c → courrier → échap → badges » + « on va modifier le texte de la note du facteur, car il ne garde pas les lettres, on met le jeu en pause c'est différent » → role play « le village est parti en vacances », ending « La rentrée, c'est le <date> ».
- Triage: adjustments.
- Changed: Badge card on top of the note; new note text with tomorrow's date. Implementer.
- Review: deferred → /wa-validate. Verify: manual, not run by agent.
