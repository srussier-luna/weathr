---
issue: 14
status: to-close
wiki: [[mail]], [[architecture]], [[updater]]
note: "Split de #12 (sprint Timeline histoire). Sert à vérifier #13, #4, #5 à l'écran."
created: 2026-10-26
---

## Context / Decisions

- `weathr --day N` previews the story on Jour de jeu N. Builds on #12 (`StoryClock`, `Timeline`).
- Flag named in English like the other CLI flags (`--simulate`, `--demo`…). `N` ≥ 1; `--day 0` refused by clap.
- Runs a **fresh in-memory game**: `Progress` started N−1 days ago, nothing read, no answers.
  - Courrier holds every mail of days 1 to N, unread, readable.
  - Garden shows Jour de jeu N (Apparitions once #13 lands).
- Never touches the real save: no `progress.json` write, no `lettres.txt` write. No Badge, no toast for Badges.
- `StoryClock` frozen on day N: no rollover at `courrier.heure`.
- HUD starts with `Jour N | `, same slot as the `--demo` label (`AppState::demo_label`).
- Combines with `--simulate`, `--night`, `--christmas`, `--fireworks`, `--leaves`. With `--demo`, the demo label wins (YAGNI: no combined label).
- Real weather and real time still drive the scenery.

## Acceptance criteria

- [ ] `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` all green.
- [ ] `weathr --day 12`: HUD starts with `Jour 12`; Courrier lists every mail of days 1–12, all unread.
- [ ] `weathr --day 8` with a real save on day 30: reading two Lettres then quitting leaves `progress.json` byte-identical.
- [ ] `weathr --day 8` left open past `courrier.heure`: still `Jour 8`, no new mail.
- [ ] `weathr --day 8 --simulate rain --night`: rain at night, HUD `Jour 8`.
- [ ] `weathr --day 0`: refused with an error, app doesn't start.
- [ ] `weathr --help` lists `--day`.
- [ ] No Badge toast appears in `--day` mode.

## Implementation

stacked on: wa/12-migrer-lhistoire-sur-la-timeline @ 14c7322

Autopilot 2026-10-26, one implementer.

- `main.rs`: `--day <N>` (`NonZeroUsize`, help "Preview the story on day N in a fresh game, without touching your save") → `SimulationOptions.day`. 3 clap tests.
- `app.rs`: day mode → `StoryClock::frozen(today, mail_hour, N)`, `Progress::in_memory(clock.date_of(1))`, `letters::read()`. HUD label `Jour N` via `set_demo_label` unless `--demo`. `award()` no-op when `!awards_badges` (all Badges, milestones included).
- `progress.rs`: `#[serde(skip)] in_memory` flag → `save()` no-op. `Progress::in_memory(start)`.
- `story/clock.rs`: `StoryClock::frozen` → `day_on` always N. Still no `Local::now` in `src/story`.
- `story/content/letters.rs`: `read()` (defaults + `lettres.txt` overrides, never writes); `load()` = `read()` + write defaults if missing.
- Proof: fmt ✅ · clippy ✅ · `cargo test` ✅ 172 passed.

Open points (spec wording, not blocking):
- HUD: label sits in the `--demo` slot, i.e. `… Aide [h] | Jour 8 | Weather: …`, not at the very start. Spec said both "starts with" and "same slot as --demo"; the slot won. Moving it first = one-line change, would move the `--demo` label too.
- Courrier (`l`) lists only Lettres already read, as before: fresh `--day` game shows `Lettres reçues (0)`. Days 1–N are read with `c` (`Lettre 1/12` on day 8).
- Rebased 2026-10-27 onto `sprint/timeline-histoire` (#12 merged, PR #26). Conflicts in `app.rs`, `progress.rs`: develop's release notes / Accueil / badge cards kept; `--day` → `Launch::Suppressed` (no notes, no Accueil, no Facteur visit, real save never loaded or written). #14 doc comments dropped. fmt ✅ · clippy ✅ · test ✅.

## Review

Round validation (2026-10-29), one verifier, autofix 2 rounds.

- LENSES: style ✓ · elegance ✓ · structure ✓ · correctness ✓
- Fixed by autofix:
  - 🟡 [correctness] huge `--day` panicked (NaiveDate overflow): flag is `NonZeroU16`, clap refuses 0 and > 65535; `i64::from`; tests for refusal and `NonZeroU16::MAX`.
  - 🟡 [elegance] non-zero type carried from CLI to `StoryClock::frozen`, `max(1)` clamp dropped.
  - 🟡 [elegance] redundant `record_visit` guard dropped; `in_memory` save is a no-op.
  - 🟡 [correctness] Facteur no longer rides at real `courrier.heure` in `--day` mode: `Postman::set_story_frozen`, `due_round` → None; test.
  - 🟡 [style] test imports `NonZeroU16` instead of full path.
- Open: none.
- Proof: fmt ✅ · clippy `--all-targets -D warnings -A clippy::double_must_use` ✅ · test ✅ 261 passed.
- Code changed after the user's test: `main.rs`, `app.rs`, `story/clock.rs`, `animation/postman.rs`, `animation_manager.rs`.

## Verification

Pty harness, `HOME` = temp dir seeded with a save on day 30 (real save shasum unchanged after all runs):
- ✅ `--day 8 --simulate rain --night`, keys `l`,`c`,`c`,`q`: HUD `Jour 8`, `Lettre 1/12`, exit 0; seeded `progress.json` byte-identical; no `lettres.txt` created; 0 Badge toast.
- ✅ `--day 12 --simulate clear`: `Jour 12`, `Lettre 1/19`, 0 Badge toast.
- ✅ `--day 3 --demo`: demo label wins.
- ✅ `--day 0`: `error: invalid value '0' for '--day <N>'`, exit 2.
- ✅ `--help` lists `--day <N>`.
- ✅ Frozen past `courrier.heure`: clock + timeline unit tests.

## Feedback
