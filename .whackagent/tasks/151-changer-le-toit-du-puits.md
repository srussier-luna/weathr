---
issue: 151
status: todo
wiki: [[scene]], [[animation]]
note: changer la forme du haut du puits
created: 2026-11-24
---

## Context / Decisions

- Le Puits (`WELL`, `src/animation/lore.rs:12`) garde sa largeur (`WELL_WIDTH` = 9) et ses 2 lignes du bas.
- Nouveau haut, toit de tuiles (choix B) :

  ```
    _____
   /vvvvv\
   |  o  |
  (_______)
  ```

  soit `["  _____", " /vvvvv\\", " |  o  |", "(_______)"]`.
- Hors périmètre : couleurs, position, autres dessins (Coffre, Cabane), Lettres qui décrivent le Puits.

## Acceptance criteria

- Au jardin, une fois le Puits apparu, son haut montre le toit de tuiles ci-dessus ; le bas (`|  o  |`, `(_______)`) est inchangé.
- Le Puits garde sa place à gauche du grand Arbre, sans chevaucher le Coffre ni l'Arbre (tests de `garden.rs` verts).
- Aucune ligne du dessin ne dépasse 9 caractères.

## Implementation

## Review

## Verification

## Feedback
