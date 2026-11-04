---
issue: 97
status: todo
wiki: [[animation]], [[mail]]
note: "jours 14 lettre de 13 des pas du puits a la maison / ils sont a l'ancienne emplacement du puits / verifie tous les dessins de pas pour cahque jours et push ke en hot fix pour faire une 1.6.1 sans note de version utilisateur"
created: 2026-11-04
---

## Context / Decisions

> jours 14 lettre de 13 des pas du puits a la maison
> ils sont a l'ancienne emplacement du puits
> verifie tous les dessins de pas pour cahque jours et push ke en hot fix pour faire une 1.6.1 sans note de version utilisateur

- **Bug seen** on the real game (1.6.1, Jour de jeu 14): brown `:` Traces de boue still on the ground, from the Arbre to the door. The day-13 creepy Lettre says « Les traces vont du puits à ta porte », so they look like the Puits trail drawn at the wrong spot.
- **Cause**: an Apparition is permanent. `Unlocks` is built from `timeline.arrived_by(day)` (`src/app.rs` `unlocks_on`), so `footprints` (village day 8) and `wet_footprints` (creepy, day 13) stay true forever.
  - Traces de boue: `src/animation/lore.rs`, `unlocks.footprints`, `:` in `MUD`, from `trunk_x + trunk_width + 4` to the door.
  - Traces mouillées: `src/animation/creepy.rs`, `unlocks.wet_footprints && !is_day`, `.` in `WET`, from the Puits' right edge to the door (left edge when the Puits is hidden).
- **Every footprint-related text, by Jour de jeu**:
  - 8 · encre bleue « quelqu'un a marché dans ton jardin… les traces mènent quelque part »
  - 11 · « Ces traces sont petites »
  - 13 · creepy Lettre « Les traces vont du puits à ta porte »
  - 16 · Léo « Pardon pour les traces de boue »
  - 30 · Marcel « de la boue près du puits, encore. Des pas qui vont vers ta maison »
- **Decision — Traces de boue**: Jour de jeu 8 only (day and night). Gone from day 9.
- **Decision — Traces mouillées**: nights of Jours de jeu 13, 14 and 30 only. Path unchanged (Puits → door).
- Positions unchanged: only *when* each trail shows changes.
- Demo mode and any other caller of `Unlocks` follow the same rule (one source of truth for the days).
- **Hotfix 1.6.2**, user's call:
  - Task branch `wa/97-corriger-les-traces-de-pas` forked from `main` (not `develop`).
  - Bump `Cargo.toml` / `Cargo.lock` to `1.6.2`.
  - PR to `main`, title « Version 1.6.2 », label `v1.6.2`.
  - **No player release note**: `NOUVEAUTES.md` untouched; PR body has no « Changements » section for players.
  - After merge into `main`: merge `main` back into `develop` (as for 1.6.1, PR #81).
- Out of scope: new trails, moving the Puits or the Arbre, rewording Lettres.

## Acceptance criteria

- [ ] `cargo build` and `cargo test` green.
- [ ] Jour de jeu 7: no Traces de boue.
- [ ] Jour de jeu 8, day and night: Traces de boue from the Arbre to the door.
- [ ] Jour de jeu 9 to 61: no Traces de boue (e.g. day 14 shows none).
- [ ] Night of Jour de jeu 13 and 14: Traces mouillées from the Puits' right edge to the door.
- [ ] Day of Jour de jeu 13 and 14: no Traces mouillées.
- [ ] Nights of Jours de jeu 12, 15, 29, 31: no Traces mouillées.
- [ ] Night of Jour de jeu 30: Traces mouillées from the Puits to the door.
- [ ] Unit test covers which Jours de jeu show each trail.
- [ ] `Cargo.toml` version `1.6.2`, `NOUVEAUTES.md` unchanged vs `main`.

## Implementation

## Review

## Verification

## Feedback
