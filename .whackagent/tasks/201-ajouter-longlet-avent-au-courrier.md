---
issue: 201
status: todo
wiki: [[mail]]
note: "dans L ajoute un onglet calendrier de l'avent — suite de l'écart de #197 (mots lus via [c], pas relisibles dans [l])"
created: 2026-11-27
---

## Context / Decisions

- Bloquée par #197 (mots de Chocolutin, lus via `[c]`, aujourd'hui pas relisibles).
- Nouvel onglet **Avent** dans le Courrier `[l]`, après les onglets existants.
- Il liste les **mots de Chocolutin déjà lus**, datés (« Mot de Chocolutin du 2 décembre »), le plus récent en haut, relisibles comme les Lettres (flèches, `[<]` `[>]` dans l'onglet).
- Relire un mot montre le même contenu (mot + carte du chocolat) sans redonner le chocolat.
- La grille du calendrier reste sur `[v]` ; l'onglet ne la montre pas.
- Visible dès qu'un mot a été lu, comme les autres onglets ; reste ensuite toute l'année.
- Nouveau champ de sauvegarde : jours dont le mot a été lu (par année), `#[serde(default)]`. Les chocolats reçus en personne (Chocolutin au jardin) ne sont pas des mots : pas dans l'onglet.
- La liste `[l]` suit la timeline par jour d'histoire : intégrer l'onglet sans casser `letters_read`, les compteurs « Lettre n/total », `entry_beside` ni l'Album.
- Sprint Avent, livré en 1.8.3.
- Ligne de glossaire au close : Onglet Avent.

## Acceptance criteria

- Avant tout mot lu : pas d'onglet Avent dans `[l]`.
- Lire le mot du 2 décembre via `[c]` → `[l]` montre l'onglet Avent avec « Mot de Chocolutin du 2 décembre ».
- Ouvrir ce mot depuis l'onglet → même mot + carte du chocolat n° 2 ; le calendrier ne change pas.
- Deux mots lus → ordre du plus récent au plus ancien ; les flèches passent de l'un à l'autre sans sortir de l'onglet.
- Un chocolat reçu en personne n'apparaît pas dans l'onglet.
- Les compteurs et la navigation des autres onglets sont inchangés.
- En janvier, l'onglet et ses mots sont toujours là.

## Implementation

## Review

## Verification

## Feedback
