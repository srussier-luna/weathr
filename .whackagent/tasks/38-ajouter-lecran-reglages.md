---
issue: 38
status: to-test
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
- `src/error.rs` : erreurs d'écriture de config. `Cargo.toml` : `toml_edit 0.25` (toml_parser / toml_writer montent aussi).
- Échec d'écriture → réglage appliqué quand même, message « Réglage non enregistré : vérifie config.toml ».
- Limite : relancer le jeu le jour même applique tout de suite la nouvelle heure du Facteur (config.toml relu au lancement, pas de date d'effet stockée).
- L'écran montre les réglages en cours, options de lancement comprises (`--hide-hud`, `--imperial`).
- Build : `cargo build`, `cargo fmt --check`, `cargo test` verts. Clippy : seule l'erreur connue #79.

## Review

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
