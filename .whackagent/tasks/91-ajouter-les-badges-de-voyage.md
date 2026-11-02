---
issue: 91
status: todo
wiki: [[architecture]], [[weather]]
note: "ajouter un badge voyageur pour une personne qui a visiter 7 villes / badge baroudeur 15 villes visiter"
created: 2026-11-02
---

## Context / Decisions

- Deux Badges de la famille **Fidélité**, à côté de « 7 jours d'affilée » : **Voyageur** à 7 Villes visitées, **Baroudeur** à 15.
- **Ville visitée** = ville renvoyée par la position auto au lancement (`detect_location`, `src/main.rs`, seulement si `location.auto`). Il faut vraiment se déplacer.
- Ne compte pas : ville choisie dans les Réglages (#82), coordonnées écrites à la main, détection sans nom de ville.
- Une ville = son nom, sans tenir compte de la casse. Stocké dans `Progress` (`visited_cities: BTreeSet<String>`, `#[serde(default)]`).
- Le compte démarre avec cette version : la ville du premier lancement compte pour 1. Aucun historique avant.
- Attribution comme les autres Badges à compteur (`streak_7`, `riddle_10`) dans `collect_badges` (`src/app.rs`). Pas d'attribution en `--day` (règle existante).
- Icônes 3 × 5 dans le style des autres (valise pour Voyageur, sac à dos pour Baroudeur), au choix de l'implémentation.
- Limite connue : la position auto passe par l'IP, un VPN ou un réseau mobile peut donner une autre ville.
- Troisième Badge, famille **Secrets**, à Paliers : **Sur les traces de Sam**.
  - **Ville de Sam** = position auto au lancement à 10 km ou moins d'une des 40 villes des Cartes postales (`CITIES`, `src/story/content/chronicles.rs`).
  - Les 40 villes comptent, que leur Carte postale soit déjà arrivée ou non.
  - Comparaison par coordonnées, pas par nom : ipinfo renvoie « Lisbon », `CITIES` dit « Lisbonne ».
  - Aucun calcul de distance dans le code aujourd'hui : à écrire (distance à vol d'oiseau).
  - Mêmes exclusions que la Ville visitée : Réglages, coordonnées à la main, `--day`.
  - Stocké dans `Progress` (`sam_cities: BTreeSet<usize>`, index dans `CITIES`, `#[serde(default)]`). Une ville de Sam compte une fois. Le compte démarre avec cette version.
- **Paliers** du badge de Sam : chocolat à 1 Ville de Sam, bronze à 5, argent à 15, or à 30, platine à 40 (toutes les villes de `CITIES`).
  - Un seul Badge qui évolue : une ligne dans l'Album, compté une fois dans le total.
  - Le palier se déduit du nombre de Villes de Sam, rien d'autre à stocker.
  - Même icône à tous les Paliers, seul le nom change.
  - Le système de Badges change : nom et indice sont fixes aujourd'hui (`Badge` en `const`, `badge_card_view` dans `src/views.rs`, `badge_cell` dans `src/overlay.rs`). Il faut un nom et une ligne qui dépendent du Palier.
- Album, avant le chocolat : `???` comme les autres Secrets. Ensuite visible, avec le Palier et la progression :
  - `[x] Sur les traces de Sam · argent (12 villes, prochain palier : or à 30)` ;
  - à platine : `[x] Sur les traces de Sam · platine (40 villes)`.
- Cartes :
  - chocolat → « Nouveau badge ! », « Sur les traces de Sam · chocolat », « visité la même ville que Sam » ;
  - paliers suivants → « Nouveau palier ! », « Sur les traces de Sam · bronze », « 5 villes de Sam visitées ».
  - Espace insécable fine avant « ! » (`\u{202F}`), comme « Nouveau badge ».
  - Plusieurs Paliers franchis d'un coup (impossible en pratique : une ville par lancement) → une carte par Palier.
- Lignes de glossaire ajoutées avec cette tâche :
  `| Ville visitée | | | \`visited_cities\` | ville détectée par la position auto ; compte pour Voyageur et Baroudeur |`
  `| Palier | | niveau | \`tier\` | rang du badge de Sam : chocolat, bronze, argent, or, platine |`
  `| Ville de Sam | | | \`sam_cities\` | ville d'une Carte postale où la position auto est passée (10 km) |`
- Hors périmètre : compteur « n / 7 » pour Voyageur et Baroudeur, carte des villes, Badges au-delà de 15, lien avec les Cartes postales reçues, icône par Palier.

## Acceptance criteria

- Position auto activée, 7 villes différentes détectées au fil des lancements → carte de nouveau Badge « Voyageur ».
- 15 villes différentes → carte de nouveau Badge « Baroudeur ».
- Même ville relancée plusieurs fois, ou casse différente (« Lyon » / « lyon ») → comptée une seule fois.
- Ville choisie dans les Réglages ou position auto désactivée → la ville n'est pas comptée.
- Position auto à 10 km ou moins d'une ville de Sam (ex. Lisbonne), même au premier lancement → carte « Nouveau badge ! · Sur les traces de Sam · chocolat · visité la même ville que Sam ».
- 5e Ville de Sam → carte « Nouveau palier ! · Sur les traces de Sam · bronze · 5 villes de Sam visitées » ; idem à 15 (argent), 30 (or), 40 (platine).
- Même Ville de Sam relancée → comptée une seule fois, pas de nouvelle carte.
- Position auto à plus de 10 km de toutes les villes de Sam → rien ne bouge.
- Lisbonne choisie dans les Réglages ou écrite à la main → rien ne bouge.
- Album, Secrets : `???` avant le chocolat ; à 12 Villes de Sam → `Sur les traces de Sam · argent (12 villes, prochain palier : or à 30)` ; à 40 → `· platine (40 villes)`.
- Album → Voyageur et Baroudeur dans Fidélité ; le total de la Barre du haut passe de 40 à 43, quel que soit le Palier.
- Sauvegarde d'une version précédente (sans `visited_cities` ni `sam_cities`) → se charge sans erreur, comptes à 0.
- `weathr --day N` → aucune ville comptée, aucun Badge attribué.
- `cargo fmt --check` et `cargo test` verts ; clippy rouge uniquement sur `double_must_use` (`src/weather/provider.rs:23`).

## Implementation

## Review

## Verification

## Feedback
