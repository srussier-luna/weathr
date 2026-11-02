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
- Troisième Badge, famille **Secrets** : **Sur les traces de Sam**, indice « visité la même ville que Sam ».
  - Gagné quand la position auto au lancement tombe à 10 km ou moins d'une des 40 villes des Cartes postales (`CITIES`, `src/story/content/chronicles.rs`).
  - Les 40 villes comptent, que leur Carte postale soit déjà arrivée ou non.
  - Comparaison par coordonnées, pas par nom : ipinfo renvoie « Lisbon », `CITIES` dit « Lisbonne ».
  - Aucun calcul de distance dans le code aujourd'hui : à écrire (distance à vol d'oiseau).
  - Mêmes exclusions que la Ville visitée : Réglages, coordonnées à la main, `--day`.
- Ligne de glossaire « Ville visitée » ajoutée avec cette tâche :
  `| Ville visitée | | | \`visited_cities\` | ville détectée par la position auto ; compte pour Voyageur et Baroudeur |`
- Hors périmètre : compteur « n / 7 » affiché, carte des villes, Badges au-delà de 15, lien avec les Cartes postales reçues.

## Acceptance criteria

- Position auto activée, 7 villes différentes détectées au fil des lancements → carte de nouveau Badge « Voyageur ».
- 15 villes différentes → carte de nouveau Badge « Baroudeur ».
- Même ville relancée plusieurs fois, ou casse différente (« Lyon » / « lyon ») → comptée une seule fois.
- Ville choisie dans les Réglages ou position auto désactivée → la ville n'est pas comptée.
- Position auto à 10 km ou moins d'une ville de Sam (ex. Lisbonne), même au premier lancement → carte « Sur les traces de Sam » avec l'indice « visité la même ville que Sam ».
- Position auto à plus de 10 km de toutes les villes de Sam → pas de Badge.
- Lisbonne choisie dans les Réglages ou écrite à la main → pas de Badge.
- Album → Voyageur et Baroudeur dans Fidélité, Sur les traces de Sam dans Secrets ; le total de la Barre du haut passe de 40 à 43.
- Sauvegarde d'une version précédente (sans `visited_cities`) → se charge sans erreur, compte à 0.
- `weathr --day N` → aucune ville comptée, aucun Badge attribué.
- `cargo fmt --check` et `cargo test` verts ; clippy rouge uniquement sur `double_must_use` (`src/weather/provider.rs:23`).

## Implementation

## Review

## Verification

## Feedback
