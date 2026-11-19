---
issue: 94
status: to-test
wiki: [[animation]], [[render]]
note: "ajouter un event quand un jours de fete est afficher, nouvel encadré, v 1.8"
created: 2026-11-18
---

## Context / Decisions

> ajouter un event quand un jours de fete est afficher, nouvel encadré, v 1.8

- **Quand** : à chaque lancement un jour de Fête, après les fenêtres de lancement existantes (Sauvegarde, Accueil, Mot des voisins, Nouveautés). `[Echap]` ferme.
- **Fêtes** (`Holiday`, `src/holiday.rs`) : Noël (25/12), Nouvel An, Épiphanie, Poisson d'avril, Pâques, Fête des voisins, Fête de la musique, 14 juillet, Halloween.
  - Aussi les pluies d'étoiles filantes : Perséides, Géminides.
  - Chandeleur (#95) et 1er mai (#96) apportent chacune leur texte, dans la tâche qui atterrit en second.
- **Contenu** : nom de la Fête en titre, 2-3 lignes dans la voix du village, dont ce qu'il faut guetter (ex. Fête de la musique : « Ce soir, le village chante. Ouvre l'œil : des notes s'échappent des fenêtres. »).
- Textes écrits au code sur le ton des Lettres, validés par l'utilisateur·rice au test.
- Les Fêtes forcées (`--christmas`, `--fireworks`, `--neighbours`, `--music`) montrent aussi l'encadré.
- Nouvel `Overlay` à côté des fenêtres de lancement, vue dans `src/views.rs`, cadre existant.

## Acceptance criteria

- `--music` (ou un 21 juin) → au lancement, encadré « Fête de la musique » avec ses lignes, après les autres fenêtres de lancement ; `[Echap]` → la scène.
- Relancer le même jour → encadré à nouveau.
- Jour sans Fête → pas d'encadré.
- Chaque Fête de la liste et chaque pluie d'étoiles a son titre et ses lignes (test sur toutes).
- `cargo fmt --check`, `cargo test` verts.

## Implementation

Round 1 — /wa-autopilot 2026-11-19.

- `src/holiday.rs`: `HolidayBox { title, lines, is_due }`, `HOLIDAY_BOXES` (11), `Holiday::holiday_box()`; 4 tests.
- `src/views.rs`: `holiday_box_view` (`Frame::Plain`, footer `[Echap] fermer`).
- `src/app.rs`: `Overlay::HolidayBox`, `pending_holiday_box`, shown after the launch windows and before pending Badge cards.
- The box follows the `Holiday` flags, so it shows on every day they mark: Halloween all October, Pâques ±7 days, Nouvel An 25/12–8/01, Noël 24–25/12, 14 juillet 13–14/07, Perséides and Géminides 7 days each. To validate.
- Overlaps: one box per launch, the one-day Fête wins (Noël before Nouvel An, Épiphanie before Nouvel An, Poisson d'avril before Pâques).
- `--demo` → no box; `--day N` → box if the date is a Fête.
- Texts to validate:
  1. Noël — « Le sapin s'est habillé de lumières. » / « Regarde à son pied : quelqu'un y a déposé des cadeaux. »
  2. Épiphanie — « Au village, on tire les rois. » / « Lève les yeux vers le toit : ta maison a eu la fève. »
  3. Poisson d'avril — « Au village, les farceurs sont de sortie. » / « Guette ta boîte aux lettres et le dos du Facteur. »
  4. Fête des voisins — « Aujourd'hui, le village dresse la table près de la rivière. » / « Même le toit s'est paré de fanions. »
  5. Fête de la musique — « Ce soir, le village chante. » / « Ouvre l'œil : des notes s'échappent des fenêtres. »
  6. 14 juillet — « Ce soir, tout le village a le nez en l'air. » / « Dès la nuit, des feux d'artifice éclatent dans le ciel. »
  7. Halloween — « Ce mois-ci, le village aime se faire peur. » / « Des citrouilles gardent ta porte, un fantôme rôde la nuit. »
  8. Perséides — « Cette semaine, il pleut des étoiles filantes. » / « Par nuit claire, lève les yeux : de quoi faire mille vœux. »
  9. Géminides — « En décembre aussi, il pleut des étoiles filantes. » / « Par nuit claire, couvre-toi bien et lève les yeux. »
  10. Nouvel An — « Le village fête la nouvelle année. » / « Chaque nuit, le ciel s'allume de feux d'artifice. »
  11. Pâques — « Les cloches sont passées par le village. » / « Cherche dans l'herbe : des œufs colorés s'y cachent. »
- Build: `cargo build`, `cargo test` (495) green · fmt OK · clippy: existing `double_must_use` only.

## Review

## Verification

Round 1 — runtime check by the agent (pty 100×32, fresh `HOME`):

- ✅ First launch `--music`: Accueil first, `[Echap]` → Encadré « Fête de la musique », `[Echap]` → scene with notes.
- ✅ Relaunch, same HOME → Encadré again (no Accueil).
- ✅ `--christmas`, `--fireworks`, `--neighbours` → their Encadré, then the scene.
- ✅ Plain launch (19/11) → no Encadré.

## Feedback
