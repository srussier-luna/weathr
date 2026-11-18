---
issue: 69
status: todo
wiki: [[animation]], [[scene]]
note: "Une animation quand il est dans l'eau"
created: 2026-11-18
---

## Context / Decisions

> Une animation quand il est dans l'eau

- Biscuit (`src/animation/dog.rs`) marche au hasard (`Activity::Sitting` / `Walking`) ; la Mare est à `puddle_x` / `PUDDLE_WIDTH` (`src/animation/duck.rs`), gelée en grand froid (`src/scene/river.rs`, `FROZEN`).
- **Deux choses** (décision utilisateur·rice) :
  - **Baignade** : parfois (~1 balade sur 6), Biscuit va exprès à la Mare. Pattes cachées par des vaguelettes, éclaboussures `' . '` autour, puis en sortant il s'ébroue : gouttes qui volent quelques secondes.
  - **En passant** : quand sa marche au hasard le fait traverser la Mare, éclaboussures à ses pattes pendant la traversée.
- **Mare gelée** : la Baignade devient une **glissade** — il traverse plus vite, pattes écartées, sans gouttes ni éclaboussures. En passant sur la glace : pas d'éclaboussure.
- Pluie, orage, nuit : dans la niche, comme aujourd'hui. Moment « staring » (voisin n°13) : figé, comme aujourd'hui.
- Coin-Coin et Rainette restent où ils sont, pas d'interaction.
- Nouvelle activité dans `dog.rs` (ex. `Activity::Bathing`), état gelé lu depuis ce que la scène sait déjà ; pas de nouveau module.

## Acceptance criteria

- Beau temps, jour, Mare liquide, Biscuit arrivé → au fil des minutes, il va parfois à la Mare : vaguelettes à ses pattes, éclaboussures, puis gouttes qui partent en sortant.
- Marche au hasard qui croise la Mare → éclaboussures à ses pattes pendant la traversée.
- Mare gelée → glissade : traversée plus rapide, pattes écartées, aucune goutte ; croiser la glace → aucune éclaboussure.
- Pluie, orage ou nuit → Biscuit dans sa niche, rien.
- Avant l'arrivée de Biscuit (`--day 8`) → rien.
- Coin-Coin et Rainette inchangés.
- Tests unitaires : choix de la Baignade, glissade si gelée, éclaboussures en traversée ; `cargo fmt --check`, `cargo test` verts.

## Implementation

## Review

## Verification

## Feedback
