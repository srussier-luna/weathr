---
issue: 17
status: to-close
wiki: [[animation]], [[mail]], [[scene]], [[updater]], [[architecture]]
note: Fête de la musique : notes ♪ qui sortent de la maison le 21 juin
created: 2026-10-30
---

## Context / Decisions

- Fête de la musique = le 21 juin seulement, toute la journée, jour et nuit (comme le poisson d'avril ou la galette).
- Nouveau flag `Holiday.is_music_day` dans `src/holiday.rs`, calculé par `Holiday::on`.
- Animation : notes ♪ ♫ qui sortent des fenêtres de la maison, montent en ondulant, disparaissent sous le toit (fenêtres collées à l'avant-toit : quelques rangs au-dessus). Plusieurs à la fois.
- Petite et grande maison : les notes partent des fenêtres présentes dans le sprite affiché.
- Les Habitants visibles, Léo et le Facteur chantent aussi : de temps en temps une note ♪ ou ♫ monte au-dessus de leur tête, mêmes couleurs que les notes des fenêtres (retour round 1).
- Météo sans effet : notes par tous les temps (la musique joue dedans).
- Nouveau système dans `src/animation/`, à côté de `epiphany.rs`, branché dans `AnimationManager`.
- Badge « Fête de la musique », `BadgeKind::Holiday`, gagné en lançant le jeu le 21 juin, comme Noël ou Halloween.
- Aperçu : une scène « Fête de la musique » dans `--demo`, et un flag `--music` comme `--christmas` / `--fireworks`.

## Acceptance criteria

- Date simulée au 21 juin → des notes ♪ ♫ montent depuis les fenêtres de la maison, de jour comme de nuit.
- 21 juin → des notes ♪ ♫ montent aussi, de temps en temps, au-dessus de la tête des Habitants visibles, de Léo et du Facteur quand il est là.
- 21 juin, de jour → à la place du soleil, un dessin de 12 lignes : une croche puis deux doubles croches reliées par des traits droits, têtes alignées, sur la course du soleil (retour round 2).
- 21 juin, de nuit → le même dessin à la place de la lune.
- 21 juin sous la pluie, l'orage ou la neige → le dessin reste dans le ciel, devant les nuages.
- La couleur du dessin change de temps en temps.
- Ciel trop bas pour 12 lignes → soleil ou lune habituels.
- 20 juin et 22 juin → aucune note, soleil et lune habituels.
- 21 juin sous la pluie → les notes sont toujours là.
- Petite maison (terminal étroit) et grande maison → les notes partent bien des fenêtres de chacune.
- Lancer le jeu le 21 juin → Badge « Fête de la musique » gagné, visible dans l'Album.
- `weathr --music` → la maison joue de la musique, quelle que soit la date.
- `weathr --demo` → une scène « Fête de la musique » passe, avec son label dans la Barre du haut.
- `Holiday::on` : vrai le 21 juin, faux le 20 et le 22 (test unitaire).

## Implementation

- `src/holiday.rs` : `Holiday.is_music_day` (21 juin), `Holiday::music_day()`, test `test_music_day`.
- `src/animation/music.rs` (nouveau) : `MusicNotes`, 10 notes ♪ ♫ au plus, 15 % de chance par frame, montée 0,06 rang/frame, ondulation ±1,5 colonne, couleurs rose, or, cyan, vert clair. Les notes s'effacent en atteignant l'avant-toit (`~~~`) ; celles des fenêtres collées à l'avant-toit montent jusqu'à `rise_top(y)` (`RISE_ROWS`, comme les têtes).
- `src/scene/house.rs` : `House::windows()` (chaque `[`…`]` des murs, 8 petite maison, 19 grande), `House::eave_row()`, 2 tests.
- `src/animation_manager.rs` : notes dessinées dans `render_garden` après la couronne de l'Épiphanie.
- `src/badges.rs` : Badge `music_day` « Fête de la musique » après Géminides (41 Badges) ; `src/app.rs` : attribué comme Noël.
- `src/main.rs` : `--music` ; `src/demo.rs` : scène « Fête de la musique ».
- Hors demande : `weathr --music` ajouté au bloc « Fêtes » du README.
- Build : `cargo build`, `cargo fmt --check`, `cargo test` verts. Clippy : seule l'erreur connue #79.

- Round 1 (feedback) : chant des Habitants, de Léo et du Facteur. `MusicNotes::update(holiday, layout, singers, rng)` : notes en coordonnées écran, chacune avec sa rangée de disparition (fenêtres : avant-toit ; têtes : 4 rangs au-dessus). Plafond 16 notes, 1 % par frame et par chanteur visible, fenêtres inchangées (15 %). Accesseurs de tête sur chat, chien, hibou, canard, grenouille, escargot, écureuil, Léo, et `Postman::rider_head` (Facteur visible). Notes dessinées après Léo, donc par-dessus les animaux. Chatons, canetons et ami du hibou ne chantent pas. Fichiers : `src/animation/music.rs`, `src/animation_manager.rs`, `src/animation/{cat,dog,owl,duck,frog,snail,squirrel,kid,postman}.rs`.

- Round 2 (feedback) : dessin de notes à la place du soleil et de la lune le 21 juin (et `--music`). `src/animation/music.rs` : `SKY_NOTES` (12 lignes × 27 colonnes, une croche + deux doubles croches, traits droits, têtes alignées), `SkyNotes` (`HEIGHT`, `width()`, `update()`, `render()` via `garden::draw`, `shows(holiday, horizon_y)`), couleur qui tourne dans `PALETTE` toutes les `COLOR_STEP_FRAMES = 90` frames (~3 s). `src/animation_manager.rs` : course du soleil extraite dans `on_sun_path` ; le jour, dessin sur la course du soleil avec sa vraie taille, la nuit à `MoonSystem::position()` (nouveau, `src/animation/moon.rs`), lune et soleil non dessinés ; dessiné après le banc de nuages, par tous les temps. Sans course du soleil (`day_progress` = None) : centré en haut du ciel. Ciel < 12 lignes → soleil / lune habituels. 5 tests.

## Review

### Round validation — 2026-11-03
- Scope : diff cumulé `origin/develop...HEAD` (2 rounds de feedback compris) + correctifs (src/animation/{music,sun_path}.rs, src/animation_manager.rs).
- Findings (autofix, 2 tours, tous corrigés) :
  - 🟠 [correctness] notes des fenêtres collées à l'avant-toit effacées après une frame (5/8 petite maison, 8/19 grande) → elles montent jusqu'à `rise_top(y)` ; `play_window`, `rise()` ; test `test_every_window_note_stays_on_screen_for_a_while`.
  - 🟡 [structure] `on_sun_path` entre struct et impl → `src/animation/sun_path.rs` avec `SUN_MARGIN` et son test.
  - 🟡 [elegance] repli inutile `choose().unwrap_or` → `random_range(0..LEN)`.
  - 🟡 [elegance] `width()` recalculée → `SkyNotes::WIDTH`.
  - 🟡 [style] littéral `70.0` → `SINGER.0 as f32`.
  - 🟡 [structure] `HEAD_RISE` sert aussi aux fenêtres → `RISE_ROWS`.
- Preuve : `cargo build`, `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings -A clippy::double_must_use`, `cargo test --bin weathr` (242) verts.
- LENSES : style ✓ · elegance ✓ · structure ✓ · correctness ✓ — VERDICT : clean.

## Verification

Captures pty, HOME jetable (jamais le vrai) :
- ✅ `--music` jour, `--music --night`, `--music --night --simulate rain` : notes aux fenêtres.
- ✅ Grande maison (200×45) : notes sur les rangées de fenêtres.
- ✅ `--demo` : `DEMO 34/37 Fête de la musique`.
- ✅ `Holiday::on` vrai le 21 juin, faux le 20 et le 22 (test).
- ✅ Album : 41 Badges comptés.
- ⚠️ Badge gagné un vrai 21 juin : non vu (modes forcés sans Badges), câblé comme Noël.

- Round 2 : manual validation — not run by agent (run: yours). Capture texte pty, HOME jetable, jour pluie 100×30 : traits au-dessus du banc de nuages, pluie devant. Repli « ciel < 12 lignes » : à 20 lignes (minimum du jeu) lune habituelle (capture 80×20 nuit), dessin dès 21 lignes (test).

## Feedback

### Round 1 — 2026-11-02
- Asked : « fais chanter les habitants, leo et le facteur » ; choix : ♪ au-dessus de la tête.
- Triage : adjustment (choisi par l'utilisateur·rice) → critère d'acceptation élargi.
- Changes : voir `## Implementation` round 1 (implementer).
- Build : `cargo build`, `cargo fmt --check`, `cargo test` verts ; clippy : seule l'erreur connue #79.
- Review : deferred (→ /wa-validate).
- Verify : captures `--music` jour / nuit / pluie : notes au-dessus du chien, de la niche, du canard ; chat, hibou, écureuil, escargot, grenouille, Léo, Facteur couverts par tests seulement (tirage aléatoire). Round 1 : manual validation — not run by agent.
- Rule captured : aucune.

### Round 2 — 2026-11-03
- Asked : « remplacer le soleil ou la lune par un dessin de (croche, deux double croche) — montre-moi les dessins, s'il pleut mets-la aussi — hauteur de 15 lignes » ; puis « trois notes à la suite une croche et deux double croche », « par tous les temps et elle change de couleur de temps en temps », « presque colle-les », « mets-les sur 10 lignes », image de référence : « non, comme ça », « le trait du haut doit être le même en diagonale, propose aussi une version droite, mets-les sur 12 lignes » → version droite choisie.
- Triage : adjustment → critères d'acceptation mis à jour (dessin droit de 12 lignes, nuit, tous temps, couleur, repli ciel trop bas).
- Changes : voir `## Implementation` round 2 (implementer).
- Build : `cargo build`, `cargo fmt -- --check`, `cargo clippy -- -D warnings -A clippy::double_must_use`, `cargo test --bin weathr` (241) verts.
- Review : deferred (→ /wa-validate).
- Verify : manual validation — not run by agent.
- Rule captured : aucune.
