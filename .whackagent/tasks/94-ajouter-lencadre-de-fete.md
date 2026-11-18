---
issue: 94
status: todo
wiki: [[animation]], [[render]]
note: "ajouter un event quand un jours de fete est afficher, nouvel encadré, v 1.8"
created: 2026-11-18
---

## Context / Decisions

> ajouter un event quand un jours de fete est afficher, nouvel encadré, v 1.8

- **Quand** : à chaque lancement un jour de Fête, après les fenêtres de lancement existantes (Sauvegarde, Accueil, Mot des voisins, Nouveautés). `[Echap]` ferme.
- **Fêtes** (`Holiday`, `src/holiday.rs`) : Noël (25/12), Nouvel An, Épiphanie, Poisson d'avril, Pâques, Fête des voisins, Fête de la musique, 14 juillet, Halloween.
  - Aussi les pluies d'étoiles filantes : Perséides, Géminides.
  - Chandeleur (#95) et 1er mai (#96) apportent chacune leur texte, dans la tâche qui atterrit en second.
- **Contenu** : nom de la Fête en titre, 2-3 lignes dans la voix du village, dont ce qu'il faut guetter (ex. Fête de la musique : « Ce soir, le village chante. Ouvre l'œil : des notes s'échappent des fenêtres. »).
- Textes écrits au code sur le ton des Lettres, validés par l'utilisateur·rice au test.
- Les Fêtes forcées (`--christmas`, `--fireworks`, `--neighbours`, `--music`) montrent aussi l'encadré.
- Nouvel `Overlay` à côté des fenêtres de lancement, vue dans `src/views.rs`, cadre existant.

## Acceptance criteria

- `--music` (ou un 21 juin) → au lancement, encadré « Fête de la musique » avec ses lignes, après les autres fenêtres de lancement ; `[Echap]` → la scène.
- Relancer le même jour → encadré à nouveau.
- Jour sans Fête → pas d'encadré.
- Chaque Fête de la liste et chaque pluie d'étoiles a son titre et ses lignes (test sur toutes).
- `cargo fmt --check`, `cargo test` verts.

## Implementation

## Review

## Verification

## Feedback
