---
issue: 173
status: to-close
wiki: [[mail]]
note: idée du 2026-11-24 ; Lettres de Sam, Nadia et Claire, Marcel découpées dans #174
created: 2026-11-24
---

## Context / Decisions

> j'aimerai bien améliorer le wording des lettres. En fait elles ont un ton qui font que ça fait pas hyper naturel, dur à expliquer mais un humain parlerait pas comme ça. Aussi, on a plusieurs personnages mais on ressent pas leur personnalité différentes à travers les lettres. Ce serait cool de faire un /wa-proto avec les persona des gens et de faire des règles sur le wording à mettre dans le wiki pour les prochaines écritures de lettres. Ce serait top aussi si des personnages avaient des expressions. Il faut varier en gros. Que ça fasse humain

- Les Lettres sonnent comme un texte écrit par une machine : formules de conte (« C'est la tradition du village »), modes d'emploi, morales finales, et partout le même rythme. Les personnages se ressemblent tous.
- Proto : [[protos/changer-le-ton-des-lettres-personas]] · https://claude.ai/artifact/SSh2dpsVMXNS5cLTzneY7A. La v3 est validée : 18 fiches, chacune avec une Lettre avant et après, et 10 règles communes. Les textes « après » du proto servent de point de départ.
- Nouvelle page wiki `[[letters]]`, qui devient la référence pour toute Lettre future :
  - une fiche par personnage : sa voix, son tic de langage, ce qu'il ne dit jamais, et une Lettre d'exemple ;
  - les règles communes du proto ;
  - `[[mail]]` et `index.md` pointent vers elle, et `wording.md` la cite dans « Project voice ».
- Les 18 personnages : Rose, Gaston, Paulo, Odile, Léo, L'encre bleue (avant la révélation), Jeanne (après), Minouche, Biscuit, Hulotte, Coin-Coin, Rainette, Noisette, Gustave, le Voisin du n°13, Sam, Nadia et Claire, Marcel.
- Un tic de langage se glisse dans la phrase, jamais en signature à la fin. Il apparaît au plus une fois par Lettre, et pas dans toutes. Celui de Noisette est « tchic ».
- Paulo écrit avec des apostrophes oubliées (`Javais`, `cest beau`), parce que Jeanne dit plus tard qu'il écrit « avec des fautes d'orthographe, comme avant ». Pas d'autre faute, pour qu'on ne croie pas à un bug.
- Cette tâche réécrit les 60 Lettres du village (`DEFAULT_LETTERS`, `letters.rs`). La réécriture est libre, sauf :
  - les Énigmes de L'encre bleue : texte et réponse inchangés ;
  - ce qui déclenche quelque chose au jardin : la Lettre qui annonce un Cadeau l'annonce toujours, les Questions `[1]`/`[2]` et leurs Échos restent, et Jeanne se révèle à la même Lettre ;
  - le nombre et l'ordre des Lettres.
- Les Lettres de Sam, de Nadia et Claire, et de Marcel, ainsi que la relecture du Voisin du n°13, sont dans #174.
- Les joueurs actuels ont un fichier de Lettres écrit avec les anciens textes, qui écrase ceux du jeu. On garde donc les anciens textes dans le code. Une Lettre du fichier identique à son ancien texte prend le nouveau, et une Lettre réécrite par le joueur reste la sienne. Le fichier est ensuite réécrit avec les nouveaux textes pour les Lettres que le joueur n'a pas touchées.
- Sprint Voix des Lettres. Version v1.9.

## Acceptance criteria

- `.whackagent/wiki/letters.md` existe, avec 18 fiches (voix, tic, ne dit jamais, exemple) et les règles communes. `index.md` et `mail.md` la citent, et `wording.md` la cite dans « Project voice ».
- Les 60 Lettres du village sont réécrites. Chacune suit la fiche de son personnage, et aucune ne contient « C'est la tradition du village », ni une morale ou un mode d'emploi en fin de Lettre.
- Le texte de chaque Énigme de L'encre bleue est identique à avant.
- Chaque Lettre qui annonçait un Cadeau (banc, nœud, puits, os, clé, cabane, guirlande…) le nomme toujours.
- Les Questions `[1]`/`[2]` et leurs Échos ont les mêmes ids et les mêmes numéros de Lettre. `cargo test` passe.
- Jeanne se révèle toujours à la même Lettre.
- Aucune Lettre ne finit par le tic de son personnage. Aucun tic n'apparaît deux fois dans la même Lettre.
- Les Lettres de Paulo ont des apostrophes oubliées, et aucune autre faute.
- Fichier de Lettres d'avant la tâche, où la Lettre 1 n'a pas été touchée : au lancement, la Lettre 1 affiche le nouveau texte de Rose, et le fichier contient ce nouveau texte.
- Fichier où le joueur a réécrit la Lettre 4 : elle affiche toujours son texte, et le fichier le garde.

## Implementation

- autopilot 2026-11-24, wa-implementer, round 1.
- `.whackagent/wiki/letters.md` (new): 18 persona sheets (voix, tic, ne dit jamais, example) + 10 common rules. Linked from `index.md`, `mail.md`, `wording.md` « Project voice ».
- `letters.rs`: 60 `DEFAULT_LETTERS` bodies rewritten, senders unchanged. Lettre 5 (riddle) byte-identical, Lettre 8 still opens « Bravo, c'était le grand arbre. ». Every gift / Apparition still named. Jeanne reveal still Lettre 38. Echo letters end so appended echoes read naturally. `\u{202F}` before ? ! : ; in new texts.
- Tics checked by script: never twice in a Lettre, never last words. Signatures removed from bodies (game adds « — sender »); Minouche signs « M. » once (Lettre 3).
- Paulo: missing apostrophes only. Proto tic « voila » written « voilà », « Cest beau » capitalised, because criteria forbid any other mistake.
- `previous_letters.rs` (new): `PREVIOUS_LETTERS`, 60 old texts. `apply_overrides` skips a file letter equal to its previous default (`is_outdated`), also in `read()` (`--day`). `load_from` rewrites the file (`write_defaults` → `write_letters`) only when it held such a letter or is missing; player edits and letters 61+ kept; player comments in a rewritten file are lost. Unreadable existing file never overwritten.
- Tests: +6 (riddle + senders unchanged, migration: untouched Lettre 1, edited Lettre 4, appended 61, missing file, up-to-date file untouched). `app.rs` footprint test quote updated (still checks the mud tracks). `save_37_golden.txt` recaptured.
- Proof: `cargo clippy --all-targets` clean (only existing manifest warning) · `cargo build` ok · `cargo test` 166 + 569 + 10 + 7 + 3 + 5 passed.

## Review

### validation — 2026-11-24 · clean

Round 1, 8 findings (all 🟡), autofixed:
- [correctness] file rewrite dropped player comments, ignored and empty blocks → only outdated `=== N` blocks swapped, rest byte-for-byte; + test
- [correctness] `fs::write` over a player file → temp file + rename (`progress::write_creating_dir`)
- [elegance] file parsed twice → parsed once, `Block` carries its line range, `Block::is_outdated`
- [style] Minouche « M. » + game signature = two signatures → « M. » dropped, sheet fixed
- [style] « P.S. » vs `NIGHT_POSTSCRIPTS` « P.-S. » → « P.-S. » everywhere + rule in letters.md
- [style] Lettre 22 Paulo sentences over 10 words → split
- [style] Lettre 28 Odile did not lead into its Question → ends asking the player's opinion
- [style] exclamations out of voice in Lettres 45 and 60 → removed

Round 2: all fixed, nothing new. LENSES: style ✓ · elegance ✓ · structure ✓ · correctness ✓. Build (clippy strict) ✅ · tests ✅ (166 + 570 + 10 + 7 + 3 + 5).

## Verification

- manual validation: not run by agent (TUI). Migration and story invariants covered by unit tests.

## Feedback
