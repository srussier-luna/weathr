---
issue: 148
status: to-test
wiki: [[animation]], [[scene]]
note: "les chaton de minouche sont dans le vide"
created: 2026-11-19
---

## Context / Decisions

> les chaton de minouche sont dans le vide

- Les chatons `=^.^= =^.^=` (`src/animation/cat.rs`, `unlocks.kittens`) sont dessinés à `roof_perch + 8`, une ligne sous Minouche. Le toit descend à cet endroit : ils flottent au-dessus.
- **Décision** : chaque chaton est posé sur la ligne du toit sous lui, juste à côté de Minouche. Même couleur (blancs ou noirs selon la réponse), même moment (Minouche sur le toit).
- Petite et grande maison (`HouseSize`), toutes largeurs de terminal.
- Minouche, le nœud et la fenêtre ne bougent pas.

## Acceptance criteria

- Chatons arrivés (Jour de jeu 33+), Minouche sur le toit, petite maison puis grande (terminal ≥ 170×32) → chaque chaton touche le toit, aucun vide sous lui.
- Chatons blancs / noirs selon la réponse, comme avant.
- Pluie ou nuit (Minouche à la fenêtre) → pas de chatons sur le toit, comme avant.
- Test unitaire : la ligne de chaque chaton est juste au-dessus du toit, pour les deux tailles ; `cargo fmt --check`, `cargo test` verts.

## Implementation

Round 1 — /wa-autopilot 2026-11-23.

- No roof right of Minouche on either house size → one kitten on each side: left one at the top of the slope (Minouche's row), right one lower down the slope (two rows below).
- `src/scene/house.rs`: `House::roof_seat(columns)` = row right above the lowest roof point under those columns (rows down to the eave).
- `src/animation/cat.rs`: `KITTEN`, `kitten_seats(house)` (x `perch_x - 5` and `perch_x + 3`), each kitten drawn separately; colours and visibility unchanged. Each kitten covers the last `_` of its roof row so no column floats.
- Unchanged: Minouche herself still floats slightly above the roof (out of scope).
- Build: `cargo build`, `cargo test` (718) green · fmt OK · clippy clean.

## Review

## Verification

Round 1 — runtime check by the agent (pty, fresh `HOME`, `--silent --simulate clear --day 40`):

- ✅ Small house 100x30 and large house 180x40: each kitten sits on the roof slope, no gap.
- ✅ Rain: no kittens on the roof, Minouche at the window; bow and window unchanged.

## Feedback
