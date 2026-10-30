---
issue: 35
status: todo
wiki: [[animation]], [[mail]]
note: fin mai : table dehors, les Voisins réunis au jardin
created: 2026-10-30
---

## Context / Decisions

- Fête des voisins = dernier vendredi de mai, calculé chaque année (29 mai 2026, 28 mai 2027). Le cas rare du premier vendredi de juin n'est pas géré.
- Toute la journée, jour et nuit, par tous les temps.
- Nouveau flag `Holiday` dans `src/holiday.rs`, calculé par `Holiday::on`.
- Une table dressée devant la maison, sur l'herbe, entre la maison et la rivière. Les animaux du jardin ne changent pas.
- Voisins à table : Rose, Paulo, Odile, Léo, Gaston, un sprite chacun de part et d'autre de la table.
- Un Voisin vient seulement si sa 1re Lettre est déjà livrée (`timeline.first_mail`, expéditeur de la Lettre).
- Jeanne (l'encre bleue) ne vient jamais : son identité est un secret.
- Léo et Gaston réutilisent leurs sprites existants (`kid.rs`, `postman.rs`) si possible.
- Gaston fait sa tournée à l'heure du courrier comme d'habitude, et il est à table le reste du temps.
- Badge « Fête des voisins », `BadgeKind::Holiday`, gagné en lançant le jeu ce jour-là.
- Aperçu : une scène « Fête des voisins » dans `--demo` avec tous les Voisins, et un flag `--neighbours`.
- Nouveau système dans `src/animation/`, branché dans `AnimationManager`.

## Acceptance criteria

- Date simulée au dernier vendredi de mai → une table dressée devant la maison, de jour comme de nuit.
- Jeudi et samedi autour de cette date → aucune table.
- Sous la pluie → la table et les Voisins sont toujours là.
- Joueur qui n'a reçu que la Lettre de Rose → seule Rose est à table (plus Gaston, s'il a déjà écrit).
- Joueur qui a reçu les Lettres des 5 Voisins → Rose, Paulo, Odile, Léo et Gaston sont à table.
- Jeanne n'est jamais à table.
- À l'heure du courrier, Gaston fait sa tournée ; le reste du temps il est assis à table.
- Lancer le jeu ce jour-là → Badge « Fête des voisins » gagné, visible dans l'Album.
- `weathr --neighbours` → table et Voisins, quelle que soit la date.
- `weathr --demo` → une scène « Fête des voisins » passe, avec son label dans la Barre du haut.
- `Holiday::on` : vrai le 29 mai 2026 et le 28 mai 2027, faux les jours voisins (test unitaire).

## Implementation

## Review

## Verification

## Feedback
