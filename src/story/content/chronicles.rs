pub const SAM_SENDER: &str = "Sam, le voisin du moulin";
pub const SAM_LETTERS: [&str; 5] = [
    "Bonjour d'en face\u{202F}! Moi c'est Sam, le vieux moulin au bord de la rivière. Je dessinais des cartes, avant. Maintenant je les regarde, surtout. Tu savais\u{202F}? Ton jardin est sur trois de mes plans. Sur le plus vieux, ta maison n'a même pas de cheminée.",
    "Rose m'a vendu un croissant brûlé. Le moins brûlé, d'après elle. Je l'ai mangé en traçant une route au crayon, d'ici jusqu'à Valparaíso. Onze centimètres sur ma carte. Vu comme ça, ça paraît faisable, non\u{202F}?",
    "Quarante petites croix. C'est ce qu'il y a sur la carte du monde de mon grand-père, retrouvée au grenier. Chaque croix, une ville qu'il voulait voir. Il n'en a vu aucune, tu savais\u{202F}? Celle de Lisbonne, il l'a appuyée si fort que le papier est troué.",
    "J'ai acheté une valise. Marron, avec des coins en cuir. Elle est encore vide, au milieu du moulin. Paulo est passé, il l'a regardée longtemps et il a dit\u{202F}: «\u{202F}Faut la remplir.\u{202F}» C'est la plus longue phrase qu'il m'ait jamais dite.",
    "Je pars demain, par le car de 6 h 10. Je vais suivre les croix de mon grand-père, une par une. Chaque semaine, je t'envoie une carte postale avec le temps qu'il fait là-bas. Tu me diras s'il fait meilleur chez toi\u{202F}? Garde un œil sur mon moulin. La porte du bas ferme mal, faut la soulever un peu.",
];

pub const NEIGHBOURS_SENDER: &str = "Nadia et Claire, les apicultrices";
pub const NEIGHBOURS_LETTERS: [&str; 5] = [
    "Coucou\u{202F}! Nadia et Claire, les nouvelles de la maison derrière le verger. On est arrivées avec quatre ruches (Claire en voulait six). Si tu vois des abeilles chez toi, ce sont les nôtres. Elles adorent ton arbre\u{202F}: Claire dit qu'elles ont bon goût, Nadia dit qu'elles sont juste gourmandes. Le premier pot est pour toi.",
    "Rose nous apprend la langue des signes, le jeudi, entre deux fournées. Claire a retenu «\u{202F}miel\u{202F}» et «\u{202F}merci\u{202F}» dès la première leçon. Nadia confond encore les deux. Du coup, elle remercie tout le monde pour le miel. Même Gaston, qui n'avait rien demandé.",
    "Premier pot de miel. Il t'attend sur ta boîte aux lettres. Rose en a pris dix pour ses gâteaux, Claire voulait lui en vendre vingt. Marcel n'en veut pas. Il dit que nos abeilles évitent le n°13 et que c'est mauvais signe. Nadia a regardé\u{202F}: c'est vrai qu'elles font un détour. Claire trouve qu'il se fait des idées.",
    "Dix ans ce soir. C'est Odile qui nous a mariées, à la mairie, sous une pluie battante, l'écharpe tricolore trempée. Claire dit que ça porte bonheur, Nadia dit surtout que ça mouille. On fête ça avec une tarte au miel. Claire l'a sortie du four trop tôt, on ne lui dira pas.",
    "Une ruche a essaimé ce matin. Tout le nuage d'abeilles s'est posé dans ton arbre, sur la troisième branche. Claire est montée avec un carton avant qu'on trouve les gants. Nadia tenait l'échelle et donnait des conseils, que personne n'a suivis. Si une abeille traîne chez toi, t'inquiète pas, elle connaît le chemin.",
];

