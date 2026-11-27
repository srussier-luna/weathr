---
issue: 201
status: to-test
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

stacked on: #203 (wa/203-ajouter-la-date-de-test, contient #196 et #197)

- Sauvegarde : `chocolutin_notes_read: BTreeMap<i32, BTreeSet<u32>>` (`#[serde(default)]`), `mark_chocolutin_note_read(date)`, `read_chocolutin_notes()` (dernière année avec des mots lus, du plus récent au plus ancien). Écrit seulement à la lecture via `[c]`.
- `src/story/content/tabs.rs` : `Tab::Advent` « Avent », dernier onglet, hors timeline ; `visible_tabs`, `opening_tab`, `switch_tab`, `has_neighbours` prennent la liste des mots.
- `src/views.rs` : lignes « > Mot de Chocolutin du 2 décembre » dans l'onglet Avent.
- `src/advent/view.rs` : `reread_note_view(date)` (pied de page de relecture, sans `[c]`).
- `src/app.rs` : `Overlay::RereadChocolutinNote(usize)` ; flèches dans l'onglet ; `[c]` sans effet en relecture ; Echap → onglet Avent sur le même mot. Relire ne change ni le calendrier ni la sauvegarde.
- Timeline intacte : `read_entries`, `letters_read`, `entries_in`, `entry_beside`, `album_pages`, « Lettres reçues (n) ».
- Choix : seule la dernière année avec des mots lus est affichée.
- Build : clippy OK (avec et sans dev-date), build OK, fmt OK ; tests sans 185 + 696, avec 189 + 701 (+ suites annexes), 0 échec.

## Review

## Verification

round 1: manual validation — not run by agent.

## Feedback
