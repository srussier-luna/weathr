---
issue: 5
status: todo
wiki: [[scene]], [[animation]]
note: "Slack Salomé/Ben 23/10 : l'arbre qui pousse au fur et à mesure"
created: 2026-10-23
---

## Context / Decisions

- The Arbre grows with the days since install, in 4 stages:
  - day 1: a baby tree — the smallest tree the drawing allows (`Tree` at its minimum height, ~4 canopy rows, thin trunk), not a sprout:
    ```
      ########
    ############
    ############
      ########
        _||_
    ```
  - day 4: the current tree at 1/3 of its fitted height;
  - day 8: at 2/3;
  - day 12 and after: full size, same as today.
- Day = the Jour de jeu from `StoryClock` (#12), so the Arbre changes at `courrier.heure`, without restart.
- Full size on day 12 on purpose: Hulotte, who lives in the Arbre, arrives with Lettre 12 (#4). Later tree props (treehouse Lettre 22, Noisette Lettre 27, chest Lettre 59) always find a full-size Arbre.
- Reduced stages reuse the current drawing scaled down (`Tree::fit` with a smaller height cap), so seasons keep working at every stage: spring blossoms, autumn leaves, fir in winter and December.
- Forced modes (`--demo`, `--simulate`, `--christmas`, `--fireworks`, `--night`) always show the full Arbre, like the Habitants in #4.
- Players who installed earlier follow their own install day: day 12 or later → full Arbre at once.
- Terminal too small for any tree today → still no tree.
- No announcement for growth stages.

## Acceptance criteria

- [ ] Progress started today (day 1): a baby tree stands left of the house — small round canopy (~4 rows) on a thin trunk with roots `_||_`.
- [ ] Day 4: the Arbre is about 1/3 of its full height; day 8: about 2/3; day 12 and after: full height, as before this task.
- [ ] On day 7 at 8:59 the Arbre has its day-4 size; with weathr left open, at `courrier.heure` (9:00 by default) on day 8 it switches to the 2/3 size without restart.
- [ ] Seasons apply at every stage: a young Arbre in December is a small decorated fir.
- [ ] `weathr --demo`: full Arbre in every scene.
- [ ] Progress started 20 days ago: full Arbre at launch.

## Implementation

## Review

## Verification

## Feedback
