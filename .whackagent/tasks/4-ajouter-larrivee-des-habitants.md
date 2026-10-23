---
issue: 4
status: todo
wiki: [[animation]], [[mail]]
note: "Slack Salomé/Ben 23/10 : les animaux peuvent arriver petit à petit et pas tous être là dès le premier jour"
created: 2026-10-23
---

## Context / Decisions

- Each Habitant (garden animal) enters the garden when its first Lettre is read, not on day 1.
  - Minouche: Lettre 3 · Biscuit (with its doghouse): Lettre 9 · Hulotte: Lettre 12 · Coin-Coin: Lettre 18 · Gustave: Lettre 21 · Noisette: Lettre 27 · Rainette: Lettre 30.
  - Lettre number = village Lettre read count, same mechanism as gifted props (`Unlocks::after_reading` in `src/animation/lore.rs`). Derived, never stored.
- Existing visibility rules stay: Coin-Coin only in rain, Hulotte only at night, etc. Arrival only adds a gate.
- Léo stays as today: out of scope, handled by #6.
- Forced modes (`--demo`, `--simulate`, `--christmas`, `--fireworks`, `--night`) show every Habitant, never an arrival card.
- Arrival card ("parchemin"), shown once per Habitant, on first appearance:
  - Reuses the current Lettre window (`overlay::render_view`). The parchment look comes later with #7 (Lettres design).
  - Title `Nouvel habitant`, the Habitant's ASCII sprite, then `<Nom> s'installe dans le jardin.`; `[Echap]` closes it.
  - Opens as soon as no other window (Lettre, Album, Badges, Aide…) is open.
  - Several Habitants due at once → one card each, in Lettre order.
  - "Already announced" is saved in progress, so a card never comes back after a restart.
- Players updating with Lettres already read: Habitants already due are present at once and marked announced, no card.
- Glossary: `Habitant` = garden animal, `Voisin` = person from another house.

## Acceptance criteria

- [ ] Fresh progress (no Lettre read): no Habitant in the garden — no cat, no dog and no doghouse, no owl, no duck, no snail, no squirrel, no frog.
- [ ] Reading Lettre 3 makes Minouche appear; Lettre 9 Biscuit and its doghouse; 12 Hulotte (at night); 18 Coin-Coin (in rain); 21 Gustave; 27 Noisette; 30 Rainette.
- [ ] Closing Lettre 9 opens a `Nouvel habitant` card with Biscuit's sprite and `Biscuit s'installe dans le jardin.`; `[Echap]` closes it.
- [ ] After closing it and restarting weathr, Biscuit's card never shows again.
- [ ] Progress file with 40 Lettres read and no announced Habitants: every Habitant present at launch, no card.
- [ ] `weathr --demo`: every Habitant visible from the first scene, no card.

## Implementation

## Review

## Verification

## Feedback
