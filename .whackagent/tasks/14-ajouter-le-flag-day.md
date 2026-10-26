---
issue: 14
status: todo
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

## Review

## Verification

## Feedback
