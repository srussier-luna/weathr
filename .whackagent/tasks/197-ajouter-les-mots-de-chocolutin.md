---
issue: 197
status: todo
wiki: [[mail]]
note: "split de #196 (calendrier de l'Avent)"
created: 2026-11-27
---

## Context / Decisions

- Bloquée par #196 (Chocolutin, chocolats, calendrier, sauvegarde des jours reçus).
- Absent un jour de l'Avent (1–24 décembre, date réelle) : Chocolutin laisse un **mot** dans la boîte aux lettres.
- **Un mot par jour manqué**, daté, chacun avec son chocolat. Ils arrivent dans le Courrier `[c]`, onglet Lettres.
- Couvre aussi les jours avant le premier lancement de décembre (premier lancement le 5 → mots du 1 au 4).
- **Lire le mot donne le chocolat** : la case du calendrier se remplit.
- Voix de Chocolutin : fiche à ajouter dans `.whackagent/wiki/letters.md` (proposer au code, valider le ton).
- Lignes de glossaire au close : Mot de Chocolutin.

## Acceptance criteria

- Jeu ouvert le 1er, fermé le 2, rouvert le 3 : un mot daté du 2 est dans le Courrier ; Chocolutin passe pour le 3.
- Lire le mot du 2 : la carte du chocolat n° 2 s'affiche, la case 2 du calendrier montre ce chocolat.
- Premier lancement le 5 décembre : 4 mots (du 1 au 4).
- Un mot non lu laisse sa case vide dans le calendrier.
- Pas de mot pour un jour où le chocolat a déjà été reçu.

## Implementation

## Review

## Verification

## Feedback
