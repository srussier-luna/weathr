use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const FILE_HEADER: &str = "# Lettres de weathr : une nouvelle lettre arrive chaque jour.
# Chaque lettre commence par une ligne \"=== numéro | expéditeur\".
# Tu peux réécrire les textes librement ou ajouter des lettres après la dernière.
";

pub const VILLAGE_LETTER_COUNT: usize = 60;

const DEFAULT_LETTERS: [(&str, &str); VILLAGE_LETTER_COUNT] = [
    (
        "Rose, la boulangère",
        "Bienvenue au bout du chemin\u{202F}! Moi c'est Rose, la boulangère. Le matin, si ça sent le beurre chaud devant chez toi, c'est que je passe avec mon panier. Sonne pas, hein, je suis sourde\u{202F}: un signe par la fenêtre, c'est parfait. Gaston t'apportera une lettre par jour, il adore ça.\nP.-S. Je t'ai gardé un croissant. Le moins brûlé.",
    ),
    (
        "L'encre bleue",
        "Tu ne me connais pas. Moi, je connais ta maison. Chaque pierre, la marche qui grince, la fenêtre qui ferme mal. Elle garde un secret depuis longtemps. Si tu veux le trouver, lis mes lettres. Toutes.",
    ),
    (
        "Minouche, la chatte du toit",
        "Le toit de ta maison est à moi depuis six hivers. Je te le prête. De là-haut, je lis les étoiles\u{202F}: ce mois-ci, elles te sont favorables, ce qui m'étonne un peu. Quand il pleut, j'entre par ta fenêtre. Inutile de la fermer, je sais l'ouvrir.",
    ),
    (
        "Paulo, le pêcheur",
        "Javais trop de bois flotté dans la cabane. Alors je tai fait un banc, voilà. Il est devant chez toi. Le soir on voit le soleil tomber dans la rivière. Cest beau.",
    ),
    (
        "L'encre bleue",
        "Première énigme : je suis la plus vieille habitante du jardin, j'étais là avant la maison, et chaque automne je perds mes cheveux. Qui suis-je ?",
    ),
    (
        "Minouche, la chatte du toit",
        "Rose m'a offert un nœud. Les astres l'avaient prédit, évidemment\u{202F}: je suis officiellement la chatte la plus élégante du village. Biscuit est jaloux. Biscuit ne lit pas les étoiles. Biscuit lit à peine sa gamelle. Regarde mon oreille. Longuement.",
    ),
    (
        "Odile, la maire",
        "Le conseil municipal (c'est-à-dire moi, Rose, et Gaston quand il a le temps) a voté à l'unanimité\u{202F}: le village fera sa grande fête cette année. À l'ordre du jour\u{202F}: le vieux puits à réparer, des décorations partout, et de la musique. Ta maison est au cœur du projet. En somme, ce n'est pas une faveur, c'est une réquisition amicale.",
    ),
    (
        "L'encre bleue",
        "Bravo, c'était le grand arbre. Cette nuit, quelqu'un est passé dans ton jardin. Regarde le sol, près des racines. Les traces vont quelque part.",
    ),
    (
        "Biscuit, le chien",
        "La niche, c'est chez moi. Cette nuit j'ai rien vu. Je dormais. Enfin, presque. J'ai entendu des petits pas. Tout légers. Et après, un rire. Un rire de quelqu'un de PETIT.",
    ),
    (
        "Gaston, le facteur",
        "Trente ans de tournée, et je vais pas te mentir\u{202F}: ta boîte aux lettres, c'est ma préférée. Elle grince comme la porte de ma grand-tante Lucienne, paix à son âme. Bref. Quand le petit drapeau rouge est levé, y a du courrier. Baissé, y en a pas. C'est pas sorcier, mais tu serais surpris du nombre de gens qui regardent jamais.",
    ),
    (
        "L'encre bleue",
        "Ces traces sont petites. Celui qui les a laissées grimpe aux arbres et saute dans les flaques. Il en sait plus qu'il ne le dit.",
    ),
    (
        "Hulotte, le hibou",
        "La nuit, je veille dans le grand arbre. Je vois passer les chauves-souris, les étoiles filantes, les rêves des chats. Et parfois, une petite lumière bleue. Hou. Ce n'est pas une étoile. Les étoiles ne clignent pas comme ça.",
    ),
    (
        "Odile, la maire",
        "Communiqué de la mairie\u{202F}: le vieux puits de ton jardin est réparé. Paulo a remonté les pierres, Gaston a fourni la corde, et moi j'ai supervisé, ce qui est le plus fatigant. Autrefois, on y faisait des vœux. Je ne te l'impose pas. Je ne te l'interdis pas non plus. Le puits est à toi.",
    ),
    (
        "L'encre bleue",
        "Le puits, oui. Il y a longtemps, quelqu'un y a jeté une pièce. Le vœu tenait en quelques mots\u{202F}: que cette maison reste heureuse. Ce vœu, c'est moi qui l'ai fait.",
    ),
    (
        "Biscuit, le chien",
        "J'ai trouvé un os. Un ÉNORME os. Trop bien. En creusant près de ma niche. Je l'ai laissé dehors pour que tout le monde le voie. Personne n'y touche. Normal. Tu l'as vu\u{202F}? Il est beau. Wouf.",
    ),
    (
        "Léo, le petit voisin",
        "Salut c'est Léo\u{202F}! Celui qui saute dans tes flaques. Pardon pour la boue, mais franchement tes flaques sont les meilleures du village. Mamie dit que ta maison c'était la sienne avant. C'est vrai\u{202F}?? Je peux continuer à venir jouer\u{202F}? Dis oui.",
    ),
    (
        "L'encre bleue",
        "J'ai laissé une petite clé sous ta boîte aux lettres. Garde-la. Elle n'ouvre aucune porte de la maison.",
    ),
    (
        "Coin-Coin, le canard",
        "Ta mare, quand il pleut, c'est la plus belle de la région. Je ne dis pas ça parce que j'y vis. Bon, un peu. Je la partage avec la grenouille. Elle chante faux, je dis ça, je dis rien. Même les têtards ont déménagé.",
    ),
    (
        "Rose, la boulangère",
        "Léo m'a commandé un gâteau «\u{202F}pour fêter la cabane\u{202F}». En langue des signes, s'il te plaît\u{202F}! Il a appris ça tout seul, ce petit. Une cabane\u{202F}? Dans le grand arbre, je parie. Ils croient que je vois rien. Je suis sourde, pas aveugle.",
    ),
    (
        "L'encre bleue",
        "La clé et le grand arbre vont ensemble. Pas tout de suite. Il faut attendre, longtemps. Un escargot est déjà en route vers ta boîte aux lettres. Il arrivera avant la suite.",
    ),
    (
        "Gustave, l'escargot",
        "Parti de la mare il y a trois jours. J'arriverai à ta boîte aux lettres un jour, rien ne presse. Commence à dîner sans moi.",
    ),
    (
        "Paulo, le pêcheur",
        "Avec Léo on a fait une cabane. Dans le grand arbre. Une échelle. Un toit. Une fenêtre sur la rivière. Jai pas le droit dy monter. Cest Léo qui la dit. Cest notre cabane quand même.",
    ),
    (
        "L'encre bleue",
        "Autrefois, quelqu'un posait une lanterne bleue à ta fenêtre. Un pêcheur rentrait tard. La lumière le ramenait. Regarde bien la fenêtre du haut.",
    ),
    (
        "Hulotte, le hibou",
        "Mon cousin le grand-duc est arrivé hier soir. Il trouve ton arbre confortable. Il reste. Il ne m'a pas demandé mon avis. Il a pris la branche du haut, celle que je préférais.",
    ),
    (
        "Gaston, le facteur",
        "Quelle saucée ce matin. Mon vélo flottait presque, et ton canard m'a doublé, je te jure, avec un petit air supérieur. Ça m'a rappelé l'hiver où j'avais fini la tournée dans les bottes de pêche de Paulo. Bref. Ta boîte aux lettres était déjà ouverte quand je suis arrivé. Toute mouillée à l'intérieur. Comme si quelqu'un avait mis la main dedans avant moi.",
    ),
    (
        "L'encre bleue",
        "Ce soir, la petite fenêtre en haut à gauche. Regarde-la. La lanterne bleue est revenue. Quelqu'un pense encore au pêcheur.",
    ),
    (
        "Noisette, l'écureuil",
        "J'ai caché 147 noisettes dans ton arbre. Tchic. Ou 148. Je sais plus où. Si t'en trouves une, elle est à moi. Si t'en trouves deux, elles sont à moi aussi.",
    ),
    (
        "Odile, la maire",
        "Point suivant de l'ordre du jour\u{202F}: les décorations. Chaque maison en fournit une, c'est voté. Les guirlandes, c'est réglé avec Rose, qui a dit oui avant la fin de ma phrase. Reste ta façade, et sur ce point le conseil tient à recueillir ton avis.",
    ),
    (
        "L'encre bleue",
        "Le pêcheur s'appelle Paulo. Il ne sait pas que c'est moi qui t'écris. Il ne doit pas le savoir. Pas encore.",
    ),
    (
        "Rainette, la grenouille",
        "Je ne chante pas faux, naturellement. C'est le canard qui n'a pas d'oreille. Quand il pleut, je saute dans la mare\u{202F}: PLOUF. Mon grand numéro.",
    ),
    (
        "Rose, la boulangère",
        "Léo et moi, on a accroché une guirlande lumineuse sous ton porche. Il tenait l'escabeau, moi je tenais Léo. Elle s'allume à la tombée du jour. On la voit depuis la route, même Gaston s'est arrêté pour la regarder.",
    ),
    (
        "L'encre bleue",
        "Cinquante ans. C'est le temps que j'ai passé dans ta maison. J'y ai été heureuse. En partant, j'ai laissé quelque chose pour celui ou celle qui viendrait après moi.",
    ),
    (
        "Minouche, la chatte du toit",
        "J'ai eu deux chatons. Une nuit de pleine lune, évidemment, ce qui en fait des chatons très sages. L'un a une tache en forme d'étoile sur le front. Biscuit fait semblant de ne pas les aimer. Il les a reniflés onze fois.",
    ),
    (
        "Léo, le petit voisin",
        "Ma mamie elle s'appelle Jeanne et elle habite la maison de retraite au bord de la rivière. Et quand je vais la voir elle me pose plein de questions sur ta maison. Sur le grand arbre et sur le puits. Même sur la fenêtre du haut\u{202F}! Pourquoi\u{202F}?? C'est juste une fenêtre.",
    ),
    (
        "L'encre bleue",
        "J'ai tracé une croix au pied du grand arbre. Ne creuse pas. Pas encore. Attends ma dernière lettre.",
    ),
    (
        "Biscuit, le chien",
        "Une croix ROUGE. Par terre. Près de l'arbre. Trop bien, une croix\u{202F}! Je voulais creuser. Mais Minouche m'a regardé. Avec ses yeux. J'ai pas creusé. Presque pas.",
    ),
    (
        "Gaston, le facteur",
        "Je vais te dire un truc, mais tu le répètes pas. Les lettres à l'encre bleue ont un timbre de la maison de retraite. Je reconnais le tampon, c'est le même que pour la tante d'Odile. Les autres, celles sans timbre, je sais pas qui les dépose. Pas moi, en tout cas. Et je distribue rien au numéro 13. Y a pas de numéro 13. Trente ans que je compte les boîtes, y en a douze.",
    ),
    (
        "Jeanne, l'encre bleue",
        "Tu as sûrement deviné, mon petit. Je m'appelle Jeanne, je suis la mamie de Léo. J'écris à l'encre bleue parce que c'est la seule qui ne bave pas sur mon vieux papier. Paulo et moi, on s'est aimés, il y a longtemps. Et puis la vie a fait ce qu'elle fait.",
    ),
    (
        "Hulotte, le hibou",
        "Cette nuit, Paulo est resté longtemps au bord de la rivière. Il regardait la lanterne bleue. Il n'a pas lancé sa ligne une seule fois. Les humains sont compliqués.",
    ),
    (
        "Odile, la maire",
        "Décision du conseil, adoptée à une voix contre zéro\u{202F}: le village a loué une montgolfière pour la fête. Elle passera au-dessus de ta maison les jours de beau temps. Le pilote s'appelle Bernard et il salue tout ce qui bouge. En somme, si tu lui fais signe, tu fais partie du spectacle.",
    ),
    (
        "Jeanne, l'encre bleue",
        "Sous l'arbre, il y a un coffre. Dedans, nos lettres d'amour, à Paulo et à moi, et des photos du village quand il n'avait pas encore de route. Il y a aussi une recette de Rose. Elle avait dix ans, et déjà des idées sur le beurre.",
    ),
    (
        "Coin-Coin, le canard",
        "Je suis papa. Des canetons, nés ce matin au bord de la mare. Ils me suivent partout, même sous la pluie. Surtout sous la pluie. Ils ont mon talent pour la nage. La grenouille dit qu'ils ont surtout mes pieds.",
    ),
    (
        "Paulo, le pêcheur",
        "Une dame ma écrit. À lencre bleue. Je connais cette écriture. Avant je rentrais tard de la pêche. Il y avait une lanterne bleue à la fenêtre. Pour moi. Jai pas dormi.",
    ),
    (
        "Jeanne, l'encre bleue",
        "Paulo a reçu ma lettre. Il m'a répondu, avec des fautes d'orthographe, comme avant. Trois lignes, et j'ai pleuré comme une sotte. Merci d'avoir gardé la lanterne.",
    ),
    (
        "Noisette, l'écureuil",
        "J'ai retrouvé une noisette. Ah non. C'est un bouton. Ça compte quand même, je le garde.",
    ),
    (
        "Rose, la boulangère",
        "Gros secret. Paulo m'a commandé un gâteau en forme de cœur. Paulo\u{202F}! Un cœur\u{202F}! J'ai écrit «\u{202F}Pour qui\u{202F}?\u{202F}» sur mon carnet. Il est devenu rouge comme une framboise et il est reparti sans répondre. Il a même oublié sa monnaie.",
    ),
    (
        "Jeanne, l'encre bleue",
        "Je viendrai au village pour la fête. Cela fait des années que je n'ai pas revu ma maison. Ta maison, pardon. J'ai encore du mal à le dire.",
    ),
    (
        "Gustave, l'escargot",
        "Arrivé à ta boîte aux lettres. Il m'a fallu des semaines. La vue est belle d'ici. Je repars vers la mare demain. Ou après-demain.",
    ),
    (
        "Gaston, le facteur",
        "J'ai monté une girouette sur ton toit, cadeau des anciens du village. Le cousin d'Odile me tenait l'échelle en me parlant de sa hanche, une heure au moins. Elle indique le vent, la girouette, pas la hanche. Sauf la nuit. La nuit, elle tourne vers le puits, même quand y a pas un souffle.",
    ),
    (
        "Jeanne, l'encre bleue",
        "La petite clé sous ta boîte aux lettres, c'est celle du coffre. Je l'ai gardée sur moi pendant quarante ans. Le jour de la fête, nous l'ouvrirons ensemble, avec Paulo et Léo.",
    ),
    (
        "Rainette, la grenouille",
        "Les canetons veulent sauter comme moi. Je leur donne des cours. Ils tombent tous dans la mare. En même temps, c'est une mare.",
    ),
    (
        "Léo, le petit voisin",
        "On a fait les décorations pour la fête avec Rose et j'ai eu de la colle partout, même dans les cheveux. Mamie arrive bientôt\u{202F}!!",
    ),
    (
        "Jeanne, l'encre bleue",
        "Il reste une chose dans le coffre. Une lettre, écrite il y a quarante ans, pour la personne qui habiterait ma maison après moi. Je ne savais pas qui ce serait. C'est toi.",
    ),
    (
        "Minouche, la chatte du toit",
        "Mes chatons grimpent sur la cheminée, maintenant. Ils prétendent que de là-haut on entend mieux les étoiles. On n'entend pas les étoiles. Je fais semblant de ne pas les connaître.",
    ),
    (
        "Odile, la maire",
        "Dernière séance du conseil avant la fête. Lieu\u{202F}: ton jardin, sous le grand arbre. Gâteaux\u{202F}: Rose. Musique\u{202F}: Paulo, qui a promis de ne pas chanter. Il ne manque plus que toi, et Gaston, en retard à sa propre réunion.",
    ),
    (
        "Jeanne, l'encre bleue",
        "Demain, je viens. J'ai sorti ma robe bleue, la belle, et je l'ai essayée deux fois. Paulo dit qu'il apportera des fleurs de la rivière. À mon âge, mon petit, on a encore le trac.",
    ),
    (
        "Biscuit, le chien",
        "Plein de MONDE dans le jardin. Des pieds partout. Mon os est caché. Je dis pas où. Et je surveille la croix rouge. Personne creuse sans moi.",
    ),
    (
        "Rose, la boulangère",
        "Quelle fête\u{202F}! Les décorations ont tenu jusqu'au bout, même avec le vent. Paulo et Jeanne ont dansé. Lentement, mais ils ont dansé. Léo a mangé trois parts de gâteau, hein, trois. Moi, je me suis mise tout contre les enceintes\u{202F}: la musique, je la sens mieux dans le ventre.\nP.-S. Il reste du gâteau. Viens avant Léo.",
    ),
    (
        "Jeanne, l'encre bleue",
        "Nous avons ouvert le coffre au pied de l'arbre. Paulo a retrouvé ses vieilles lettres et il a fait semblant d'avoir une poussière dans l'œil. Le coffre reste là, plein de souvenirs. La lettre qui t'était destinée dit seulement\u{202F}: prends soin de cette maison, elle te le rendra. Ma plume fuyait, ce jour-là\u{202F}: il y a une tache bleue sur le mot «\u{202F}maison\u{202F}».",
    ),
    (
        "Tout le village",
        "Merci de nous avoir lus jusqu'au bout. Rose fait toujours ses croissants. Paulo pêche, et maintenant Jeanne tient l'épuisette. Léo saute dans les flaques. Gaston passe tous les matins, même les jours sans courrier, pour discuter. À bientôt.",
    ),
];

