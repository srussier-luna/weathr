---
issue: 95
status: to-test
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

Round 1 — /wa-autopilot 2026-11-19.

- New `src/animation/sky_sprite.rs`: trait `SkySprite { size, update, render }`, `fits`, `in_place_of_sun`, `in_place_of_moon`; `SkyNotes` implements it (behaviour and tests unchanged). Reused by #96.
- New `src/animation/candlemas.rs`: `GiantCrepe` 19×7, opaque; golden side `Rgb(225,155,50)` with `o` `.`, pale side `Rgb(250,235,195)`; flip every ~5 s, ~0.5 s (half, thin, edge, thin, half).
- `src/animation_manager.rs`: `sky_sprite()` picks the sprite of the day; no sun/moon when one shows.
- `src/holiday.rs`: Chandeleur (2/02), `Holiday::candlemas()` (also winter, like a real 2/02). `src/main.rs`: `--chandeleur`.
- `src/badges.rs`: `candlemas` « Chandeleur » (Fêtes, after Géminides), icon crêpe; total 43 → 44. `src/app.rs`: badge event, not in forced mode.
- Not done (out of scope): Chandeleur in `--demo`, README Fêtes list, NOUVEAUTES.md, Encadré de Fête text (#94 not in develop yet).
- Build: `cargo build`, `cargo test` (512) green · fmt OK · clippy: existing `double_must_use` only.

## Review

## Verification

Round 1 — runtime check by the agent (pty 120×40, fresh `HOME`, `faketime` for real dates):

- ✅ `--chandeleur --simulate clear`: crêpe on the sun's course, no sun; `--night`: in place of the moon.
- ✅ Rain and snow: crêpe still visible.
- ✅ Flip observed (~0.5 s, every ~5 s), golden ↔ pale.
- ✅ Real 2/02: « Nouveau badge ! Chandeleur », `b` → ticked, Badges x/44; `--chandeleur` → no badge.
- ✅ 1/02 and 3/02: normal sun and moon; `--music`: Notes du ciel unchanged; 70×20: no crash.
- Note: `--night` alone needs `--simulate` to show night (existing behaviour).

## Feedback
