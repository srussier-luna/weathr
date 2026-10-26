---
issue: 21
status: todo
wiki: [[architecture]], [[scene]], [[animation]], [[render]]
note: "Découpé de #9 : bascule jardin ↔ autre scène (Potager #9, fête #11). Une touche par lieu (p = Potager), Esc revient au jardin. Même ciel et météo réelle."
created: 2026-10-26
---

## Context / Decisions

> Découpé de #9 : bascule jardin ↔ autre scène (Potager #9, fête #11). Une touche par lieu (p = Potager), Esc revient au jardin. Même ciel et météo réelle.

- Sprint **Saison 2**. No blocker. Used by #9 (Potager) and #11 (Fête du miel).
- **Mechanism only**: no place but the garden ships here. No visible change for the player → refactor.
- `Place` = where the player looks: `Garden` only in this task. #9 adds `Potager`, #11 adds `HoneyFete`.
  - Each place declares when it's available (e.g. Potager after its Arrival, Fête du miel on Jours de jeu 119–120).
  - Each place declares whether it shows the sky. Garden and Fête du miel: yes. Potager: no (top-down view, #9).
- **Lieux menu** (replaces the per-place keys of the note): key `m` opens a small overlay listing available places.
  - Garden always listed, current place marked. `↑`/`↓` + `Enter` goes there, `Esc` closes.
  - Only the garden available → `m` does nothing, Aide doesn't list it.
  - Opens like the other overlays (`Overlay` enum), same box style (`render`).
- **`Esc`**: closes an open overlay first, else returns to the garden from any other place.
- **Switch is instant**: the scene changes from one frame to the next. Status line shows the place name outside the garden.
- **Layers per place**: weather (`render_foreground`), HUD and overlays everywhere. Sky (`render_background`) only where the place shows it. Scene + garden layers swapped per place.
- **Away from the garden**:
  - Lettres, Courrier, Badges, Album keys (`c`, `l`, `b`, `a`…) work as in the garden.
  - Sky sightings (shooting stars…) award Badges only where the sky is drawn.
  - Facteur and drifting Bouteilles keep their schedule, just not drawn. Mail waiting → HUD indicator as today.
  - Garden-only actions (`r` pick up) do nothing.
- Lives next to the scene code: `Place` owns which layers run; `App` keeps the current place and routes keys.
- Tests: unit tests (`#[cfg(test)]`) with test-only dummy places (one with sky, one without).
- Out of scope: Potager scene (#9), Fête du miel scene (#11), transitions/animations.

## Acceptance criteria

- [ ] `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` all green.
- [ ] Running `weathr`: garden, keys and Aide look exactly as before; `m` does nothing.
- [ ] Test, dummy place available: `m` lists Garden (marked) + dummy; `↓` + `Enter` switches to it.
- [ ] Test, in dummy place: `Esc` returns to the garden.
- [ ] Test, dummy place unavailable: `m` does nothing, Aide doesn't list it.
- [ ] Test, dummy place + Lettre open: `Esc` closes the Lettre, place unchanged; second `Esc` → garden.
- [ ] Test, dummy place without sky: no sky layer, weather layer drawn, no shooting-star Badge.
- [ ] Test, dummy place with sky: sky drawn, shooting star still awards its Badge; `r` does nothing.
- [ ] Adding a place = one `Place` variant + its scene + availability + sky flag; shown by the dummy places.

## Implementation

## Review

## Verification

## Feedback
