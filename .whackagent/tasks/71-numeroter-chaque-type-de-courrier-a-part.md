---
issue: 71
status: to-test
wiki: [[mail]]
note: "dans le decompte de lettre la gazette est compter il fat les enlever du decompte. exemple lettre 4/7 gazette lettre 5/7 aujourd'hui apres gazette on a 6/8 au lie de 5/7"
created: 2026-10-30
---

## Context / Decisions

> dans le decompte de lettre la gazette est compter il fat les enlever du decompte
> exemple lettre 4/7 gazette lettre 5/7 aujourd'hui apres gazette on a 6/8 au lie de 5/7

- Sprint **Timeline histoire**, version **v1.6**.
- Today the `x/y` in mail titles is the position in the whole delivered mail (`content::entry_view(day, entry, index + 1, available, …)`): Gazette, Cartes postales, Archives, Recettes all bump it.
- **Each kind of mail has its own counter**:
  - **Lettre** — one shared counter for every `Lettre x/y`: village (`Entry::Main`), Sam, voisins, Marcel, lettres glissées sous la porte (`Creepy`).
  - **Carte postale x/y**, **Archives de la mairie x/y**, **Recette du mois x/y** — each its own series.
  - **Gazette** — no counter, keeps its dated title (`LA GAZETTE · lundi 26 octobre`).
- `x` = rank of this item among delivered mail of its kind; `y` = how many of that kind are delivered so far (same "delivered" as today).
- Courrier list labels (`content::label`) unchanged: they show no counter.
- Mail order, `letters_read`, delivery: unchanged — display only.
- Tests: unit tests next to the code (`#[cfg(test)]`).

## Acceptance criteria

- [ ] `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` all green.
- [ ] Lettre 4/7, then a Gazette, then the next Lettre reads `Lettre 5/7`, not `6/8` (test).
- [ ] A Sam / voisins / Marcel / sous-la-porte Lettre continues the same `Lettre x/y` series as the village Lettres (test).
- [ ] First Carte postale reads `Carte postale 1/n`, first Recette `Recette du mois 1/n`, first Archive `Archives de la mairie 1/n`, `n` = delivered of that kind (test).
- [ ] Gazette title unchanged, no counter.
- [ ] Courrier list (`l`) unchanged.

## Implementation

- Branch brought up to date with `sprint/timeline-histoire` (merge, brings #43).
- `src/story/content/mod.rs`: private `Series` (Letter, Postcard, Archive, Recipe, Gazette) + `series(Entry)`; `entry_view(delivered: &[(usize, Entry)], index, ctx) -> Option<View>` counts rank and total within the entry's series. Gazette still shows no counter. `Entry`, `Timeline`, `content::label` unchanged.
- `src/app.rs`: Letter overlay passes `timeline.first_mail(available_letters())`; an index past `available` renders nothing (was a `9/8`-style title; unreachable through navigation).
- `src/story/testdata/save_37_golden.txt`: regenerated, only `title:` counters change (e.g. Lettre 10/101 → 9/84, Archives 47/101 → 1/5, Recette 54/101 → 1/1, Carte postale 71/101 → 1/3).
- Tests: `test_gazette_does_not_bump_the_lettre_counter`, `test_correspondents_continue_the_lettre_series`, `test_postcards_archives_and_recipes_count_on_their_own`.
- Proof: `cargo fmt -- --check && cargo clippy -- -D warnings -A clippy::double_must_use && cargo test` green.

## Review

## Verification

manual validation — not run by agent

## Feedback
