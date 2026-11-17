---
issue: 122
status: to-close
wiki: [[mail]], [[architecture]]
note: "il faut bloquer le passage du facteur du matin dans les preference, avec impossibilite de le faire passer apres 14h et avant 6h"
created: 2026-11-17
---

## Context / Decisions

> il faut bloquer le passage du facteur du matin dans les preference, avec impossibilite de le faire passer apres 14h et avant 6h

- **Heure du Facteur** limitée de 6 h à 14 h inclus. Le Facteur reste un passage du matin ou de la mi-journée.
- **Réglages** (`src/settings/mod.rs`, ligne « Heure du Facteur ») : aujourd'hui ←/→ tourne sur 24 h (`rem_euclid`). Désormais ←/→ s'arrête aux bornes : 14 h → → reste 14 h, 6 h ← reste 6 h. Pas de boucle.
- **config.toml** (décision revue en validation) : une heure hors 6–14 (écrite à la main, ou enregistrée par les anciens Réglages) est ramenée à la borne la plus proche au chargement : < 6 → 6, > 14 → 14. Le reste de la config est gardé, le jeu démarre. Pas de réécriture du fichier. (Avant : erreur de config, mais `main.rs` repartait alors de la config par défaut et ignorait tout le fichier.)
- Valeur par défaut inchangée : 9 h.
- Effet voulu sur le secret (sprint « Le secret ») : l'Arcano-Facteur passe entre 18 h et 2 h.
- Changement d'heure déjà enregistré dans la sauvegarde (`mail_hour_change`, ancienne heure hors plage) : gardé tel quel, il ne sert qu'à l'historique des jours.
- Accepté (validation) : le jour de la mise à jour, une personne réglée sur 0–5 h qui lance le jeu avant 6 h voit le Jour de jeu reculer d'un jour et le Facteur repasser à 6 h, comme après une modification à la main du config.toml.
- Hors périmètre : réécrire un config.toml hors plage ; changer le repli général de `main.rs` sur erreur de config.

## Acceptance criteria

- Réglages → Heure du Facteur à 14 h, → → reste 14 h ; à 6 h, ← → reste 6 h ; entre les deux, ←/→ change d'une heure. À une borne, rien n'est réenregistré.
- config.toml avec `heure = 5` → le jeu démarre avec 6 h ; `heure = 15` → 14 h ; ville, unités et autres réglages du fichier gardés. `heure = 6` et `heure = 14` → inchangées.
- config.toml sans heure → 9 h, comme avant.
- Tests unitaires : bornes des Réglages, chargement de config (5, 6, 14, 15).
- `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings -A clippy::double_must_use`, `cargo test --bin weathr` verts.

## Implementation

- `src/config/mod.rs` : `MAIL_HOURS = 6..=14` (privé) et `pub fn clamp_mail_hour` ; `Mail.hour` lu via `deserialize_mail_hour` : toute heure du config.toml ramenée dans 6–14, le reste de la config gardé ; défaut 9 inchangé ; plus de contrôle d'heure dans `validate`.
- `src/settings/mod.rs` : `Row::MailHour` → `clamp_mail_hour(heure ± 1)`, plus de boucle ; `None` à une borne (rien n'est enregistré, `App::step_setting` sort tôt) ; `HOURS_PER_DAY` supprimé.
- `src/error.rs` : `ConfigError::InvalidMailHour` supprimé (plus utilisé). Une heure ≥ 24 est désormais ramenée à 14 au lieu d'une erreur.
- `README.md` : commentaire de la config d'exemple « Heure d'arrivée du courrier (0 à 23) » → « Heure du Facteur (6 à 14) ».
- Tests : `test_mail_hour_stops_at_14`, `test_mail_hour_stops_at_6` (`None` aux bornes), `test_mail_hour_from_config` (6, 14, défaut 9), `test_mail_hour_outside_6_to_14_is_brought_back` (0→6, 5→6, 15→14, 23→14, ville gardée), via `Config::load_from_path` sur un vrai fichier. `test_saved_settings_load_back` passe à 14.
- Build : `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings -A clippy::double_must_use`, `cargo build`, `cargo test` (bin 331, lib 129) verts.
- Round validation : décision config.toml revue (erreur de config → borne la plus proche), car `main.rs` ignorait tout le fichier sur erreur.

## Review

### Round validation — 2026-11-17
- Scope : diff cumulé `origin/develop..HEAD` (d96ccb3) + correctifs.
- Findings (autofix, 2 tours) :
  - 🔴 [correctness] heure hors 6–14 → `ConfigError`, mais `main.rs` repart de `Config::default()` : tout le config.toml ignoré (ville, unités, position auto) → décision utilisateur·rice : heure ramenée à la borne au chargement (`deserialize_mail_hour` / `clamp_mail_hour`), `InvalidMailHour` supprimé.
  - 🟡 [elegance] touche sans effet à une borne réécrivait progress.json et config.toml → `stepped` renvoie `None`.
  - 🟡 [style] `test_saved_settings_load_back` avec `MailHour(23)` → 14.
  - 🟡 [style] README « Heure d'arrivée du courrier » → terme du glossaire « Heure du Facteur ».
  - 🟡 [style] sections Implementation / Verification de la tâche à jour.
- Accepté par l'utilisateur·rice : Jour de jeu qui recule d'un jour le jour de la mise à jour (heure 0–5 h, lancement avant 6 h).
- Reporté à `/wa-close` (wiki) : `architecture.md` « hour wraps 0–23 » ; `mail.md` sans la borne 6–14.
- Preuve : `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings -A clippy::double_must_use`, `cargo build`, `cargo test` (bin 331, lib 129, intégration 0 échec) verts.
- LENSES : style ✓ · elegance ✓ · structure ✓ · correctness ✓ — VERDICT : clean (1 finding accepté, 1 reporté au wiki).

## Verification

Round autopilot (binaire debug, HOME jetable, implémenteur) — avant la validation :
- ✅ Réglages en pty : 14 puis → → reste 14 ; 14 puis ← ×10 → 6 ; 6 puis ← ← reste 6 ; 6 puis → → 7.
- ~~`heure = 5` / `heure = 15` → erreur de config~~ : comportement remplacé en validation.

Round validation : manual validation — not run by agent (`heure = 5` → 6 h, `heure = 15` → 14 h, ville gardée : couvert par tests sur un vrai fichier).

## Feedback
