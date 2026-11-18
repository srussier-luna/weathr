---
issue: 92
status: todo
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

## Review

## Verification

## Feedback
