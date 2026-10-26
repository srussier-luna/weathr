---
issue: 44
status: todo
wiki: [[mail]], [[animation]], [[weather]], [[render]]
note: "dans la saison 2 ajouter aussi la gestion du potager avec des voisins qui donnent des outils ; le Potager doit amener des badges, action d'arroser, la pluie arrose aussi"
created: 2026-10-26
---

## Context / Decisions

> dans la saison 2 ajouter aussi la gestion du potager avec des voisins qui donnent des outils
> Le potager doit amener des badges, action d'arroser, la pluie arrose aussi

- Sprint **Saison 2**. Blocked by:
  - #9: the Potager scene (Lieux menu `m`, top-down view, real weather, winter rest). #9 no longer grows the Potager by Jour de jeu: it opens with 9 empty beds; growth is this task.
  - #46: the Inventaire (seeds, tools and vegetables live there).
- Seeds come from the bird (#45). The Voisin's help (help offer after 3 days, take-over at 30 %, « J'ai arrosé ton potager » when the Boîte is full) is a separate task, blocked by this one.
- **No spoilers**: which Voisin gives the tools and when come from the season 2 Lettres (#8).
- **9 Carrés** in the Potager scene (3 × 3 beds in #9's view).
- **Controls** (Potager scene only):
  - Arrows → select a Carré (visible highlight).
  - `Entrée` → the action of the moment: empty → sow (small list of held seeds, `↑`/`↓` + `Entrée`, `Esc` cancels) · planted → water · ripe → harvest.
  - `o` → « s'occuper du Potager »: water every plant, harvest every ripe one, sow free Carrés with held seeds (most-held seed first).
  - Keys shown in the scene footer / Aide only while in the Potager.
- **Tools** (Inventaire, #46): Arrosoir needed to water, Transplantoir needed to sow. Both given when the Potager opens. Without the tool the action is not offered (`o` skips it too).
- **Seeds per vegetable**: radis, salade, petits pois, tomate, courgette, haricot — one seed item per vegetable (`graines-<légume>` ids) and one vegetable item each, in the Inventaire catalog.
- **Jour de plante** = a day weathr is opened (first launch of the day). Days the app stays closed do not count: holidays never kill plants.
  - Growth: a plant is ripe after **5** watered Jours de plante.
  - Water: watering covers the current Jour de plante. **3** Jours de plante without water → **fanée** (drawn wilted, growth paused). Watering a fanée plant revives it, growth resumes.
  - **3** more Jours de plante fanée without water → **morte**: the Carré frees, seed lost.
- **Rain waters**: real rain (Open-Meteo, current conditions or the day's precipitation) on a Jour de plante waters every plant of that day.
- **Seasons**: sow, water and grow in spring and summer. Autumn: no sowing, remaining plants can still ripen and be harvested. Winter: Potager asleep (#9), no plant, nothing counted.
- **Harvest** → 1 vegetable of that kind into the Inventaire, Carré frees.
- **Badges** (new, Badge table):
  - Première récolte — first vegetable harvested by the player.
  - Main verte — all 9 Carrés planted by the player at the same time.
  - Merci la pluie — a plant watered by real rain.
  - Potager complet — each of the 6 vegetables harvested at least once.
- Save: Carrés (seed, sown by, Jours de plante watered, days without water, state), last Jour de plante date — in `Progress`, `#[serde(default)]`.
- Forced modes (`--demo`, `--simulate`, …): Potager shown, no Jour de plante counted, no Badge.
- Wording: `{conventions}/wording.md` — village voice for item descriptions; footer keys short.

## Acceptance criteria

- [ ] `cargo fmt --check`, `cargo test` green.
- [ ] Potager open, spring, Arrosoir + Transplantoir + `graines-radis` held: arrows move the highlight over 9 Carrés; `Entrée` on an empty one lists held seeds; picking radis sows it, seed count −1.
- [ ] `Entrée` on a planted Carré waters it; `o` waters all, harvests ripe ones, sows free Carrés with held seeds.
- [ ] Without Arrosoir: no water action, `o` doesn't water. Without Transplantoir: no sow action.
- [ ] 5 watered Jours de plante → ripe; `Entrée` harvests: radis +1 in the Inventaire, Carré free.
- [ ] 3 Jours de plante without water → fanée (drawn wilted, no growth); watering → back to normal; 3 more without water → morte, Carré free.
- [ ] Days weathr stays closed don't count (no wilting after a week away).
- [ ] Rain on a Jour de plante → all plants count as watered.
- [ ] Autumn: no sowing; existing plants still ripen and harvest. Winter: no plant, nothing counted.
- [ ] Badges Première récolte, Main verte, Merci la pluie, Potager complet awarded on their conditions, not in forced modes.
- [ ] Unit tests: Jour de plante counting, growth / fanée / morte transitions, rain watering, `o` order, badge conditions.

## Implementation

## Review

## Verification

## Feedback
