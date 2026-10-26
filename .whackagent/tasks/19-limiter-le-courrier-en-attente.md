---
issue: 19
status: todo
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
- **Facteur note**: while the box is full, a note signed by the Facteur sits at the top of the Courrier. Not a Lettre: doesn't count in the 7, no Badge, gone once a slot frees.
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
- [ ] Box full: Courrier (`l`) shows the Facteur note on top; it disappears once a Lettre is read and the next day comes.
- [ ] Box full: the Facteur doesn't come to the mailbox.
- [ ] 6 unread village Lettres + 3 Gazettes: no pause, no Facteur note.

## Implementation

## Review

## Verification

## Feedback
