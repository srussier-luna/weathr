---
issue: 125
status: to-test
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

## Verification

- Autopilot, HOME jetable, pseudo-terminal (implémenteur) : cas de test vérifiés, détails hors du dépôt.

## Feedback
