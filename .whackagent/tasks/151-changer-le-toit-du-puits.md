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
   _______
  /vvvvvvv\
   |  o  |
  (_______)
  ```

  soit `[" _______", "/vvvvvvv\\", " |  o  |", "(_______)"]`.
- La 2e ligne doit faire exactement 9 caractères (`const _: () = assert!(WELL[1].len() == WELL_WIDTH)`, `lore.rs:13`) : le toit de tuiles est élargi d'une tuile de chaque côté par rapport à la proposition B.
- Hors périmètre : couleurs, position, autres dessins (Coffre, Cabane), Lettres qui décrivent le Puits.

## Acceptance criteria

- Au jardin, une fois le Puits apparu, son haut montre le toit de tuiles ci-dessus ; le bas (`|  o  |`, `(_______)`) est inchangé.
- Le Puits garde sa place à gauche du grand Arbre, sans chevaucher le Coffre ni l'Arbre (tests de `garden.rs` verts).
- Aucune ligne du dessin ne dépasse 9 caractères.

## Implementation

## Review

## Verification

## Feedback
