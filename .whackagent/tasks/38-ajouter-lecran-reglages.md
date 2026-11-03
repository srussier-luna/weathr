---
issue: 38
status: to-close
wiki: [[render]], [[architecture]], [[weather]]
note: Écran Réglages : ville, unités, heure du Facteur, barre du haut modifiables dans le jeu, sans éditer config.toml
created: 2026-10-30
---

## Context / Decisions

- Écran Réglages ouvert par la touche `s`, ajoutée à l'Aide. `Échap` le ferme.
- Réglages proposés :
  - unité de température (°C / °F) ;
  - unité de vent (m/s, km/h, mph, kn) ;
  - unité de pluie (mm / in) ;
  - heure du Facteur (`courrier.heure`) ;
  - Barre du haut affichée ou masquée (`hide_hud`).
- Hors périmètre : la ville. Aucune recherche de ville par nom n'existe aujourd'hui, ce sera une tâche à part.
- Hors périmètre : `silent`, qui ne sert qu'aux messages de démarrage, et la sonnette du Facteur, qui est un choix d'histoire.
- Enregistrement dans `config.toml`, en ne modifiant que les lignes changées : commentaires et ordre conservés (`toml_edit`). `config.toml` reste la seule source.
- Effet immédiat pour les unités et la Barre du haut.
- Nouvelle heure du Facteur : prise en compte dès le lendemain, pour ne pas rejouer ni sauter la tournée du jour.
- Vue overlay comme les autres écrans (`views.rs`, [[render]]).

## Acceptance criteria

- Touche `s` → l'écran Réglages s'ouvre avec les valeurs actuelles de `config.toml`.
- L'Aide liste la touche `s`.
- Passer la température en °F → la Barre du haut affiche des °F aussitôt, sans redémarrer.
- Changer le vent en km/h ou la pluie en in → la Barre du haut suit aussitôt.
- Masquer la Barre du haut → elle disparaît aussitôt ; la réafficher → elle revient.
- Changer l'heure du Facteur → la tournée du jour garde l'ancienne heure, celle du lendemain prend la nouvelle.
- Après un changement → `config.toml` contient la nouvelle valeur, et ses commentaires et autres lignes sont intacts.
- Relancer le jeu → les réglages choisis sont toujours là.
- `config.toml` absent → l'écran fonctionne et crée le fichier au premier changement.
- `Échap` → l'écran se ferme et le jeu reprend.

## Implementation

- `src/config.rs` → `src/config/mod.rs` (+ `Setting`, `Config::save`, `save_to_path`) ; `src/config/writer.rs` (nouveau) : `write_value(path, keys, value)` avec `toml_edit`, une seule ligne modifiée, commentaires et ordre gardés, tables manquantes créées en fin de fichier, tests sur fichiers temporaires.
- `src/settings.rs` (nouveau) : `Row`, `ROWS`, `Settings::stepped` / `apply`, choix en boucle, heure 0–23.
- `src/story/clock.rs` : `mail_hour_on`, `change_mail_hour` → nouvelle heure du Facteur le lendemain, tests.
- `src/weather/units.rs` : fonctions `*_symbol` ; `src/app_state.rs` : `set_units` + test.
- `src/views.rs` : `settings_view`, `[s] réglages` dans l'Aide, test.
- `src/app.rs` : `Overlay::Settings(usize)`, `toggle_settings`, `change_setting`, touches ↑↓ / ←→ / Entrée / s / Échap.
- `src/error.rs` : erreurs d'écriture de config. `Cargo.toml` : `toml_edit 0.24` (partage toml_datetime, winnow, toml_parser, toml_writer avec `toml 0.9`).
- Échec d'écriture → réglage appliqué quand même, message « Réglage non enregistré : vérifie config.toml ».
- Heure du Facteur changée dans les Réglages : date d'effet + ancienne heure gardées dans `progress` (`mail_hour_change`), relues par `story_clock()` ; une relance le jour même garde l'ancienne heure, le Jour de jeu ne recule jamais. Sous `--day`, l'heure ne change pas. Une modification à la main de config.toml s'applique dès la relance.
- L'écran montre les réglages en cours, options de lancement comprises (`--hide-hud`, `--imperial`).
- Build : `cargo build`, `cargo fmt --check`, `cargo test` verts. Clippy : seule l'erreur connue #79.

## Review

### Round validation — 2026-11-03
- Scope : diff cumulé `origin/develop...HEAD` + correctifs (src/app.rs, src/app_state.rs, src/config/{mod,writer}.rs, src/error.rs, src/main.rs, src/settings.rs, src/story/clock.rs, src/progress.rs, src/views.rs, src/weather/units.rs, Cargo.toml, Cargo.lock).
- Findings (autofix, 2 tours, tous corrigés) :
  - 🟠 [correctness] relance le même jour : nouvelle heure du Facteur appliquée aussitôt, Jour de jeu pouvant reculer → date d'effet + ancienne heure dans `progress`, relues par `story_clock()`.
  - 🟡 [correctness] config.toml et progress pouvaient diverger (échec d'écriture, `--day`) → `App::change_mail_hour` : horloge changée seulement si les deux enregistrements réussissent, rien sous `--day`.
  - 🟡 [correctness] échec d'écriture : un toast par touche → `push_toast` sans doublon.
  - 🟡 [style] `\u{a0}` avant « : » → `\u{202F}`.
  - 🟡 [structure] `App.mail_hour` doublait `StoryClock` → lu via `clock.mail_hour()`.
  - 🟡 [structure] `toml_edit 0.25` doublait `toml_datetime` / `winnow` → `toml_edit 0.24`.
  - 🟡 [elegance] deux `Option` parallèles → `mail_hour_change: Option<MailHourChange>`.
  - 🟡 [style] écran Réglages absent du glossaire → ligne Screens `Réglages` / `settings_view` (validée par l'utilisateur·rice).
- Preuve : `cargo build`, `cargo fmt -- --check`, `cargo clippy -- -D warnings -A clippy::double_must_use`, `cargo test --bin weathr` (246) verts ; `cargo tree -d` sans doublon toml*/winnow.
- LENSES : style ✓ · elegance ✓ · structure ✓ · correctness ✓ — VERDICT : clean.

## Verification

Captures pty, HOME jetable avec un config.toml commenté (jamais le vrai) :
- ✅ `s` ouvre l'écran avec les valeurs de config.toml ; l'Aide liste `[s] réglages`.
- ✅ °F, m/s, mph, in : la Barre du haut suit aussitôt.
- ✅ Barre du haut masquée puis réaffichée aussitôt.
- ✅ Heure du Facteur 10 h : « Nouvelle heure du Facteur dès demain. », Jour de jeu inchangé ; règle prouvée par tests de clock.rs.
- ✅ config.toml : valeurs écrites, commentaires et autres lignes intacts.
- ✅ Relance : réglages conservés.
- ✅ config.toml absent : écran OK, fichier créé au premier changement.
- ✅ Échap ferme, le jeu reprend.

## Feedback
