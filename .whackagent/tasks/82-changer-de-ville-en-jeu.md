---
issue: 82
status: to-close
wiki: [[weather]], [[render]], [[architecture]]
note: Changer la ville météo depuis l'écran Réglages (#38) : recherche par nom, choix parmi les homonymes. Sorti du périmètre de #38.
created: 2026-10-30
---

## Context / Decisions

- Bloquée par #38 (écran Réglages) : ajoute deux lignes à cet écran, « Ville » et « Position auto ».
- Ligne « Ville » : le joueur tape un nom, `Entrée` lance la recherche, une liste de 5 résultats au plus s'affiche (« Lyon, Rhône, France »…), il en choisit un.
- Recherche par nom via Nominatim, déjà utilisé pour `reverse_geocode` (`src/geolocation.rs`) : même client, même User-Agent, mêmes délais.
- Ville choisie → `location.latitude`, `location.longitude`, `location.city` écrits dans `config.toml` (même écriture que #38), `location.auto = false`.
- Ligne « Position auto » : oui/non. Repasser à oui → détection auto comme au lancement.
- Effet immédiat : la météo est rechargée pour la nouvelle position, le nom affiché dans la Barre du haut change.
- Pas de réseau → « Pas de connexion ». Aucun résultat → « Aucune ville trouvée ». Ville inchangée, champ gardé pour réessayer.

## Acceptance criteria

- Réglages → ligne « Ville » → taper « Lyon » + `Entrée` → liste de 5 résultats au plus, avec région et pays.
- Choisir « Lyon, Rhône, France » → la Barre du haut affiche Lyon et la météo de Lyon, sans redémarrer.
- `config.toml` contient alors les coordonnées et le nom de Lyon, `auto = false`, commentaires intacts.
- Relancer le jeu → toujours Lyon.
- Taper « zzzzqx » → « Aucune ville trouvée », ville inchangée.
- Sans réseau → « Pas de connexion », ville inchangée.
- « Position auto » sur oui → la position détectée remplace la ville choisie, `auto = true` dans `config.toml`.
- `Échap` pendant la recherche → retour aux Réglages, ville inchangée.

## Implementation

- `src/settings.rs` → `src/settings/mod.rs` : `Row::City`, `Row::AutoLocation` (7 lignes), `stepped` → `Option<Setting>`. `src/settings/city.rs` (nouveau) : `CitySearch`, `Status { Typing, Searching, Found, NotFound, NoConnection }`, tests.
- `src/geolocation.rs` : `search_cities(query, language)` (Nominatim, `featureType=city`, `limit=5`), parseur pur `places_from` + `place_label` (région = state_district, sinon county, sinon state), doublons retirés ; `nominatim_get` partagé avec le reverse geocoding ; `detect_location_with_city` ; `Place`. Tests sur un exemple JSON.
- `src/config/writer.rs` : `write_values` (None retire la clé) ; `src/config/mod.rs` : `Config::save_location`, test.
- `src/app.rs` : boucle météo dans `spawn_weather`, relancée avec la nouvelle position (et l'alerte pluie) ; recherche non bloquante lue à chaque frame ; `handle_city_key`, `toggle_auto_location`, `move_to`. `src/app_state.rs` : `set_location`. `src/views.rs` : lignes Ville / Position auto, `city_search_view`, test.
- Erreur de recherche (réseau, HTTP, JSON) → « Pas de connexion » ; liste vide → « Aucune ville trouvée » ; texte gardé.
- Position auto « non » écrit la position courante avec auto = false ; « oui » écrit la position détectée avec auto = true ; détection ratée → « Pas de connexion », rien ne change.
- Choisir une ville écrit aussi `location.display = "city"` (`Config::save_city`) et l'applique aussitôt (`AppState::set_location_display`) : la Barre du haut montre le nom même si elle était en coordonnées (décision utilisateur·rice, validation). « Position auto » sur oui ne touche pas `display`.
- Build : `cargo build`, `cargo fmt --check`, `cargo test` verts. Clippy : seule l'erreur connue #79.

## Review

### Round validation — 2026-11-03
- Scope : diff depuis le point d'empilement (#38) + correctifs ; #82 regroupé en un commit et rebasé sur le nouveau #38 (conflits de `src/app.rs` résolus, comportements des deux tâches gardés).
- Findings (autofix, 3 tours + 1 choisi, tous corrigés) :
  - 🟠 [correctness] `Entrée` répété dépassait 1 requête/s Nominatim → limite commune dans `nominatim_get` (recherche + nom de ville d'une position).
  - 🟡 [correctness] Barre du haut en coordonnées après le choix d'une ville → `display = "city"` écrit et appliqué (décision utilisateur·rice).
  - 🟡 [correctness] commentaire perdu quand `city` est retiré (y compris dernière clé de table) → reporté sur la clé / l'en-tête suivant.
  - 🟡 [correctness] anciennes boucles météo / pluie survivantes → `select!` sur `sender.closed()`.
  - 🟡 [correctness] « Position auto » sur non relançait météo et pluie → enregistrement seul.
  - 🟡 [correctness] recherche ou détection annulée continuait → `settings::send_unless_cancelled<T>`.
  - 🟡 [style] `\u{a0}` → `\u{202F}` ; « Pas de connexion » via `push_toast` ; `Status` → `SearchStatus` ; délais en `Duration` ; `let _ =` sur deux `RequestBuilder` de test.
  - 🟡 [elegance] deux bool pour trois états → `AutoLocation { Off, Locating, On }`.
  - 🟡 [structure] `receive_location_search` séparé en `receive_city_search` / `receive_location_detection` ; `send_unless_cancelled` dans `settings/mod.rs`.
  - 🟡 [style] « Ville » absente du glossaire → ligne Concepts (validée par l'utilisateur·rice).
- Preuve : `cargo build`, `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings -A clippy::double_must_use`, `cargo test --bin weathr` (272) verts.
- LENSES : style ✓ · elegance ✓ · structure ✓ · correctness ✓ — VERDICT : clean.

## Verification

Captures pty, HOME jetable avec config.toml commenté (jamais le vrai) :
- ✅ Réglages : « Ville < Paris > », « Position auto non ».
- ✅ « zzzzqx » → « Aucune ville trouvée », ville inchangée.
- ✅ « Lyon » → « Lyon, Rhône, France » / « Lyon, Coahoma County, United States » (doublons retirés), ↓ déplace.
- ✅ Choix Lyon → Barre du haut « Location: Lyon » et météo de Lyon sans redémarrer.
- ✅ config.toml : coordonnées et nom de Lyon, auto = false, commentaires intacts ; relance → Lyon.
- ✅ Échap pendant la recherche → Réglages, ville inchangée.
- ✅ Position auto oui → position détectée, auto = true.
- ⚠️ « Pas de connexion » hors ligne : non vu, couvert par tests unitaires.

## Feedback
