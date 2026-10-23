---
issue: 7
status: todo
wiki: [[render]], [[mail]]
note: "Grill #4 23/10 : ajoute aussi une tâche, design de la lettre et événement badge reçu"
created: 2026-10-23
---

## Context / Decisions

- One frame style per window, so each kind is recognized at a glance. One ticket on purpose (user's call), even if large.
- Unchanged: Aide (`help_view`) and Courrier (`index_view`).
- Frames (sketches, text wraps inside, same max width as today):
  - Lettre and the `Nouvel habitant` card (#4): torn-edge parchment, old-paper color.
    ```
      .~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~.
     (  Lettre 9/12                     )
      )  Wouf ! C'est moi qui dors…    (
     (                       — Biscuit  )
      '~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~'
    ```
  - Gazette: newspaper, `====` rules, title line `||   LA GAZETTE · lundi 17 août   ||`, a `----` rule before the Énigme.
  - Carte postale: stamp box `[~]` in the top-right corner.
  - Archives: folder tab with `Archives` on the top-left edge.
  - Recette: punched card, `o|` holes down the left edge.
  - Bouteille: waves `~^~^~^` on top and bottom, no side borders.
  - Album: photo corners `/----\` and `\----/`.
  - Badges: star border `*~*~*~*`.
- Each style gets its own color; Lettre = old paper (~ rgb 222,196,140).
- New Badge = big centered card instead of the row-0 toast:
  - Round medal without ribbon (style A), the Badge's own drawing inside, then `Nouveau badge !` and the Badge name.
  - Secret Badge: its condition under the name (same text as the Badges window).
  - `[Echap]` or `[Entrée]` closes it. Several Badges at once → one card each, in award order.
  - Opens only when no other window is open; waits otherwise.
  - Forced modes still award no Badge, so no card there.
- Medal template (icon = 3 lines × 5 chars):
  ```
     .-"""-.
    / 11111 \
    | 22222 |
    \ 33333 /
     '-...-'
  ```
- One drawing per Badge (validated gallery, line 1 / 2 / 3):
  ```
  Ciel dégagé         " \|/ " "--O--" " /|\ "
  Éclaircies          "\|/  " "-O.-." " (__)"
  Nuageux             "  _  " " ( ) " "(___)"
  Couvert             " _ _ " "(_)_)" "(___)"
  Brouillard          "~~~~~" " ~~~ " "~~~~~"
  Bruine              "(___)" " ' ' " "' ' '"
  Pluie               "(___)" " / / " "/ / /"
  Pluie verglaçante   "(___)" " /*/ " "*/ /*"
  Averses             "(___)" "/////" "/////"
  Neige               "  *  " "* * *" "  *  "
  Grésil              "(___)" " .:. " ".:.:."
  Averses de neige    "(___)" "* * *" " * * "
  Orage               "(___)" " _/  " " /   "
  Orage de grêle      "(___)" " _/ o" "o/ o "
  Noël                "  *  " " /^\ " "/___\"
  14 juillet          "\ * /" "* + *" "/ * \"
  Halloween           " _|_ " "(^ ^)" "(\_/)"
  Nouvel An           " \ / " "  |  " " _|_ "
  Pâques              " .-. " "(~~~)" " '-' "
  Arc-en-ciel         " ___ " "/ _ \" "|/ \|"
  Étoile filante      "    *" " .*' " "*'   "
  Facteur             " .-. " "|__|>" "  |  "
  Bonhomme de neige   " _|_ " " (o) " "(   )"
  Bouteille à la mer  "  _  " " | | " "~[_]~"
  3 jours d'affilée   "[===]" "| 3 |" "|___|"
  7 jours d'affilée   "[===]" "| 7 |" "|___|"
  30 jours d'affilée  "[===]" "|30 |" "|___|"
  Toutes les lettres  "_____" "|\_/|" "|___|"
  Première énigme     " .-. " "  .' " "  .  "
  Dix énigmes         "?   ?" " 10  " "?   ?"
  Tous les objets     "_____" "|_$_|" "|___|"
  Oiseau de nuit      " ___ " "(o,o)" "{`\"`}"
  Lève-tôt            " \|/ " "--O--" "_____"
  Chasseur d'étoiles  "*  * " "  *  " " *  *"
  Collection complète " _ _ " "|\|/|" "|___|"
  Patience            " ___ " " \:/ " " /.\ "
  Lecture nocturne    "  ,  " " |=| " " |_| "
  ```
- Other toasts (found object, Énigme result) stay on row 0.
- #4 (`Nouvel habitant` card) and this task are independent: whichever lands second applies the Lettre frame to the card.

## Acceptance criteria

- [ ] Opening a Lettre shows the torn-edge frame: `.~~~.` top, sides alternating `(` / `)`, `'~~~'` bottom.
- [ ] Gazette shows `====` rules and `|| LA GAZETTE · <date> ||`; Carte postale shows the `[~]` stamp top-right; Archives shows the `Archives` tab; Recette shows `o|` holes; Bouteille shows `~^~^` waves; Album shows `/--\` corners; Badges shows the `*~*~*` border.
- [ ] Aide (`h`) and Courrier (`l`) look exactly as before.
- [ ] Earning `Pluie` opens a centered card: medal with `(___)` / ` / / ` / `/ / /` inside, `Nouveau badge !`, `Pluie`; no row-0 badge toast; `[Echap]` closes it.
- [ ] Earning `Patience` shows its condition `weathr ouvert 1 h d'affilée` on the card.
- [ ] Two Badges earned in the same frame → two cards, one after the other.
- [ ] A Badge earned while a Lettre is open → its card opens right after the Lettre is closed.
- [ ] At 80 columns every frame is fully visible and text stays inside it.

## Implementation

## Review

## Verification

## Feedback
