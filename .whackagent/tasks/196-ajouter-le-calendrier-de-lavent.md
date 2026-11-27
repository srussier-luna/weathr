---
issue: 196
status: to-test
wiki: [[animation]], [[mail]], [[render]]
note: "il nous faut un calendrier de l'avent — split en #196, #197, #198, sprint Avent, livré en 1.8.3"
created: 2026-11-27
---

## Context / Decisions

- Sprint **Avent**, livré dans une version à part **1.8.3**, avant le 1er décembre. Branche `sprint/avent` depuis `develop`.
- Découpe : #196 (ici) Chocolutin + 24 chocolats + calendrier · #197 mots de Chocolutin si absent · #198 Père Noël du 25 + bébé renne.
- **Chocolutin** : le lutin chocolatier du Père Noël.
  - Du 1er au 24 décembre (date réelle, jamais en `--day`/démo), au premier lancement du jour, il traverse la scène jusqu'à la maison, comme le Facteur (`src/animation/postman.rs`).
  - Arrivé, une carte montre le chocolat du jour : son nom et son dessin.
  - Une fois par jour : relancer le même jour ne le refait pas venir.
- **Chocolats** : 24, tous différents, chocolats de luxe. Un nom gourmand et un dessin ASCII propre à chacun. Le chocolat du jour J est le n° J.
- **Calendrier de l'Avent** : vue à 24 cases ouverte par `[v]`.
  - `[v] calendrier` dans la Barre du bas du 1er au 31 décembre seulement, absent le reste de l'année.
  - Case reçue : le chocolat (nom + dessin). Case à venir : fermée avec son numéro. Case passée non reçue : vide (#197 la remplira via le mot).
- Sauvegarde : jours reçus pour l'année en cours (année + ensemble de jours). Une nouvelle année repart à zéro. Champs `#[serde(default)]`.
- Hors périmètre : mots si absent (#197), Père Noël et renne (#198).
- Lignes de glossaire au close : Chocolutin, Calendrier de l'Avent, Chocolat.

## Acceptance criteria

- 1er passage : Chocolutin entre par la gauche comme le Facteur et s'arrête devant la porte ; une lettre de présentation s'affiche dans le cadre de Chocolutin, puis la carte du chocolat n° 1 (phrase du jour, dessin, nom).
- Passages suivants : pas de présentation. Chaque jour, la carte montre ce que dit Chocolutin (2–3 phrases : ses maladresses, l'atelier de chocolat qu'il regrette) puis le chocolat avec sa phrase de papillote (blague ou anecdote sur le Père Noël), différente de ce qu'il dit.
- Chocolutin a le dessin bonnet plein + pompon (5 lignes) et sautille à chaque pas en marchant ; il est au sol devant la porte.
- Relancer le même jour : Chocolutin ne revient pas, pas de carte.
- 30 novembre ou 26 décembre : Chocolutin ne vient pas.
- En décembre, la Barre du bas montre `[v] calendrier` ; `[v]` ouvre la grille de 24 cases.
- Dans la grille : les cases reçues montrent leur chocolat, les cases à venir sont fermées avec leur numéro.
- En novembre ou en janvier : pas de `[v]` dans la Barre du bas, la touche ne fait rien.
- Les 24 chocolats ont chacun un nom et un dessin différents.
- Les anciennes sauvegardes chargent sans erreur ; l'année suivante, le calendrier repart vide.

## Implementation

- Nouveau dossier `src/advent/` : `mod.rs` (`is_advent`, `chocolate(day)`, `due_day`, états de case Received / Missed / Closed), `chocolates.rs` (24 chocolats nommés, dessin 3×8 chacun), `view.rs` (grille 6×4, curseur aux flèches, carte « Chocolat n°J »).
- `src/animation/chocolutin.rs` : sprite et marche ; arrive par la droite, s'arrête juste à droite de la maison (le Facteur garde la boîte à gauche).
- Sauvegarde : `advent_year: Option<i32>`, `advent_days: BTreeSet<u32>` (`#[serde(default)]`) ; `receive_advent_day(year, day)` remet à zéro à l'année suivante (réutilisable par #197).
- Venue : date réelle seulement (pas en démo, `--holiday`, `--condition`, `--night`, `--day`), jour pas encore reçu. Attend que les fenêtres de lancement soient fermées (Sauvegarde, Bienvenue, Nouveautés, Noël, Badges). Jour enregistré à l'ouverture de la carte ; quitter pendant la marche → il revient au prochain lancement du jour.
- `[v]` bascule le calendrier quand `holiday.is_december` ; ignorée sinon. Barre du bas `[v] Calendrier` (majuscule comme les autres touches) et aide en décembre seulement.
- Modifiés : `src/animation/mod.rs`, `src/animation_manager.rs`, `src/app.rs`, `src/hud/toolbar.rs`, `src/main.rs`, `src/progress/mod.rs`, `src/views.rs`.
- Build : clippy OK, build OK, fmt OK ; tests 182 + 659 + 10 + 7 + 3 + 5, 0 échec.

## Review

## Verification

round 4: manual validation — not run by agent.

round 3: manual validation — not run by agent.

round 2: manual validation — not run by agent (date non simulable sur cette branche ; #203 le permet).

round 1: build + tests only — date réelle non simulable (27/11), Chocolutin non observé à l'écran. À tester : run: yours.

## Feedback

### Round 2 — 2026-11-27

- Asked : « chocolutin devrait arriver par la droite » (précisé : entrer par la gauche comme le Facteur, s'arrêter devant la porte) · « le premier jour il devrait se présenter » (précisé : lettre avant le chocolat, une petite phrase drôle ou anecdote sur le Père Noël chaque jour, cadre propre au lutin, ton particulier) · « propose d'autres designs » (choisi : lutin à bonnet long).
- Triage : 3 ajustements (critères mis à jour).
- Changes (implementer) : entrée par la gauche, arrêt visage centré sur la porte, sortie à gauche plateau vide ; sprite 5 lignes bonnet long ; `src/advent/chocolutin.rs` (présentation, signature, 24 phrases) ; `Frame::Gift` ; champ `chocolutin_introduced` ; carte du chocolat avec phrase du jour. Fichiers : src/advent/chocolutin.rs (nouveau), src/animation/chocolutin.rs, src/animation_manager.rs, src/advent/mod.rs, src/advent/view.rs, src/overlay.rs, src/progress/mod.rs, src/app.rs.
- Inline : `src/advent/chocolutin.rs` « Fringuant » → « Fringant » (nom du renne).
- Wiki : fiche de voix « Chocolutin » ajoutée dans letters.md.
- Review : deferred (→ /wa-validate). Verify : manual, run: yours.
- Note : ° est de largeur ambiguë (2 colonnes en terminal CJK).
- Build : clippy OK, fmt OK, tests 183 + 667 + 10 + 7 + 3 + 5, 0 échec.

### Round 3 — 2026-11-27

- Asked : « le bonnet est transparent, mets le pompon au-dessus du bonnet » · « il a la tête un peu trop sur la gauche, recentre la tête sur le corps ».
- Triage : 2 ajustements visuels, inline (1 fichier, constantes du sprite).
- Changes (inline) : `src/animation/chocolutin.rs` — bonnet plein `/███\` rouge, pompon `*` blanc au-dessus ; tout le sprite centré sur la colonne 5 (visage, bras, plateau `[###]`, pieds) ; `SPRITE_WIDTH` 9, `FACE_CENTER` 5 (arrêt devant la porte conservé).
- Review : deferred. Verify : manual, run: yours.
- Build : clippy OK, fmt OK, tests 183 + 667 + 10 + 7 + 3 + 5, 0 échec.

### Round 4 — 2026-11-27

- Asked : « dans le lore, vu que c'est pas le plus malin des lutins, c'est pour ça que le Père Noël l'envoie mettre le chocolat » · « remets-le comme avant » (dessin collé par le joueur) · « il faut qu'il sautille à chaque pas » · « Chocolutin se pose des questions, se demande pourquoi il ne travaille plus à l'atelier de chocolat, raconte ses maladresses un peu tous les jours » · « il faut que Chocolutin nous parle tous les jours en nous donnant notre chocolat » · « les phrases du jour dans le chocolat et celle dite par Chocolutin ne doivent pas être les mêmes ».
- Triage : 4 ajustements (critères mis à jour).
- Changes (implementer) : sprite du joueur (bonnet plein, pompon, `[##]`, `SPRITE_WIDTH` 8, `FACE_CENTER` 4) ; saut d'une ligne un pas sur deux ; présentation réécrite (sous-entend la raison, ne la dit pas) ; `SPEECHES`/`speech` (24 discours) + champ `wrapper` sur chaque chocolat (24 phrases de papillote) ; carte : discours, « — Chocolutin », dessin, n°, nom, papillote ; `advent::view::chocolate_lines(day)` pour #197. Fichiers : src/animation/chocolutin.rs, src/advent/chocolutin.rs, src/advent/chocolates.rs, src/advent/view.rs.
- Wiki : fiche Chocolutin — ligne Lore + Voix mise à jour.
- Review : deferred. Verify : manual, run: yours.
- Build : clippy OK, fmt OK, tests 184 + 669 + 10 + 7 + 3 + 5, 0 échec.

