---
issue: 9
status: todo
wiki: [[scene]], [[animation]], [[mail]]
note: "Découpé de #3. Ruches de Nadia et Claire près du verger, potager qui pousse au fil des Lettres."
created: 2026-10-26
---

## Context / Decisions

> Ruches de Nadia et Claire près du verger, potager qui pousse au fil des Lettres.

- Sprint **Saison 2**. Blocked by:
  - #8: the season 2 Lettres that introduce ruche and Potager, and the choices whose answers pick variants.
  - #13: both appear as `StoryEvent::Arrival` beats, on the Jour de jeu of the Lettre that explains them.
  - #21: the scene switch used by the Potager scene.
- **No spoilers**: arrival days and variants come from the Lettres #8 writes. Task, PR and commits never summarise them.
- **Ruche in your garden**, a gift from Nadia and Claire.
  - Placed between the extra pine and the pond. No extra pine (narrow terminal) or no room → no ruche.
  - 2–3 bees fly around it: day only, no rain, no snow, not in winter. Same guards as the butterflies.
  - Nadia and Claire's own rucher belongs to the fête scene (#11), not here.
- **Potager = its own scene**, reached through the Lieux menu of #21 (`m`, `Esc` back to the garden). Not drawn in the garden.
  - Listed in the Lieux menu only from the Potager's Arrival.
  - **Top-down view at an angle** (~45°): beds seen from above, receding slightly. Not the garden's side view.
  - **No sky** (Place without sky, #21): no sun, moon, stars or shooting stars. Real weather still shows: rain and snow fall over the beds, day/night light on the ground.
  - **Stages by Jour de jeu**: tilled → sprouts → vegetables, each on the day of the Lettre that explains it.
  - **Real seasons** on top: winter → bare, mulched beds whatever the stage; vegetables only spring to autumn.
- **Variants**: season 2 answers (#8) about ruche or Potager change their look here (`Choices`). Unanswered → default variant.
- Forced modes (`--demo`, `--simulate`, …): ruche and Potager present at their final stage.
- Out of scope: hens and goat (#10), other season 2 garden variants (#20), fête scene (#11).

## Acceptance criteria

- [ ] `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` all green.
- [ ] Before the ruche's Jour de jeu: no ruche. From that day's `courrier.heure`: ruche between the extra pine and the pond (`--day`).
- [ ] Narrow terminal with no extra pine: no ruche, no crash.
- [ ] Ruche present, summer day, clear sky: bees fly around it. Night, rain or winter: no bees.
- [ ] Before the Potager's Arrival: Potager not in the Lieux menu (`m`).
- [ ] After: `m` → Potager opens the scene, seen from above at an angle, no sky; `Esc` returns to the garden.
- [ ] Potager, rain: raindrops over the beds. Night: ground darker.
- [ ] Potager in summer: tilled, sprouts, vegetables on their three Jours de jeu (`--day`).
- [ ] Potager in winter: bare mulched beds at every stage.
- [ ] Each season 2 answer about ruche or Potager shows its variant; unanswered → default.

## Implementation

## Review

## Verification

## Feedback