pub const CREEPY_SENDER: &str = "Le voisin du n°13";
pub const CREEPY_LETTER_COUNT: usize = 15;

pub const CREEPY_LETTERS: [&str; CREEPY_LETTER_COUNT] = [
    "Bonjour. J'habite au numéro 13, juste derrière chez toi. Tu ne vois pas ma maison d'ici, c'est normal. Je voulais simplement te dire que la lumière de ta cuisine est très jolie, le soir.",
    "Tu as bien dormi ? Moi, je dors peu. Je regarde les fenêtres du village. La tienne reste allumée plus tard que les autres.",
    "J'ai marché dans ton jardin cette nuit. J'espère que ça ne te dérange pas. Les traces vont du puits à ta porte : c'est le chemin le plus court.",
    "Gaston ne passe jamais chez moi. Alors je dépose mes lettres moi-même, sous ta porte. Je fais très attention à ne pas réveiller le chien.",
    "Ce soir, je suis passé te voir. Tu étais dans l'autre pièce. Je n'ai pas voulu déranger, je suis resté à la fenêtre un moment.",
    "Tu as demandé à Marcel où se trouve le numéro 13 ? Il t'a dit qu'il n'existe pas. Il dit ça à tout le monde. Il a raison de le dire.",
    "Ton chat me connaît. Il me regarde quand je passe. Le chien aussi, mais lui fait semblant de dormir.",
    "Le puits n'est pas aussi profond qu'on le croit. Penche-toi, un soir. On entend quelqu'un répondre.",
    "J'ai retrouvé une photo très ancienne de ta maison. Il y a quelqu'un à la fenêtre du haut. Je crois que c'est moi. Je ne me souviens pas d'avoir été photographié.",
    "Les canetons ne s'approchent jamais du puits. Les animaux savent des choses. Tu devrais les observer plus souvent.",
    "Parfois, ils s'arrêtent tous en même temps et regardent dans la même direction. Quand ça arrive, ne te retourne pas. Regarde-les, eux.",
    "Treize maisons, treize fenêtres, treize marches jusqu'à ta porte. J'ai compté. Tu devrais compter aussi.",
    "Jeanne connaissait mon nom, autrefois. Elle ne le prononce plus. Demande-lui pourquoi la lanterne était bleue, et pas jaune.",
    "La fête approche. Tout le village sera dans ton jardin. Je viendrai aussi. Personne ne me verra, mais je serai là, au bord du puits.",
    "Merci pour la fête. Tu ne m'as pas vu, mais j'étais là. Je suis toujours là. Ce soir, quand la lumière de la cuisine s'éteindra, compte jusqu'à treize.",
];

