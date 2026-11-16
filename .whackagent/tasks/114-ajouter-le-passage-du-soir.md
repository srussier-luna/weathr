---
issue: 114
status: todo
wiki: [[animation]], [[mail]], [[scene]], [[render]]
note: "Découpé de #113 (parapluie « Ajouter un secret »), sprint « Le secret ». Débloqué par #115, Lettres écrites dans #116."
created: 2026-11-16
---

## Context / Decisions

> Spoiler : ce fichier décrit le secret. Ticket, PR et commits restent vagues (« passage du soir », « secret »).

- **Arcano-Facteur** : second Facteur, débloqué par la piste du secret (#115). Avant le déblocage : absent, rien ne change.
  - Pas la forme du Facteur (retour utilisateur·rice) : un **vieux mage violet sur un balai**, qui vole dans le ciel. Violet électrique, bien lisible sur fond noir (ex. RGB 180, 90, 255). Dessin à valider au test.
  - Passe chaque jour à l'heure du Facteur du jour + 12 h (`clock.mail_hour_on(today)`, ex. 9 h → 21 h).
  - Passage : il arrive en volant, **s'arrête en l'air au-dessus de la Boîte violette sans descendre**, fait une **invocation magique** (étincelles), l'Arcane apparaît dans la boîte, puis il repart en volant. Jour sans Arcane à déposer : il traverse le ciel sans s'arrêter.
  - Après la dernière Arcane : continue de passer chaque soir, sans Lettre.
- **Boîte violette** : dès le déblocage, mini boîte aux lettres violette au-dessus de la boîte jaune (`src/scene/decorations.rs`), avec son propre drapeau **bleu turquoise** levé quand une Arcane attend. Reste pour toujours.
- **Arcane** : Lettre de l'Arcano-Facteur.
  - Titre « Arcane n° 3 », numérotation à part des Lettres (ne touche ni `letters_read` ni le compteur des Lettres).
  - Même Courrier (touche `c`), mêlées aux autres par ordre d'arrivée.
  - Cadre **Nuit étoilée** (nouveau `Frame`) : haut `.:*~*:.` en boucle, bords `)` `(`, en violet électrique.
  - Ne compte pas dans la Boîte pleine : n'arrête ni ne relance l'histoire.
- **Rythme** (décision utilisateur·rice) : au plus une Arcane par jour, jamais plus d'une en attente.
  - Jeu ouvert au moment du passage, ou ouvert plus tard le même jour → l'Arcane du jour arrive au passage (ex. ouvert vendredi 20 h : rien, puis l'Arcane à 21 h).
  - Jours sans jeu ouvert : rien ne s'accumule (dimanche 22 h après un samedi fermé : une seule Arcane).
  - Arcane précédente non lue → pas de nouvelle Arcane ; la suivante arrive à un passage après sa lecture (lire à 22 h ne fait pas repasser l'Arcano-Facteur le soir même).
- **Sauvegarde** (`Progress`, `#[serde(default)]`) : date de déblocage, nombre d'Arcanes reçues, nombre lues, date de la dernière livraison. Ancienne sauvegarde → secret non débloqué.
- **Textes** : les Arcanes viennent de #116 (écrites à deux). Tant que la liste est vide, aucune Arcane n'arrive.
- **Test** : option cachée `--arcane` (absente de `--help`, clap `hide`), comme `--music` : débloque sans toucher à la sauvegarde et fournit une Arcane d'exemple (« Arcane n° 0 », texte de démonstration) visible seulement sous cette option. `--night` pour le voir passer de nuit.
- Où : nouvelle animation par feature (`src/animation/`, ex. `arcane.rs` : mage, vol, invocation, heure), réutilise l'horloge du Facteur (`src/animation/postman.rs`) sans copier son dessin, `src/scene/decorations.rs` (boîte violette + drapeau), nouvelle source dans `src/story/sources/` (heure de livraison propre, pas l'heure du Facteur), `src/overlay.rs` (`Frame` Nuit étoilée), `src/progress.rs`, `src/main.rs` (flag).
- Pas de Badge, rien dans l'Aide (`h`) : le secret reste secret.
- Lignes de glossaire ajoutées avec cette tâche : Arcano-Facteur, Arcane, Boîte violette.
- Hors périmètre : la piste qui débloque (#115), le texte des Arcanes (#116), choix du nombre d'Arcanes.

## Acceptance criteria

- Partie normale, secret non débloqué → ni Arcano-Facteur, ni Boîte violette ; Lettres et numérotation inchangées.
- `weathr --arcane` → Boîte violette au-dessus de la boîte jaune ; à l'heure du Facteur + 12 h, un vieux mage violet sur un balai arrive en volant, s'arrête en l'air au-dessus de la Boîte violette, fait une invocation magique, puis repart en volant.
- Jour sans Arcane à déposer (en attente ou plus de texte) → il traverse le ciel sans s'arrêter.
- `weathr --arcane`, après son passage → drapeau turquoise levé ; `c` → « Arcane n° 0 » dans la liste, cadre Nuit étoilée violet ; la lire → drapeau baissé.
- `weathr --help` → `--arcane` n'apparaît pas.
- Tests unitaires du rythme (secret débloqué, Facteur à 9 h) :
  - vendredi 20 h → aucune Arcane ; vendredi 21 h → n° 1 ;
  - n° 1 non lue, dimanche 22 h → toujours seulement n° 1 ;
  - n° 1 lue dimanche 22 h → rien ce soir-là ; lundi 21 h → n° 2 ;
  - samedi sans jeu ouvert → rien ne s'accumule ;
  - après la dernière Arcane → il passe, aucune Arcane.
- Ancienne sauvegarde (sans les nouveaux champs) → se charge, secret non débloqué.
- `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings -A clippy::double_must_use`, `cargo test --bin weathr` verts.

## Implementation

## Review

## Verification

## Feedback