pub const MARCEL_SENDER: &str = "Marcel, le garde champêtre";
pub const MARCEL_LETTERS: [&str; 5] = [
    "Bonsoir. Marcel, garde champêtre. Ronde chaque nuit, de la mairie au bout de ton chemin. Si tu vois une lanterne derrière ta haie vers minuit, c'est moi. Rien à signaler. Sauf ta haie, qui déborde de quarante centimètres. Je note pas. Pour cette fois.",
    "Paraît que tu poses des questions sur un numéro 13. Vingt-deux ans que je fais cette ronde. Le village a toujours eu douze maisons. Je les compte tous les soirs. Douze.",
    "Ronde de 0 h 40. Boue près du puits, encore. Rien à signaler. Des pas vers ta maison, aucun qui revient. Sûrement les miens. J'ai dû mal compter.",
    "Ronde de 23 h 15. Compté les maisons, comme chaque soir. Douze. Au retour, treize fenêtres allumées. Recompté\u{202F}: douze. Pas noté au registre. Odile dit de tout noter. Pas ça. C'était la buée sur ma lanterne.",
    "La fête approche. Un conseil, un seul. Le soir, si on frappe trois coups à ta porte, réponds pas. Tout le village a appris à pas répondre. Moi aussi, j'ai appris. Ma première année de ronde. Je dors très bien, merci.",
];

pub struct City {
    pub name: &'static str,
    pub country: &'static str,
    pub latitude: f64,
    pub longitude: f64,
    pub note: &'static str,
}

const fn city(
    name: &'static str,
    country: &'static str,
    latitude: f64,
    longitude: f64,
    note: &'static str,
) -> City {
    City {
        name,
        country,
        latitude,
        longitude,
        note,
    }
}

