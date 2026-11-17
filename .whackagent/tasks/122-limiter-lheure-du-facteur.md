---
issue: 122
status: todo
wiki: [[mail]], [[architecture]]
note: "il faut bloquer le passage du facteur du matin dans les preference, avec impossibilite de le faire passer apres 14h et avant 6h"
created: 2026-11-17
---

## Context / Decisions

> il faut bloquer le passage du facteur du matin dans les preference, avec impossibilite de le faire passer apres 14h et avant 6h

- **Heure du Facteur** limitée de 6 h à 14 h inclus. Le Facteur reste un passage du matin ou de la mi-journée.
- **Réglages** (`src/settings/mod.rs`, ligne « Heure du Facteur ») : aujourd'hui ←/→ tourne sur 24 h (`rem_euclid`). Désormais ←/→ s'arrête aux bornes : 14 h → → reste 14 h, 6 h ← reste 6 h. Pas de boucle.
- **config.toml** (décision utilisateur·rice) : une heure hors 6–14 (écrite à la main, ancienne valeur) → **erreur de config** au lancement, comme une heure > 23 aujourd'hui (`ConfigError::InvalidMailHour`, `src/config/mod.rs` `validate`). Message : « invalid mail hour {h} (must be between 6 and 14) » (les messages de config sont en anglais).
- Valeur par défaut inchangée : 9 h.
- Effet voulu sur le secret (sprint « Le secret ») : l'Arcano-Facteur passe entre 18 h et 2 h.
- Changement d'heure déjà enregistré dans la sauvegarde (`mail_hour_change`, ancienne heure hors plage) : gardé tel quel, il ne sert qu'à l'historique des jours.
- Hors périmètre : migration automatique d'un config.toml hors plage.

## Acceptance criteria

- Réglages → Heure du Facteur à 14 h, → → reste 14 h ; à 6 h, ← → reste 6 h ; entre les deux, ←/→ change d'une heure.
- config.toml avec `heure = 5` ou `heure = 15` → le jeu ne démarre pas et affiche « invalid mail hour 5 (must be between 6 and 14) » ; `heure = 6` et `heure = 14` → démarre.
- config.toml sans heure → 9 h, comme avant.
- Tests unitaires : bornes des Réglages, validation de config (5, 6, 14, 15).
- `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings -A clippy::double_must_use`, `cargo test --bin weathr` verts.

## Implementation

## Review

## Verification

## Feedback
