---
issue: 96
status: to-close
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

Round 1 — /wa-autopilot 2026-11-19.

- stacked on: wa/95-ajouter-la-chandeleur @ 0c91ca5 (merge), reuses `sky_sprite.rs`.
- New `src/animation/may_day.rs`: `GiantMuguet` 19×9, opaque, green stem and leaves `Rgb(60,170,60)`, white bells `(_)`, no animation.
- `src/holiday.rs`: 1er mai, `Holiday::may_day()` (also spring). `src/main.rs`: `--muguet`. `src/animation_manager.rs`: one branch in `sky_sprite()`.
- `src/badges.rs`: « 1er mai » (Fêtes), icon muguet; total 44 → 45 on this stacked branch. `src/app.rs`: badge event, not in forced mode.
- Not done: Encadré de Fête text (#94 not in this branch), README (no `--chandeleur` listed either).
- Build: `cargo build`, `cargo test` green · fmt OK · clippy: existing `double_must_use` only.

## Review

Round validation 2 — 2026-11-19: #95 reworked in its review (`SkySpriteKind`); merged into #96 (d376b83), Muguet adapted (variant, `on` branch, match arm). Runtime re-run by the agent (Muguet day/night/rain, Crêpe, Notes du ciel). Re-verified clean.

Round validation — 2026-11-19, one wa-verifier, LENSES style ✓ elegance ✓ structure ✓ correctness ✓.
- Clean, no finding (delta vs wa/95).

## Verification

Round 1 — runtime check by the agent (pty 120×40, fresh `HOME`, `faketime`):

- ✅ `--muguet --simulate clear`: Muguet on the sun's course, no sun; `--night`: in place of the moon.
- ✅ Rain, snow, storm: still visible. `--chandeleur` and `--music` unchanged.
- ✅ Real 1/05: « Nouveau badge ! 1er mai », Badges x/45; `--muguet` → no badge.
- ✅ 30/04 and 2/05: normal sun and moon; 70×20: no crash.

## Feedback
