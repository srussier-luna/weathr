---
issue: 182
status: to-close
wiki: [[mail]]
note: "Badge contributeur de type écrivain ou plume d'or, pour le contributeur qui a réécrit les Lettres (#177)."
created: 2026-11-26
---

## Context / Decisions

- Nouveau Badge « Plume du village », id `village_quill`, carte « Plume du village · a prêté sa plume au village ». Remercie l'auteur des Lettres réécrites (#177).
- **Identité** : nom git `bpisano` en clair, dans une liste à part de `CONTRIBUTORS` (ex. `WRITERS`), vérifiée avec `is_listed` et le `git_user_name()` déjà lu une fois dans `App::new`. Choix assumé : revient sur la règle « aucun nom dans le code » de #109, pour ce Badge seulement.
- **Maître du temps reste retiré** : `CONTRIBUTORS` reste vide, le commentaire « do not remove » reste. bpisano l'a déjà dans sa sauvegarde.
- **Nouvelle rangée « Contribution »** (`BadgeKind::Contribution`) : Badges donnés à des personnes. Vigie du ciel, Maître du temps et Plume du village y passent (ils quittent Secrets).
  - Place : après Secrets, avant Mauvais points.
  - Un Badge Contribution n'est affiché et compté que s'il est gagné. Rangée entière cachée pour qui n'en a aucun (`badge_lines` saute déjà les familles vides).
  - Cette règle remplace la liste `RETIRED` de #109 (même effet pour Vigie et Maître).
- Hors périmètre : pas de Trophée, pas de note de version.

Glossaire à ajouter :

| Terme | Évite | Code | Sens |
|---|---|---|---|
| Contribution | contributeur | `BadgeKind::Contribution` | rangée des Badges donnés à des personnes, cachée pour qui n'en a aucun → [[mail]] |
| Plume du village | plume d'or, écrivain | `village_quill` | Badge Contribution de l'auteur des Lettres → [[mail]] |

## Acceptance criteria

- `git config user.name` = `bpisano` → au lancement, carte « Plume du village · a prêté sa plume au village » ; `b` → Badge coché dans la rangée « Contribution ».
- Autre nom, pas de nom, pas de git → aucune carte Plume du village, ni Maître du temps, ni Vigie du ciel.
- Sauvegarde sans Badge Contribution → écran Badges : pas de rangée « Contribution », total inchangé par ces trois Badges.
- Sauvegarde avec Vigie du ciel et/ou Maître du temps → rangée « Contribution » affichée après Secrets, seulement avec les Badges gagnés, comptés dans le total.
- Rangée Secrets : ne contient plus Vigie du ciel ni Maître du temps.
- `bpisano` avec Maître du temps déjà gagné → ne le perd pas, gagne Plume du village une seule fois (pas de carte au lancement suivant).

## Implementation

- `src/badges.rs` : `BadgeKind::Contribution` (« Contribution »), dans `KINDS` après Secret, avant Demerit. Constructeur `contribution(...)`. Vigie du ciel, Maître du temps et nouveau `village_quill` (« Plume du village », « a prêté sa plume au village », icône plume) dans ce bloc, ids inchangés. `RETIRED` supprimé : `is_shown` = gagné, ou ni Demerit ni Contribution. `WRITERS = ["bpisano"]` + `is_village_quill` via `is_listed`. `CONTRIBUTORS` vide et commentaires « do not remove » gardés.
- `src/app.rs` : champ `is_village_quill` depuis le même `user_name` lu dans `App::new`, attribution à côté de `time_master` (`earn` n'attribue qu'une fois).
- `src/overlay.rs` : aucun rendu changé (rangée vide déjà sautée) ; test `test_contribution_row_shows_only_once_owned`.
- Tests : titres des rangées, `is_village_quill` (nom, espaces, autre, vide, None), rangée Contribution, collection et total avec/sans Badges, total Tricheur ajusté.
- Glossaire : Contribution, Plume du village ; ligne Badge. Wiki mail : rangée Contribution.
- Preuve : clippy, build, tests (179 + 614 + 10 + 7 + 3 + 5), fmt verts.

## Review

Round validation — 2026-11-26, one wa-verifier, LENSES style ✓ elegance ✓ structure ✓ correctness ✓.
- Clean, no finding. Tests verts, fmt OK.

## Verification

round 1: manual validation — not run by agent

## Feedback
