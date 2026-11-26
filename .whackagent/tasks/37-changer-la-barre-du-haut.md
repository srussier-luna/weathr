---
issue: 37
status: to-close
wiki: [[render]], [[architecture]]
note: "barre du haut à refaire" — découpée au grill en #37, #162 Encadrer les toasts, #163 Signaler les nouveaux Badges
created: 2026-11-24
---

## Context / Decisions

> barre du haut à refaire
>
> Ajouté un ticket pour refaire la barre du haut

- Aujourd'hui la barre du haut est une seule ligne cyan qui mélange l'heure, le Jour, les indices de jeu, les touches et la météo, avec des libellés anglais. Le but : séparer les infos météo (le cœur de l'app) des contrôles (secondaires).
- proto: [[protos/changer-la-barre-du-haut-separation]] · https://claude.ai/artifact/U5Kc6zxWSthjRk6ru3W5eC (v10 validé)
- Deux barres :
  - **Barre du haut** (code `weather_bar`) : en haut, sur plusieurs lignes.
  - **Barre du bas** (code `toolbar`) : sur la dernière ligne du terminal, calée à droite.
- Barre du haut, bloc gauche, une info par ligne, chaque ligne commence par une majuscule :
  1. `Éclaircies 12 °C` : ciel + température sur la même ligne. Les noms du ciel en français sont ceux des Badges météo (`Ciel dégagé`, `Éclaircies`, `Couvert`…). Température arrondie à l'entier.
  2. `Vent 8 km/h`
  3. `Pluie 1,2 mm`
  4. `Pluie vers 16:00` ou `Pluie dans l'heure` : l'alerte pluie, raccourcie (plus de « prends ton parapluie »).
  5. `Hors ligne`
  - Une valeur à 0 ne s'affiche pas, et sa ligne disparaît (pas de vent → pas de ligne Vent).
  - Pendant le chargement : `Météo en route…` seul.
- Barre du haut, bloc droit : l'heure (`14:32`) en haut, `Lyon • Jour 12` dessous. La Ville suit le mode d'affichage actuel (nom, coordonnées, mixte).
- Couleurs, une par info :
  - l'heure en clair et en gras ;
  - la Ville et le Jour en gris ;
  - le ciel en couleur de texte ;
  - le vent en gris, la pluie en bleu, les alertes en jaune.
- La température est teintée en 7 paliers, en gras : ≤ 0 bleu glacé · 1 à 5 bleu · 6 à 11 turquoise · 12 à 17 vert · 18 à 24 jaune · 25 à 29 orange · ≥ 30 rouge.
- Barre du bas : `[c] Courrier   [b] Badges 3/12   [h] Aide   [q] Quitter`, en gris.
  - Quand une Lettre attend, `[c] Courrier` devient `[c] Lettre !`. Quand un objet est trouvé, `[r] Objet !` apparaît.
  - Ces deux-là sont en rose et clignotent : rose ↔ gris toutes les 500 ms, le rythme des chevrons.
- Le réglage « Barre du haut » est renommé « Barres ». « masquée » (et `--hide-hud`) cache les deux barres. Toasts et cartes s'affichent quand même.
- Où vit le code : nouveau dossier `src/hud/` (`mod.rs`, `weather_bar.rs`, `toolbar.rs`). Ce sont des fonctions pures : état → lignes de segments colorés, testables. Elles remplacent la chaîne `cached_weather_info` construite dans `AppState::update_cached_info`. `AppState` garde les données, `App::run` dessine.
- Hors scope, dans d'autres tâches du sprint Barres :
  - #162 Encadrer les toasts : cadre, position au-dessus de la Barre du bas, texte tapé.
  - #163 Signaler les nouveaux Badges : `[b] Badge !` qui clignote, la vue Badges, le « pas encore vu » sauvegardé. La carte « Nouveau Badge » est gardée.
- Version : v1.9.

## Acceptance criteria

- Au lancement avec la météo chargée, la ligne 1 en haut à gauche montre le ciel et la température en français (`Éclaircies 12 °C`), et le coin haut droit montre `HH:MM`, avec `<Ville> • Jour N` dessous.
- Avec un vent de 0 km/h et 0 mm de pluie, il n'y a ni ligne Vent ni ligne Pluie. Avec du vent, la ligne `Vent 8 km/h` est sous le ciel.
- Avec une alerte pluie active, une ligne jaune `Pluie vers HH:MM` (ou `Pluie dans l'heure`) s'affiche. « prends ton parapluie » n'apparaît plus.
- À −8, 3, 9, 14, 21, 27 et 34 °C, la température prend la couleur de son palier.
- La dernière ligne du terminal montre `[c] Courrier   [b] Badges x/y   [h] Aide   [q] Quitter`, calé à droite, en gris. Aucune touche n'est sur la ligne du haut.
- Lettre en attente → `[c] Lettre !` en rose, qui clignote toutes les 500 ms. Objet trouvé → `[r] Objet !` pareil.
- Réglages : la ligne s'appelle « Barres ». Sur « masquée », les deux barres disparaissent tout de suite. `--hide-hud` fait pareil au lancement.
- Pendant le chargement, la Barre du haut montre `Météo en route…` avec l'heure et `Ville • Jour N` à droite.
- À 70 colonnes, aucun texte des barres ne déborde ni ne se chevauche.
- Plus aucun libellé anglais (`Weather:`, `Temp:`, `Wind:`, `Precip:`, `Location:`, `Press 'q' to quit`, `OFFLINE`) n'est affiché.

## Implementation

- autopilot 2026-11-24, wa-implementer, round 1.
- `src/hud/` (new): `mod.rs` (`Segment`, `Line`, `Placed`, `lay_out`, `truncate`), `weather_bar.rs` (`WeatherBar::new(&AppState)`, temperature steps, `location_label`), `toolbar.rs` (`toolbar::line(&AppState, is_lit, max_width)`). Pure functions + unit tests.
- `app_state.rs`: `cached_weather_info`, `update_cached_info`, `get_condition_text`, `LoadingState` removed; `update_clock`, `take_weather_info_update` added.
- `app.rs`: `weather_bar` cache rebuilt on flag; toolbar built per frame (blink); `render_placed`; nothing drawn when `hide_hud`. Find hints → `Objet` / `Bouteille` / `Objet et bouteille`.
- `overlay.rs`: `blink_tick()` (500 ms) shared by chevrons + toolbar.
- `rain_alert.rs`: `message()` → `Pluie vers HH:MM` / `Pluie dans l'heure` (single caller).
- `views.rs`: Réglages label « Barre du haut » → « Barres ».
- Sky names from `badges::name(weather_badge(..))`.
- 70 cols: right block cut with `…` 2 cols from left line; toolbar falls back to compact form (`[c]`/`[h]`/`[q]` without word, 2-space gap).
- Open: bold (clock, temperature) not done: renderer `Cell` has colour only. `--demo` label stays English, grey line under left block. Coordinates keep `40.71°N, 74.01°W`.
- Proof: `cargo build` ok · `cargo test` 166 + 543 + 10 + 7 + 3 + 5 passed.

## Review

### validation — 2026-11-24 · clean

Round 1, 5 findings (all 🟡), autofixed:
- [correctness] `hud/mod.rs` Barre du bas never cut to width → `truncate` to inner width, alerts first so `[h]`/`[q]` go before them; 50-col test
- [elegance] `rain_line` read zero from the formatted string → compares the rounded number
- [elegance] toolbar `Item` two bools → enum `Key` / `Counter` / `Alert`
- [structure] `MUTED` duplicated in both bars → once in `hud/mod.rs`
- [style] positional `format!` args → inlined

Round 2: all fixed, nothing new. Build (clippy strict) ✅ · tests ✅.

## Verification

- manual validation — not run by agent (TUI, needs a TTY). Criteria covered by unit tests in `src/hud/`.

## Feedback
