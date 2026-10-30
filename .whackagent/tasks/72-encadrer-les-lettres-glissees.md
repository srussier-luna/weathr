---
issue: 72
status: todo
wiki: [[render]], [[mail]]
note: donner un cadre different au lettre glisser sous la porte v1.7
created: 2026-10-30
---

## Context / Decisions

- Lettres glissées sous la porte = `Entry::Creepy` (le voisin du n°13), aujourd'hui en `Frame::Parchment` comme les autres Lettres.
- Nouveau cadre `Frame` dans `src/overlay.rs` : papier déchiré, bords irréguliers façon feuille arrachée.
- Couleur sombre gris-rouge, bien distincte du parchemin.
- `Entry::Creepy` utilise ce cadre dans `content_for` (`src/story/content/mod.rs`). Titre et compteur inchangés.

## Acceptance criteria

- Ouvrir une Lettre glissée sous la porte → cadre aux bords déchirés, gris-rouge.
- Ouvrir une Lettre du village, de Sam ou des Voisins → toujours le cadre parchemin.
- Le texte reste entier et lisible dans le nouveau cadre, terminal étroit compris.
- Test unitaire : la vue d'une `Entry::Creepy` porte le nouveau cadre.

## Implementation

## Review

## Verification

## Feedback
