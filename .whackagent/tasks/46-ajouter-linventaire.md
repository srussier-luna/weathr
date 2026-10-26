---
issue: 46
status: todo
wiki: [[mail]], [[render]], [[architecture]]
note: "On va aussi ajouter un système d'inventaire pour gérer nos objets"
created: 2026-10-26
---

## Context / Decisions

> On va aussi ajouter un système d'inventaire pour gérer nos objets

- Sprint Saison 2. Blocks #44 (outils des Voisins) and #45 (graines de l'oiseau): they add their items and actions on top.
- **Scope**: only items that are *used* (graines, outils, broche…). The 25 garden Objets, Bouteilles and chest souvenirs stay in the Album, unchanged.
- No item can be obtained in game yet: #46 ships the storage, the catalog, the screen and the key. Obtaining items is #44 / #45.
- **Catalog**: new module `src/inventory.rs` — static list of item definitions (id, name, one-line description, optional action), in display order. Ships with one item: `graines` « Graines », description in the village voice (e.g. « Une poignée de graines. Le Potager en voudra. », per `{conventions}/wording.md`), no action yet.
- **Storage**: `Progress.inventory: BTreeMap<String, u32>` (`#[serde(default)]`, old saves keep loading). Quantity 0 → entry removed. Ids not in the catalog → not shown, kept in the save.
- **Screen Inventaire** (`Overlay::Inventory`, view next to `album_pages` in `src/views.rs`):
  - Key `i` / `I` opens and closes it; `Esc` closes.
  - One line per held item, catalog order: name + quantity (`Graines ×3`), description under the selected one or beside it — match the Album box style.
  - `↑` / `↓` move the selection.
  - `Entrée` → the item's action; no action yet → one line in the box: « Ça servira bien un jour. »
  - Footer like the Album: `[↑] [↓] choisir  [Entrée] utiliser  [i] ou [Echap] fermer`.
- **Empty**: key `i` does nothing and is absent from the Aide until the first item is held. Then Aide shows `[i] inventaire`.
- Manual test (weathr-test): add `"inventory": {"graines": 3}` to `progress.json`.

## Acceptance criteria

- [ ] `cargo fmt --check`, `cargo test` green.
- [ ] Old `progress.json` without `inventory` loads; inventory empty; `i` does nothing; Aide has no `[i]`.
- [ ] `progress.json` with `"inventory": {"graines": 3}` → Aide shows `[i] inventaire`; `i` opens the box with `Graines ×3` and its description; `i` or `Esc` closes it.
- [ ] With two held items (unit test, catalog extended in the test), `↑` / `↓` move the selection.
- [ ] `Entrée` on Graines → « Ça servira bien un jour. » in the box.
- [ ] Unknown id in `inventory` → not shown, still in `progress.json` after a save.
- [ ] Unit tests: add / remove quantities (0 removes the entry), catalog order, inventory view lines.

## Implementation

## Review

## Verification

## Feedback
