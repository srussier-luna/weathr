---
issue: 82
status: to-test
wiki: [[weather]], [[render]]
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

- stacked on: #38 (`wa/38-ajouter-lecran-reglages`).
- `src/settings.rs` → `src/settings/mod.rs` : `Row::City`, `Row::AutoLocation` (7 lignes), `stepped` → `Option<Setting>`. `src/settings/city.rs` (nouveau) : `CitySearch`, `Status { Typing, Searching, Found, NotFound, NoConnection }`, tests.
- `src/geolocation.rs` : `search_cities(query, language)` (Nominatim, `featureType=city`, `limit=5`), parseur pur `places_from` + `place_label` (région = state_district, sinon county, sinon state), doublons retirés ; `nominatim_get` partagé avec le reverse geocoding ; `detect_location_with_city` ; `Place`. Tests sur un exemple JSON.
- `src/config/writer.rs` : `write_values` (None retire la clé) ; `src/config/mod.rs` : `Config::save_location`, test.
- `src/app.rs` : boucle météo dans `spawn_weather`, relancée avec la nouvelle position (et l'alerte pluie) ; recherche non bloquante lue à chaque frame ; `handle_city_key`, `toggle_auto_location`, `move_to`. `src/app_state.rs` : `set_location`. `src/views.rs` : lignes Ville / Position auto, `city_search_view`, test.
- Erreur de recherche (réseau, HTTP, JSON) → « Pas de connexion » ; liste vide → « Aucune ville trouvée » ; texte gardé.
- Position auto « non » écrit la position courante avec auto = false ; « oui » écrit la position détectée avec auto = true ; détection ratée → « Pas de connexion », rien ne change.
- La Barre du haut n'affiche le nom de ville qu'avec `location.display = "city"` ou `"mixed"` (comportement existant, non forcé).
- Build : `cargo build`, `cargo fmt --check`, `cargo test` verts. Clippy : seule l'erreur connue #79.

## Review

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
