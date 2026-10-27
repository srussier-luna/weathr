---
issue: 53
status: to-test
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
  2. Big speech bubble above him: arrival text. Footer `[Entrée] suite  [Echap] passer` (existing footer style).
  3. `Entrée` → the note **unfolds** (box grows progressively, under 1 s), then shows the full note text.
  4. `Entrée` → departure bubble. **At that moment the red mailbox flag rises** and the top bar shows `Lettre ! [c]` (if a Lettre is unread).
  5. `Entrée` → bubble closes, Facteur leaves. Then the Nouveautés box opens if release notes are pending (**Accueil first, Nouveautés after**).
- **During the Accueil**: flag down, no `Lettre ! [c]`, `c` `l` `a` `b` `h` `e` `r` do nothing; `q` quits. Badge cards wait until the end (Accueil counts as an open window).
- **`Échap`** at any step → Accueil ends at once: bubbles and note gone, Facteur rides off on his bike like after the last `Entrée` (feedback round 1), flag up if unread, marked seen.
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
  - Signature: « — Le Facteur ». Footer: `[Entrée] suite  [Echap] passer` (existing footer style).
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
- [ ] `Échap` on the arrival bubble, the note or the departure bubble → Accueil ends at once, Facteur rides off on his bike, flag up, `c` opens Lettre 1.
- [ ] During the Accueil, `c` `l` `a` `b` `h` do nothing; `q` quits; next launch shows the Accueil again from the start.
- [ ] Accueil finished or skipped → next launch: no Accueil.
- [ ] Existing save (e.g. day 40, no new field) → Accueil once at next launch, same texts; never after.
- [ ] `--silent`, `--demo` or `--simulate` on a save that hasn't seen it → no Accueil; next normal launch shows it.
- [ ] Save with unseen release notes → Accueil first, Nouveautés box right after the Facteur leaves.
- [ ] Badge earned during the Accueil → its card shows only after the Accueil.
- [ ] Terminal 60×20 → note readable, wrapped, `↑`/`↓` scroll; no punctuation alone at a line start.
- [ ] Old `progress.json` loads; new field defaults to not seen (test). Sequence steps and `Échap` skip covered by tests.

## Implementation

- `src/welcome.rs` (new): `Step` machine (Arriving → Greeting → Unfolding → Reading → Farewell), texts (U+202F before `: ? ! ;)`), `note_view`, `unfolding_view`, `scroll_limit`, tests.
- `src/progress.rs`: `welcome_seen: bool`, `#[serde(default)]` → fresh and old saves see the Accueil once; saved at last Entrée or Échap only.
- `src/animation/postman.rs`: trips `Arriving` (2.5 s ride) and `Staying` (holds at the mailbox, no labels); `visit`, `leave`, `is_arriving`, `head`; daily round can't start during a visit. `src/animation_manager.rs`: pass-throughs.
- `src/overlay.rs`: `Frame::Bubble` + `render_bubble` (≤ 40 cols, tail toward the Facteur's head, clamped to edges); layout split into `lay_out` / `draw_rows`; `wrap` splits on ASCII whitespace only, U+202F stays glued (test).
- `src/views.rs`: Nouveautés scroll logic shared as `scrolled_view` / `scroll_limit`, same output.
- `src/app.rs`: `Overlay::Welcome(Step)`, launch rule, Nouveautés deferred until the Accueil ends (then Badge cards), flag + HUD hidden before the departure bubble, Accueil keys, rendering, tests. `src/main.rs`: `mod welcome;`.
- Narrow terminal (< ~71 cols, no mailbox): no ride, bubbles centred near the bottom without tail; note wraps and scrolls.
- Entrée ignored while he rides in; Entrée during the unfold shows the full note. Scrolling note → footer `[↑] [↓] défiler  [Entrée] suite  [Echap] passer`.
- First launch with no save + `--demo` / `--simulate` / `--silent` → counts as Fresh, Accueil shown (only an existing save in those modes skips it).
- Build: `cargo build` ✅ · fmt ✅ · `cargo test` ✅ · clippy ❌ only on pre-existing `clippy::double_must_use` in `src/weather/provider.rs:23` (untouched, `#[async_trait]` expansion); clean with that lint allowed.

- Validation autofix: `Échap` → `Postman::dismiss` (Facteur vanishes at once), last `Entrée` → `leave` (rides off) · `WelcomeOutcome { Next, Finish, Skip }` decides the exit in one place · `notes_overlay(has_notes)` shared by `opening_overlay` / `end_welcome` · `Step::scroll(self, delta, last)`.

- Feedback round 1 hunks: `src/app.rs` (`WelcomeOutcome { Next(Step), Done }`, `end_welcome` calls `postman_leave` for `Échap` and last `Entrée`, test updated) · `src/animation/postman.rs` (`dismiss` + its test removed) · `src/animation_manager.rs` (`postman_dismiss` removed).

## Review

### Round validation — 2026-10-27

LENSES: style ✓ · elegance ✓ · structure ✓ · correctness ✓

- 🟡 [correctness] `Échap` made the Facteur ride off, not vanish (`app.rs`) → fixed (autofix 1)
- 🟡 [elegance] `opening_overlay(false, …)` literal flag → fixed, `notes_overlay` (autofix 1)
- 🟡 [style] `step` param shadowed in `Step::scroll` → fixed, `delta` (autofix 1)
- 🟡 [elegance] finish vs skip decided in two places (from autofix 1) → fixed, `WelcomeOutcome` (autofix 2)

VERDICT: clean · 2 autofix rounds · proof: build ✅ fmt ✅ test ✅ (226) · clippy ✅ except pre-existing `double_must_use` (`src/weather/provider.rs:23`)

## Verification

manual validation — not run by agent

validation: manual validation — not run by agent · run: yours

round 1: manual validation — not run by agent · run: yours

## Feedback

### Round 1 — 2026-10-27

- Asked: « non je veux qu'il reparte a velo comme avant » (avec `[Echap]`).
- Triage: adjustment → Decisions + criteria updated (`Échap` → Facteur rides off like after the last `Entrée`).
- Changes: implementer — `src/app.rs`, `src/animation/postman.rs`, `src/animation_manager.rs`; wiki `animation.md` aligned.
- Build: `cargo build` ✅ · fmt ✅ · `cargo test` ✅ (224) · clippy ❌ only on pre-existing `double_must_use` (`src/weather/provider.rs:23`).
- Review: deferred → /wa-validate.
- Verify: manual validation — not run by agent.
- Rule captured: none.
