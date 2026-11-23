---
issue: 125
status: to-close
wiki:
note: "Découpé de #124 (parapluie « Ajouter la détection de triche »), sprint « Anti-triche ». Punitions : #126."
created: 2026-11-17
---

## Context / Decisions

- Détails gardés hors du dépôt.
- La triche repérée ne rapporte jamais rien et s'ajoute au compteur lu par #126.
- Après #41.

## Acceptance criteria

- Partie jouée normalement, relancée plusieurs fois → jamais comptée comme triche.
- `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings -A clippy::double_must_use`, `cargo test` verts.

## Implementation

- stacked on: wa/41-corriger-la-perte-de-progression @ 31bc28d
- Détails hors du dépôt.
- Build : `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings -A clippy::double_must_use`, `cargo test` verts.

## Review

Round validation — 2026-11-19, one wa-verifier, LENSES style ✓ elegance ✓ structure ✓ correctness ✓.
- Détails hors du dépôt. Constats corrigés en 2 tours, revérifié propre. Code modifié depuis le test : conseil de `--diagnostic` pour un fichier de progression illisible.

## Verification

- Autopilot, HOME jetable, pseudo-terminal (implémenteur) : cas de test vérifiés, détails hors du dépôt.

## Feedback
