---
issue: 151
status: to-close
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

- `src/animation/lore.rs` : `WELL` lignes 0-1 → `" _______"`, `"/vvvvvvv\\"` ; bas, couleurs et dessin inchangés.
- `cargo build` vert · `cargo test` vert · `cargo fmt -- --check` propre.
- Non commité : commit/push réservés à `/wa-close` (règle perso).

## Review

### validation — 2026-11-24

- Verdict : clean · style ✓ · elegance ✓ · structure ✓ · correctness ✓
- `cargo test` vert (tests `garden.rs` placement/chevauchement inclus).
- Aucun autofix, code inchangé depuis le test.

## Verification

manual validation — not run by agent

## Feedback
