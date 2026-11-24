---
issue: 174
status: to-close
wiki: [[mail]]
note: découpée de #173 au grill du 2026-11-24
created: 2026-11-24
---

## Context / Decisions

- Les Lettres des Voisins, rangées dans `chronicles.rs`, ont le même défaut que celles du village : ton écrit, personnages qui se ressemblent.
- Proto : [[protos/changer-le-ton-des-lettres-personas]] · https://claude.ai/artifact/SSh2dpsVMXNS5cLTzneY7A. Les fiches de Sam, Nadia et Claire, et Marcel sont validées en v3.
- On réécrit les Lettres de Sam (5), de Nadia et Claire (5) et de Marcel (5), en suivant leur fiche de la page wiki `[[letters]]` (#173) :
  - Sam : rêveur, ancien cartographe, tic « tu savais ? » ;
  - Nadia et Claire : un duo qui se taquine, Claire fonce, Nadia doute, tic « Claire dit… Nadia dit… » ;
  - Marcel : style rapport de ronde, nerveux, tic « Rien à signaler. ».
- Les 15 Lettres du Voisin du n°13 et les post-scriptum de nuit sont seulement relus : ils sonnent déjà juste et servent de modèle. On ne corrige qu'une phrase qui casse une règle de `[[letters]]`.
- Les Cartes postales et les Bouteilles ne bougent pas.
- Les indices restent : Marcel nie le n°13 et compte douze maisons, les abeilles évitent le n°13, et Sam annonce les Cartes postales chaque semaine.
- Bloquée par #173 : la page `[[letters]]` doit exister.
- Sprint Voix des Lettres. Version v1.9.

## Acceptance criteria

- Les 5 Lettres de Sam, les 5 de Nadia et Claire et les 5 de Marcel sont réécrites et suivent leur fiche de `[[letters]]`.
- Aucune ne finit par le tic de son personnage. Aucun tic n'apparaît deux fois dans la même Lettre.
- Marcel affirme toujours que le village a douze maisons. Nadia et Claire disent toujours que leurs abeilles évitent le n°13.
- La dernière Lettre de Sam annonce toujours une Carte postale par semaine.
- Les Lettres du Voisin du n°13 et les post-scriptum de nuit sont inchangés, sauf une phrase qui casserait une règle (citée dans `## Implementation`).
- `cargo test` passe.

## Implementation

- autopilot 2026-11-24, wa-implementer, round 1.
- `chronicles.rs`: SAM_LETTERS, NEIGHBOURS_LETTERS, MARCEL_LETTERS bodies rewritten per [[letters]] sheets; senders unchanged. Tics once per Lettre, never last words (Sam 1 + 3, Nadia et Claire 1 + 4, Marcel 1 + 3).
- Story facts kept: grand-father's map with crosses (now « Quarante petites croix » = 40 CITIES), empty suitcase, Sam 5 announces one Carte postale a week + watch the mill; sign language, bees avoid n°13, ten years married by Odile, swarm; twelve houses, mud by the well, three knocks.
- Added concrete details (6 h 10 bus, Odile's sash, Marcel's hedge 40 cm, 23 h 15 patrol…), no new story beat.
- CREEPY_LETTERS + NIGHT_POSTSCRIPTS reviewed: no rule broken, unchanged. They keep plain spaces before ? : (no `\u{202F}`): typography-only change left out of scope.
- Tests: `app.rs` footprint test quote updated (Marcel 2 still mentions footsteps to the house). `save_37_golden.txt`: 15 text lines swapped.
- Proof: `cargo clippy --all-targets` clean · `cargo build` ok · `cargo test` 166 + 569 + 10 + 7 + 3 + 5 passed.

## Review

### validation — 2026-11-24 · clean

Diff from stack point (wa/173). LENSES: style ✓ · elegance ✓ · structure ✓ · correctness ✓. No findings: sheets followed, tics once and never last, clues kept, n°13 + postscripts untouched, CITIES has Lisbonne + Valparaíso and 40 entries, test quote still a substring check.

## Verification

- manual validation: not run by agent (TUI).

## Feedback
