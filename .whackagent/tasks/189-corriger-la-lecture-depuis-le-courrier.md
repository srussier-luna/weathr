---
issue: 189
status: todo
wiki: [[mail]]
note: "hot fix 1.8.2 : quand on lit les Lettres depuis L, garder les flèches jaunes pour passer à la suivante et la précédente ; depuis un courrier lu depuis L, Échap retourne sur L au lieu de tout quitter"
created: 2026-11-26
---

## Context / Decisions

> hot fix 1.8.2 quand on lit les lettres depuis L on garde les flèches jaunes pour passer à la suivante et précédente, et depuis un courrier lu depuis L échap retourne sur L au lieu de tout quitter

Correctif de #105, livré en 1.8.2 (depuis `develop`, puis release 1.8.2).

- **Flèches depuis [l]** : une entrée ouverte depuis le Courrier (`Overlay::Letter { from: Some((tab, selected)) }`) se feuillette avec `←`/`→` dans **son onglet**, dans l'ordre de la liste (`tabs::entries_in`), quel que soit le jour de lecture. Revient sur la décision #105 « Lettre lue un jour passé ouverte depuis `l` → `←`/`→` sans effet ».
  - Bout de liste : pas de mouvement, pas de chevron de ce côté.
  - La sélection suit l'entrée affichée : `Échap` revient au Courrier, même onglet, sur la dernière entrée lue.
- **Depuis le jardin (`c`)** : inchangé (Lettres du jour, #105).
- **Touche [c] depuis [l]** : c'est elle qui fermait tout (pas `Échap`). Lecture ouverte depuis [l] → `[c]` ne fait rien, et disparaît des pieds de page (Lettre, Gazette avec énigme…).
- **Nom des Gazettes** : aujourd'hui toutes « La Gazette du lundi » (`content/mod.rs` titre d'`Entry::Gazette`). Devient « Gazette du lundi 23 novembre » : date du jour de parution, même source que la date déjà montrée dans la liste et dans le texte (`clock.date_of(day)`), mois en toutes lettres (`MONTHS`).
- Hors périmètre : ordre des onglets, contenu des Gazettes.

## Acceptance criteria

- `l`, onglet Lettres, ouvrir la 3e entrée → `→` ouvre la 4e, `←` la 2e, y compris pour des Lettres lues des jours passés.
- Même chose dans les onglets Cartes postales, Archives, Recettes, Gazettes : on reste dans l'onglet.
- Première entrée de l'onglet : `←` sans effet et pas de chevron gauche ; dernière : pareil à droite.
- Après `→` deux fois puis `Échap` → Courrier, même onglet, sélection sur l'entrée affichée en dernier.
- Lecture ouverte depuis `l` : pied de page sans `[c]` ; appuyer sur `c` ne ferme rien.
- Lecture ouverte par `c` depuis le jardin : `c`, `←`/`→` et pied de page inchangés.
- Onglet Gazettes : chaque ligne s'appelle « Gazette du lundi <jour> <mois> » (ex. « Gazette du lundi 23 novembre »), date cohérente avec la date jj/mm de la ligne ; le titre de la Gazette ouverte aussi.

## Implementation

## Review

## Verification

## Feedback
