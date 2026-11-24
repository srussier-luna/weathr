---
issue: 163
status: to-close
wiki: [[render]], [[mail]]
note: découpée de #37 au grill du 2026-11-24
created: 2026-11-24
---

## Context / Decisions

- Un Badge gagné ne se voit aujourd'hui que dans sa carte « Nouveau Badge » : une fois fermée, plus rien ne le rappelle. Le but : le signaler comme une Lettre ou un Objet, jusqu'à ce que le joueur le voie.
- proto: [[protos/changer-la-barre-du-haut-separation]] · https://claude.ai/artifact/U5Kc6zxWSthjRk6ru3W5eC (v10 validé : état « Nouveau Badge », écran « vue Badges »)
- Tant qu'un Badge n'a pas été vu, la Barre du bas (#37) affiche `[b] Badge ! 4/12` à la place de `[b] Badges 4/12`. C'est en rose, avec le même clignotement que Lettre ! / Objet ! (rose ↔ gris, 500 ms).
- Dans la vue Badges (`b`), chaque Badge pas encore vu clignote en rose dans sa colonne, au même rythme.
- Les Badges pas encore vus sont gardés dans la sauvegarde (`Progress`), à côté des Badges gagnés : ils restent nouveaux après un redémarrage.
- Ouvrir la vue Badges les marque tous comme vus. Après la fermeture, ils ne clignotent plus nulle part.
- La carte « Nouveau Badge » est gardée telle quelle. Elle ne marque pas le Badge comme vu : seule la vue Badges le fait.
- Une sauvegarde d'avant cette tâche n'a aucun Badge « pas encore vu ».
- Bloquée par #37 : le clignotement vient de la Barre du bas.
- Version : v1.9.

## Acceptance criteria

- On gagne un Badge et on ferme sa carte : la Barre du bas montre `[b] Badge ! x/y` en rose, qui clignote toutes les 500 ms.
- On quitte et on relance weathr : `[b] Badge !` clignote toujours.
- On appuie sur `b` : le nouveau Badge clignote dans la vue Badges, les autres non.
- On ferme la vue Badges : la Barre du bas revient à `[b] Badges x/y` en gris. Si on rouvre la vue, plus rien ne clignote.
- Deux Badges gagnés avant d'ouvrir la vue : les deux clignotent dans la vue.
- Une ancienne sauvegarde se charge sans erreur et sans Badge qui clignote.

## Implementation

- autopilot 2026-11-24, wa-implementer, round 1.
- `progress/mod.rs`: `unseen_badges: BTreeSet<String>` (`#[serde(default)]`).
- `app.rs`: `mark_unseen` on `award()` + tier cards (`record_trip`); `toggle_badges()` replaces inline `b` handler: on open, `reveal_badges` empties the set, saves Progress, keeps the copy in `revealed_badges` to drive this view's blink.
- `app_state.rs`: `has_new_badge` + `set_new_badge`.
- `hud/toolbar.rs`: `[b] Badge ! x/y` pink ↔ grey while unseen; compact form drops the count (`[b] Badge !`).
- `overlay.rs`: `Highlight { line, start, len }`, `BadgeLines`; `frame_rows` / `render_box` take highlights (pink over cell); `is_blink_lit()`.
- Tricheur doesn't go through `earn()` → never unseen. Sam tier cards mark Sur les traces de Sam unseen again.
- Proof: `cargo build` ok · `cargo test` 166 + 556 + 10 + 7 + 3 + 5 passed · fmt + clippy clean.

## Review

### validation — 2026-11-24 · clean

Rebased onto `sprint/barres` after #164 + #165 merged. Conflict in `src/hud/toolbar.rs` (#37 review turned `Item` into an enum): Badge alert redone as an `Alert` with a detail shown only when not compact.

Round 1, 6 findings, autofixed:
- 🟠 [correctness] `story/reach.rs` `Reach::fit` revoked Badges but left them unseen → `[b] Badge !` blinked for nothing, revoked Badge blinked in the view; now unseen ∩ earned, + test
- [structure] `PINK` imported from `hud::toolbar` → moved to `hud/mod.rs`
- [style] `BadgeLines.unseen` held highlights → `highlights`
- [elegance] positional `Alert` tuple → named fields
- [style] `Item::text` arm order
- [style] shadowed `first_line` in `frame_rows` → `first_body_line` / `first_line_row`

Round 2: all fixed, nothing new. Build (clippy strict) ✅ · tests ✅.

## Verification

- manual validation — not run by agent (TUI). Each criterion covered by unit tests (app, progress, toolbar, overlay).

## Feedback
