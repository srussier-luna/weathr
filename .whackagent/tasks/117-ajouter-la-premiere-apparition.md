---
issue: 117
status: todo
wiki: [[animation]], [[render]]
note: "Découpé de #113 (parapluie « Ajouter un secret »), sprint « Le secret ». Scène jouée après la piste du secret (#115), avec le mage de #114."
created: 2026-11-16
---

## Context / Decisions

> Spoiler : ce fichier décrit le secret. Ticket, PR et commits restent vagues (« scène du secret »).

- Blocked by #114 (dessin, vol et invocation du mage, Boîte violette) et #115 (bonne réponse à l'énigme de la Carte de tarot = déclencheur).
- Titre du ticket : « Ajouter la scène du secret » (et non « apparition » : le glossaire réserve **Apparition** aux objets et Habitants qui arrivent au jardin).
- **Invocation** = la scène, jouée **une seule fois**, **juste après la bonne réponse** à l'énigme, quelle que soit l'heure.
- Déroulé (~10 s, au jardin, sur le modèle de l'Accueil `src/welcome.rs`) :
  1. le ciel s'assombrit, étincelles violettes ;
  2. le mage surgit sur son balai (dessin de #114) ;
  3. il fait naître la Boîte violette au-dessus de la boîte jaune (invocation) ;
  4. une bulle : une phrase du mage ;
  5. il s'envole et sort de l'écran ; le jardin reprend.
- Phrase du mage : Claude la rédige, l'utilisateur·rice la relit (`tu`, typographie du projet).
- `Échap` passe la scène ; elle est marquée vue dans `Progress` (`#[serde(default)]`) et ne se rejoue jamais.
- Test : `--arcane` (option cachée de #114) joue la scène une fois au lancement, sans rien enregistrer.
- Pas de Badge, rien dans l'Aide.
- Ligne de glossaire ajoutée avec cette tâche : Invocation (Screens).
- Hors périmètre : passages du soir suivants (#114), piste (#115), Arcanes (#116).

## Acceptance criteria

- Bonne réponse à l'énigme de la Carte de tarot → la scène se joue aussitôt : ciel sombre, étincelles violettes, mage sur son balai, Boîte violette qui apparaît au-dessus de la boîte jaune, bulle, envol.
- `Échap` pendant la scène → elle s'arrête, le jardin reprend, la Boîte violette est là.
- Relancer le jeu ensuite → la scène ne se rejoue pas.
- `weathr --arcane` → la scène se joue une fois au lancement ; la sauvegarde n'est pas modifiée.
- Ancienne sauvegarde → se charge, scène non vue.
- `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings -A clippy::double_must_use`, `cargo test --bin weathr` verts.

## Implementation

## Review

## Verification

## Feedback
