# Glossary

> whackagent convention module. One word per thing, same word everywhere: UI strings, conversation, tasks, reports, PRs, wiki, code. Yours to edit — every skill and both agents read it. Rules below are plugin text, refreshed by `/wa-update`.

## Rules

- **One thing, one term.** Listed → use it. Never a word from `Never`.
- **Also** = words people say for it. Understand them, never write them.
- **Code** = its name in source. New type, file, variable for it → that name. Existing code named otherwise → never rename unasked.
- **Means** = one line, ≤ 12 words. Detail → `→ [[wiki page]]`.
- **Missing term** → propose the row, add on yes. Never coin a second word for a listed thing.
- Short: what users see or the team argues about. Not a dictionary.
- Review: UI string with a `Never` word → `correctness`. New type or variable off `Code`, or a `Never` word in code → `style`.

## Concepts

Product objects and notions, their states, roles, plans. Users see these: one column per UI language, source first.

| fr | Also | Never | Code | Means |
|---|---|---|---|---|
| Lettre | courrier | Message | `Entry` | courrier daté d'un habitant, livré par le Facteur |
| Carte postale | carte | | `City` | de Sam en voyage, une ville par carte |
| Gazette | journal | | | chaque lundi : rumeurs, annonces, Énigme |
| Énigme | devinette | | `Riddle` | question de la Gazette, réponse touche `e` |
| Bouteille | message à la mer | | `bottle` | message dérivant sur la rivière |
| Badge | succès | | `Badge` | récompense : météo, fête, rare, fidélité, secret |
| Fête | jour férié | | `Holiday` | jour spécial qui décore la scène |
| Facteur | postier | | `Postman` | livre les Lettres à la boîte aux lettres |
| Habitant | animal | | | animal du jardin (Minouche, Biscuit…), arrive à sa 1re Lettre |
| Voisin | | | | personne d'une autre maison du village (Sam, Nadia et Claire…) |
| Arbre | | | `Tree` | grand arbre à gauche de la maison, pousse du jour 1 au jour 12 |
| Perséides | | | | étoiles filantes plus fréquentes la nuit, du 9 au 15 août ; Badge dédié |
| Géminides | | | | étoiles filantes plus fréquentes la nuit, du 10 au 16 décembre ; Badge dédié |
| Boîte pleine | | | | 7 Lettres du village non lues ; l'histoire se met en pause |
| Ruche | | | | cadeau de Nadia et Claire, entre le sapin et la mare |
<!-- | Inbox | mailbox | Messages | `Inbox` | received mail, not archived, all accounts | -->
<!-- | Message | mail | Email | `Message` | one mail. States: Draft, Sent, Pending | -->

## Screens

One precise screen. Team names, in the discussion language. UI projects only.

| Name | Also | Code | Means |
|---|---|---|---|
| Courrier | boîte aux lettres, index | `index_view` | liste des Lettres reçues |
| Lettre | | `entry_view` | une Lettre ouverte, numéro / total |
| Album | | `album_pages` | Badges et souvenirs collectés |
| Aide | help | `help_view` | touches disponibles selon déblocages |
| Potager | | | scène du potager, touche p ; pousse par Jour de jeu, suit les saisons |
<!-- | Login screen | login page | `LoginView` | email + password, forgot-password link | -->

## Parts

A named piece of UI, reused or talked about on its own. UI projects only.

| Name | Also | Code | On | Means |
|---|---|---|---|---|
<!-- | Composer | editor | `ComposerView` | Inbox, Thread | writes or replies to a message | -->

## Flows

Screens chained for one goal. Steps name **Screens**. UI projects only.

| Name | Also | Steps | Means |
|---|---|---|---|
<!-- | Login flow | sign-in | Welcome → Login screen → 2FA → Inbox | first sign-in, until Inbox | -->
