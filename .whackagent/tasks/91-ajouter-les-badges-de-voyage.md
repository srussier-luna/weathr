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
- Ligne de glossaire « Ville visitée » ajoutée avec cette tâche :
  `| Ville visitée | | | \`visited_cities\` | ville détectée par la position auto ; compte pour Voyageur et Baroudeur |`
- Hors périmètre : compteur « n / 7 » affiché, carte des villes, Badges au-delà de 15.

## Acceptance criteria

- Position auto activée, 7 villes différentes détectées au fil des lancements → carte de nouveau Badge « Voyageur ».
- 15 villes différentes → carte de nouveau Badge « Baroudeur ».
- Même ville relancée plusieurs fois, ou casse différente (« Lyon » / « lyon ») → comptée une seule fois.
- Ville choisie dans les Réglages ou position auto désactivée → la ville n'est pas comptée.
- Album, famille Fidélité → Voyageur et Baroudeur listés ; le total de Badges de la Barre du haut passe de 40 à 42.
- Sauvegarde d'une version précédente (sans `visited_cities`) → se charge sans erreur, compte à 0.
- `weathr --day N` → aucune ville comptée, aucun Badge attribué.
- `cargo fmt --check` et `cargo test` verts ; clippy rouge uniquement sur `double_must_use` (`src/weather/provider.rs:23`).

## Implementation

## Review

## Verification

## Feedback