pub const CITIES: [City; 40] = [
    city(
        "Lisbonne",
        "Portugal",
        38.72,
        -9.14,
        "Les tramways jaunes grimpent les collines comme des chèvres têtues.",
    ),
    city(
        "Reykjavik",
        "Islande",
        64.15,
        -21.94,
        "Le vent ici a du caractère. Il a essayé de me voler cette carte.",
    ),
    city(
        "Marrakech",
        "Maroc",
        31.63,
        -7.99,
        "J'ai acheté une théière. Je n'ai pas de cuisine. Je l'adore.",
    ),
    city(
        "Kyoto",
        "Japon",
        35.01,
        135.77,
        "Des milliers de portes rouges sur une montagne. J'en ai compté 213.",
    ),
    city(
        "Valparaíso",
        "Chili",
        -33.05,
        -71.62,
        "Chaque maison est peinte d'une couleur différente. La tienne irait bien ici.",
    ),
    city(
        "Édimbourg",
        "Écosse",
        55.95,
        -3.19,
        "Un château sur un volcan éteint. Les gens trouvent ça normal.",
    ),
    city(
        "Hanoï",
        "Viêt Nam",
        21.03,
        105.85,
        "Traverser la rue est un art. Je suis encore en apprentissage.",
    ),
    city(
        "Québec",
        "Canada",
        46.81,
        -71.21,
        "On y parle français, mais avec plus de neige dedans.",
    ),
    city(
        "Istanbul",
        "Turquie",
        41.01,
        28.98,
        "Un pied en Europe, un pied en Asie. Je n'ai pas su où poser ma valise.",
    ),
    city(
        "Le Cap",
        "Afrique du Sud",
        -33.92,
        18.42,
        "Une montagne plate comme une table. Les nuages viennent y mettre la nappe.",
    ),
    city(
        "Oaxaca",
        "Mexique",
        17.06,
        -96.73,
        "Du chocolat dans la sauce du poulet. Je ne reviendrai peut-être pas.",
    ),
    city(
        "Tromsø",
        "Norvège",
        69.65,
        18.96,
        "La nuit dure presque toute la journée. Le hibou adorerait.",
    ),
    city(
        "Hobart",
        "Tasmanie",
        -42.88,
        147.33,
        "Je suis presque à l'autre bout du monde. Ton jardin doit être sous mes pieds.",
    ),
    city(
        "Cusco",
        "Pérou",
        -13.53,
        -71.97,
        "L'air est si léger ici que mes cartes postales s'envolent toutes seules.",
    ),
    city(
        "Zanzibar",
        "Tanzanie",
        -6.16,
        39.19,
        "Ça sent le clou de girofle partout. Même mes chaussettes.",
    ),
    city(
        "Helsinki",
        "Finlande",
        60.17,
        24.94,
        "Ici, on va au sauna puis on saute dans la mer glacée. Volontairement.",
    ),
    city(
        "Buenos Aires",
        "Argentine",
        -34.60,
        -58.38,
        "On danse le tango dans la rue. J'ai essayé. La rue s'en souvient.",
    ),
    city(
        "Petra",
        "Jordanie",
        30.33,
        35.44,
        "Une ville entière taillée dans la roche rose. Pas une seule boîte aux lettres.",
    ),
    city(
        "Ushuaïa",
        "Argentine",
        -54.80,
        -68.30,
        "Le bout du monde a un panneau. Je l'ai pris en photo pour toi.",
    ),
    city(
        "Bergen",
        "Norvège",
        60.39,
        5.32,
        "Il pleut deux cents jours par an. Ton canard serait au paradis.",
    ),
    city(
        "Luang Prabang",
        "Laos",
        19.89,
        102.13,
        "Les moines passent à l'aube en silence. Même les coqs se taisent.",
    ),
    city(
        "La Havane",
        "Cuba",
        23.11,
        -82.37,
        "Les voitures ont l'âge de mes cartes. Elles roulent mieux que moi.",
    ),
    city(
        "Nairobi",
        "Kenya",
        -1.29,
        36.82,
        "Une girafe m'a regardé de haut. Littéralement.",
    ),
    city(
        "Oulan-Bator",
        "Mongolie",
        47.92,
        106.92,
        "J'ai dormi sous une yourte. Le ciel était plus étoilé que ton jardin.",
    ),
    city(
        "Venise",
        "Italie",
        45.44,
        12.33,
        "Ici, les rues sont des rivières. Coin-Coin se sentirait chez lui.",
    ),
    city(
        "Dublin",
        "Irlande",
        53.35,
        -6.26,
        "Dans un pub, chaque table a un nom. La mienne s'appelle « Patience ».",
    ),
    city(
        "Sydney",
        "Australie",
        -33.87,
        151.21,
        "Ici, c'est le printemps quand chez toi c'est l'automne. Tout est à l'envers.",
    ),
    city(
        "Ubud",
        "Indonésie",
        -8.51,
        115.26,
        "Des rizières en escalier jusqu'au ciel. Les canards y travaillent aussi.",
    ),
    city(
        "Santorin",
        "Grèce",
        36.39,
        25.46,
        "Tout est blanc et bleu. Même mes rêves ont changé de couleur.",
    ),
    city(
        "Chefchaouen",
        "Maroc",
        35.17,
        -5.27,
        "Une ville entièrement peinte en bleu. Jeanne aimerait.",
    ),
    city(
        "Katmandou",
        "Népal",
        27.72,
        85.32,
        "Les drapeaux de prière claquent au vent. Chacun porte un vœu.",
    ),
    city(
        "Longyearbyen",
        "Svalbard",
        78.22,
        15.65,
        "Il y a plus d'ours que d'habitants. Je ne sors pas sans chanter.",
    ),
    city(
        "Rio de Janeiro",
        "Brésil",
        -22.91,
        -43.17,
        "Un géant de pierre ouvre les bras au-dessus de la ville. J'ai fait pareil.",
    ),
    city(
        "Prague",
        "Tchéquie",
        50.08,
        14.44,
        "Une horloge fait défiler des squelettes à chaque heure. Ton voisin du 13 aimerait.",
    ),
    city(
        "Séoul",
        "Corée du Sud",
        37.57,
        126.98,
        "On mange dans la rue à minuit. Je ne dors plus, je goûte.",
    ),
    city(
        "San Pedro de Atacama",
        "Chili",
        -22.91,
        -68.20,
        "Le désert le plus sec du monde. Ici, la pluie est une légende.",
    ),
    city(
        "Copenhague",
        "Danemark",
        55.68,
        12.57,
        "Tout le monde est à vélo. Gaston serait jaloux.",
    ),
    city(
        "Dakar",
        "Sénégal",
        14.72,
        -17.47,
        "La mer, le vent, les pirogues colorées. J'ai appris trois mots de wolof.",
    ),
    city(
        "Vancouver",
        "Canada",
        49.28,
        -123.12,
        "Des montagnes, la mer et la forêt dans la même rue.",
    ),
    city(
        "Tbilissi",
        "Géorgie",
        41.72,
        44.79,
        "On m'a servi du pain en forme de bateau. Je l'ai mangé sans couler.",
    ),
];

