---
issue: 155
status: to-test
wiki: [[mail]], [[weather]]
note: ajouter un badge lorsque la geolocalisation est activé v1..8.1
created: 2026-11-24
---

## Context / Decisions

> ajouter un badge lorsque la geolocalisation est activé v1..8.1

- Nouveau Badge « Boussole », rangée Commun (créée par #157, d'où le blocage).
- Gagné quand la Ville vient de CoreLocationCLI (installé et autorisé), pas du cache `location.json` ni de l'IP. Position auto est activée par défaut : seule la vraie position de l'appareil compte.
- Détection au lancement ou depuis Réglages : les deux comptent. Joueurs déjà équipés → Badge au prochain lancement.
- Jamais en `--day`, `--demo` ni simulation, comme les autres Badges.
- Icône boussole, 3 × 5 comme les autres.

## Acceptance criteria

- CoreLocationCLI installé et autorisé, Position auto `oui`, lancement → carte « Nouveau badge ! » « Boussole ».
- CoreLocationCLI absent, Ville trouvée par l'IP → pas de Badge.
- Ville tirée du cache `location.json` → pas de Badge.
- Réglages : Position auto passée à `oui`, CoreLocationCLI répond → Badge.
- Écran Badges (`b`) : « Boussole » dans la rangée Commun.
- `--day N` avec CoreLocationCLI → pas de Badge.

## Implementation

stacked on: wa/157-ranger-les-badges-par-rarete @ d43bf9c

- `geolocation::detect_location` / `detect_location_with_city` renvoient `DetectedLocation { location, is_from_device }` ; `detect_from(device, fallback)` décide, testé. Le cache `location.json` n'est écrit qu'après l'IP : une position du cache ne donne jamais le Badge.
- `App::new` prend `Option<&DetectedLocation>` ; drapeau de session `is_located_by_device`, posé au lancement et par la détection des Réglages.
- Badge « Boussole » (`compass`, `BadgeKind::Common`, après « Bouteille à la mer ») attribué dans `collect_badges` parmi les événements non simulés (`--night` le bloque aussi, comme les Badges météo). `BADGES` 52 → 53.
- Preuve : clippy sans nouveau warning, build OK, `cargo test` tout vert.

## Review

## Verification

Autopilot, app pilotée en pty (`HOME` à part), CoreLocationCLI autorisé :
- ✅ CoreLocationCLI → carte « Boussole » au lancement.
- ✅ Sans CoreLocationCLI, Ville par l'IP → pas de Badge.
- ✅ Ville du cache `location.json` → pas de Badge.
- ✅ Réglages : Position auto `oui`, CoreLocationCLI répond → carte « Boussole ».
- ✅ Écran Badges : « Boussole » dans Commun.
- ✅ `--day 3` → pas de Badge.

## Feedback
