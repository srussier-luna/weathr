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
| Ville | | | `location.city` | ville du joueur, détectée ou choisie dans les Réglages → [[weather]] |
| Nuage | | | `Cloud` | traverse le ciel de gauche à droite ; blanc ciel clair, gris clair Éclaircies, gris foncé sinon → [[animation]] |
| Position auto | détection auto | | `location.auto` | Ville détectée seule : CoreLocationCLI, cache, puis IP → [[weather]] |
| Gazette | journal | | | chaque lundi : prévisions de la semaine, rumeurs, annonces, Énigme |
| Alerte météo | vigilance | | | ligne ⚠ de la Gazette : canicule, grand froid, vent fort, orage ; calculée sur les prévisions |
| Énigme | devinette | | `Riddle` | question de la Gazette, réponse touche `e` |
| Question | choix | | `Choice` | question [1]/[2] d'une Lettre du village ; réponse modifiable dans l'Album → [[mail]] |
| Écho | | | `echoes` | phrase d'une Lettre suivante qui rappelle la réponse à une Question |
| Bouteille | message à la mer | | `bottle` | message dérivant sur la rivière |
| Badge | succès | | `Badge` | récompense : météo, fête, fidélité, commun, rare, épique, secret, contribution → [[mail]] |
| Commun | commune | | `BadgeKind::Common` | rangée des Badges les plus faciles à gagner → [[mail]] |
| Épique | epic | | `BadgeKind::Epic` | rangée des Badges « tout collectionner » : Lettres, objets, Énigmes, Recettes → [[mail]] |
| Contribution | contributeur | | `BadgeKind::Contribution` | rangée des Badges donnés à des personnes (Vigie du ciel, Maître du temps, Plume du village), affichée seulement pour qui en a un → [[mail]] |
| Plume du village | plume d'or, écrivain | | `village_quill` | Badge Contribution de l'auteur des Lettres réécrites (#177) → [[mail]] |
| Boussole | | | `compass` | Badge Commun : Ville trouvée par CoreLocationCLI → [[weather]] |
| Ville visitée | | | `visited_cities` | ville détectée par la position auto ; compte pour Voyageur et Baroudeur |
| Ville de Sam | | | `sam_cities` | ville d'une Carte postale où la position auto est passée (10 km) |
| Palier | | niveau | `tier` | rang du badge de Sam : chocolat, bronze, argent, or, platine |
| Carte de badge | | | `Award` | carte « Nouveau badge ! » ou « Nouveau palier ! » au lancement |
| Trophée | récompense, cadeau | | `Trophy` | objet du jardin offert par un Badge, affiché ou rangé dans l'Album → [[animation]] |
| Nain de jardin | | | | Trophée du Badge « 60 jours d'affilée », à côté de la niche de Biscuit |
| Statue | | | | Trophée du Badge « 100 jours d'affilée », à côté de la niche de Biscuit |
| Cadran solaire | | | | Trophée du Badge « 365 jours d'affilée », devant la clôture |
| Fête | jour férié | | `Holiday` | jour spécial qui décore la scène → [[animation]] |
| Encadré de Fête | | | `HolidayBox` | fenêtre au lancement un jour de Fête : nom et ce qu'il faut guetter → [[render]] |
| Fête de la musique | | | `is_music_day` | 21 juin : notes aux fenêtres, chanteurs, Notes du ciel ; Badge dédié → [[animation]] |
| Chandeleur | | | `is_candlemas` | 2 février : Crêpe géante à la place du soleil et de la lune ; Badge dédié → [[animation]] |
| Crêpe géante | | | `GiantCrepe` | crêpe dorée ou pâle à la place du soleil et de la lune le 2 février ; se retourne |
| 1er mai | | | `is_may_day` | Muguet géant à la place du soleil et de la lune ; Badge dédié → [[animation]] |
| Muguet géant | | | `GiantMuguet` | brin de muguet à la place du soleil et de la lune le 1er mai |
| Notes du ciel | dessin de notes, croches | | `SkyNotes` | croche et deux doubles croches, remplace soleil ou lune → [[animation]] |
| Facteur | postier | | `Postman` | livre les Lettres à la boîte aux lettres |
| Heure du Facteur | heure du courrier | | `mail_hour` | heure de la tournée, de 6 h à 14 h ; changée dans Réglages, effet le lendemain → [[mail]] |
| Jour de jeu | jour | | `StoryClock` | jour de l'histoire depuis le début, change à `courrier.heure` → [[mail]] |
| Timeline | | | `Timeline` | événements de l'histoire classés par Jour de jeu |
| Apparition | déblocage | | `Arrival` | objet ou Habitant qui arrive au jardin un Jour de jeu |
| Habitant | animal | | | animal du jardin (Minouche, Biscuit…), arrive à sa 1re Lettre |
| Chaton | | | `kittens` | les deux petits de Minouche, sur le toit près d'elle (dès le Jour de jeu 33) |
| Recette | | | `Recipe` | Lettre de Rose avec une recette ; tous les 10 jours après sa question → [[mail]] |
| Régime | régime alimentaire | | `diet` | ce que le joueur ne mange pas (8, cumulables) ; filtre les Recettes → [[mail]] |
| Liste à cocher | | | `diet_checklist` | cases [1]…[8] des Régimes, dans la Lettre de Rose et les Réglages |
| Voisin | | | | personne d'une autre maison du village (Sam, Nadia et Claire…) |
| Tic de langage | expression | | | mot propre à un personnage, glissé dans ses Lettres, jamais en signature (« tchic » de Noisette) → [[letters]] |
| Arbre | | | `Tree` | grand arbre à gauche de la maison, pousse du jour 1 au jour 12 |
| Puits | | | `well` | vieux puits réparé, à gauche du grand Arbre |
| Coffre | | | `chest` | coffre de Jeanne, à gauche du Puits |
| Traces de boue | pas | | `footprints` | `:` marron de l'Arbre à la porte, Jours de jeu 8, 11 et 16 |
| Traces mouillées | pas | | `wet_footprints` | `.` bleus du Puits à la porte, nuits 13 et 30 |
| Clé | | | `key` | clé `o-m` jaune près de la boîte aux lettres, dès le Jour de jeu 17 |
| Cabane | | | `treehouse` | cabane de Léo, dans le feuillage de l'Arbre |
| Perséides | | | | étoiles filantes plus fréquentes la nuit, du 9 au 15 août ; Badge dédié |
| Géminides | | | | étoiles filantes plus fréquentes la nuit, du 10 au 16 décembre ; Badge dédié |
| Boîte pleine | | | | 7 Lettres du village non lues ; l'histoire se met en pause |
| Tricheur | anti-Badge | | `cheater` | anti-Badge de la 1re triche, gardé pour toujours → [[mail]] |
| Village dégarni | | | `bare_village` | Habitants partis, Arbre sans feuilles : punition de triche |
| Jours sans courrier | | | `mail_ban` | 3e triche et suivantes : Jour de jeu figé, rien livré |
| Chevron de courrier | | | `render_with_chevrons` | `«`/`»` animés au bord d'une Lettre ou de l'Album : il reste à voir → [[render]] |
| Ruche | | | | cadeau de Nadia et Claire, entre le sapin et la mare |
| Mare | flaque | | `puddle` | petite eau à droite du jardin, chez Coin-Coin et Rainette ; gèle en grand froid → [[scene]] |
| Baignade | | | `Bathing` | Biscuit va à la Mare, s'éclabousse puis s'ébroue ; glisse si elle est gelée → [[animation]] |
| Poulailler | | | | abri des poules la nuit et sous la pluie, au jardin et au Potager |
| Carré | parcelle | case | | une des 9 planches du Potager : vide, plantée, fanée, mûre |
| Jour de plante | | | | jour où weathr est ouvert ; fait pousser et faner les plantes du Potager |
| Mise à jour forcée | force update | | `min_version` | mise à jour installée sans question quand la version locale est plus vieille que `[package.metadata.weathr] min_version` du `Cargo.toml` distant → [[updater]] |
| Sauvegarde | export | | `backup` | fichier écrit par `--export` : progression + réglages, relu par `--import` |
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
| Potager | | | scène du potager vue de dessus en biais, sans ciel ; pousse par Jour de jeu, suit les saisons |
| Fête du miel | scène de fête | | maison de Nadia et Claire, Jours de jeu 119–120 seulement |
| Lieux | menu des lieux | | menu touche m : Jardin, Potager, Fête du miel disponibles |
| Accueil | onboarding | `welcome` | Facteur arrive, bulle, note dépliée, bulle de départ ; une fois par joueur |
| Mot des voisins | | `neighbours_note` | fenêtre à l'ouverture : triche repérée, punition, voisins déçus |
| Réglages | | `settings_view` | écran ouvert par `s` : unités, Heure du Facteur, Barres, Ville, Position auto → [[architecture]] |
<!-- | Login screen | login page | `LoginView` | email + password, forgot-password link | -->

## Parts

A named piece of UI, reused or talked about on its own. UI projects only.

| Name | Also | Code | On | Means |
|---|---|---|---|---|
| Barre du haut | weather bar, HUD, ligne d'état | `weather_bar` | scène | météo en haut : ciel, température, vent, pluie, alertes ; heure, Ville • Jour → [[render]] |
| Barre du bas | toolbar | `toolbar` | scène | touches en bas à droite ; Lettre !, Objet ! clignotent ; masquée avec la Barre du haut → [[render]] |
| Toast | | `toasts` | scène | texte bref centré dans un cadre au-dessus de la Barre du bas, tapé lettre par lettre, 4 s, un à la fois → [[render]] |
<!-- | Composer | editor | `ComposerView` | Inbox, Thread | writes or replies to a message | -->

## Flows

Screens chained for one goal. Steps name **Screens**. UI projects only.

| Name | Also | Steps | Means |
|---|---|---|---|
<!-- | Login flow | sign-in | Welcome → Login screen → 2FA → Inbox | first sign-in, until Inbox | -->
