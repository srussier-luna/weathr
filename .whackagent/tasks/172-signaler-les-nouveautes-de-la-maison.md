---
issue: 172
status: todo
wiki: [[render]], [[mail]]
note: découpée de #171 au grill du 2026-11-24
created: 2026-11-24
---

## Context / Decisions

- Aujourd'hui, seuls Lettre !, Objet ! et Badge ! (#163) clignotent dans la Barre du bas. Ce qu'on trouve et qui va dans l'Album n'est signalé nulle part. Le but : signaler toute nouveauté de la Maison (#171) jusqu'à ce qu'on l'ouvre.
- Proto : [[protos/ajouter-la-maison-plan]] · https://claude.ai/artifact/Hz22JAbSAdJFwSMdgZgrvz.
- Ce qui compte comme nouveau : tout ce qu'on trouve, c'est-à-dire Objet, Bouteille, Carte postale, Recette, Cadeau, Dessin de Léo, Souvenir du coffre, et les Badges. Les réponses aux Questions ne comptent pas.
- Les éléments pas encore vus sont gardés dans la sauvegarde (`Progress`), comme `unseen_badges` de #163. Ils restent nouveaux après un redémarrage, et une ancienne sauvegarde n'en a aucun.
- Les Badges réutilisent `unseen_badges`.
- Le clignotement va du haut vers le bas, rose ↔ gris toutes les 500 ms :
  1. `[m] Maison !` dans la Barre du bas ;
  2. la Salle sur le plan (`Bureau !`) ;
  3. l'entrée de la Salle (`Bouteilles !`) ;
  4. l'élément dans son écran.
- Ouvrir une entrée marque ses éléments comme vus. Ils clignotent pendant cette visite, puis plus du tout.
- Une Salle s'éteint quand toutes ses entrées ont été vues, et la Barre du bas quand toutes les Salles l'ont été.
- Une Salle à une seule entrée (Trophées, Cuisine) ouvre directement son écran : y entrer marque ses éléments comme vus.
- Bloquée par #171 : la Maison doit exister.
- Sprint Maison. Version v1.9.

## Acceptance criteria

- On ramasse un Objet : la Barre du bas montre `[m] Maison !` en rose, qui clignote toutes les 500 ms.
- On ouvre la Maison : le Grenier clignote, les autres Salles non.
- On entre au Grenier : l'entrée `Objets !` clignote, et Souvenirs du coffre non.
- On ouvre Objets : le nouvel Objet clignote dans la page.
- On revient au plan : le Grenier ne clignote plus. On ferme : `[m] Maison` est gris.
- Une Bouteille et une Recette nouvelles : on ouvre Bouteilles au Bureau. La Cuisine clignote encore, et `[m] Maison !` aussi.
- On quitte et on relance weathr avec une nouveauté pas encore vue : `[m] Maison !` clignote toujours.
- Un Badge gagné : la Salle des trophées clignote. Y entrer ouvre la vue Badges, avec le Badge qui clignote comme dans #163.
- Une ancienne sauvegarde se charge sans erreur et sans rien qui clignote.

## Implementation

## Review

## Verification

## Feedback
