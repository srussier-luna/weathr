---
issue: 10
status: todo
wiki: [[animation]], [[scene]], [[mail]]
note: "Découpé de #3. Nouveaux Habitants, arrivée comme #4 (à la lecture de leur 1re Lettre)."
created: 2026-10-26
---

## Context / Decisions

> nouveaux Habitants de la saison 2

- Sprint **Saison 2**. Blocked by:
  - #8: the season 2 Lettres that introduce them (names, days) and the choices whose answers pick variants.
  - #4: Habitants arrive by Jour de jeu (`StoryEvent::Arrival`), not by reading. Hens and goat use the same mechanism.
  - #9: the Potager scene they also live in.
- **No spoilers**: names, arrival days and variants come from #8's Lettres. Task, PR and commits never summarise them.
- **3 hens and 1 goat**, new Habitants.
  - Each arrives on the Jour de jeu of the Lettre that introduces it. Before: absent from both scenes.
  - Players updating late: Habitants already due are present at once (same as #4).
- **Present in both scenes at once**: garden and Potager each draw the same hens and goat.
- **Garden**: right of the fence. Poulailler and goat shelter behind the fence. No room → not drawn, no crash.
- **Potager**: drawn in its top-down view (#9): hens peck between the beds, goat beside them, their shelters seen from above.
- **Shelters**: hens in the poulailler, goat in its shelter at night, in rain and thunderstorm (like Biscuit's doghouse). Outside otherwise, snow included.
- **Variants**: season 2 answers (#8) about hens or goat change their look (`Choices`). Unanswered → default.
- Forced modes (`--demo`, `--simulate`, …): every hen and the goat present.
- Out of scope: ruche, Potager growth (#9), other garden variants (#20).

## Acceptance criteria

- [ ] `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` all green.
- [ ] Before a hen's or the goat's Jour de jeu: absent from garden and Potager. From that day's `courrier.heure`: present in both (`--day`).
- [ ] Late player past every arrival day: 3 hens and the goat at once.
- [ ] Garden, clear day: hens and goat right of the fence, poulailler and shelter behind it.
- [ ] Narrow terminal: no hens, goat or shelters drawn, no crash.
- [ ] Potager, clear day: same hens and goat visible.
- [ ] Night, rain or thunderstorm: hens in the poulailler, goat in its shelter, in both scenes. Snow by day: outside.
- [ ] Each season 2 answer about hens or goat shows its variant; unanswered → default.

## Implementation

## Review

## Verification

## Feedback
