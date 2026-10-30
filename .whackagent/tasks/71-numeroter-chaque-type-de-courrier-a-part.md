---
issue: 71
status: todo
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

## Review

## Verification

## Feedback
