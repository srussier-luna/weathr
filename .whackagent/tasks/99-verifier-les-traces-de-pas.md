---
issue: 99
status: to-close
wiki: [[animation]], [[mail]]
note: "ok, ajoute un draft poue verifier toutes les traces de pas de la saison 1, avjoute le label 1.8"
created: 2026-11-18
---

## Context / Decisions

> ok, ajoute un draft poue verifier toutes les traces de pas de la saison 1, avjoute le label 1.8

- Aujourd'hui (#97) : Traces de boue au Jour de jeu 8 seulement (`FOOTPRINT_DAYS`), Traces mouillées les nuits 13, 14 et 30 (`WET_FOOTPRINT_DAYS`, `src/app.rs`).
- Courriers de la saison 1 qui parlent de traces (à compléter par un relevé exhaustif) :
  - Lettres du village (`src/story/content/letters.rs`) : « Cette nuit, quelqu'un a marché dans ton jardin. Regarde le sol », « Ces traces sont petites », Léo « Pardon pour les traces de boue » ;
  - voisin n°13 : « Les traces vont du puits à ta porte », « de la boue sur ton paillasson » ;
  - ronde (`src/story/content/chronicles.rs`) : « de la boue près du puits… des pas qui vont vers ta maison ».
- **Règle** (décision utilisateur·rice) : les traces dont parle un courrier sont visibles le Jour de jeu où il arrive.
  - Traces de boue : toute la journée.
  - Traces mouillées : restent nocturnes, la nuit de ce jour.
- **Sortie** : un test liste chaque courrier qui parle de traces avec son Jour de jeu et vérifie les bonnes traces ce jour-là. Les jours faux sont corrigés dans `FOOTPRINT_DAYS` / `WET_FOOTPRINT_DAYS`.
- Pas de nouvelle trace, pas de texte modifié.

## Acceptance criteria

- Relevé de tous les courriers de la saison 1 qui parlent de traces, de boue ou de pas, avec leur Jour de jeu, dans `## Implementation`.
- Pour chacun, `--day N` (et `--day N --night` pour les mouillées) → les traces décrites sont visibles.
- Aucun jour de traces sans courrier qui l'explique.
- Test qui relie chaque courrier à son jour de traces ; `cargo fmt --check`, `cargo test` verts.

## Implementation

Round 1 — /wa-autopilot 2026-11-19. **blocked** — unambiguous part done and proven.

- Answered 2026-11-19 — neither counts (doormat P.-S., Biscuit's sound): nothing more to change. Was: do these count as mail about footprints?
  - Night P.-S. « il y a de la boue sur ton paillasson » (`NIGHT_POSTSCRIPTS[4]`, Lettres 5, 15, 25, 35, 45, 55 read 22 h–6 h). Recommended: no (doormat, not garden). Yes → wet footprints nights 15, 25, 35, 45, 55 (day 5 impossible: wet footprints arrive day 13).
  - Lettre n°9, Biscuit « J'ai entendu des petits pas, très légers » (day 9). Recommended: no (a sound). Yes → mud on day 9.
- Audit (season 1):

| Courrier | Citation | Jour | Traces attendues | Avant | Après |
|---|---|---|---|---|---|
| Lettre n°8 (`Main(7)`) | « quelqu'un a marché dans ton jardin. Regarde le sol » | 8 | boue | boue | boue |
| Lettre n°9 (`Main(8)`) | « J'ai entendu des petits pas, très légers » | 9 | question ouverte | aucune | aucune |
| Lettre n°11 (`Main(10)`) | « Ces traces sont petites » | 11 | boue | aucune | boue |
| Lettre n°16, Léo (`Main(15)`) | « Pardon pour les traces de boue » | 16 | boue | aucune | boue |
| Voisin du n°13, 3 (`Creepy(2)`) | « Les traces vont du puits à ta porte » | 13 | mouillées, nuit | nuit | nuit |
| Marcel, 3 (`Marcel(2)`) | « de la boue près du puits… des pas vers ta maison » | 30 | mouillées, nuit | nuit | nuit |
| P.-S. de nuit (`NIGHT_POSTSCRIPTS[4]`) | « de la boue sur ton paillasson » | 5, 15… 55 | question ouverte | aucune | aucune |
| aucun courrier | — | 14 | aucune | mouillées | aucune |

- Out of scope / false positives: Archive n°10 « parquet mouillé » (mairie, day 82), Énigme n°30 (escargot), « je trace des routes », « ça mouille », flaques.
- `src/app.rs`: `FOOTPRINT_DAYS = [8, 11, 16]`, `WET_FOOTPRINT_DAYS = [13, 30]`; tables `FOOTPRINT_MAIL` / `WET_FOOTPRINT_MAIL` (Entry, citation); 3 tests: quote in the mail text, footprints exactly on the Timeline delivery days (days 1–120).
- After the answer: glossary rows (Traces de boue, Traces mouillées) and wiki `animation.md` to update at close.
- Build: `cargo build`, `cargo test` green · fmt OK.

## Review

Round validation — 2026-11-19, one wa-verifier, LENSES style ✓ elegance ✓ structure ✓ correctness ✓.
- 2 minor findings (duplicated tests, bare unwrap) fixed by autofix round 1 (tests only), re-verified clean. Game code unchanged since the user's test.

## Verification

Round 1 — runtime check by the agent (pty 120×40, fresh `HOME`, `--simulate clear --day N [--night]`):

- ✅ Mud on days 8, 11, 16 (day and night); none on 7, 9, 10, 12–15, 17, 30.
- ✅ Wet footprints nights 13 and 30; none on night 14 (old wrong day), 12, 15, 29, 31, nor by day.

## Feedback
