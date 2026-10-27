---
issue: 53
status: todo
wiki: [[animation]], [[render]], [[mail]], [[updater]]
note: "on va ajouter un text d'on-boarding pour les personne qui ne conaisse pas le jeu pour leur expliquer comment ouvrir son courier etc. propose plusieur texte avec plusieur maniere, comique, curieu etc"
created: 2026-10-27
---

## Context / Decisions

> on va ajouter un text d'on-boarding pour les personne qui ne conaisse pas le jeu pour leur expliquer comment ouvrir son courier etc.

- **Accueil** (glossary, code `welcome`): the Facteur arrives, speaks in a big bubble, hands a note that unfolds, then leaves with a departure bubble. Shown **once per player**.
- **Who**:
  - fresh save (no `progress.json`) → shown;
  - existing save without the new field → shown once at the next normal launch, same texts (user decision: no separate variant for existing players);
  - `--silent`, `--demo`, forced/simulated modes (same rule as `Launch::Suppressed` for release notes) → not shown, not marked; next normal launch shows it.
- **Saved** in `progress.json`, new field `#[serde(default)]` (old saves load, default = not seen). Marked when the Accueil ends or is skipped. `q` mid-way → not marked, replays from the start next launch.
- **Sequence**:
  1. Facteur rides to the mailbox (existing postman look and path), stops.
  2. Big speech bubble above him: arrival text. Footer `[Entrée] suite · [Échap] passer`.
  3. `Entrée` → the note **unfolds** (box grows progressively, under 1 s), then shows the full note text.
  4. `Entrée` → departure bubble. **At that moment the red mailbox flag rises** and the top bar shows `Lettre ! [c]` (if a Lettre is unread).
  5. `Entrée` → bubble closes, Facteur leaves. Then the Nouveautés box opens if release notes are pending (**Accueil first, Nouveautés after**).
- **During the Accueil**: flag down, no `Lettre ! [c]`, `c` `l` `a` `b` `h` `e` `r` do nothing; `q` quits. Badge cards wait until the end (Accueil counts as an open window).
- **`Échap`** at any step → Accueil ends at once: Facteur gone, flag up if unread, marked seen.
- No timeline entry: the note is not a Lettre and never goes in the Courrier (`letters_read` is a prefix, inserting would shift every save). No key to replay it; `[h]` Aide unchanged.
- Note uses the Lettre parchment frame (`Frame::Parchment`). Small terminal → text wraps at box width, `↑`/`↓` scroll like the Nouveautés box.
- Texts (validated by the user, French, `tu`):
  - Arrival bubble: « Salut, nouveau voisin. Te voilà. Le village a beaucoup de choses à te raconter. Tiens, c'est pour toi. »
  - Note title: « Bienvenue au village »
  - Note body:
    ```
    Règle n°1 du village : on lit son courrier. Règle n°2 : on ne demande pas qui habite au n°13.
    Chaque jour, une lettre t'attend. La plupart viennent des voisins. Pour les autres… on ne sait pas trop.

    Drapeau rouge levé sur la boîte : du courrier pour toi.
    [c]  lire le courrier
    [l]  relire les lettres reçues

    Au fil des jours, tu gagnes des badges (météo, fêtes, secrets…) et des souvenirs du village.
    [b]  badges
    [a]  album des souvenirs

    Un trou de mémoire ? [h] montre tout ce que tu peux faire. Le rappel est toujours en haut de l'écran.
    [q]  quitter

    Prends ton temps. Le village n'est pas pressé. Lis bien tout. Certains détails comptent plus qu'ils n'en ont l'air ;)
    ```
  - Signature: « — Le Facteur ». Footer: `[Entrée] suite · [Échap] passer`.
  - Departure bubble: « Je viens de te déposer ton courrier. Moi, je file, à demain ! »
  - `;)` and `!` are the user's choice for the Facteur (wording.md validated examples).
- Wording gotcha: French `: ; ! ?` take U+202F before them; wrapping must not split a word from its punctuation (see Nouveautés gotcha in [[updater]]).
- Where: feature module for the Accueil (sequence state + texts), Facteur bubble with the postman animation, note through the existing `View` / overlay path. No new dependency.

## Acceptance criteria

- [ ] `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test` all green.
- [ ] No `progress.json`: launch → Facteur rides to the mailbox, arrival bubble with the arrival text; flag down, no `Lettre ! [c]` in the top bar.
- [ ] `Entrée` → note grows progressively, then shows title, full body, signature and footer as written above.
- [ ] `Entrée` → departure bubble « Je viens de te déposer ton courrier. Moi, je file, à demain ! »; flag rises and `Lettre ! [c]` appears at that moment.
- [ ] `Entrée` → Facteur leaves; `c` opens Lettre 1.
- [ ] `Échap` on the arrival bubble, the note or the departure bubble → Accueil ends at once, flag up, `c` opens Lettre 1.
- [ ] During the Accueil, `c` `l` `a` `b` `h` do nothing; `q` quits; next launch shows the Accueil again from the start.
- [ ] Accueil finished or skipped → next launch: no Accueil.
- [ ] Existing save (e.g. day 40, no new field) → Accueil once at next launch, same texts; never after.
- [ ] `--silent`, `--demo` or `--simulate` on a save that hasn't seen it → no Accueil; next normal launch shows it.
- [ ] Save with unseen release notes → Accueil first, Nouveautés box right after the Facteur leaves.
- [ ] Badge earned during the Accueil → its card shows only after the Accueil.
- [ ] Terminal 60×20 → note readable, wrapped, `↑`/`↓` scroll; no punctuation alone at a line start.
- [ ] Old `progress.json` loads; new field defaults to not seen (test). Sequence steps and `Échap` skip covered by tests.

## Implementation

## Review

## Verification

## Feedback