pub const NIGHT_POSTSCRIPTS: [&str; 10] = [
    "P.-S. (d'une autre écriture, au crayon) : tu as oublié d'éteindre en bas.",
    "P.-S. : ne lis pas cette lettre près de la fenêtre.",
    "P.-S. : le chien a aboyé à 3 h 13. Tu l'as entendu ?",
    "P.-S. : quelqu'un a lu cette lettre avant toi.",
    "P.-S. : il y a de la boue sur ton paillasson. Elle n'est pas à toi.",
    "P.-S. : treize.",
    "P.-S. : ferme bien la porte du jardin, cette nuit.",
    "P.-S. : tu entends le puits ?",
    "P.-S. : je t'ai vu lire.",
    "P.-S. : va dormir. Moi, je reste.",
];

pub struct Choice {
    pub id: &'static str,
    pub letter: usize,
    pub prompt: &'static str,
    pub options: [&'static str; 2],
    pub echo_letter: usize,
    pub echoes: [&'static str; 2],
}

const fn choice(
    id: &'static str,
    letter: usize,
    prompt: &'static str,
    options: [&'static str; 2],
    echo_letter: usize,
    echoes: [&'static str; 2],
) -> Choice {
    Choice {
        id,
        letter,
        prompt,
        options,
        echo_letter,
        echoes,
    }
}

pub const CHOICES: [Choice; 12] = [
    choice(
        "bow",
        3,
        "Rose veut m'offrir un nœud. Mes cartes hésitent : quelle couleur ?",
        ["Rouge", "Bleu"],
        6,
        [
            "Il est rouge, comme tu me l'as conseillé.",
            "Il est bleu, comme tu me l'as conseillé.",
        ],
    ),
    choice(
        "well",
        7,
        "Pour le toit du vieux puits, tu préfères ?",
        ["Des tuiles rouges", "De la mousse verte"],
        13,
        [
            "Son toit est en tuiles rouges, comme tu l'as voulu.",
            "Son toit est couvert de mousse verte, comme tu l'as voulu.",
        ],
    ),
    choice(
        "bell",
        10,
        "Veux-tu que je sonne quand je passe ?",
        ["Oui, sonne !", "Non, en silence"],
        25,
        [
            "J'ai quand même sonné en passant : dring !",
            "Je suis passé sans sonner, comme promis.",
        ],
    ),
    choice(
        "owl",
        12,
        "La nuit, je hulule ou je me tais ?",
        ["Hulule !", "Tais-toi, je dors"],
        24,
        [
            "Nous hululerons à deux, puisque tu aimes ça.",
            "Nous resterons silencieux, promis.",
        ],
    ),
    choice(
        "puddles",
        16,
        "Je peux sauter dans les flaques chez toi ?",
        ["Oui, dans toutes !", "Non, pense à tes bottes"],
        34,
        [
            "Merci pour les flaques : je rentre trempé tous les jours.",
            "J'évite les flaques maintenant, mamie est fière de moi.",
        ],
    ),
    choice(
        "treehouse",
        19,
        "Pour la cabane de Léo, quelle couleur ?",
        ["Rouge", "Bleue"],
        22,
        [
            "Léo l'a peinte en rouge, sur ton conseil.",
            "Léo l'a peinte en bleu, sur ton conseil.",
        ],
    ),
    choice(
        "secret",
        23,
        "Garderas-tu mon secret ?",
        ["Oui, promis", "Je vais tout dire à Paulo"],
        29,
        [
            "Merci de garder le secret.",
            "Tu voulais tout lui dire ? Attends encore un peu, s'il te plaît.",
        ],
    ),
    choice(
        "nuts",
        27,
        "Je peux cacher des noisettes près de ta boîte aux lettres ?",
        ["Oui, vas-y", "Non, c'est pour le courrier"],
        45,
        [
            "J'en ai retrouvé une près de ta boîte aux lettres, merci !",
            "J'ai tout caché dans l'arbre, ta boîte aux lettres est tranquille.",
        ],
    ),
    choice(
        "party",
        28,
        "Pour décorer ta maison : fanions ou lampions ?",
        ["Des fanions", "Des lampions"],
        52,
        [
            "On a peint des fanions rouges, bleus et jaunes.",
            "On a fabriqué des lampions en papier de toutes les couleurs.",
        ],
    ),
    choice(
        "song",
        30,
        "Qui chante le mieux, moi ou le canard ?",
        ["Toi, Rainette", "Le canard"],
        51,
        [
            "Tu as dit que je chantais le mieux : le canard boude depuis.",
            "Tu as dit que le canard chantait mieux. Je boude.",
        ],
    ),
    choice(
        "kittens",
        33,
        "Mes chatons sont-ils noirs ou blancs, à ton avis ? Mon pendule refuse de répondre.",
        ["Noirs", "Blancs"],
        54,
        [
            "Comme ils sont noirs, la suie ne se voit même pas.",
            "Eux qui étaient tout blancs, ils sont gris de suie maintenant.",
        ],
    ),
    choice(
        "ducklings",
        42,
        "Un dernier œuf n'a pas encore éclos. On l'attend ?",
        ["Oui, attendons", "Deux canetons, c'est bien"],
        57,
        [
            "Les trois canetons de Coin-Coin me suivent partout.",
            "Les deux canetons de Coin-Coin me suivent partout.",
        ],
    ),
];

pub fn choice_for(letter_number: usize) -> Option<&'static Choice> {
    CHOICES.iter().find(|choice| choice.letter == letter_number)
}

pub fn echo_for(
    letter_number: usize,
    answers: &std::collections::BTreeMap<String, u8>,
) -> Option<&'static str> {
    let choice = CHOICES
        .iter()
        .find(|choice| choice.echo_letter == letter_number)?;
    let answer = *answers.get(choice.id)?;
    choice
        .echoes
        .get(usize::from(answer).checked_sub(1)?)
        .copied()
}

pub fn answered_choices(
    answers: &std::collections::BTreeMap<String, u8>,
) -> impl Iterator<Item = (&'static Choice, usize)> {
    CHOICES.iter().filter_map(|choice| {
        let option = usize::from(*answers.get(choice.id)?).checked_sub(1)?;
        (option < choice.options.len()).then_some((choice, option))
    })
}

pub struct Letter {
    pub sender: String,
    pub body: String,
}

pub fn defaults() -> Vec<Letter> {
    DEFAULT_LETTERS
        .iter()
        .map(|(sender, body)| Letter {
            sender: sender.to_string(),
            body: body.to_string(),
        })
        .collect()
}

pub fn load() -> Vec<Letter> {
    let letters = read();
    if let Some(path) = letters_path()
        && !path.exists()
    {
        let _ = write_defaults(&path, &letters);
    }
    letters
}

pub fn read() -> Vec<Letter> {
    let mut letters = defaults();
    if let Some(content) = letters_path().and_then(|path| fs::read_to_string(path).ok()) {
        apply_overrides(&mut letters, &content);
    }
    letters
}

fn letters_path() -> Option<PathBuf> {
    dirs::config_dir().map(|dir| dir.join("weathr").join("lettres.txt"))
}

fn write_defaults(path: &Path, letters: &[Letter]) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut content = String::from(FILE_HEADER);
    for (index, letter) in letters.iter().enumerate() {
        content.push_str(&format!(
            "\n=== {} | {}\n{}\n",
            index + 1,
            letter.sender,
            letter.body
        ));
    }
    fs::write(path, content)
}

fn apply_overrides(letters: &mut Vec<Letter>, content: &str) {
    for (number, letter) in parse(content) {
        if number == 0 || letter.body.is_empty() {
            continue;
        }
        if number <= letters.len() {
            letters[number - 1] = letter;
        } else if number == letters.len() + 1 {
            letters.push(letter);
        }
    }
}

fn parse(content: &str) -> Vec<(usize, Letter)> {
    let mut parsed = Vec::new();
    let mut current: Option<(usize, String, Vec<&str>)> = None;

    for line in content.lines() {
        if let Some(header) = line.strip_prefix("=== ") {
            if let Some(letter) = current.take() {
                parsed.push(finish(letter));
            }
            let (number, sender) = header.split_once('|').unwrap_or((header, ""));
            current = number
                .trim()
                .parse()
                .ok()
                .map(|number| (number, sender.trim().to_string(), Vec::new()));
        } else if let Some((_, _, body)) = current.as_mut() {
            body.push(line);
        }
    }
    if let Some(letter) = current.take() {
        parsed.push(finish(letter));
    }
    parsed
}

fn finish((number, sender, body): (usize, String, Vec<&str>)) -> (usize, Letter) {
    (
        number,
        Letter {
            sender,
            body: body.join("\n").trim().to_string(),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choices_echo_later_letters() {
        for choice in &CHOICES {
            assert!(choice.echo_letter > choice.letter, "{}", choice.id);
            assert!(choice.echo_letter <= DEFAULT_LETTERS.len(), "{}", choice.id);
        }
        let answers = std::collections::BTreeMap::from([("bow".to_string(), 2)]);
        assert_eq!(echo_for(6, &answers), Some(CHOICES[0].echoes[1]));
        assert_eq!(echo_for(13, &answers), None);
    }

    #[test]
    fn test_answered_choices_follow_question_order_and_skip_unknown_answers() {
        let answers = std::collections::BTreeMap::from([
            ("well".to_string(), 2),
            ("bow".to_string(), 1),
            ("bell".to_string(), 3),
            ("owl".to_string(), 0),
            ("nobody".to_string(), 1),
        ]);
        let answered: Vec<(&str, usize)> = answered_choices(&answers)
            .map(|(choice, option)| (choice.id, option))
            .collect();
        assert_eq!(answered, [("bow", 0), ("well", 1)]);
        assert_eq!(
            answered_choices(&std::collections::BTreeMap::new()).count(),
            0
        );
    }

    #[test]
    fn test_overrides_replace_and_append() {
        let mut letters = vec![Letter {
            sender: "A".to_string(),
            body: "Un".to_string(),
        }];
        apply_overrides(
            &mut letters,
            "# commentaire\n=== 1 | B\nDeux\n\nlignes\n=== 2 | C\nTrois\n=== 9 | D\nIgnorée\n",
        );

        assert_eq!(letters.len(), 2);
        assert_eq!(letters[0].sender, "B");
        assert_eq!(letters[0].body, "Deux\n\nlignes");
        assert_eq!(letters[1].body, "Trois");
    }
}
