---
issue: 116
status: todo
wiki: [[mail]]
note: "Découpé de #113 (parapluie « Ajouter un secret »), sprint « Le secret ». Arcanes livrées par l'Arcano-Facteur de #114."
created: 2026-11-16
---

## Context / Decisions

> Spoiler : ce fichier décrit le secret. Ticket, PR et commits restent vagues (« Lettres du secret »).

- Blocked by #114 : livraison, Courrier, numérotation et cadre Nuit étoilée des Arcanes viennent de #114. Ici : le texte et ses choix.
- **21 Arcanes**, calquées sur les arcanes majeurs du tarot : la Carte de tarot de la piste (#115) est « l'Arcane sans numéro » (le Mat), puis Arcanes n° 1 (le Bateleur) à n° 21 (le Monde). Chaque Arcane suit sa carte.
- **Ton** (utilisateur·rice) : mystique, une histoire à part du village, avec du suspense et une fin surprenante, qu'on n'attend pas.
- **Écriture** : Claude rédige, l'utilisateur·rice relit tout avant le code, fin comprise (décision : pas de partie cachée à l'utilisateur·rice). Relecture par lots : 1–7, 8–14, 15–21 + fins.
- **Choix** : chaque Arcane pose un choix [1] / [2], touches `1` / `2` comme les Lettres.
  - La réponse change **une phrase de l'Arcane suivante** (deux variantes).
  - L'ensemble des réponses choisit entre **2 fins** (n° 21) ; la règle (ex. majorité) se fixe avec le texte.
  - Une Arcane compte comme lue une fois son choix fait : l'Arcane suivante n'arrive qu'après (rythme de #114).
  - Réponses gardées dans `Progress` (`#[serde(default)]`), à part des réponses du village.
- Contenu statique dans `src/story/content/` (ex. `arcanes.rs`), comme les autres textes ; l'Arcane d'exemple de `--arcane` (#114) reste séparée.
- Français, `tu`, typographie du projet (`\u{202F}` avant `: ! ?`).
- Hors périmètre : piste et déblocage (#115), cinématique (#117), Badge.

## Acceptance criteria

- `weathr --arcane` (secret débloqué), soirs successifs → Arcanes n° 1 à 21 dans l'ordre, chacune avec sa carte de tarot et son choix [1] / [2].
- Arcane sans réponse → l'Arcane suivante n'arrive pas ; après la réponse → elle arrive au passage suivant.
- Réponse [1] puis [2] à une même Arcane (deux parties) → une phrase de l'Arcane suivante diffère.
- Deux jeux de réponses opposés → deux fins différentes à l'Arcane n° 21.
- Toutes les Arcanes et les deux fins relues et validées par l'utilisateur·rice avant la fusion.
- Ancienne sauvegarde → se charge, aucune réponse d'Arcane.
- Tests unitaires : 21 Arcanes présentes, variante choisie selon la réponse précédente, fin selon la règle des réponses.
- `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings -A clippy::double_must_use`, `cargo test --bin weathr` verts.

## Implementation

## Review

## Verification

## Feedback
