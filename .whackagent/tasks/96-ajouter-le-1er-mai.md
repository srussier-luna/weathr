---
issue: 96
status: todo
wiki: [[animation]], [[mail]]
note: "1er mai : brin de muguet au jardin + Badge"
created: 2026-11-18
---

## Context / Decisions

> 1er mai : brin de muguet au jardin + Badge

- **Muguet géant** (décision utilisateur·rice, à la place du brin au jardin) : le 1er mai, un brin de muguet géant remplace le soleil (sur sa course, `on_sun_path`) et la lune (`MoonSystem::position()`), comme la Crêpe géante de la Chandeleur (#95) et les Notes du ciel, par tous les temps.
- Tige et feuilles vertes, clochettes blanches. Soleil et lune non dessinés ce jour-là. Pas d'autre animation.
- **Badge** « 1er mai » dans la famille Fêtes, gagné au lancement le 1er mai, icône muguet 3 × 5. Pas en mode forcé.
- **`--muguet`** : force le 1er mai, comme `--chandeleur`, sans Badge.
- `Holiday` gagne le 1er mai (`src/holiday.rs`). Sprite à côté de la Crêpe géante (#95) : la tâche qui atterrit en second réutilise le mécanisme « remplace le soleil et la lune » de la première.
- Encadré de Fête (#94) : la tâche qui atterrit en second ajoute le texte du 1er mai.

## Acceptance criteria

- `--muguet` de jour → Muguet géant sur la course du soleil, pas de soleil ; `--muguet --night` → à la place de la lune, pas de lune.
- Pluie ou neige → le Muguet géant reste visible.
- Un vrai 1er mai → Badge « 1er mai » annoncé puis coché dans `b` (Fêtes) ; `--muguet` → pas de Badge.
- 30 avril et 2 mai → soleil et lune normaux.
- Total des Badges + 1.
- Tests : date, sprite ; `cargo fmt --check`, `cargo test` verts.

## Implementation

## Review

## Verification

## Feedback