pub const ARCHIVES: [(&str, &str); 10] = [
    (
        "Registre communal, 1887",
        "Mot d'Odile : puisque tu poses tant de questions, je t'ouvre les archives. Lis-les dans l'ordre, et pas trop tard le soir. — « Le conseil décide de creuser un puits dans le jardin de la maison au bout du chemin. Les ouvriers disent entendre comme des voix au fond. Le travail est achevé en onze jours. »",
    ),
    (
        "Plan du village, 1902",
        "Sur ce plan, le village compte treize maisons. La treizième, derrière la maison au bout du chemin, a été grattée au couteau. Dans la marge, une main a écrit : « ne pas reporter ».",
    ),
    (
        "Rapport du garde-champêtre, 1911",
        "« La rivière dépose parfois sur la berge des bouteilles fermées, avec des messages dedans. Certaines sont datées de l'année suivante. Je recommande de ne pas les ouvrir. Personne ne m'écoute. »",
    ),
    (
        "Lettre d'une habitante, 1934",
        "« Chez nous, on pose une lanterne bleue à la fenêtre quand quelqu'un n'est pas rentré. La jaune attire ceux qu'on attend. La bleue éloigne ceux qu'on n'attend pas. »",
    ),
    (
        "La Gazette, 3 novembre 1956",
        "« Un promeneur affirme avoir vu de la lumière dans une maison, derrière celle du bout du chemin. Il n'y a là qu'un terrain vague. Le promeneur est reparti le lendemain, sans laisser d'adresse. »",
    ),
    (
        "Acte de vente, 1961",
        "La maison au bout du chemin est vendue à Jeanne et à son époux. Clause ajoutée à la main : « L'acquéreur s'engage à entretenir le puits et à ne jamais le combler. »",
    ),
    (
        "Note du maire, 1977",
        "« Rappel aux habitants : le soir, ne répondez pas quand on frappe trois coups. Ce n'est personne du village. Ce n'est personne. »",
    ),
    (
        "Photographie, vers 1980",
        "Description jointe au cliché : « Vue de la maison au bout du chemin, prise depuis la rivière. On compte treize fenêtres éclairées. La maison n'en a que douze. »",
    ),
    (
        "Registre des naissances",
        "La page 13 est arrachée. Sur la page suivante, quelqu'un a recopié treize fois le même mot, effacé depuis. On devine seulement qu'il avait treize lettres.",
    ),
    (
        "Note manuscrite d'Odile",
        "Je t'ai envoyé tout ce que j'avais. Il manque un dossier, le numéro 13. Il était là hier. Ce matin, l'étagère est vide et le parquet est mouillé.",
    ),
];

pub const RUMORS: [&str; 20] = [
    "On dit que Rose prépare un croissant au chocolat… et à la noisette.",
    "Gaston aurait été vu en train de parler à sa sacoche. Il dément.",
    "Le canard de la mare a été élu « canard le plus bavard » du canton.",
    "Quelqu'un a repeint la boîte aux lettres de la mairie en rose. Enquête en cours.",
    "Paulo a pêché une botte. Il la garde « pour la paire ».",
    "Les chats du village tiendraient des réunions secrètes sur le toit de l'église.",
    "Odile cherche un nom pour le chemin qui mène au puits. Les propositions sont ouvertes.",
    "Léo affirme avoir vu un écureuil faire ses comptes.",
    "La girouette de la mairie indique le nord-sud-ouest. Les experts sont perplexes.",
    "Une lumière aurait été aperçue près du vieux moulin, alors que Sam est en voyage.",
    "On murmure que la prochaine fête sera « mémorable ». Personne ne sait pourquoi.",
    "Le hibou du grand arbre hululerait en rythme. On parle d'une carrière musicale.",
    "La boulangerie a battu son record : 214 croissants un dimanche.",
    "Un chien inconnu a été vu au marché. Biscuit refuse de commenter.",
    "Le puits du bout du chemin ferait de l'écho. Un écho qui répond trop tard.",
    "Les escargots du village préparent une course. Départ prévu le mois prochain.",
    "Quelqu'un dépose des fleurs devant le terrain vague, chaque jeudi.",
    "La rivière aurait changé de couleur pendant une heure, mardi. Personne n'a de photo.",
    "Le club de pétanque recrute. Niveau requis : savoir où est le cochonnet.",
    "Selon le dernier plan, le village compte douze maisons et demie. Erreur d'impression, assure la mairie.",
];

