---
issue: 92
status: to-test
wiki: [[animation]]
note: "les empreintes mouillées ne cachent plus la clé"
created: 2026-11-18
---

## Context / Decisions

> La nuit, les empreintes mouillées (creepy.rs:116, '.' tous les 3 colonnes de la porte au puits, sur horizon_y) sont dessinées après la clé 'o-m' (lore.rs:253, mailbox_x+2) et en écrasent le 'o' ou le '-' selon la largeur. Vu au jour 30. Les empreintes doivent sauter les 3 colonnes de la clé.

- Les empreintes mouillées (`src/animation/creepy.rs`) sautent les 3 colonnes de la clé `o-m` (`src/animation/lore.rs`, `mailbox_x + 2`) quand la clé est visible.
- La clé seulement (décision utilisateur·rice) : les noisettes `oo` ne changent pas.
- Le reste du chemin d'empreintes ne bouge pas (même pas, même départ, même arrivée).

## Acceptance criteria

- Nuit, empreintes mouillées et clé visibles, à plusieurs largeurs de terminal → `o-m` entier, aucun `.` dessus.
- Clé pas encore là → empreintes inchangées.
- Jour → pas d'empreintes mouillées, clé inchangée.
- Test unitaire : aucune empreinte dans les colonnes de la clé ; `cargo fmt --check`, `cargo test` verts.

## Implementation

Round 1 — /wa-autopilot 2026-11-19.

- `src/animation/garden.rs`: `KEY_WIDTH`, `KEY_OFFSET`, `GardenLayout::key_x()` (= `mailbox_x + 2`), like `well_x`/`WELL_WIDTH`.
- `src/animation/lore.rs`: `KEY` const, drawn at `layout.key_x()` (same place and colour).
- `src/animation/creepy.rs`: `wet_footprint_columns` — same path (start, end, step 3), minus the key's columns when `unlocks.key`; 3 tests.
- The key arrives on Jour de jeu 17 (`Thing::Key`).
- Nuts `oo` untouched (spec): footprints may still cover them.
- Build: `cargo build`, `cargo test` green · fmt OK · clippy: existing `double_must_use` only.

## Review

## Verification

Round 1 — runtime check by the agent (`--silent --simulate clear --day N [--night]` in a pty, fresh `HOME`, 100x30 / 160x40 / 220x50):

- ✅ Night 30 (key + wet footprints): `o-m` whole, no `.` on it; one gap in the path, same first and last columns.
- ✅ Night 13 (no key yet): footprints regular every 3 columns.
- ✅ Day 30: no wet footprints, key whole.

## Feedback
