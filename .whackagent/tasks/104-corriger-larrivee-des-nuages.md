---
issue: 104
status: todo
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
- Changement de météo (rafraîchissement) : les Nuages déjà là prennent la nouvelle couleur d'un coup. Pas de fondu (YAGNI).
- Ligne de glossaire ajoutée avec cette tâche :
  `| Nuage | | | \`Cloud\` | traverse le ciel de gauche à droite ; blanc ciel clair, gris clair Éclaircies, gris foncé sinon → [[animation]] |`
- Hors périmètre : banc de nuages de pluie (`cloud_bank.rs`), vitesse et nombre de Nuages, sortie à droite.

## Acceptance criteria

- Nouveau Nuage → il entre par le bord gauche colonne après colonne dès sa première frame, jamais affiché en entier à la colonne 0 en restant immobile.
- `--simulate partly-cloudy` → tous les Nuages gris clair, nouveau-nés compris, sans aucune frame d'une autre couleur.
- `--simulate cloudy` et `--simulate overcast` → tous les Nuages gris foncé dès leur première frame.
- Ciel clair → Nuages blancs, comme avant.
- Tests unitaires : rendu d'un Nuage à `x` négatif = seulement sa partie visible ; couleur d'un nouveau Nuage = couleur des Nuages déjà là, pour chaque condition.
- `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings -A clippy::double_must_use` et `cargo test --bin weathr` verts.

## Implementation

## Review

## Verification

## Feedback
