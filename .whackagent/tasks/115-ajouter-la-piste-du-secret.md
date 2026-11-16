---
issue: 115
status: todo
wiki: [[mail]], [[animation]]
note: "Découpé de #113 (parapluie « Ajouter un secret »), sprint « Le secret ». Débloque l'Arcano-Facteur de #114."
created: 2026-11-16
---

## Context / Decisions

> Spoiler : ce fichier décrit le secret. Ticket, PR et commits restent vagues (« piste du secret »).

- Blocked by #114 : la piste se termine en débloquant l'Arcano-Facteur (champ de déblocage de `Progress` posé par #114).
- **Ouverture** : à partir du Jour de jeu 45.
- **Indices**, autour d'un **personnage mystérieux** :
  - **Gazette** : à partir du Jour 45, chaque Gazette porte une **deuxième rumeur**, en plus de la rumeur habituelle. Elle reste après le déblocage :
    - avant : rumeurs floues sur le personnage mystérieux ;
    - après : rumeurs sur l'Arcano-Facteur et les Arcanes.
  - **Lettres** : à partir du Jour 45 et jusqu'au déblocage, une Lettre du village sur 3 porte une allusion au personnage (ligne en plus, type post-scriptum). Après le déblocage : plus d'allusion.
- **Étapes** (dans l'ordre) :
  1. **Minuit** : jeu ouvert entre 0 h et 0 h 59 (heure locale), un jour à partir du Jour 45 → une **Carte de tarot** violette apparaît dans l'herbe du jardin, une seule fois, et y reste jusqu'à ce qu'on la ramasse.
  2. **Ramasser** : `r` près de la carte (même mécanique que les Objets) → elle s'ouvre : « l'Arcane sans numéro », avec une **énigme**.
  3. **Énigme** : réponse tapée avec `e` (même saisie que l'Énigme de la Gazette). La bonne réponse débloque le secret ; une mauvaise ne change rien. La saisie reste partagée : la réponse est comparée à l'Énigme de la semaine et à celle de la carte.
- Déblocage → date enregistrée (champ de #114) : l'Arcano-Facteur commence à passer dès le soir même. La première apparition en cinématique est une tâche à part (#117).
- **Textes** (rumeurs floues et après déblocage, allusions, texte de la carte, énigme et sa réponse) écrits ensemble : l'utilisateur·rice donne les grandes lignes, Claude rédige, l'utilisateur·rice relit avant le code. Listes courtes, qui tournent en boucle.
- `--day N` : montre rumeurs et allusions du jour N, ne débloque rien dans la sauvegarde. Étape minuit testée par tests unitaires (pas d'option d'heure).
- Pas de Badge, rien dans l'Aide : le secret reste secret.
- Lignes de glossaire ajoutées avec cette tâche : Carte de tarot, Personnage mystérieux.
- Hors périmètre : cinématique d'apparition (#117), Arcanes (#114, #116).

## Acceptance criteria

- Avant le Jour 45 → Gazette avec une seule rumeur, aucune allusion, minuit sans effet.
- `weathr --day 46` → la Gazette porte une deuxième rumeur, floue, sur un personnage mystérieux.
- À partir du Jour 45 et avant le déblocage → une Lettre du village sur 3 porte une allusion au personnage.
- Jour ≥ 45, jeu ouvert à 0 h 30 → une Carte de tarot violette apparaît au jardin et reste tant qu'elle n'est pas ramassée ; à 1 h 30 un autre jour sans carte → rien.
- `r` près de la carte → elle s'ouvre avec son énigme.
- `e` + bonne réponse → secret débloqué, l'Arcano-Facteur passe le soir même ; mauvaise réponse → rien ne change.
- Après le déblocage → plus d'allusion dans les Lettres ; la deuxième rumeur parle de l'Arcano-Facteur et des Arcanes.
- Ancienne sauvegarde → se charge, piste non commencée.
- Tests unitaires : seuil du Jour 45, une Lettre sur 3, fenêtre de minuit, carte unique, déblocage par la bonne réponse.
- `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings -A clippy::double_must_use`, `cargo test --bin weathr` verts.

## Implementation

## Review

## Verification

## Feedback
