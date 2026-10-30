---
issue: 82
status: todo
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

## Review

## Verification

## Feedback
