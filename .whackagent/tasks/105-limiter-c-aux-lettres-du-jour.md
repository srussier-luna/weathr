---
issue: 105
status: to-close
wiki: [[mail]], [[render]]
note: il faudrait que les lettre dans la boite au letrre soit suele;met celle du jours, les autres sont lisble dans L uniquement — + séparation dans L des lettres, des recettes, des gazettes
created: 2026-11-24
---

## Context / Decisions

> c montre le courrier du jour, le reste dans l

### `c` : feuilleter les Lettres lues pendant le Jour de jeu en cours

- `c` ouvre toujours la prochaine non lue (ordre de lecture inchangé, `letters_read` reste un préfixe).
- `←`/`→` (`browse_letters`, `src/app.rs`) ne défilent que parmi les Lettres **lues pendant le Jour de jeu en cours**, quel que soit leur jour d'arrivée.
  - Jeu pas ouvert à J-1 → les Lettres de J-1 lues à J restent feuilletables toute la journée J.
  - `→` peut toujours ouvrir la non lue suivante (déjà livrée) ; elle entre dans la plage.
  - Le jour bascule à l'Heure du Facteur (Jour de jeu, comme l'histoire).
- Début de plage = `letters_read` au premier passage du Jour de jeu, sauvegardé dans `Progress` (champ `#[serde(default)]`, + Jour de jeu associé) : relancer le même jour garde la plage.
- Lettre ouverte depuis `l` et lue un jour passé → `←`/`→` sans effet, retour au Courrier par `Échap`. (Choix par défaut : question restée sans réponse au grill, à confirmer au test.)

### `l` : Courrier en onglets

- Un onglet par type : **Lettres** (village, Sam, Voisins, Marcel, sous la porte, Régimes), **Cartes postales**, **Archives**, **Recettes**, **Gazettes**. Regroupement = séries de `content/mod.rs` (`series`), Gazette à part.
- `←`/`→` changent d'onglet, `↑`/`↓` + `Entrée` dans la liste (comme aujourd'hui).
- Un onglet sans aucune entrée lue n'apparaît pas (pas de spoiler : pas d'onglet « Cartes postales » avant la première).
- Ouverture du Courrier sur l'onglet Lettres.
- Compatible avec #76 (onglets par saison) : rien de plus ici.

- Hors périmètre : lecture dans le désordre, changement de la Boîte pleine, Facteur note.

## Acceptance criteria

- Jour 12, 2 Lettres lues ce jour : `c` puis `←`/`→` passent de l'une à l'autre, jamais vers une Lettre lue au jour 11.
- Jeu non ouvert au jour 11 : au jour 12, les Lettres du jour 11 lues aujourd'hui + celles du jour 12 se feuillettent ensemble.
- Relance le même jour → même plage ; Jour de jeu suivant → plage vide jusqu'à la première lecture.
- `→` sur la dernière Lettre lue, avec une non lue livrée → l'ouvre et la marque lue.
- `l` → onglets Lettres / Cartes postales / Archives / Recettes / Gazettes ; seuls ceux qui ont au moins une entrée lue s'affichent ; `←`/`→` changent d'onglet.
- Lettre lue un jour passé ouverte depuis `l` → `←`/`→` sans effet.
- Ancienne sauvegarde (sans le nouveau champ) → se charge, plage = Lettres lues à partir de maintenant.
- Tests unitaires : calcul de la plage (même jour, rattrapage J-1, changement de jour, relance), répartition par onglet, onglets vides cachés.

## Implementation

- `src/story/today_letters.rs` (nouveau) : plage des Lettres lues pendant le Jour de jeu (`TodayLetters { day, start }`, `on_day`, `contains`, `browse`), 8 tests.
- `src/progress/mod.rs` : `today_letters_day`, `today_letters_start` (`#[serde(default)]`).
- `src/story/content/tabs.rs` (nouveau) : onglets Lettres / Cartes postales / Archives / Recettes / Gazettes, onglets vides cachés, bascule bornée, 5 tests.
- `src/app.rs` : plage recalée à chaque bascule de jour (`settle_today_letters`), `browse_letters` limité à la plage, `Overlay::Index { tab, selected }`, `←`/`→` changent d'onglet.
- `src/views.rs` : barre d'onglets, numérotation par onglet, pied `[←] [→] onglets` ; titre « Lettres reçues (n) » gardé.
- `src/story/testdata/save_37_golden.txt` : partie Courrier régénérée (onglets).
- Chevron droit : affiché seulement sur une Lettre de la plage. Pas de chevron gauche, comme avant.
- `cargo build` vert · `cargo test` vert (543 lib). Clippy : seule l'erreur `double_must_use` d'`async_trait` (`src/weather/provider.rs`), déjà présente avant.
- Non commité : commit/push réservés à `/wa-close`.

## Review

### validation — 2026-11-26

- Tour 1 : 4 remarques, corrigées automatiquement.
  - 🟠 correctness : `c` depuis une Lettre passée feuilletait les anciennes → va à la prochaine non lue, sinon Mot du Facteur, sinon ferme (`next_mail(range_start)`).
  - 🟠 correctness : `Échap` sur une Lettre rouverte depuis `l` → revient au Courrier, même onglet et sélection (`Overlay::Letter { index, from }`).
  - 🟡 structure : chevrons d'onglets via `tabs::has_neighbours`.
  - 🟡 elegance : champs `today_letters_*` privés, accesseurs `today_letters()` / `set_today_letters()`.
- Tour 2 : clean · style ✓ · elegance ✓ · structure ✓ · correctness ✓
- `cargo test` vert (546 lib).

## Verification

manual validation — not run by agent

## Feedback

### 2026-11-26

- `l` : petites flèches jaunes à gauche/droite quand un onglet précédent/suivant existe (`render_with_chevrons`, `src/app.rs`).
