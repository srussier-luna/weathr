---
issue: 11
status: todo
wiki: [[scene]], [[animation]], [[mail]]
note: "Découpé de #3. Jours 119 et 120 : la scène montre la maison de Nadia et Claire (verger, ruches, stands, lampions) au lieu de ton jardin ; Sam arrive par surprise. Après la fête, une touche permet d'y retourner."
created: 2026-10-26
---

## Context / Decisions

> Jours 119 et 120 : la scène montre la maison de Nadia et Claire (verger, ruches, stands, lampions) au lieu de ton jardin ; Sam arrive par surprise. Après la fête, une touche permet d'y retourner.

- Sprint **Saison 2**. Blocked by:
  - #8: the season 2 Lettres announcing the fête (days 119–120).
  - #21: the scene switch.
  - #4 and #10: Habitants (season 1 and 2) come to the fête.
  - #14: `--day`, used by the acceptance criteria.
- **No spoilers** beyond what is written here: details of the fête come from #8's Lettres.
- **Fête scene** = Nadia and Claire's house: verger, their rucher, stands, lampions.
- **Jours de jeu 119 and 120**: weathr opens on the fête scene instead of the garden, from `courrier.heure`.
  - `Esc` goes back to the garden. A key (via #21) brings you back to the fête, only on these two days.
- **After day 120: no way back.** The scene and its key disappear. Overrides the 23/10 note ("une touche permet d'y retourner").
- **Guests**: Nadia, Claire, the Voisins of the Lettres, and every Habitant already arrived.
- **Sam**: absent on day 119. Present from the start of day 120, with a suitcase.
- **Real weather**: same sky as the garden. Rain → stands under tarps. Night → lampions lit.
- Forced modes (`--demo`, `--simulate`, …): no fête unless `--day` 119/120.
- Out of scope: season 3, Badge for the fête, Nadia and Claire's house outside the fête.

## Acceptance criteria

- [ ] `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` all green.
- [ ] `--day 118`: weathr opens on the garden, no fête key.
- [ ] `--day 119`: weathr opens on the fête scene: house, verger, rucher, stands, lampions; Nadia, Claire, Voisins, arrived Habitants; no Sam.
- [ ] `--day 120`: same scene, Sam present with a suitcase.
- [ ] Day 119 or 120: `Esc` → garden; the fête key → back to the fête.
- [ ] `--day 121`: garden, fête key does nothing, Aide doesn't list it.
- [ ] Day 119, rain: stands under tarps. Night: lampions lit.

## Implementation

## Review

## Verification

## Feedback
