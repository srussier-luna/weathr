---
issue: 17
status: todo
wiki: [[animation]], [[mail]]
note: Fête de la musique : notes ♪ qui sortent de la maison le 21 juin
created: 2026-10-30
---

## Context / Decisions

- Fête de la musique = le 21 juin seulement, toute la journée, jour et nuit (comme le poisson d'avril ou la galette).
- Nouveau flag `Holiday.is_music_day` dans `src/holiday.rs`, calculé par `Holiday::on`.
- Animation : notes ♪ ♫ qui sortent des fenêtres de la maison, montent en ondulant, disparaissent sous le toit. Plusieurs à la fois.
- Petite et grande maison : les notes partent des fenêtres présentes dans le sprite affiché.
- Météo sans effet : notes par tous les temps (la musique joue dedans).
- Nouveau système dans `src/animation/`, à côté de `epiphany.rs`, branché dans `AnimationManager`.
- Badge « Fête de la musique », `BadgeKind::Holiday`, gagné en lançant le jeu le 21 juin, comme Noël ou Halloween.
- Aperçu : une scène « Fête de la musique » dans `--demo`, et un flag `--music` comme `--christmas` / `--fireworks`.

## Acceptance criteria

- Date simulée au 21 juin → des notes ♪ ♫ montent depuis les fenêtres de la maison, de jour comme de nuit.
- 20 juin et 22 juin → aucune note.
- 21 juin sous la pluie → les notes sont toujours là.
- Petite maison (terminal étroit) et grande maison → les notes partent bien des fenêtres de chacune.
- Lancer le jeu le 21 juin → Badge « Fête de la musique » gagné, visible dans l'Album.
- `weathr --music` → la maison joue de la musique, quelle que soit la date.
- `weathr --demo` → une scène « Fête de la musique » passe, avec son label dans la Barre du haut.
- `Holiday::on` : vrai le 21 juin, faux le 20 et le 22 (test unitaire).

## Implementation

## Review

## Verification

## Feedback
