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
- **Textes validés par l'utilisateur·rice (16/11)** — à reprendre tels quels :
  - Rumeurs floues (Gazette, Jour ≥ 45, avant déblocage, en boucle) :
    1. On raconte qu'une lueur violette a traversé le ciel au-dessus du lavoir, bien après minuit.
    2. Le boulanger jure avoir vu un vieil homme assis… dans les airs. Il dit qu'il ne boit que de l'eau.
    3. Quelqu'un aurait laissé une carte sur le banc du square. Personne n'a osé la retourner.
    4. Les chats du village fixent tous le même coin de ciel, les nuits sans lune.
    5. Une voix très vieille, très douce, fredonne parfois vers minuit, du côté de chez toi.
    6. Certaines nuits, il manquerait une étoile. Elle ne serait jamais loin.
  - Rumeurs après déblocage (en boucle) :
    1. Certains soirs, un balai passe au-dessus des toits. Il ne vole jamais au hasard.
    2. Une petite boîte violette est apparue chez quelqu'un. Une seule personne sait l'ouvrir.
    3. Les anciens parlent d'un facteur qui ne livre que le soir. Ils baissent la voix en le disant.
    4. Il y aurait 22 cartes en tout. Qui lit la dernière connaît la fin, dit-on.
  - Allusions (fin d'une Lettre du village sur 3, en boucle) :
    1. P.-S. : Tu n'as rien vu passer cette nuit ? J'ai cru… non, rien.
    2. P.-S. : Ma grand-mère disait qu'à minuit, le jardin garde ce qu'on y perd.
    3. P.-S. : Drôle de lumière violette hier soir. Sûrement l'orage.
    4. P.-S. : Les cartes trouvées la nuit ne se ramassent pas par hasard, paraît-il.
    5. P.-S. : Si tu entends un froissement de paille au-dessus du toit, ne te retourne pas trop vite.
    6. P.-S. : Quelqu'un rôde après minuit. Pas méchant, je crois. Juste… très ancien.
  - Carte de tarot, titre « L'Arcane sans numéro » :
    - Texte : Une carte usée, violette, tiède comme une main. Un voyageur y marche vers le bord d'une falaise, un baluchon sur l'épaule, sans regarder ses pieds. Au dos, d'une écriture tremblée :
    - Énigme : « Je n'ai pas de numéro, pourtant je commence tout. Je marche sans savoir où, et c'est pour ça qu'on me suit. Qui suis-je ? »
    - Réponses acceptées (majuscules et accents ignorés) : « le mat », « mat », « le fou », « fou ».
  - Typographie du projet à appliquer au code (`\u{202F}` avant `: ! ?` et dans « »).
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
