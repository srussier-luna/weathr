---
issue: 203
status: todo
wiki: [[architecture]], [[updater]]
note: "pour tester le sprint Avent avant le 1er décembre : la vraie date ne se simule pas (--holiday / --day désactivent Chocolutin, les mots et le Père Noël)"
created: 2026-11-27
---

## Context / Decisions

- But : tester #196, #197, #198 (et #201) avant le 1er décembre.
- Option de lancement `--today AAAA-MM-JJ` : le jeu se croit à cette date, **comme en vrai** (pas une simulation : Chocolutin, les mots, le Père Noël, la série, les Badges réagissent).
- **Date seule**, l'heure reste l'heure réelle.
- Réservée aux tests : n'existe que dans un build avec la fonction Cargo `dev-date`. Les builds joueurs (mise à jour `cargo build --release`) ne l'ont pas. Sans la fonction, `--today` est inconnue.
- Le script local `~/.local/bin/weathr-test` (hors dépôt) passe à `cargo build --release --features dev-date`. Validé par le joueur.
- Un seul point d'entrée pour « maintenant » / « aujourd'hui » dans le code ; tous les appels directs à l'horloge passent par lui. Règle clippy pour interdire les appels directs restants.
- Empilée sur #197 (contient #196) pour couvrir le code Avent. #198 sera rebasée dessus à son close.
- Rien dans le README ni l'aide joueur.
- Ligne de glossaire au close : Date de test.

## Acceptance criteria

- `weathr-test --today 2026-12-01` avec une sauvegarde de test neuve : Chocolutin arrive, carte du chocolat n° 1.
- `weathr-test --today 2026-12-03` ensuite : un mot de Chocolutin daté du 2 dans `[c]`, Chocolutin passe pour le 3.
- `--today 2026-12-03` : la Barre du bas montre `[v] Calendrier`.
- L'heure affichée reste l'heure réelle.
- Build sans la fonction `dev-date` : `weathr --today 2026-12-01` est refusé comme option inconnue, `--help` ne la mentionne pas.
- Date invalide (`--today 2026-13-01`) : message d'erreur clair, le jeu ne démarre pas.

## Implementation

## Review

## Verification

## Feedback
