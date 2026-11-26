# Letters — voix des personnages

Reference for any Lettre text (village, correspondents, n°13, future ones). One sheet per character: voix, Tic de langage, ne dit jamais, one example. Then the common rules. Source: proto [[protos/changer-le-ton-des-lettres-personas]] (v3 validated, task #173). Mechanics of delivery: [[mail]].

## Where the texts live

- Village Lettres: `DEFAULT_LETTERS` (`src/story/content/letters.rs`), 60 `(sender, body)`. Sender string never changes (Courrier, guests and tests key on it). Body `\n` = new paragraph; game adds `— sender` itself, so no signature in the body.
- Texts before #173: `PREVIOUS_LETTERS` (`src/story/content/previous_letters.rs`). Only for `lettres.txt` migration: a file letter equal to its previous text takes the new one; a player edit stays. Rewriting texts again → move the current ones there first.
- Correspondents (Sam, Nadia et Claire, Marcel), n°13: `chronicles.rs`, `CREEPY_LETTERS`. Rewritten to these sheets (#174); n°13 + night P.-S. kept as model.
- French typography: `\u{202F}` before `? ! : ;` and inside `« »`. Exception: Énigme texts stay byte-identical.

## Common rules

1. Write like people talk. Short sentences, oral per voice: « sonne pas », « y a du courrier ».
2. No user manual. A game mechanic is said once, in the character's words: « drapeau levé, y a du courrier ».
3. No moral, no fairy-tale formula. Cut « C'est la tradition du village. », « ça ne coûte rien », « la tienne continue ».
4. Vary openings. Onomatopoeia (Wouf, Miaou) one Lettre in three at most. A Lettre may start mid-thought.
5. Vary length. Two lines for Gustave, six for Gaston. Never all alike.
6. One concrete detail per Lettre: a name, an object, a number (« 147 noisettes », « grand-tante Lucienne ») over a generality.
7. Characters talk about each other, with an opinion: Minouche despises Biscuit, Coin-Coin and Rainette bicker.
8. Tic de langage slipped inside a sentence. Never a signature, never the last words. Once per Lettre at most, not in every Lettre (a 2-Lettre character: one only).
9. No list of three in every sentence. The three-beat rhythm sounds machine-written. Odile's agenda is the exception.
10. L'encre bleue Énigmes never change: text and answer. Cadeaux, Questions `[1]`/`[2]` and their Échos stay in their Lettre.

Plus:

- Question Lettre: the body leads to the prompt shown after the signature (`CHOICES`, unchanged). Écho Lettre: the echo line is appended after the body, so the last sentence must let it follow (« Son toit… » right after the puits).
- Story clues stay where they are: lanterne bleue, fenêtre en haut à gauche, traces, lumière bleue, numéro 13, fautes de Paulo.
- Postscript is written « P.-S. », like `NIGHT_POSTSCRIPTS`. No em-dash as separator. Exclamation marks only where the voice wants them (Rose, Léo, Biscuit).

## Rose — la boulangère

- Voix: deaf, writes like on her notepad, short and direct. Warm without fuss, talks butter and oven. Often ends on a greedy P.-S.
- Tic: « hein »
- Ne dit jamais: long polite sentences; complaints about her deafness.

> Bienvenue au bout du chemin ! Moi c'est Rose, la boulangère. Le matin, si ça sent le beurre chaud devant chez toi, c'est que je passe avec mon panier. Sonne pas, hein, je suis sourde : un signe par la fenêtre, c'est parfait. Gaston t'apportera une lettre par jour, il adore ça.
> P.-S. Je t'ai gardé un croissant. Le moins brûlé.

## Gaston — le facteur

- Voix: chatty, digresses then catches himself. Thirty years on a bike, knows every rumour. Familiar, oral (« y a », « je te jure »).
- Tic: « Bref. »
- Ne dit jamais: straight to the point; a two-line Lettre.
- « franchement » belongs to Léo: Gaston does not use it.

> Trente ans de tournée, et je vais pas te mentir : ta boîte aux lettres, c'est ma préférée. Elle grince comme la porte de ma grand-tante Lucienne, paix à son âme. Bref. Quand le petit drapeau rouge est levé, y a du courrier. Baissé, y en a pas. C'est pas sorcier, mais tu serais surpris du nombre de gens qui regardent jamais.

## Paulo — le pêcheur

- Voix: taciturn, very short sentences. Forgets apostrophes (`Javais`, `tai`, `cest`, `lencre`), because Jeanne later says he writes « avec des fautes d'orthographe, comme avant ». No other mistake (accents, capitals, agreement all right), so nobody thinks it is a bug. Shows feelings, never names them.
- Tic: « voilà » (with its accent: no other mistake)
- Ne dit jamais: a complicated word; a sentence over ten words.

> Javais trop de bois flotté dans la cabane. Alors je tai fait un banc, voilà. Il est devant chez toi. Le soir on voit le soleil tomber dans la rivière. Cest beau.

## Odile — la maire

- Voix: official tone overflowing with good mood. Talks « conseil municipal », « ordre du jour ». Deadpan about her own seriousness.
- Tic: « En somme »
- Ne dit jamais: familiarities (« salut »); admitting she does not know.

> Le conseil municipal (c'est-à-dire moi, Rose, et Gaston quand il a le temps) a voté à l'unanimité : le village fera sa grande fête cette année. À l'ordre du jour : le vieux puits à réparer, des décorations partout, et de la musique. Ta maison est au cœur du projet. En somme, ce n'est pas une faveur, c'est une réquisition amicale.

## Léo — le petit voisin

- Voix: eight, enthusiastic, chains with « et ». Direct questions, double question mark. Talks about Mamie all the time.
- Tic: « franchement »
- Ne dit jamais: adult vocabulary; polite formulas.

> Salut c'est Léo ! Celui qui saute dans tes flaques. Pardon pour la boue, mais franchement tes flaques sont les meilleures du village. Mamie dit que ta maison c'était la sienne avant. C'est vrai ?? Je peux continuer à venir jouer ? Dis oui.

## L'encre bleue — avant la révélation

- Voix: brief, mysterious, never says who she is. Short sentences, soft imperatives. Énigmes unchanged.
- Tic: « Regarde bien. »
- Ne dit jamais: her name; a funny sentence.

> Autrefois, quelqu'un posait une lanterne bleue à ta fenêtre. Un pêcheur rentrait tard. La lumière le ramenait. Regarde bien la fenêtre du haut.

## Jeanne — l'encre bleue, après la révélation

- Voix: tender, a little aged, careful sentences. Reveals herself at Lettre 38, not before.
- Tic: « mon petit »
- Ne dit jamais: abbreviations, loose oral; everything at once.

> Tu as sûrement deviné, mon petit. Je m'appelle Jeanne, je suis la mamie de Léo. J'écris à l'encre bleue parce que c'est la seule qui ne bave pas sur mon vieux papier. Paulo et moi, on s'est aimés, il y a longtemps. Et puis la vie a fait ce qu'elle fait.

## Minouche — la chatte du toit

- Voix: haughty, thinks she is an astrologer (stars, pendulum). Despises Biscuit with elegance. No signature in the body: the game signs for her.
- Tic: « évidemment »
- Ne dit jamais: an apology; admitting she loves someone.

> Rose m'a offert un nœud. Les astres l'avaient prédit, évidemment : je suis officiellement la chatte la plus élégante du village. Biscuit est jaloux. Biscuit ne lit pas les étoiles. Biscuit lit à peine sa gamelle. Regarde mon oreille. Longuement.

## Biscuit — le chien

- Voix: excited, choppy sentences, capitals. Proud of nothing, forgets what he was saying. Wouf at the start or the end, never both.
- Tic: « trop bien »
- Ne dit jamais: subtlety; subordinate clauses.

> J'ai trouvé un os. Un ÉNORME os. Trop bien. En creusant près de ma niche. Je l'ai laissé dehors pour que tout le monde le voie. Personne n'y touche. Normal. Tu l'as vu ? Il est beau. Wouf.

## Hulotte — le hibou

- Voix: slow, calm, watches humans from above. Calm sentences, dry endings. Finds humans « compliqués ».
- Tic: « Hou. »
- Ne dit jamais: exclamations; hurrying.

> La nuit, je veille dans le grand arbre. Je vois passer les chauves-souris, les étoiles filantes, les rêves des chats. Et parfois, une petite lumière bleue. Hou. Ce n'est pas une étoile. Les étoiles ne clignent pas comme ça.

## Coin-Coin — le canard

- Voix: proud of his mare, bad faith, false modesty. Rivalry with Rainette.
- Tic: « je dis ça, je dis rien »
- Ne dit jamais: a compliment to Rainette.

> Ta mare, quand il pleut, c'est la plus belle de la région. Je ne dis pas ça parce que j'y vis. Bon, un peu. Je la partage avec la grenouille. Elle chante faux, je dis ça, je dis rien. Même les têtards ont déménagé.

## Rainette — la grenouille

- Voix: diva, thinks she is an artist. Answers the duck. Onomatopoeia in capitals.
- Tic: « naturellement »
- Ne dit jamais: a doubt about her talent.

> Je ne chante pas faux, naturellement. C'est le canard qui n'a pas d'oreille. Quand il pleut, je saute dans la mare : PLOUF. Mon grand numéro.

## Noisette — l'écureuil

- Voix: scatterbrained, corrects herself mid-sentence. Possessive with her nuts. « Tchic » inside the Lettre, never as the last word.
- Tic: « tchic »
- Ne dit jamais: remembering something.

> J'ai caché 147 noisettes dans ton arbre. Tchic. Ou 148. Je sais plus où. Si t'en trouves une, elle est à moi. Si t'en trouves deux, elles sont à moi aussi.

## Gustave — l'escargot

- Voix: slow, never in a hurry. Very short Lettres. Puts everything off.
- Tic: « rien ne presse »
- Ne dit jamais: a long Lettre; a precise date.

> Parti de la mare il y a trois jours. J'arriverai à ta boîte aux lettres un jour, rien ne presse. Commence à dîner sans moi.

## Le voisin du n°13 — celui qui n'existe pas

- Voix: polite, calm, never threatening: that is what scares. Knows intimate details of your house. Numbers too precise, often 13.
- Tic: « C'est normal. »
- Ne dit jamais: an exclamation; a direct threat; an explanation.
- Letters (`CREEPY_LETTERS`) reviewed in #174, unchanged: the model voice. Plain spaces before ? : (no `\u{202F}`).

> Bonjour. J'habite au numéro 13, juste derrière chez toi. Tu ne vois pas ma maison d'ici. C'est normal. Je voulais simplement te dire que la lumière de ta cuisine est très jolie, le soir. Surtout vers 22 h 13.

## Sam — le voisin du moulin, en voyage

- Voix: dreamer, former cartographer. Talks places, roads, crosses on the map. Asks questions without waiting for the answer.
- Tic: « tu savais ? »
- Ne dit jamais: a complaint; what he left behind.
- Letters rewritten in #174. Cartes postales keep their voice.

> Bonjour d'en face ! Moi c'est Sam, le vieux moulin au bord de la rivière. Je dessinais des cartes, avant. Maintenant je les regarde, surtout. Tu savais ? Ton jardin est sur trois de mes plans.

## Nadia et Claire — les apicultrices

- Voix: a duo writing « on », teasing each other on the page. Claire rushes, Nadia doubts. Tender, funny, concrete (hives, honey).
- Tic: « Claire dit… Nadia dit… »
- Ne dit jamais: a single voice; agreeing on everything.
- Letters rewritten in #174.

> Coucou ! Nadia et Claire, les nouvelles de la maison derrière le verger. On est arrivées avec quatre ruches (Claire en voulait six). Si tu vois des abeilles chez toi, ce sont les nôtres. Elles adorent ton arbre : Claire dit qu'elles ont bon goût, Nadia dit qu'elles sont juste gourmandes. Le premier pot est pour toi.

## Marcel — le garde champêtre

- Voix: writes like a patrol report: time, facts, telegraphic. Nervous, denies what he just said. Twenty-two years of patrols.
- Tic: « Rien à signaler. »
- Ne dit jamais: admitting he is scared; a long sentence.
- Letters rewritten in #174.

> Ronde de 0 h 40. Boue près du puits, encore. Rien à signaler. Des pas vers ta maison, aucun qui revient. Sûrement les miens. J'ai dû mal compter.

## Le village — la boîte Nouveautés

Not a Lettre: the in-game release notes (`NOUVEAUTES.md`, see [[updater]]). Factual GitHub notes stay outside this sheet (see `wording.md`).

- Voix: neighbours passing news over the hedge. One line per novelty, « tu », a wink at the end. Puts the player on the trail, never tells everything.
- Tic: « Ouvre l'œil » — once per note at most.
- Ne dit jamais: technical jargon (correctif, bug, version, optimisation); spoilers (hidden mechanic, object, place or Lettre the player may not have seen).
- One exclamation mark per note at most.

> De nouvelles petites bouilles pourraient bien s'installer au jardin. Ouvre l'œil !
> Le grand Arbre a des envies de grandeur. Laisse-lui le temps.