pub const ADS: [&str; 20] = [
    "Vends vélo de facteur, sonnette fatiguée. S'adresser à Gaston.",
    "Cherche chaussette gauche, motif écossais. Récompense : un croissant.",
    "Donne chatons… Ah non, Minouche refuse. Annonce annulée.",
    "Échange recette de tarte contre secret de pêche. Écrire à Paulo.",
    "Perdu : noisettes, quantité inconnue. Contacter Noisette, grand arbre.",
    "Cours de hululement le jeudi soir. Débutants acceptés.",
    "Vends barque, prend un peu l'eau, chante beaucoup.",
    "Recherche bénévoles pour compter les étoiles filantes. Patience exigée.",
    "Loue cabane dans un arbre, vue imprenable, interdite aux adultes.",
    "Cherche partenaire pour une partie de dames. Niveau : grand-mère redoutable.",
    "Donne cartes anciennes. Certaines mènent quelque part.",
    "Vends parapluie jamais servi. Léo préfère les flaques.",
    "Perdu clé ancienne. Elle ouvre quelque chose d'important. On ne sait pas quoi.",
    "Cherche témoin d'un bruit de pas, nuit de mardi, près du puits.",
    "Cours de tricot pour chats. Résultats non garantis.",
    "Vends lanterne bleue, très bon état. Vendeur inconnu.",
    "Échange girouette contre boussole. La girouette a son caractère.",
    "Cherche fanfare pour la fête. Une trompette suffira.",
    "Donne bocaux de confiture de mûres, récolte miraculeuse.",
    "Recherche plan complet du village. Le nôtre a un trou.",
];

pub struct Riddle {
    pub question: &'static str,
    pub answers: &'static [&'static str],
}

