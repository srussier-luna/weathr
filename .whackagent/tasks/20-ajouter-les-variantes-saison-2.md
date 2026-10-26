---
issue: 20
status: todo
wiki: [[animation]], [[mail]]
note: "Découpé de #8 : variantes au jardin des 12 choix saison 2, sur les choses de la saison 1"
created: 2026-10-26
---

## Context / Decisions

> Découpé de #8 : variantes au jardin des 12 choix saison 2, sur les choses de la saison 1

- Sprint **Saison 2**. Blocked by #8: the 12 season 2 choices and their answer ids.
- **Every season 2 choice shows somewhere**, like season 1. Split by target:
  - ruche, Potager → #9
  - hens, goat → #10
  - everything already in the garden in season 1 (Habitants, Arbre, house, well, bench, props…) → this task
- **Which choice targets what** comes from #8: its `## Implementation` lists each choice id and its target. Read it first.
- **No spoilers**: task, PR and commits name choice ids, never the story.
- Same mechanism as season 1: new `Choices` flags built from answers (`Choices::from_answers`, `src/animation/lore.rs`), read by the drawing code.
- Variant shows from the moment the answer is given, not by Jour de jeu (same as season 1, kept by #13). Unanswered → default look.
- Forced modes (`--demo`, `--simulate`, …): default look.
- Out of scope: variants on ruche, Potager, hens, goat; new garden things.

## Acceptance criteria

- [ ] `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` all green.
- [ ] Every season 2 choice id targeting a season 1 thing maps to a `Choices` flag (test, like the season 1 mapping).
- [ ] Each such choice: answer `1` and answer `2` give two different looks of its target in the garden.
- [ ] Unanswered: target keeps its season 1 look.
- [ ] Season 1 variants unchanged.

## Implementation

## Review

## Verification

## Feedback
