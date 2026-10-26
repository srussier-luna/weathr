---
issue: 21
status: todo
wiki: [[architecture]], [[scene]], [[animation]]
note: "Découpé de #9 : bascule jardin ↔ autre scène (Potager #9, fête #11). Une touche par lieu (p = Potager), Esc revient au jardin. Même ciel et météo réelle."
created: 2026-10-26
---

## Context / Decisions

> Découpé de #9 : bascule jardin ↔ autre scène (Potager #9, fête #11). Une touche par lieu (p = Potager), Esc revient au jardin. Même ciel et météo réelle.

- Sprint **Saison 2**. No blocker. Used by #9 (Potager) and #11 (Fête du miel).
- **Mechanism only**: no place but the garden ships here. No visible change for the player → refactor.
- `Place` = where the player looks: `Garden` only in this task. #9 adds `Potager`, #11 adds `HoneyFete`.
  - Each place declares its key and when it's available (e.g. Potager after its Arrival, Fête du miel on Jours de jeu 119–120).
  - Unavailable place → its key does nothing, Aide doesn't list it.
- **Keys**: a place's key opens it. `Esc` closes an open overlay first, else returns to the garden.
- **Switch is instant**: the ground changes from one frame to the next. Status line shows the place name outside the garden.
- **Shared layers**: sky (`render_background`), weather (`render_foreground`), HUD and overlays stay the same in every place. Only the scene + garden layers are swapped per place.
- **Away from the garden**:
  - Lettres, Courrier, Badges, Album keys (`c`, `l`, `b`, `a`…) work as in the garden.
  - Shooting stars and other sky sightings still award Badges.
  - Facteur and drifting Bouteilles keep their schedule, just not drawn. Mail waiting → HUD indicator as today.
  - Garden-only actions (`r` pick up) do nothing.
- Lives next to the scene code: `Place` owns which scene/garden renderer runs; `App` keeps the current place and routes keys.
- Tests: unit tests (`#[cfg(test)]`) with a test-only dummy place.
- Out of scope: Potager scene (#9), Fête du miel scene (#11), transitions/animations.

## Acceptance criteria

- [ ] `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` all green.
- [ ] Running `weathr`: garden, keys and Aide look exactly as before.
- [ ] Test, dummy place available: its key switches to it; `Esc` returns to the garden.
- [ ] Test, dummy place unavailable: its key does nothing, Aide doesn't list it.
- [ ] Test, dummy place + overlay open: `Esc` closes the overlay, place unchanged; second `Esc` → garden.
- [ ] Test, dummy place: sky and weather layers still render; garden layer doesn't.
- [ ] Test, dummy place: a shooting star sighting still awards its Badge; `r` does nothing.
- [ ] Adding a place = one `Place` variant + its scene + its key/availability; shown by the dummy place.

## Implementation

## Review

## Verification

## Feedback
