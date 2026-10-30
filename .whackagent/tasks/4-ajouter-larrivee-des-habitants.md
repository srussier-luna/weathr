---
issue: 4
status: to-test
wiki: [[animation]], [[mail]]
note: "Slack Salomé/Ben 23/10 : les animaux peuvent arriver petit à petit et pas tous être là dès le premier jour"
created: 2026-10-23
---

## Context / Decisions

- Re-grilled in #12 (2026-10-26): Habitants arrive by **Jour de jeu**, not by reading. Stardew-like: you see a newcomer, the Lettre explains. No arrival card.
- Builds on #13: each Habitant = one `StoryEvent::Arrival(Thing)` beat on the Timeline, the garden reads `Timeline` + today's Jour de jeu.
- Each Habitant enters the garden on the Jour de jeu of the Lettre that introduces it, not on day 1:
  - Minouche: day 3 · Biscuit (with its doghouse): day 9 · Hulotte: day 12 · Coin-Coin: day 18 · Gustave: day 21 · Noisette: day 27 · Rainette: day 30.
  - Derived from the Timeline, never stored.
- Existing visibility rules stay: Coin-Coin only in rain, Hulotte only at night, etc. Arrival only adds a gate.
- Léo stays as today: out of scope, handled by #6.
- Forced modes (`--demo`, `--simulate`, `--christmas`, `--fireworks`, `--night`) show every Habitant.
- Players updating late: Habitants already due are present at once.
- Glossary: `Habitant` = garden animal, `Voisin` = person from another house.

## Acceptance criteria

- [ ] `weathr --day 1`: no Habitant in the garden — no cat, no dog and no doghouse, no owl, no duck, no snail, no squirrel, no frog.
- [ ] `--day 3` shows Minouche; `--day 9` Biscuit and its doghouse; `--day 12` Hulotte (at night); `--day 18` Coin-Coin (in rain); `--day 21` Gustave; `--day 27` Noisette; `--day 30` Rainette.
- [ ] Arrival does not depend on reading: progress on day 10 with no Lettre read still shows Minouche and Biscuit.
- [ ] No `Nouvel habitant` card, ever.
- [ ] Progress started 40 days ago: every Habitant present at launch.
- [ ] `weathr --demo`: every Habitant visible from the first scene.
- [ ] `cargo test` covers the arrival day of each Habitant against the Timeline.

## Implementation

- `src/story/event.rs` : `Thing::{Cat, Dog, Owl, Duck, Snail, Squirrel, Frog}`.
- `src/story/sources/village.rs` : 7 arrivées dans `ARRIVALS` (3 Cat, 9 Dog, 12 Owl, 18 Duck, 21 Snail, 27 Squirrel, 30 Frog), Lettre N → Jour de jeu N.
- `src/animation/lore.rs` : drapeaux `Unlocks` par Habitant, `Unlocks::with_every_habitant()`.
- `src/app.rs` : `unlocks_on` lit `Timeline::arrived_by` ; `shows_every_habitant` en mode forcé ; `refresh_unlocks()` dès la construction. Tests : `ARRIVAL_DAYS` étendu, jour 1 vide, joueur en retard (jour 41), modes forcés.
- `src/animation_manager.rs` (`render_garden`) : chaque animal derrière son drapeau ; niche avec Biscuit, `StarGazing` avec Hulotte. Léo intact.
- Choix : `--day N` prime sur les modes forcés (sinon `--day 12 --night` ne montrerait jamais l'arrivée de Hulotte). Sans `--day`, les modes forcés montrent tous les Habitants.
- Build : `cargo build`, `cargo fmt --check`, `cargo test` verts. Clippy : seule l'erreur connue #79.

## Review

## Verification

Captures pty, HOME jetable (jamais le vrai) :
- ✅ `--day 1` : aucun Habitant, Léo présent.
- ✅ `--day 3` Minouche ; `--day 8` sans niche ; `--day 9` niche + Biscuit.
- ✅ `--day 12 --night` Hulotte ; `--day 11 --night` sans.
- ✅ `--day 18` sous la pluie Coin-Coin ; `--day 17` sans.
- ✅ `--day 30` : Gustave, Rainette, Noisette.
- ✅ `--demo`, `--simulate`, `--night` sans `--day` : tous les Habitants.
- ✅ Aucune carte « Nouvel habitant ».

## Feedback
