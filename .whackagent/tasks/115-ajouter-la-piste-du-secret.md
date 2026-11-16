---
issue: 115
status: to-test
wiki: [[mail]], [[animation]]
note: "Découpé de #113 (parapluie « Ajouter un secret »), sprint « Le secret ». Débloque l'Arcano-Facteur de #114."
created: 2026-11-16
---

## Context / Decisions

> Spoiler : ce fichier décrit le secret. Ticket, PR et commits restent vagues (« piste du secret »).

- Blocked by #114 : la piste se termine en débloquant l'Arcano-Facteur (champ de déblocage de `Progress` posé par #114).
- **Ouverture** : à partir du Jour de jeu 42.
- **Indices**, autour d'un **personnage mystérieux** :
  - **Gazette** : à partir du Jour 42, chaque Gazette porte une **deuxième rumeur**, en plus de la rumeur habituelle. Elle reste après le déblocage :
    - avant : rumeurs floues sur le personnage mystérieux ;
    - après : rumeurs sur l'Arcano-Facteur et les Arcanes.
  - **Lettres** : à partir du Jour 42 et jusqu'au déblocage, une Lettre du village sur 3 porte une allusion au personnage (ligne en plus, type post-scriptum). Après le déblocage : plus d'allusion.
- **Étapes** (dans l'ordre) :
  1. **Minuit** : jeu ouvert entre 0 h et 0 h 59 (heure locale), un jour à partir du Jour 42 → une **Carte de tarot** violette apparaît dans l'herbe du jardin, une seule fois, et y reste jusqu'à ce qu'on la ramasse.
  2. **Ramasser** : `r` près de la carte (même mécanique que les Objets) → elle s'ouvre : « l'Arcane sans numéro », avec une **énigme**.
  3. **Énigme** : réponse tapée avec `e` (même saisie que l'Énigme de la Gazette). La bonne réponse débloque le secret ; une mauvaise ne change rien. La saisie reste partagée : la réponse est comparée à l'Énigme de la semaine et à celle de la carte.
- Déblocage → date enregistrée (champ de #114) : l'Arcano-Facteur commence à passer dès le soir même. La première apparition en cinématique est une tâche à part (#117).
- **Textes** (rumeurs floues et après déblocage, allusions, texte de la carte, énigme et sa réponse) écrits ensemble : l'utilisateur·rice donne les grandes lignes, Claude rédige, l'utilisateur·rice relit avant le code. Listes courtes, qui tournent en boucle.
- **Textes validés par l'utilisateur·rice (16/11)** — à reprendre tels quels :
  - Rumeurs floues (Gazette, Jour ≥ 42, avant déblocage, en boucle) :
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

- Avant le Jour 42 → Gazette avec une seule rumeur, aucune allusion, minuit sans effet.
- `weathr --day 43` → la Gazette porte une deuxième rumeur, floue, sur un personnage mystérieux.
- À partir du Jour 42 et avant le déblocage → une Lettre du village sur 3 porte une allusion au personnage.
- Jour ≥ 42, jeu ouvert à 0 h 30 → une Carte de tarot violette apparaît au jardin et reste tant qu'elle n'est pas ramassée ; à 1 h 30 un autre jour sans carte → rien.
- `r` près de la carte → elle s'ouvre avec son énigme.
- `e` + bonne réponse → secret débloqué, l'Arcano-Facteur passe le soir même ; mauvaise réponse → rien ne change.
- Après le déblocage → plus d'allusion dans les Lettres ; la deuxième rumeur parle de l'Arcano-Facteur et des Arcanes.
- Ancienne sauvegarde → se charge, piste non commencée.
- Tests unitaires : seuil du Jour 42, une Lettre sur 3, fenêtre de minuit, carte unique, déblocage par la bonne réponse.
- `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings -A clippy::double_must_use`, `cargo test --bin weathr` verts.

## Implementation

- stacked on: wa/114-ajouter-le-passage-du-soir @ cccf6d3
- `src/story/content/secret_trail.rs` : textes validés mot pour mot (`\u{202F}`) ; `second_rumor(reading_day, week, is_unlocked)`, `mystery_allusion(letter_day, is_unlocked)`, listes en boucle.
- `ReadContext` : `reading_day`, `is_secret_unlocked`. Gazette : deuxième rumeur sous « Les échos » (Jour de jeu de lecture, pour que `--day 46` la montre ; une ancienne Gazette relue après le Jour 42 la porte aussi). Allusion : Jour de la Lettre (Lettres du village à partir du Jour 42, une sur trois), avant le P.-S. de nuit si les deux s'appliquent. Verrouillé ou non : lu au moment de la lecture.
- `src/story/tarot_card.rs` : `TarotCard { Hidden, InGrass, Picked }`, `appear(now, day, is_unlocked)` (0 h–0 h 59, Jour ≥ 42, une fois, jamais une fois débloqué), `pick_up`, `is_riddle_open`, `answer` (`chronicles::is_right_answer`) ; `Progress.tarot_card_appeared` / `tarot_card_picked` (`#[serde(default)]`).
- `src/animation/tarot_card.rs` : `[*]` violet sur la ligne d'herbe, au milieu de la maison, jour et nuit jusqu'au ramassage.
- `app.rs` : `Overlay::TarotCard` (`tarot_card_view`, cadre Nuit étoilée) ; `r` ramasse d'abord la carte ; `e` : Gazette ouverte avec Énigme non résolue → comme avant, sinon carte ramassée non résolue → ouvre la carte avec la saisie (seul accès, pas d'entrée au Courrier) ; réponse comparée à la carte puis à l'Énigme de la semaine ; toasts existants.
- `ArcanePost::unlock(today)` : date de déblocage sauvegardée, l'Arcano-Facteur passe le soir même (règles de #114). `--day N` : en mémoire, jamais écrit.
- `src/story/testdata/save_37_golden.txt` régénéré : +32 lignes (deuxième rumeur, allusions), rien d'autre.
- Build : `cargo fmt -- --check`, clippy (`--all-targets -D warnings -A clippy::double_must_use`), `cargo test --bin weathr` (374) verts.

- Round 1 (feedback) : `TRAIL_START_DAY` 45 → 42 (rumeur, allusions, Carte de tarot) ; allusions aux Jours 42, 45, 48… (la 7e reprend la 1re) ; tests renommés jour 42 avec le bord jour 41 ; golden régénéré (+12 / −8 : allusions décalées d'un cran, rumeurs inchangées). 376 tests verts.

## Review

## Verification

- Round 1 : manual validation — not run by agent (run: yours).
- Contrôle de mise en page (implémenteur, pty 80×24 et 130×40, faketime, HOME jetable, pas une preuve) : 01 h 30 pas de carte ; 00 h 30 carte affichée et sauvegardée, encore là après relance ; `r` → « L'Arcane sans numéro » ; « bateleur » → rien ; « Le Fou » → débloqué ; 21 h → Arcano-Facteur et Boîte violette ; `--day 46` un lundi → deuxième rumeur et allusion, rien d'écrit.

- Round 1 : manual validation — not run by agent (run: yours).

## Feedback

### Round 1 — 2026-11-16
- Asked : « update : l'indice arrive au jour 42 pas 45 » ; précision demandée (toute la piste ou les indices seuls) → « pas de préférence » : un seul seuil pour toute la piste (recommandé).
- Triage : adjustment → critères passés au Jour 42.
- Changes : seuil 42, tests, golden (implementer).
- Review : deferred (→ /wa-validate).
- Verify : manual validation — not run by agent.
- Rule captured : aucune.
