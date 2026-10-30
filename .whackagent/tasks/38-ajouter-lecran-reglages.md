---
issue: 38
status: todo
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

## Review

## Verification

## Feedback
