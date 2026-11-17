---
issue: 41
status: to-test
wiki: [[mail]], [[render]]
note: "Protéger la progression : si progress.json est abîmé, weathr repart de zéro sans prévenir et écrase le fichier ; garder une copie de secours et afficher un message"
created: 2026-10-27
---

## Context / Decisions

> Protéger la progression : si progress.json est abîmé, weathr repart de zéro sans prévenir et écrase le fichier ; garder une copie de secours et afficher un message

- No blocker, no sprint. Today `Progress::load` (`src/progress.rs`) turns an unreadable file into `Progress::default()`, then saves over it: the old save is lost, silently.
- **Unreadable `progress.json`** (exists, but can't be read or parsed):
  - Renamed, never overwritten: `progress.json.abime-YYYY-MM-DD-HHMM` next to it.
  - A new game starts (`Progress::default()` + start date), as for a fresh install.
  - Missing file stays the fresh-install case of #24, no message.
- **Safe write**: `Progress::save` writes `progress.json.tmp`, then renames it over `progress.json`. The file is never half written.
- **Message**: a box at launch, in the TUI, before the Nouveautés box.
  - Title `Sauvegarde`. Text: « Ta progression n'a pas pu être lue. Une copie est gardée dans <chemin>. Une nouvelle partie commence. »
  - `Esc` or `Enter` closes it. Shown once, on that launch only.
- **New frame `Frame::Floppy`** (`src/overlay.rs`), for save info: top `+-[#]-----…+`, sides `| … |`, bottom `+-----…+`, grey-blue colour.
- **Release notes**: the new game has no `notes_seen_on` → treated like an old save (#24), newest section shown after the Sauvegarde box. Not a fresh install.
- `--demo`, `--silent`, forced modes: the copy and the new game happen anyway (the file must not be overwritten); the box is shown too, since it says data moved.
- Wording: `{conventions}/wording.md`, error situation: say what happened and where the copy is.
- Out of scope: repairing the old file, merging it back, rotating several copies.

## Acceptance criteria

- [ ] `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` all green.
- [ ] `progress.json` containing `{oops`: launch → box `Sauvegarde` in the floppy frame with the copy's path; the copy holds `{oops` byte for byte.
- [ ] Same launch: new game started (Lettre 1), `progress.json` valid JSON.
- [ ] `Esc` closes the box, then the Nouveautés box shows.
- [ ] Next launch: no Sauvegarde box.
- [ ] No `progress.json`: no box, no copy (fresh install).
- [ ] `save` leaves no `progress.json.tmp` behind and never a truncated `progress.json` (test: write then read back).
- [ ] Copy name follows `progress.json.abime-YYYY-MM-DD-HHMM` (test with a fixed date).

## Implementation

- Branche remise à jour depuis `develop` (fusion, plus d'un mois de retard).
- `src/progress.rs` : `Progress::load() -> (Self, Option<PathBuf>)`, le second élément est le chemin de la copie. Fichier présent mais illisible → renommé `progress.json.abime-YYYY-MM-DD-HHMM`, jamais écrasé, nouvelle partie (`Progress::default()` + date du jour). `load_from_path(path, now)` / `save_to_path(path)` sur le modèle de `Config`. Renommage impossible → partie en mémoire seulement (`in_memory`), ni boîte ni copie.
- Écriture sûre : `progress.json.tmp` écrit, `sync_all`, puis renommé ; en cas d'erreur, le `.tmp` est supprimé.
- `src/overlay.rs` : `Frame::Floppy` (`+-[#]---…+`, `| … |`, `+---…+`, gris-bleu RGB 150, 165, 190).
- `src/views.rs` : vue « Sauvegarde » avec le chemin de la copie.
- `src/app.rs` : `Overlay::Backup { shows_welcome }` + champ `App::backup` ; seuls `Esc` / `Entrée` la ferment (`q` quitte toujours), pour ne pas laisser le Facteur de l'Accueil bloqué devant la porte. Ordre : Sauvegarde → Accueil (nouvelle partie, `welcome_seen` faux, règle de #53) → Nouveautés.
- Modes forcés (`--simulate`, `--demo`, `--silent`, `--night`) : copie + boîte quand même. `--day N` : fichier jamais touché.
- Limite : un chemin plus large que la boîte dépasse le cadre (`wrap` ne coupe jamais un mot) ; les chemins réels tiennent à 80 colonnes.
- Glossaire à ajouter à la clôture : Sauvegarde (`Progress`, `progress.json`), Copie de secours (`backup`, `progress.json.abime-…`).
- Build : `cargo build`, `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings -A clippy::double_must_use`, `cargo test` (492, 0 échec) verts.

## Review

## Verification

Autopilot, binaire debug, HOME jetable, pseudo-terminal (implémenteur) :
- ✅ `{oops` → boîte Sauvegarde, cadre floppy, chemin de la copie ; copie = `{oops` octet pour octet.
- ✅ Même lancement : nouvelle partie, « Jour 1 », `progress.json` valide.
- ⚠️ `Esc` → Accueil (nouvelle partie) → `Esc` → Nouveautés : l'Accueil passe entre les deux (spec écrite avant #53).
- ✅ Lancement suivant : ni boîte ni nouvelle copie.
- ✅ Sans `progress.json` : ni boîte ni copie, Accueil seul.
- ✅ Aucun `.tmp` laissé, jamais de fichier tronqué (test).
- ✅ Nom de copie à date fixe (test).
- ✅ `--simulate rain` : copie + boîte ; `--day 3` : fichier intact.

## Feedback
