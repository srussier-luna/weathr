---
issue: 95
status: todo
wiki: [[animation]], [[mail]]
note: "Chandeleur (2 février) : crêpes qui sautent à la fenêtre + Badge"
created: 2026-11-18
---

## Context / Decisions

> Chandeleur (2 février) : crêpes qui sautent à la fenêtre + Badge

- **Crêpe géante** (décision utilisateur·rice, à la place des crêpes à la fenêtre) : le 2 février, elle remplace le soleil (sur sa course, `on_sun_path`) et la lune (`MoonSystem::position()`), comme les Notes du ciel de la Fête de la musique, par tous les temps.
- Elle se retourne de temps en temps : face dorée (taches `o` `.`) / face pâle, comme une crêpe qu'on fait sauter.
- Soleil et lune non dessinés ce jour-là.
- **Badge** « Chandeleur » dans la famille Fêtes, gagné au lancement le 2 février (comme `music_day`), icône crêpe 3 × 5. Pas en mode forcé (règle existante).
- **`--chandeleur`** : force la Chandeleur, comme `--music` / `--christmas`, sans Badge.
- `Holiday` gagne la Chandeleur (`src/holiday.rs`). Sprite et logique à côté de `music.rs` (`SkyNotes`) dans `src/animation/`.
- Encadré de Fête (#94) : la tâche qui atterrit en second ajoute le texte de la Chandeleur.

## Acceptance criteria

- `--chandeleur` de jour → Crêpe géante sur la course du soleil, pas de soleil ; de nuit (`--chandeleur --night`) → à la place de la lune, pas de lune.
- Pluie ou neige → la Crêpe géante reste visible.
- Elle se retourne de temps en temps (face dorée ↔ face pâle).
- Un vrai 2 février → Badge « Chandeleur » annoncé puis coché dans `b` (Fêtes) ; `--chandeleur` → pas de Badge.
- 1er et 3 février → soleil et lune normaux.
- Total des Badges + 1.
- Tests : date de la Chandeleur, sprite et retournement ; `cargo fmt --check`, `cargo test` verts.

## Implementation

## Review

## Verification

## Feedback
