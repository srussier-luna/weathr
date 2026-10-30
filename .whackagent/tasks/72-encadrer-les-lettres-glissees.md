---
issue: 72
status: to-test
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

- Nouveau cadre `Frame::Torn` (`src/overlay.rs`) : bords irréguliers sur un cycle de 5 lignes, haut et bas en dents de scie (`TORN_TOP` / `TORN_BOTTOM`), couleur Rgb (150, 95, 95). Zone de texte fixe : seul le bord droit varie.
- `Entry::Creepy` utilise `Frame::Torn` (`src/story/content/mod.rs`). Titre et compteur inchangés.
- Golden `save_37_golden.txt` : `frame: Parchment` → `frame: Torn` sur les 26 Lettres glissées sous la porte, rien d'autre.
- Tests : `test_torn_frame_has_ragged_edges`, `test_torn_frame_keeps_text_in_place_on_narrow_terminals` (largeurs 12 à 66), `test_letter_slipped_under_the_door_comes_on_torn_paper`, `Frame::Torn` ajouté à `FRAMES`.
- Build : `cargo build`, `cargo fmt --check`, `cargo test` verts. Clippy : seule l'erreur connue `double_must_use` (#79).

## Review

## Verification

- Application non lancée (Lettre glissée = partie avancée) : preuve par tests de rendu (`frame_rows`, lignes exactes). Validation manuelle à faire.

## Feedback
