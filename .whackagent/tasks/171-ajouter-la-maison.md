---
issue: 171
status: todo
wiki: [[render]], [[mail]], [[architecture]]
note: idée du 2026-11-24 « rajouter [a] Album dans la toolbar, clignote si nouveau », devenue la Maison au proto ; nouveautés découpées dans #172
created: 2026-11-24
---

## Context / Decisions

> je veux rajouter dans la toolbar le [a] pour album. Idem, lorsque je recois un nouveau truc, je veux que ça clignote

- Ce qu'on collecte est aujourd'hui rangé dans deux écrans : l'Album (une seule suite de pages) et la vue Badges. La Maison les remplace comme point d'entrée. Elle ne contient rien elle-même : chaque Salle mène aux écrans qui existent déjà.
- Proto : [[protos/ajouter-la-maison-plan]] · https://claude.ai/artifact/Hz22JAbSAdJFwSMdgZgrvz. La v4 a validé le plan ASCII. Le contenu des salles du proto ne sert plus : une Salle liste ses entrées, elle ne montre pas les éléments.
- Plan de la Maison, dessiné en ASCII dans le cadre :
  - un toit avec une cheminée qui fume, et le Grenier sous le toit ;
  - deux étages de deux Salles, avec une porte au rez-de-chaussée et de l'herbe au pied ;
  - sur le plan, seulement le nom des Salles ;
  - pas de fenêtres : un `[+]` ressemblait à un bouton.
- Les Salles et leurs entrées :
  - Grenier : Objets, Souvenirs du coffre.
  - Salle des trophées : Badges.
  - Salle des souvenirs : Cadeaux, Dessins de Léo.
  - Bureau : Cartes postales, Bouteilles, Tes réponses.
  - Cuisine : Recettes.
- La sélection d'une Salle ou d'une entrée ne décale jamais le texte : pas de `> ` devant, seulement la couleur et le fond.
- Navigation dans la Maison :
  - les flèches changent de Salle (haut et bas entre les étages, gauche et droite dans un étage) ;
  - Entrée ouvre la Salle ;
  - Échap revient d'un cran, `m` ferme.
- Une Salle à plusieurs entrées affiche la liste de ses entrées, avec le compteur du sommaire actuel de l'Album (`Objets 3/25`, `Recettes 4`).
- Une Salle à une seule entrée (Trophées, Cuisine) ouvre directement son écran.
- Une entrée ouvre l'écran existant :
  - Badges → la vue Badges telle quelle ;
  - les autres → l'Album limité aux pages de ce rayon (`[<]` `[>]` ne sortent pas du rayon) ;
  - Échap ramène à la Salle, ou au plan pour une Salle à une seule entrée.
- Une entrée apparaît au premier élément trouvé, pour ne rien dévoiler (par exemple le Coffre). Une Salle sans entrée reste dessinée et affiche « Rien pour l'instant. ».
- Tes réponses passe au Bureau et garde son fonctionnement : `↑`/`↓` pour choisir, `[1]`/`[2]` pour changer.
- Le sommaire de l'Album disparaît : les Salles et leurs compteurs le remplacent.
- Touches :
  - la Barre du bas affiche `[m] Maison` à la place de `[a]` et `[b]` ;
  - `a` ouvre la Maison sur son plan, comme `m` ;
  - `b` ouvre toujours la vue Badges directement ;
  - `p` est réservé au menu Lieux de la Saison 2 (#21), qui passe de `m` à `p`.
- L'Aide `[h]` présente `m` Maison à la place de l'Album.
- Les nouveautés qui clignotent sont dans #172, bloquée par cette tâche.
- Code : un overlay Maison (`Overlay::House`, avec la Salle sélectionnée) dans `app.rs`, le dessin dans `views.rs`, et l'Album prend un rayon en filtre.
- Sprint Maison. Version v1.9.

## Acceptance criteria

- Au lancement, la Barre du bas montre `[c] Courrier   [m] Maison   [h] Aide   [q] Quitter`, sans `[a]` ni `[b]`.
- On appuie sur `m` : la Maison s'ouvre. Elle montre le toit avec la cheminée, le Grenier sous le toit, quatre Salles sur deux étages et une porte. Une Salle est sélectionnée.
- Les flèches changent la Salle sélectionnée, et le texte des noms ne bouge pas.
- Entrée sur le Bureau : la liste montre Cartes postales, Bouteilles et Tes réponses avec leurs compteurs, seulement pour ce qu'on a déjà trouvé.
- Entrée sur Cartes postales : les pages des Cartes s'ouvrent. `[>]` sur la dernière Carte ne montre pas de Recette.
- Échap depuis une Carte → Bureau. Échap depuis le Bureau → plan. Échap ou `m` depuis le plan → scène.
- Entrée sur la Cuisine ouvre directement les Recettes. Échap revient au plan.
- Entrée sur la Salle des trophées ouvre la vue Badges, la même qu'avec `b`.
- `a` ouvre la Maison sur son plan. `b` ouvre la vue Badges.
- Dans Tes réponses, au Bureau, `↓` puis `[2]` change la réponse, et la Lettre relue suit.
- Sauvegarde sans Coffre ouvert : le Grenier ne montre pas Souvenirs du coffre.
- Sauvegarde neuve : une Salle sans rien de trouvé affiche « Rien pour l'instant. ».
- À 70 colonnes, la Maison et la Barre du bas tiennent sans couper.
- L'Aide montre `[m]` pour la Maison et plus `[a]` pour l'Album.

## Implementation

## Review

## Verification

## Feedback
