---
issue: 155
status: todo
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

## Review

## Verification

## Feedback
