---
issue: 114
status: to-test
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
- Où : nouvelle animation par feature (`src/animation/`, ex. `arcane.rs` : mage, vol, invocation, heure), réutilise l'horloge du Facteur (`src/animation/postman.rs`) sans copier son dessin, `src/scene/decorations.rs` (boîte violette + drapeau), état des Arcanes dans `Progress`, hors timeline (`src/story/arcane.rs`), `src/overlay.rs` (`Frame` Nuit étoilée), `src/progress.rs`, `src/main.rs` (flag).
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

- Arcanes hors timeline (pas une `StorySource` : pas déterministes par jour), état dans `Progress`, fusionnées à l'affichage et dans le parcours `c` / `l`.
- `src/progress.rs` : `arcanes_unlocked_on`, `arcanes_received_on` (liste de dates : nombre, dernière date, ordre du sommaire), `arcanes_read`, `arcanes_passed_on` (dernier passage : lire à 22 h ne relivre pas le soir même), tous `#[serde(default)]`.
- `src/story/arcane.rs` : règle pure `pass(clock, now)` (passage = heure du Facteur du jour + 12 mod 24, une par jour, jamais deux en attente, rien d'accumulé, plus rien après la dernière) ; `ArcanePost::saved`, `sample` (`--arcane`, n° 0, jamais écrit), `round()` (passage à minuit compté le bon jour).
- `src/story/content/arcanes.rs` : `ARCANES` vide (textes : #116) + Arcane d'exemple.
- `src/story/event.rs` / `content/mod.rs` : `Entry::Arcane(n)`, sans série ni « /total », titre « Arcane n° k ». `src/story/postman_note.rs` : `next_mail` avec `waiting_arcane` (note du Facteur, Lettres non lues, puis Arcane).
- `src/overlay.rs` : `Frame::StarryNight` (`.:*~*:.`, bords `) ` ` (`, RGB 180, 90, 255, largeur multiple de 7).
- `src/scene/decorations.rs` : Boîte violette (`.-.` / `|_|`) sur la boîte jaune (constante `MAILBOX`).
- `src/animation/postman.rs` : horloge du Facteur sortie en `round_due` / `ride_time`, partagée ; comportement du Facteur inchangé.
- `src/animation/arcane.rs` : `ArcanoPostman` — mage violet, balai marron, vol à altitude fixe ; s'arrête au-dessus de la Boîte violette s'il dépose, invocation (baguette + étincelles, 3 à 9 s), drapeau turquoise (RGB 64, 224, 208) après l'invocation ; sinon traverse.
- `src/main.rs` : `--arcane` (`hide = true`), `SimulationOptions.arcane`.
- Barre du haut inchangée : une Arcane en attente n'affiche pas « Lettre ! », seul le drapeau turquoise le dit.
- Build : `cargo build`, `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings -A clippy::double_must_use`, `cargo test --bin weathr` (353) verts.

- Round 1 (feedback) : `launch_for(simulation, has_save, is_quiet)` dans `src/app.rs` — `--arcane` lance sans Accueil ni Notes (comme `--day`), même sans save ; Boîte violette 4 de large (`.--.` / `|__|`), centrée sur le corps de la boîte jaune (`VIOLET_MAILBOX_WIDTH`), drapeau, étincelles et arrêt du mage suivent. Tests `test_arcane_preview_keeps_the_welcome_off_the_mage_even_without_a_save`, `test_violet_mailbox_sits_centred_on_the_yellow_one`. 355 tests verts.

## Review

## Verification

- Round 1 : manual validation — not run by agent (run: yours).
- Contrôle de mise en page (implémenteur, pty 80×24 et 130×40, HOME jetable, pas une preuve) : Boîte violette seulement avec `--arcane` ; vers 21 h le mage s'arrête, invoque, repart ; drapeau turquoise ensuite ; `c` → « Arcane n° 0 » cadre Nuit étoilée, drapeau baissé ; jeu normal sans boîte ni mage ; save non touchée.
- Pour voir le vol : lancer à l'heure du Facteur + 12 h, binaire lancé directement (faketime ne traverse pas le script zsh `weathr-test`, macOS retire DYLD_*) : `HOME=~/.weathr-test faketime -f '@2026-11-20 20:59:50' ~/Repos/.wa-worktrees/test/target/release/weathr --arcane --night`.
- Round 1 : manual validation — not run by agent (run: yours). Capture pty (release, HOME neuf, 100×30, faketime 20:59:50) : arrivée par la gauche à 20:59:59, arrêt au-dessus de la Boîte violette à 21:00 avec baguette et étincelles, départ à droite avec drapeau turquoise levé, sortie à 16 s.

## Feedback

### Round 1 — 2026-11-16
- Asked : « faketime -f '@2026-11-20 20:59:56' weathr-test --arcane --night ne fonctionne pas, le mage ne passe pas, la boîte violette n'est pas assez centrée ».
- Triage : defect (le mage invisible) + adjustment (centrage). Causes : `faketime` ne traverse pas le script `weathr-test` (SIP) → le jeu tournait à l'heure réelle ; sur un HOME neuf, la bulle de l'Accueil cachait le mage.
- Changes : `--arcane` sans Accueil ; Boîte violette centrée (implementer).
- Review : deferred (→ /wa-validate).
- Verify : manual validation — not run by agent ; capture pty de l'implémenteur.
- Rule captured : aucune.
