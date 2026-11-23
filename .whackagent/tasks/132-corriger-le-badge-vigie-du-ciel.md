---
issue: 132
status: to-close
wiki: [[mail]]
note: "le joueur vgie n'as pas fonctionné"
created: 2026-11-23
---

## Context / Decisions

> le joueur vgie n'as pas fonctionné

- Cause (retour du joueur) : son `git config user.name` vaut « Denis Richard » ; le jeu compare à `drichard-luna` (`SKY_LOOKOUT_LOGIN`, `src/badges.rs`), donc le badge secret « Vigie du ciel » ne se déclenche jamais.
- **Décision** : comparer au nom réel, en clair : `SKY_LOOKOUT_LOGIN` = « Denis Richard ». Même règle qu'aujourd'hui (`is_listed` : espaces autour ignorés, égalité exacte).
- #109 (retirer le nom du code public) reste à part.

## Acceptance criteria

- `git config user.name` = `Denis Richard` → au lancement, carte « Vigie du ciel », case cochée dans `b`.
- `drichard-luna`, autre nom, nom vide ou git absent → pas de Vigie du ciel.
- Tests unitaires mis à jour ; `cargo fmt --check`, `cargo test` verts.

## Implementation

Round 1 — /wa-autopilot 2026-11-23.

- `src/badges.rs`: `SKY_LOOKOUT_LOGIN` renamed `SKY_LOOKOUT_NAME = "Denis Richard"`; still via `is_listed` (spaces around ignored, exact match). Tests: name with spaces, `drichard-luna` rejected, Vigie never grants Maître du temps.
- Build: `cargo build`, `cargo test` green · fmt OK · clippy clean.

## Review

Round validation — one wa-verifier, LENSES style ✓ elegance ✓ structure ✓ correctness ✓.
- Clean, no finding.

## Verification

Round 1 — runtime check by the agent (pty, fresh `HOME`, temp `GIT_CONFIG_GLOBAL`):

- ✅ `Denis Richard` → « Nouveau badge ! Vigie du ciel » at launch; `b` → ticked, Badges 2/49.
- ✅ `drichard-luna` → no card, Secrets 0/10.

## Feedback
