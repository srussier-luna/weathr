---
issue: 72
status: to-test
wiki: [[render]], [[mail]]
note: donner un cadre different au lettre glisser sous la porte v1.7
created: 2026-10-30
---

## Context / Decisions

- Lettres glissées sous la porte = `Entry::Creepy` (le voisin du n°13), aujourd'hui en `Frame::Parchment` comme les autres Lettres.
- Nouveau cadre `Frame` dans `src/overlay.rs` : feuille mouillée, cadre carré, tache d'eau dans le coin haut gauche, coin bas droit coupé en diagonale, gouttes sous le bas (retour #72 round 1 : le papier déchiré était trop chaotique).
- Couleur bleu clair (effet mouillé), bien distincte du parchemin.
- `Entry::Creepy` utilise ce cadre dans `content_for` (`src/story/content/mod.rs`). Titre et compteur inchangés.

## Acceptance criteria

- Ouvrir une Lettre glissée sous la porte → feuille mouillée bleu clair : cadre carré, tache d'eau `~` dans le coin haut gauche, coin bas droit coupé en diagonale sur 4 lignes (3 lignes de corps + le bas), gouttes sous le bord du bas. Maquette (largeur 20) :
  ```
  +~~~~--------------+
  (~~  Lettre 5/6    |
  |~                 |
  |  Je te vois.     /
  |                 /
  |  [c]           /
  +---,------,----'
      '      .
  ```
- Ouvrir une Lettre du village, de Sam ou des Voisins → toujours le cadre parchemin.
- Le texte reste entier et lisible dans le nouveau cadre, terminal étroit compris.
- Test unitaire : la vue d'une `Entry::Creepy` porte le nouveau cadre.

## Implementation

- Nouveau cadre `Frame::Torn` (`src/overlay.rs`) : bords irréguliers sur un cycle de 5 lignes, haut et bas en dents de scie (`TORN_TOP` / `TORN_BOTTOM`), couleur Rgb (150, 95, 95). Zone de texte fixe : seul le bord droit varie.
- `Entry::Creepy` utilise `Frame::Torn` (`src/story/content/mod.rs`). Titre et compteur inchangés.
- Golden `save_37_golden.txt` : `frame: Parchment` → `frame: Torn` sur les 26 Lettres glissées sous la porte, rien d'autre.
- Tests : `test_torn_frame_has_ragged_edges`, `test_torn_frame_keeps_text_in_place_on_narrow_terminals` (largeurs 12 à 66), `test_letter_slipped_under_the_door_comes_on_torn_paper`, `Frame::Torn` ajouté à `FRAMES`.
- Build : `cargo build`, `cargo fmt --check`, `cargo test` verts. Clippy : seule l'erreur connue `double_must_use` (#79).

- Round 1 (feedback) : `Frame::Torn` → `Frame::Wet`, bleu clair Rgb (150, 200, 235) ; haut `+~~~~---+` (tache fixe), bord gauche `(~~` puis `|~` puis `|`, coin bas droit `/` sur la dernière ligne, bas terminé par `'` avec `,` en colonne 4 puis tous les 7, gouttes `'` / `.` dessinées une à une sous le cadre (`Frame::drops`, `edges(row, last)`). Titre décalé de 2 espaces comme la maquette, repli sur le retour à la ligne normal si la boîte est trop étroite. Constantes `WET_STAIN`, `WET_TITLE_INDENT`, `FIRST_DROP`, `DROP_GAP`, `DROPS`. Golden : `frame: Torn` → `frame: Wet` (26 lignes). Fichiers : `src/overlay.rs`, `src/story/content/mod.rs`, `src/story/testdata/save_37_golden.txt`.
- Round 2 (feedback) : coupe du coin bas droit sur 4 lignes. `WET_CUT = ["/", " /", "  /"]` selon le nombre de lignes restantes, `edges(row, rows_below)`, bas et gouttes arrêtés à `width - WET_CUT.len() - 1`. La coupe reste à droite de la zone de texte (aucune marge réservée). Lettre de moins de 3 lignes : coupe sur les lignes existantes. Fichier : `src/overlay.rs`.

## Review

## Verification

- Application non lancée (Lettre glissée = partie avancée) : preuve par tests de rendu (`frame_rows`, lignes exactes). Validation manuelle à faire.

## Feedback

### Round 2 — 2026-11-02
- Asked : « dechirer la lettre sur 3 lignes supplementaire » ; choix : coupe sur 4 lignes.
- Triage : adjustment → critère et maquette mis à jour.
- Changes : voir `## Implementation` round 2 (implementer).
- Build : `cargo build`, `cargo fmt --check`, `cargo test` verts ; clippy : seule l'erreur connue #79.
- Review : deferred (→ /wa-validate).
- Verify : rendu exact à 20 colonnes = maquette ; largeurs 12–66 (texte jamais sous la coupe) ; lettre d'une ligne testée. Round 2 : manual validation — not run by agent.
- Rule captured : aucune.

### Round 1 — 2026-11-02
- Asked : « le cadre est trop cahotic c'est moche, remet un truc un peu plus carre, avec le coin bas droit coupe en diagonal » + « plus effet feuille mouille, et change aussi la couleur, effet mouille bleu claire » ; maquette « Tache d'eau » choisie.
- Triage : adjustment → critère d'acceptation mis à jour.
- Changes : cadre `Frame::Wet` (implementer), voir `## Implementation`.
- Build : `cargo build`, `cargo fmt --check`, `cargo test` verts ; clippy : seule l'erreur connue #79.
- Review : deferred (→ /wa-validate).
- Verify : rendu exact à 20 colonnes = maquette, largeurs 12–66 testées ; round 1 : manual validation — not run by agent.
- Rule captured : aucune.
