---
issue: 104
status: to-test
wiki: [[animation]]
note: "Retour de Denis (Slack, 05/11) : nuages qui apparaissent d'un coup à gauche, immobiles quelques secondes, parfois blancs avant de passer gris. À corriger en v1.7 avant livraison."
created: 2026-11-05
---

## Context / Decisions

> Les nuages qui apparaissent à gauche apparaissent d'un coup et restent immobiles quelques secondes, puis commencent à bouger. Parfois, ils apparaissent en blanc avant de passer gris, ce qui fait un genre de flash pas top. (Denis, Slack)

- Deux défauts, au même moment : l'arrivée d'un Nuage par la gauche (`src/animation/clouds.rs`).
- **Nuage figé** : un nouveau Nuage naît hors écran à gauche (`x = -largeur`), mais `render` ramène `x` à 0 (`max(0, x)`) et dessine la forme entière. Tant que `x < 0`, il reste affiché en entier à la colonne 0, sans bouger : `largeur / vitesse` frames, soit 4 à 13 s.
  - Décision : couper la partie hors écran à gauche, ligne par ligne. Le Nuage entre colonne après colonne dès sa première frame.
- **Flash** : deux règles de couleur se contredisent.
  - `update` crée le Nuage avec la couleur de `render_background` (`src/animation_manager.rs`) : blanc ciel clair, gris clair (`Color::Grey`) Éclaircies, gris foncé sinon.
  - `set_cloud_color(is_clear)`, appelé à chaque frame, ne connaît que blanc / gris foncé et repeint tout. Le nouveau-né passe donc une frame en gris clair, presque blanc.
  - Décision (utilisateur·rice) : une seule règle, celle que le code visait. Ciel clair → blanc · Éclaircies (`PartlyCloudy`) → gris clair · Nuageux, Couvert et le reste → gris foncé. La même règle sert à la création et à chaque frame.
  - Conséquence voulue : par temps Éclaircies, les Nuages passent de gris foncé à gris clair pour les joueurs.
- **Chevauchement** (ajouté par l'utilisateur·rice) : chaque ligne d'un Nuage est dessinée opaque, espaces compris. Le Nuage dessiné en dernier efface une bande autour de lui sur celui de derrière.
  - Décision : l'un passe devant l'autre. Chaque ligne est dessinée de son premier à son dernier caractère non blanc : l'intérieur reste plein et cache ce qui est derrière, les espaces avant et après la forme sont transparents.
- Changement de météo (rafraîchissement) : les Nuages déjà là prennent la nouvelle couleur d'un coup. Pas de fondu (YAGNI).
- Ligne de glossaire ajoutée avec cette tâche :
  `| Nuage | | | \`Cloud\` | traverse le ciel de gauche à droite ; blanc ciel clair, gris clair Éclaircies, gris foncé sinon → [[animation]] |`
- Hors périmètre : banc de nuages de pluie (`cloud_bank.rs`), vitesse et nombre de Nuages, sortie à droite.

## Acceptance criteria

- Nouveau Nuage → il entre par le bord gauche colonne après colonne dès sa première frame, jamais affiché en entier à la colonne 0 en restant immobile.
- `--simulate partly-cloudy` → tous les Nuages gris clair, nouveau-nés compris, sans aucune frame d'une autre couleur.
- `--simulate cloudy` et `--simulate overcast` → tous les Nuages gris foncé dès leur première frame.
- Ciel clair → Nuages blancs, comme avant.
- Deux Nuages qui se croisent → celui de devant cache l'autre seulement à l'intérieur de son contour ; plus de bande vide autour de lui.
- Tests unitaires : rendu d'un Nuage à `x` négatif = seulement sa partie visible ; couleur d'un nouveau Nuage = couleur des Nuages déjà là, pour chaque condition ; deux Nuages qui se chevauchent : celui de derrière reste visible hors du contour de celui de devant.
- `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings -A clippy::double_must_use` et `cargo test --bin weathr` verts.

## Implementation

- `src/animation/clouds.rs` :
  - `visible_part(line, x)` (privée) : coupe les espaces avant / après la forme (transparents), décale la colonne d'autant, puis coupe la partie hors écran à gauche par caractère (`x.floor()`). Ligne entièrement hors écran → rien. Les espaces intérieurs restent écrits (opaques).
  - `cloud_color(condition)` (publique) : Clear → blanc, PartlyCloudy → gris clair, le reste et météo inconnue → gris foncé. `set_cloud_color(is_clear)` supprimée ; `update` repeint chaque Nuage avec cette couleur et crée les nouveaux avec la même.
  - 3 tests : rendu à `x` négatif / décimal / hors écran et caractère non ASCII ; couleur identique pour tous les Nuages sur 20 000 frames par condition ; Nuage de devant qui ne cache l'autre qu'à l'intérieur de son contour.
- `src/animation_manager.rs` : `render_background` passe `clouds::cloud_color(condition)` à `update`.
- Arrondi de colonne passé de `as i16` à `floor()` : un Nuage ne reste plus deux fois plus longtemps en colonne 0 entre x = -1 et 1.
- Les Nuages laissent voir le soleil et les autres Nuages autour de leur contour (décision chevauchement).
- Build : `cargo build`, `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings -A clippy::double_must_use`, `cargo test --bin weathr` (324) verts.

## Review

## Verification

Binaire réel en pty 120×40, HOME jetable, flux ANSI décodé en grille avec couleur par case (round 1, implémenteur) :
- ✅ Entrée par la gauche (Éclaircies, Nuageux, Couvert, ciel clair) : frame 0 = `)` en colonne 0, puis `_)`, `)__)`… le Nuage avance colonne après colonne.
- ✅ Éclaircies : toutes les lignes de Nuage en gris clair, nouveau-nés compris (38;5;7).
- ✅ Nuageux et Couvert : tout en gris foncé (38;5;8) ; ciel clair : tout en blanc (38;5;15).
- ✅ Deux Nuages qui se croisent (Nuageux) : celui de derrière reste visible hors du contour de celui de devant, plus de bande vide.
- Écran d'Accueil affiché (HOME neuf), rangs 22–28 seulement, ciel non masqué.

## Feedback