const fn riddle(question: &'static str, answers: &'static [&'static str]) -> Riddle {
    Riddle { question, answers }
}

pub const RIDDLES: [Riddle; 30] = [
    riddle(
        "Plus j'ai de gardiens, moins je suis gardé. Qui suis-je ?",
        &["secret"],
    ),
    riddle(
        "Je tombe sans me faire mal, je cours sans jambes, et le canard m'adore. Qui suis-je ?",
        &["pluie"],
    ),
    riddle(
        "Je suis plein de trous et pourtant je retiens l'eau. Qui suis-je ?",
        &["eponge"],
    ),
    riddle(
        "Plus on m'enlève, plus je deviens grand. Qui suis-je ?",
        &["trou"],
    ),
    riddle(
        "J'ai des aiguilles mais je ne couds pas. Qui suis-je ?",
        &["horloge", "montre", "sapin", "pin", "herisson", "pendule"],
    ),
    riddle(
        "Je monte et je descends sans jamais bouger. Qui suis-je ?",
        &["escalier", "escaliers"],
    ),
    riddle(
        "J'ai un chapeau mais pas de tête, un pied mais pas de jambe. Qui suis-je ?",
        &["champignon"],
    ),
    riddle(
        "Je suis à toi, mais les autres m'utilisent plus que toi. Qui suis-je ?",
        &[
            "prenom",
            "nom",
            "ton prenom",
            "ton nom",
            "mon prenom",
            "mon nom",
        ],
    ),
    riddle(
        "Le matin à quatre pattes, le midi à deux, le soir à trois. Qui suis-je ?",
        &["homme", "humain", "etre humain"],
    ),
    riddle(
        "Je suis légère comme une plume, mais personne ne peut me retenir longtemps. Qui suis-je ?",
        &["respiration", "souffle"],
    ),
    riddle(
        "Je parle sans bouche et j'entends sans oreilles. Qui suis-je ?",
        &["echo"],
    ),
    riddle(
        "Je suis noir quand on m'achète, rouge quand on m'utilise, gris quand on me jette. Qui suis-je ?",
        &["charbon"],
    ),
    riddle(
        "Je disparais dès qu'on prononce mon nom. Qui suis-je ?",
        &["silence"],
    ),
    riddle(
        "J'ai des villes sans maisons, des forêts sans arbres et des rivières sans eau. Qui suis-je ?",
        &["carte", "carte geographique"],
    ),
    riddle(
        "Je me brise sans qu'on me touche. Qui suis-je ?",
        &["promesse"],
    ),
    riddle(
        "J'ai des dents mais je ne mords jamais. Qui suis-je ?",
        &["peigne", "fourchette", "rateau", "scie"],
    ),
    riddle(
        "Je vole sans ailes et je pleure sans yeux. Qui suis-je ?",
        &["nuage"],
    ),
    riddle(
        "Je suis toujours devant toi, mais tu ne me vois jamais. Qui suis-je ?",
        &["futur", "avenir", "demain"],
    ),
    riddle(
        "On me jette quand on a besoin de moi, on me remonte quand on n'en a plus besoin. Qui suis-je ?",
        &["ancre"],
    ),
    riddle(
        "Je fais le tour du jardin sans jamais bouger. Qui suis-je ?",
        &["cloture", "barriere", "haie", "grillage"],
    ),
    riddle(
        "Je suis plus grande que la maison et pourtant je ne pèse rien. Qui suis-je ?",
        &["ombre"],
    ),
    riddle(
        "Je fais le tour du monde en restant dans un coin. Qui suis-je ?",
        &["timbre"],
    ),
    riddle(
        "Qu'est-ce qui s'ouvre quand la pluie tombe ?",
        &["parapluie"],
    ),
    riddle(
        "Je commence la nuit et je termine le matin. Qui suis-je ?",
        &["n", "lettre n"],
    ),
    riddle(
        "J'ai un lit mais je ne dors jamais, une bouche mais je ne parle pas. Qui suis-je ?",
        &["riviere", "fleuve"],
    ),
    riddle(
        "Plus je sèche, plus je suis mouillée. Qui suis-je ?",
        &["serviette"],
    ),
    riddle(
        "Quel oiseau peut tourner la tête presque complètement ?",
        &["hibou", "chouette"],
    ),
    riddle(
        "Je suis le nombre que personne au village ne veut prononcer. Qui suis-je ?",
        &["13", "treize"],
    ),
    riddle(
        "Je n'ai pas de pieds mais je laisse des traces. Qui suis-je ?",
        &["escargot", "limace"],
    ),
    riddle(
        "Je réveille tout le monde sans faire le moindre bruit. Qui suis-je ?",
        &["soleil", "lumiere"],
    ),
];

pub const DRAWINGS: [(usize, &[&str]); 3] = [
    (
        16,
        &[
            "Mon dessin de ta maison :",
            "    /\\      ~~",
            "   /  \\",
            "  /____\\",
            "  | [] |    o",
            "  |_[]_|   /|\\",
            "           / \\",
        ],
    ),
    (
        34,
        &[
            "Minouche et ses chatons :",
            "  /\\_/\\    /\\_/\\   /\\_/\\",
            " ( o.o )  ( -.- ) ( o.o )",
            "  > ^ <    > ^ <   > ^ <",
        ],
    ),
    (
        52,
        &[
            "La fête dans ton jardin :",
            "  v v v v v v v v v",
            "    o    o     o",
            "   /|\\  /|\\   /|\\   ~ musique ~",
            "   / \\  / \\   / \\",
        ],
    ),
];

pub const BOTTLE_MESSAGES: [&str; 20] = [
    "Si tu lis ceci, la rivière t'a choisi. Rejette-moi à l'eau, s'il te plaît.",
    "Je m'appelle Lucie, j'ai neuf ans et j'écris en 1972. Est-ce que les voitures volent, chez toi ?",
    "Recette secrète : il n'y en a pas. Mets juste du beurre.",
    "Je cherche quelqu'un qui aime les orages autant que moi.",
    "La carte est fausse. Le trésor est sous le deuxième pont, pas sous le premier.",
    "Ne fais pas confiance à la girouette.",
    "Message pour le futur : tout va bien. On a juste oublié de prévenir.",
    "J'ai compté les étoiles cette nuit. Il en manquait une.",
    "Bonjour d'un bateau qui n'existe plus.",
    "Si tu trouves ma chaussette, elle est à moi. — Gaston",
    "Treize jours de pluie, et pourtant je suis heureuse.",
    "La rivière remonte parfois son cours. Personne ne le remarque.",
    "Le meilleur endroit pour regarder les étoiles, c'est le toit. Demande au chat.",
    "Je te souhaite une journée sans rien de grave.",
    "Cette bouteille a fait trois fois le tour du village. Elle revient toujours ici.",
    "J'ai jeté une pièce dans le puits. Elle est remontée.",
    "Merci de m'avoir sortie de l'eau. Il faisait froid.",
    "Il y a une lettre sous le banc. Elle n'est pas pour toi.",
    "Rien à signaler. Je voulais juste écrire à quelqu'un.",
    "Ce message s'effacera quand tu auras fini de le lire.",
];

pub const OBJECTS: [&str; 25] = [
    "une bille bleue",
    "un bouton de nacre",
    "une plume de hibou",
    "une clé rouillée",
    "un dé à coudre",
    "une pièce d'un franc",
    "une photo déchirée",
    "une petite cuillère",
    "un ticket de tram",
    "un coquillage",
    "une figurine en plomb",
    "un ticket de bal de 1952",
    "des lunettes cassées",
    "une boucle d'oreille",
    "une médaille de chien",
    "une noisette dorée",
    "un crayon bleu",
    "une dame de cœur",
    "un ruban rouge",
    "une capsule de limonade",
    "une boussole",
    "un harmonica",
    "une étoile de shérif",
    "un petit carnet vide",
    "une bague en fer-blanc",
];

pub const CHEST_SOUVENIRS: [&str; 4] = [
    "Les lettres d'amour de Jeanne et Paulo",
    "Des photos du village d'autrefois",
    "La recette de Rose, écrite à dix ans",
    "Une lettre pour toi, écrite il y a quarante ans",
];

pub fn normalize_answer(text: &str) -> String {
    let folded: String = text
        .to_lowercase()
        .chars()
        .map(|ch| match ch {
            'à' | 'â' | 'ä' => 'a',
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'î' | 'ï' => 'i',
            'ô' | 'ö' => 'o',
            'ù' | 'û' | 'ü' => 'u',
            'ç' => 'c',
            '\'' | '’' | '-' => ' ',
            _ => ch,
        })
        .filter(|ch| ch.is_alphanumeric() || *ch == ' ')
        .collect();
    let words: Vec<&str> = folded.split_whitespace().collect();
    let skip = match words.as_slice() {
        ["de", "la", ..] => 2,
        [first, _, ..] if ["le", "la", "les", "l", "un", "une", "des", "du"].contains(first) => 1,
        _ => 0,
    };
    words[skip..].join(" ")
}

pub fn is_right_answer(riddle: &Riddle, attempt: &str) -> bool {
    let attempt = normalize_answer(attempt);
    riddle
        .answers
        .iter()
        .any(|answer| normalize_answer(answer) == attempt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answers_ignore_case_accents_and_articles() {
        assert!(is_right_answer(&RIDDLES[2], "  L'Éponge !"));
        assert!(is_right_answer(&RIDDLES[10], "un écho"));
        assert!(is_right_answer(&RIDDLES[27], "Treize"));
        assert!(!is_right_answer(&RIDDLES[0], "un mensonge"));
    }
}
