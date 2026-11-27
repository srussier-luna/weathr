---
issue: 197
status: to-test
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

stacked on: #196 (wa/196-ajouter-le-calendrier-de-lavent)

- `src/advent/note.rs` (nouveau) : 8 textes en alternance, signature « — Chocolutin », titre « Mot de Chocolutin du 1er/2 décembre ».
- `src/advent/mod.rs` : `missed_days(received, today)` = cases `Missed` du calendrier.
- `src/advent/view.rs` : `card_lines` extrait ; `note_view(date)` = mot + signature + carte du chocolat n°d.
- `src/app.rs` : `Overlay::ChocolutinNote(NaiveDate)`, `chocolutin_notes(...)`, garde date réelle (`is_real_date`). `[c]` ouvre d'abord le mot non lu le plus ancien, puis le courrier habituel. Mots non lus → signal courrier allumé. Lire → `receive_advent_day`.
- `day_and_month` (règle « 1er ») déplacé de `views.rs` vers `story/content/mod.rs`.
- **Écart** : les mots passent par le flux `[c]` (comme le mot du Facteur), pas dans la liste `[l]` onglet Lettres. La liste `[l]` suit la timeline par jour d'histoire ; y mêler des dates réelles demande un champ sauvegardé et une liste mixte. Relecture depuis `[l]` = brique à part si voulue.
- Proposition de fiche de voix (letters.md, à valider) : Chocolutin — lutin chocolatier, joyeux et pressé, phrases courtes, parle de sa tournée ; Tic « hop » ; ne dit jamais de reproche, « Joyeux Noël » avant le 24, ni le nom du chocolat.
- Build : clippy OK, build OK, fmt OK ; tests 182 + 669 + 10 + 7 + 3 + 5, 0 échec.

## Review

## Verification

round 4: manual validation — not run by agent.

round 3: manual validation — not run by agent.

round 2: manual validation — not run by agent.

round 1: build + tests only — date réelle non simulable (27/11). À tester : run: yours.

## Feedback

### Round 2 — 2026-11-27

- Asked : « tu as oublié de remettre today ? » — #197 et #203 en retard sur #196 tour 2.
- Triage : défaut d'empilement → rebase sur #196 (945bc4f), conflits src/advent/mod.rs, view.rs, app.rs résolus par l'implementer.
- Changes : `note_view` passe au cadre `Frame::Gift` et reprend la nouvelle carte du chocolat (phrase du jour) ; `note.rs` utilise `chocolutin::SIGNATURE`. Les 8 textes suivent la fiche de voix.
- Review : deferred. Verify : manual, run: yours.
- Build : clippy OK, fmt OK, tests 183 + 677 + 10 + 7 + 3 + 5, 0 échec.

### Round 3 — 2026-11-27

- Asked : suite du tour 4 de #196 (nouvelle histoire de Chocolutin, discours + papillote).
- Triage : défaut d'empilement + ajustement de ton → rebase sur #196 (5dbb4d5), conflit src/advent/view.rs résolu par l'implementer.
- Changes : `note_view` = mot + « — Chocolutin » + `chocolate_lines(day)` (dessin, n°, nom, papillote, sans discours) dans `Frame::Gift` ; 8 mots réécrits (maladresses à la porte / boîte aux lettres, questions naïves sur l'atelier).
- Review : deferred. Verify : manual, run: yours.
- Build : clippy OK, fmt OK, tests 184 + 679 + 10 + 7 + 3 + 5, 0 échec.

### Round 4 — 2026-11-27

- Asked : « il faut ajouter un ton simplet et bienheureux à Chocolutin » (tour 6 de #196).
- Triage : rebase propre sur #196 + ajustement de ton.
- Changes (implementer) : `src/advent/note.rs` — 8 mots réécrits (petites maladresses racontées comme des merveilles, lecture naïve et heureuse des indices de l'atelier), sans reprendre les anecdotes des 24 discours.
- Review : deferred. Verify : manual, run: yours.
- Build : clippy OK, fmt OK, tests 184 + 679 + 10 + 7 + 3 + 5, 0 échec.

