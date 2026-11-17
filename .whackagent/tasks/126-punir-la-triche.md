---
issue: 126
status: to-test
wiki: [[mail]], [[animation]], [[scene]], [[render]]
note: "Découpé de #124 (parapluie « Ajouter la détection de triche »), sprint « Anti-triche ». Lit le compteur de triches de #125."
created: 2026-11-17
---

## Context / Decisions

> fair une detection de triche, modification du fichier de sauvegarde et si le jouer est un tricheur le remarquer et proposer une punition,, soit plus de courrier pensant plusieur jours, soit un anti-badge pour qu'il soit puni ou alors une modification sur son jeu: les habitant disparaisse, les arbres perde leur feuille, si plusieurs triche plusieurs punition

- Bloqué par #125 : lit son compteur de triches.
- **Escalade**, chaque nouvelle triche ajoute la suivante :
  - 1re triche → anti-Badge **Tricheur**, gardé pour toujours, dans sa propre famille de Badges ;
  - 2e → **Village dégarni** pendant 7 Jours de jeu : Habitants absents du jardin, Arbre sans feuilles, puis tout revient ;
  - 3e et suivantes → **Jours sans courrier** : 3 jours à la 3e triche, + 3 par triche (6, 9…).
- **Jours sans courrier** :
  - Jour de jeu figé comme pour la Boîte pleine : rien n'est perdu, l'histoire reprend ensuite ;
  - aucun courrier livré, pas de tournée du Facteur ;
  - le village est dégarni pendant ces jours.
- **Mot des voisins** : fenêtre à l'ouverture du jeu, signée par les voisins, dans un cadre à elle.
  - Annonce la triche et sa punition.
  - Pendant les Jours sans courrier, revient à chaque ouverture avec une phrase de voisins déçus (« tu casses le fun », ils trouvent le joueur nul…), tirée d'une liste.
- **Textes** (annonce par niveau, phrases déçues, condition du Badge Tricheur, nom de sa famille) : proposés au `/wa-code`, validés par l'utilisateur·rice avant d'être figés.
- `--day N` → aucune punition affichée.
- Arcanes (sprint « Le secret », v2) pas encore dans `develop` : à leur arrivée, les Jours sans courrier arrêtent aussi l'Arcano-Facteur.
- Hors périmètre : repérer la triche (#125), racheter une punition.

## Acceptance criteria

- Compteur de triches à 1 au lancement → Mot des voisins (annonce) ; le Badge Tricheur apparaît dans les Badges, dans sa famille, et y reste après relance.
- Compteur à 2 → Mot des voisins ; pendant 7 Jours de jeu, aucun Habitant au jardin et l'Arbre sans feuilles ; le 8e, tout revient.
- Compteur à 3 → Mot des voisins ; 3 jours sans courrier : Jour de jeu figé, pas de Facteur, Courrier inchangé, village dégarni ; chaque ouverture pendant ces jours → Mot des voisins avec une phrase déçue ; ensuite l'histoire reprend où elle en était.
- Compteur à 4 → 6 jours sans courrier ; à 5 → 9 jours.
- `--day 43` → ni Mot des voisins ni punition.
- Tests : niveau de punition selon le compteur, durées, reprise de l'histoire.
- `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings -A clippy::double_must_use`, `cargo test` verts.

## Implementation

- stacked on: wa/125-reperer-la-triche @ 09a41e4
- `src/story/punishment.rs` (nouveau) : `Punishment`, `sentence`, `is_mail_banned`, `is_village_bare` + tests. Ne lit que le compteur de #125.
- `src/story/content/neighbours_note.rs` (nouveau) : `NeighboursNote`, tous les textes du Mot des voisins.
- `src/progress/mod.rs` : `punished_cheats`, `bare_village_until` (Jour de jeu du retour), `mail_ban_until` (date de reprise, privé + accesseurs), `#[serde(default)]`.
- `src/story/clock.rs` : `pause_for` ; Jours sans courrier = pause de la Boîte pleine (`paused_days`) : N tournées sautées, reprise le jour N + 1. Boîte pleine pas jugée pendant ces jours.
- `src/badges.rs` : `BadgeKind::Demerit` (« Mauvais points »), Badge Tricheur (`cheater`), ajouté sans carte « Nouveau badge ! » (le Mot des voisins l'annonce), hors des totaux (x/43 inchangé), famille cachée tant qu'elle est vide.
- `src/overlay.rs` : `Frame::Roofs` pour le Mot des voisins. `src/views.rs` : `neighbours_note_view`.
- `src/animation/lore.rs` : `Unlocks::without_habitants` (les 7 animaux, chatons, canetons, cousin grand-duc partent ; Léo, Voisins, Cabane, objets restent). `src/scene/tree.rs`, `decorations.rs`, `mod.rs` : Arbre nu (branches `\ | /`, pas de cadeaux à Noël).
- `src/app.rs` : ordre au lancement Sauvegarde → Accueil → Mot des voisins → Nouveautés ; Mot des voisins à chaque lancement sauf `--day` (aussi en `--silent` / simulé) ; `set_story_frozen(… || is_mail_banned())` ; Habitants cachés.
- Nouvelle triche = compteur > triches déjà punies ; seul le dernier niveau s'applique, plus le Badge (0 → 3 : Tricheur + 3 jours). Nouvelle interdiction pendant une autre : N jours à partir d'aujourd'hui, sans cumul. Les jours sans courrier ne comptent pas dans les 7 Jours de jeu du village dégarni (Jour de jeu figé).
- Textes à valider : niveaux 1, 2, 3+, 8 phrases déçues, Badge « Tricheur » (« pris la main dans le sac », bonnet d'âne), famille « Mauvais points », signature « — Tes voisins ».
- Glossaire à ajouter à la clôture : Mauvais points (`BadgeKind::Demerit`), `Frame::Roofs`.
- Build : `cargo build`, `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings -A clippy::double_must_use`, `cargo test` (544, 0 échec) verts.

## Review

## Verification

Autopilot, HOME jetable, pseudo-terminal, vraies triches (Lettres lues relevées à la main puis relance) :
- ✅ 1re triche : Mot des voisins (médaille Tricheur, cadre toits, « — Tes voisins ») ; Badge dans « Mauvais points » ; gardé après relance, plus de Mot.
- ✅ 2e : Mot « 7 jours », Arbre nu, aucun Habitant ; retour au 8e jour par tests unitaires (non observé en vrai).
- ✅ 3e : Mot « 3 jours », Jour de jeu figé, Courrier inchangé, village dégarni ; chaque lancement → phrase déçue différente. Reprise de l'histoire : tests unitaires.
- ✅ 4e → 6 jours ; 5e → 9 jours.
- ✅ `--day 43` : ni Mot ni punition, sauvegarde intacte.
- ✅ Ordre : Mot des voisins puis Nouveautés.
- ✅ Pas de tournée du Facteur pendant les jours sans courrier (histoire figée, test existant).

## Feedback
