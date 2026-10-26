---
issue: 8
status: todo
wiki: [[mail]], [[animation]]
note: "Découpé de #3. Grill 23/10 : fil principal = fête du miel chez Nadia et Claire ; surprise de la fête = retour de Sam, qui ouvre une saison 3 ; le fil du n°13 continue (Lettres sous la porte tous les 4 jours). 60 Lettres jours 61–120, 12 choix [1]/[2]. Claude écrit, pas de résumé : Salomé découvre en jouant."
created: 2026-10-26
---

## Context / Decisions

> 60 Lettres, fête du miel, retour de Sam

- Sprint **Saison 2**, split from #3. Blocked by #12: new content lands in `src/story/` as sources + `MailContent`, not in today's `letters.rs`.
- **No spoilers**: Claude writes every text. Task, PR, reports and commits never summarise the story. Salomé discovers it by playing.
- **Village Lettres** 61 to 120, one per Jour de jeu, after the current 60.
  - Main thread: fête du miel at Nadia and Claire's. Surprise of the fête: Sam comes back, opening a season 3 (out of scope).
  - The n°13 thread goes on: creepy Lettres under the door every 4 days, through day 120.
- **Correspondents** go on over days 61–120: Voisins, Marcel, Habitants and every other current correspondent.
  - Sam keeps writing, never mentions coming back.
  - No Carte postale from Sam on days 107 to 120 (last two weeks).
- **12 choices** `[1]`/`[2]` in season 2 Lettres, same mechanism as season 1 (`Choice`, answers in `progress.json`).
  - Each one has an **echo**: a later Lettre recalls the answer.
  - Every choice has a visible target. `## Implementation` lists each choice id + its target: season 1 thing (#20), ruche or Potager (#9), hens or goat (#10). Ids and targets only, no story.
- Things that appear in the garden (ruches, potager, poules, chèvre, fête scene) are #9, #10, #11. #8 is text + choices + echoes only.
- `lettres.txt` override: same rule as today (`=== N` replaces Lettre N). A custom Lettre 61+ now replaces a season 2 Lettre.
- Tone and wording: `{conventions}/wording.md`, village voice, `tu`.
- Out of scope: season 3, garden variants, new Habitants/props, fête scene.

## Acceptance criteria

- [ ] `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` all green.
- [ ] Jour de jeu 61 to 120: exactly one new village Lettre per day, numbered 61 to 120 (test).
- [ ] n°13 Lettres keep their every-4-days rhythm through day 120 (test).
- [ ] Voisins, Marcel and the other correspondents have Lettres on days 61–120 (test).
- [ ] No Carte postale on days 107 to 120 (test).
- [ ] 12 season 2 Lettres ask a `[1]`/`[2]` question; each answer is echoed in a later season 2 Lettre (test, like `echo_letter <= len`).
- [ ] An old save at Lettre 60 opens Lettre 61 on the next Jour de jeu; season 1 answers and echoes unchanged.
- [ ] No story summary in the task file, PR body or commit messages.

## Implementation

## Review

## Verification

## Feedback
