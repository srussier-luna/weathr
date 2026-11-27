---
issue: 203
status: to-test
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

stacked on: #197 (wa/197-ajouter-les-mots-de-chocolutin, contient #196)

- `src/clock.rs` (nouveau) : `now()` unique ; sous `dev-date`, `set_today(NaiveDate)` fixe un décalage en jours (`OnceLock`), l'heure locale est gardée (même à travers un changement d'heure).
- `clippy.toml` (nouveau) : `disallowed-methods` sur `chrono::Local::now` et `chrono::Utc::now` (« use clock::now »).
- `Cargo.toml` : `[features] dev-date = []`.
- `src/main.rs` : champ `today` caché (`hide = true`), seulement avec `dev-date` ; `clock::set_today` juste après la lecture des options.
- Les 35 appels directs à l'horloge passent par `clock::now()` (overlay, holiday, demo, animation_manager, app_state, app, progress, postman, scene/house/river/tree, story/clock/reach).
- `--today` compte comme vraie date (`is_real_date` inchangé).
- Limites : jeu ouvert au-delà de minuit → la date de test avance aussi ; `--today` apparaît dans les complétions shell du build `dev-date` (pas du build joueur).
- Script local `~/.local/bin/weathr-test` : `cargo build --release --features dev-date`.
- Build : clippy OK (avec et sans `dev-date`), build OK, fmt OK ; tests sans 183 + 672 + 10 + 7 + 3 + 5, avec 187 + 677 + 10 + 7 + 3 + 5, 0 échec.
- Preuves CLI : build `dev-date` `--today 2026-13-01` → erreur, sortie 2 ; `--today 2026-12-01 --version` → OK ; `--help` sans `--today`. Build joueur `--today` → « unexpected argument », sortie 2.

## Review

## Verification

round 2: manual validation — not run by agent.

round 1: build + tests + preuves CLI ; jeu non piloté. À tester : run: yours.

## Feedback

### Round 2 — 2026-11-27

- Asked : « tu as oublié de remettre today ? » — `--today` absente de la branche de test après le tour 2 de #196.
- Triage : défaut d'empilement → rebase sur #197 (289f1df), conflit src/app.rs (`give_chocolate`) résolu en gardant le code #196/#197 avec `clock::now()`.
- Contrôle : aucun appel direct à l'horloge dans src/ (le code Avent neuf n'en ajoute pas).
- Review : deferred. Verify : manual, run: yours.
- Build : clippy OK (avec et sans dev-date), fmt OK ; tests sans 184 + 680 + 10 + 7 + 3 + 5, avec 188 + 685 + 10 + 7 + 3 + 5, 0 échec.

