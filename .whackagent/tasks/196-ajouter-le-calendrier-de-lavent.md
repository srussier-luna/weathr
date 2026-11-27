---
issue: 196
status: to-test
wiki: [[animation]], [[mail]], [[render]]
note: "il nous faut un calendrier de l'avent — split en #196, #197, #198, sprint Avent, livré en 1.8.3"
created: 2026-11-27
---

## Context / Decisions

- Sprint **Avent**, livré dans une version à part **1.8.3**, avant le 1er décembre. Branche `sprint/avent` depuis `develop`.
- Découpe : #196 (ici) Chocolutin + 24 chocolats + calendrier · #197 mots de Chocolutin si absent · #198 Père Noël du 25 + bébé renne.
- **Chocolutin** : le lutin chocolatier du Père Noël.
  - Du 1er au 24 décembre (date réelle, jamais en `--day`/démo), au premier lancement du jour, il traverse la scène jusqu'à la maison, comme le Facteur (`src/animation/postman.rs`).
  - Arrivé, une carte montre le chocolat du jour : son nom et son dessin.
  - Une fois par jour : relancer le même jour ne le refait pas venir.
- **Chocolats** : 24, tous différents, chocolats de luxe. Un nom gourmand et un dessin ASCII propre à chacun. Le chocolat du jour J est le n° J.
- **Calendrier de l'Avent** : vue à 24 cases ouverte par `[v]`.
  - `[v] calendrier` dans la Barre du bas du 1er au 31 décembre seulement, absent le reste de l'année.
  - Case reçue : le chocolat (nom + dessin). Case à venir : fermée avec son numéro. Case passée non reçue : vide (#197 la remplira via le mot).
- Sauvegarde : jours reçus pour l'année en cours (année + ensemble de jours). Une nouvelle année repart à zéro. Champs `#[serde(default)]`.
- Hors périmètre : mots si absent (#197), Père Noël et renne (#198).
- Lignes de glossaire au close : Chocolutin, Calendrier de l'Avent, Chocolat.

## Acceptance criteria

- 1er décembre, premier lancement : Chocolutin traverse la scène jusqu'à la maison, puis une carte montre le chocolat n° 1 (nom + dessin).
- Relancer le même jour : Chocolutin ne revient pas, pas de carte.
- 30 novembre ou 26 décembre : Chocolutin ne vient pas.
- En décembre, la Barre du bas montre `[v] calendrier` ; `[v]` ouvre la grille de 24 cases.
- Dans la grille : les cases reçues montrent leur chocolat, les cases à venir sont fermées avec leur numéro.
- En novembre ou en janvier : pas de `[v]` dans la Barre du bas, la touche ne fait rien.
- Les 24 chocolats ont chacun un nom et un dessin différents.
- Les anciennes sauvegardes chargent sans erreur ; l'année suivante, le calendrier repart vide.

## Implementation

- Nouveau dossier `src/advent/` : `mod.rs` (`is_advent`, `chocolate(day)`, `due_day`, états de case Received / Missed / Closed), `chocolates.rs` (24 chocolats nommés, dessin 3×8 chacun), `view.rs` (grille 6×4, curseur aux flèches, carte « Chocolat n°J »).
- `src/animation/chocolutin.rs` : sprite et marche ; arrive par la droite, s'arrête juste à droite de la maison (le Facteur garde la boîte à gauche).
- Sauvegarde : `advent_year: Option<i32>`, `advent_days: BTreeSet<u32>` (`#[serde(default)]`) ; `receive_advent_day(year, day)` remet à zéro à l'année suivante (réutilisable par #197).
- Venue : date réelle seulement (pas en démo, `--holiday`, `--condition`, `--night`, `--day`), jour pas encore reçu. Attend que les fenêtres de lancement soient fermées (Sauvegarde, Bienvenue, Nouveautés, Noël, Badges). Jour enregistré à l'ouverture de la carte ; quitter pendant la marche → il revient au prochain lancement du jour.
- `[v]` bascule le calendrier quand `holiday.is_december` ; ignorée sinon. Barre du bas `[v] Calendrier` (majuscule comme les autres touches) et aide en décembre seulement.
- Modifiés : `src/animation/mod.rs`, `src/animation_manager.rs`, `src/app.rs`, `src/hud/toolbar.rs`, `src/main.rs`, `src/progress/mod.rs`, `src/views.rs`.
- Build : clippy OK, build OK, fmt OK ; tests 182 + 659 + 10 + 7 + 3 + 5, 0 échec.

## Review

## Verification

round 1: build + tests only — date réelle non simulable (27/11), Chocolutin non observé à l'écran. À tester : run: yours.

## Feedback
