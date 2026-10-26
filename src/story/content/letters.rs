use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const FILE_HEADER: &str = "# Lettres de weathr : une nouvelle lettre arrive chaque jour.
# Chaque lettre commence par une ligne \"=== numéro | expéditeur\".
# Tu peux réécrire les textes librement ou ajouter des lettres après la dernière.
";

const DEFAULT_LETTERS: [(&str, &str); 60] = [
    (
        "Rose, la boulangère",
        "Bienvenue dans la maison au bout du chemin ! Ici, tout le monde se connaît. Je passe chaque matin avec mon panier de croissants : si tu sens une odeur de beurre, c'est moi. Je suis sourde : inutile de sonner chez moi, fais-moi plutôt signe par la fenêtre. Gaston, le facteur, t'apportera une lettre par jour. C'est la tradition du village.",
    ),
    (
        "L'encre bleue",
        "Tu ne me connais pas, mais je connais très bien ta maison. Chaque pierre, chaque fenêtre. Elle cache un secret depuis longtemps. Si tu veux le découvrir, lis bien chacune de mes lettres.",
    ),
    (
        "Minouche, la chatte du toit",
        "Miaou. Je suis la chatte qui dort sur ton toit. Ce toit est à moi depuis six hivers, je te le prête. D'ici, je lis les étoiles : ce mois-ci, elles te sont favorables. Quand il pleut, je rentre par la fenêtre, ne t'inquiète pas. Signé d'une patte.",
    ),
    (
        "Paulo, le pêcheur",
        "J'avais trop de bois flotté dans ma cabane, alors je t'ai fabriqué un banc. Je l'ai posé devant ta maison. On y voit très bien le coucher du soleil. Paulo, ton voisin de la rivière.",
    ),
    (
        "L'encre bleue",
        "Première énigme : je suis la plus vieille habitante du jardin, j'étais là avant la maison, et chaque automne je perds mes cheveux. Qui suis-je ?",
    ),
    (
        "Minouche, la chatte du toit",
        "Rose m'a offert un nœud. Les astres l'avaient annoncé : je suis désormais la chatte la plus élégante du village. Biscuit est jaloux, mais Biscuit ne lit pas les étoiles. Regarde bien mon oreille !",
    ),
    (
        "Odile, la maire",
        "Le village prépare sa grande fête. Il reste beaucoup à faire : réparer le vieux puits, décorer les maisons, trouver de la musique. Chaque habitant peut aider, et ta maison compte beaucoup pour nous.",
    ),
    (
        "L'encre bleue",
        "Bravo, c'était le grand arbre. Cette nuit, quelqu'un a marché dans ton jardin. Regarde le sol : les traces mènent quelque part.",
    ),
    (
        "Biscuit, le chien",
        "Wouf ! C'est moi qui dors dans la niche. Je n'ai rien vu cette nuit, je dormais. Enfin, presque. J'ai entendu des petits pas, très légers, puis un rire.",
    ),
    (
        "Gaston, le facteur",
        "Trente ans que je fais la tournée à vélo ! Ta boîte aux lettres est ma préférée : elle grince joliment. Quand son drapeau rouge est levé, c'est qu'une lettre t'attend.",
    ),
    (
        "L'encre bleue",
        "Ces traces sont petites. Elles appartiennent à quelqu'un qui aime grimper aux arbres et sauter dans les flaques. Il en sait plus qu'il ne le dit.",
    ),
    (
        "Hulotte, le hibou",
        "Hou hou. Je veille dans le grand arbre la nuit. Je vois tout : les chauves-souris, les étoiles filantes, et parfois une petite lumière bleue qui n'est pas une étoile.",
    ),
    (
        "Odile, la maire",
        "Grâce aux voisins, le vieux puits de ton jardin est réparé ! Autrefois, on y faisait des vœux. Fais-en un, ça ne coûte rien.",
    ),
    (
        "L'encre bleue",
        "Le puits, oui. Il y a longtemps, quelqu'un y a jeté une pièce en faisant un vœu : que cette maison reste heureuse. Ce vœu, c'est moi qui l'ai fait.",
    ),
    (
        "Biscuit, le chien",
        "J'ai trouvé un os énorme en creusant près de ma niche ! Je l'ai laissé dehors pour que tout le monde l'admire. Personne n'y touche. Wouf.",
    ),
    (
        "Léo, le petit voisin",
        "Bonjour, c'est Léo, celui qui saute dans les flaques chez toi. Pardon pour les traces de boue. Mamie dit que ta maison était la sienne, avant. Je peux continuer à venir jouer ?",
    ),
    (
        "L'encre bleue",
        "Sous ta boîte aux lettres, j'ai laissé une petite clé. Garde-la bien. Elle n'ouvre aucune porte de la maison.",
    ),
    (
        "Coin-Coin, le canard",
        "Coin ! La mare au bout de ton jardin est la plus belle de la région quand il pleut. Je la partage avec la grenouille, mais elle chante faux.",
    ),
    (
        "Rose, la boulangère",
        "Léo m'a commandé un gâteau « pour fêter la cabane », en langue des signes, rien que ça ! Il apprend vite. Une cabane ? Dans l'arbre ? Je crois qu'un projet se prépare en cachette.",
    ),
    (
        "L'encre bleue",
        "La clé et le grand arbre vont ensemble. Mais il faut encore attendre. Les bonnes choses arrivent lentement, comme les escargots.",
    ),
    (
        "Gustave, l'escargot",
        "Je suis parti de la mare il y a trois jours. J'arriverai à la boîte aux lettres… un jour. Ne m'attends pas pour le dîner.",
    ),
    (
        "Paulo, le pêcheur",
        "Avec Léo, on a construit une cabane dans le grand arbre. Il y a une échelle, un toit et une fenêtre pour surveiller la rivière. Interdite aux adultes, paraît-il.",
    ),
    (
        "L'encre bleue",
        "Autrefois, dans ta maison, quelqu'un posait une lanterne bleue à la fenêtre pour guider un pêcheur qui rentrait tard.",
    ),
    (
        "Hulotte, le hibou",
        "Mon cousin le grand-duc est venu me rendre visite. Il trouve ton arbre très confortable et compte rester. Nous serons deux à veiller. Hou hou.",
    ),
    (
        "Gaston, le facteur",
        "Il pleuvait si fort ce matin que mon vélo flottait presque. Le canard m'a doublé. Bizarre : ta boîte aux lettres était déjà ouverte, et toute humide à l'intérieur.",
    ),
    (
        "L'encre bleue",
        "Ce soir, regarde la petite fenêtre en haut à gauche. La lanterne bleue est revenue. Quelqu'un pense encore au pêcheur.",
    ),
    (
        "Noisette, l'écureuil",
        "Tchic tchic ! J'ai caché 147 noisettes dans ton arbre. Je ne me souviens plus où. Si tu en trouves une, elle est à moi.",
    ),
    (
        "Odile, la maire",
        "La fête approche. Chaque maison doit apporter une décoration. La tienne aura des guirlandes, j'ai déjà prévenu Rose.",
    ),
    (
        "L'encre bleue",
        "Le pêcheur s'appelle Paulo. Il ne sait pas que c'est moi qui écris. Garde le secret encore un peu.",
    ),
    (
        "Rainette, la grenouille",
        "Je ne chante pas faux, c'est le canard qui n'a pas d'oreille. Quand il pleut, je saute dans la mare : plouf ! C'est mon meilleur numéro.",
    ),
    (
        "Rose, la boulangère",
        "Avec Léo, on a accroché une guirlande lumineuse sous ton porche. Elle s'allume le soir. Tout le village la voit depuis la route !",
    ),
    (
        "L'encre bleue",
        "J'ai habité ta maison pendant cinquante ans. J'y ai été très heureuse. J'ai laissé quelque chose pour la personne qui vivrait là après moi.",
    ),
    (
        "Minouche, la chatte du toit",
        "Grande nouvelle : j'ai eu deux chatons ! Ils sont nés une nuit de pleine lune, ce qui fait d'eux des chatons très sages. L'un a une tache en forme d'étoile. Biscuit fait semblant de ne pas les aimer.",
    ),
    (
        "Léo, le petit voisin",
        "Mamie s'appelle Jeanne. Elle habite la maison de retraite au bord de la rivière. Elle me demande toujours des nouvelles de ta maison, du grand arbre et du puits.",
    ),
    (
        "L'encre bleue",
        "Au pied du grand arbre, j'ai tracé une croix. Ne creuse pas encore. Attends ma dernière lettre.",
    ),
    (
        "Biscuit, le chien",
        "Il y a une croix rouge par terre près de l'arbre. Je voulais creuser, mais Minouche m'a regardé très sévèrement. Je n'ai pas creusé. Presque pas.",
    ),
    (
        "Gaston, le facteur",
        "Les lettres à l'encre bleue arrivent avec un timbre de la maison de retraite. Les autres, celles sans timbre, je ne sais pas qui les dépose. Je ne distribue rien au numéro 13. Il n'y a pas de numéro 13.",
    ),
    (
        "Jeanne, l'encre bleue",
        "Tu as sûrement deviné. Je m'appelle Jeanne, je suis la mamie de Léo. Paulo et moi, nous nous sommes aimés quand nous étions jeunes, puis la vie nous a séparés.",
    ),
    (
        "Hulotte, le hibou",
        "Cette nuit, j'ai vu Paulo regarder longtemps la lanterne bleue depuis la rivière. Les humains sont compliqués. Hou.",
    ),
    (
        "Odile, la maire",
        "Pour la fête, le village a loué une montgolfière ! Elle passera au-dessus de ta maison les jours de beau temps. Lève les yeux et fais-lui signe : le pilote adore ça.",
    ),
    (
        "Jeanne, l'encre bleue",
        "Dans le coffre sous l'arbre, il y a nos lettres d'amour, des photos du village et une recette de Rose quand elle avait dix ans.",
    ),
    (
        "Coin-Coin, le canard",
        "Coin coin ! Des canetons sont nés au bord de la mare. Ils me suivent partout, même sous la pluie. Surtout sous la pluie.",
    ),
    (
        "Paulo, le pêcheur",
        "Une dame m'a écrit à l'encre bleue. Son écriture me rappelle quelqu'un. Il y a très longtemps, je rentrais tard de la pêche, et une lanterne bleue m'attendait…",
    ),
    (
        "Jeanne, l'encre bleue",
        "Paulo a reçu ma lettre. Il m'a répondu, avec des fautes d'orthographe, comme avant. J'ai pleuré de joie. Merci d'avoir gardé la lanterne.",
    ),
    (
        "Noisette, l'écureuil",
        "J'ai retrouvé une noisette ! Ah non, c'est un bouton. Ça compte quand même. Tchic.",
    ),
    (
        "Rose, la boulangère",
        "Gros secret : Paulo m'a commandé un gâteau en forme de cœur. Sur mon carnet, j'ai écrit « Pour qui ? ». Il a rougi et il est reparti sans répondre.",
    ),
    (
        "Jeanne, l'encre bleue",
        "Bientôt, je viendrai au village pour la fête. Ce sera la première fois depuis des années que je reverrai ma maison. Ta maison.",
    ),
    (
        "Gustave, l'escargot",
        "Je suis arrivé à la boîte aux lettres ! Il m'a fallu des semaines. Maintenant, je repars vers la mare. À dans un mois.",
    ),
    (
        "Gaston, le facteur",
        "J'ai installé une girouette sur ton toit, cadeau des anciens du village. Elle indique le vent. Sauf la nuit : la nuit, elle tourne vers le puits, même quand il n'y a pas un souffle.",
    ),
    (
        "Jeanne, l'encre bleue",
        "La clé sous la boîte aux lettres ouvre le coffre. Le jour de la fête, nous l'ouvrirons ensemble, avec Paulo et Léo.",
    ),
    (
        "Rainette, la grenouille",
        "Les canetons veulent apprendre à sauter comme moi. Ils tombent tous dans la mare. C'est normal, c'est une mare.",
    ),
    (
        "Léo, le petit voisin",
        "On a préparé les décorations pour la fête. Mamie arrive bientôt !",
    ),
    (
        "Jeanne, l'encre bleue",
        "Il y a une dernière chose dans le coffre : une lettre écrite il y a quarante ans pour la personne qui habiterait ma maison. Pour toi.",
    ),
    (
        "Minouche, la chatte du toit",
        "Mes chatons ont appris à grimper sur la cheminée. Ils disent que, de là-haut, on entend mieux les étoiles. Je fais semblant de ne pas les connaître.",
    ),
    (
        "Odile, la maire",
        "Tout le village est prêt. La fête aura lieu dans ton jardin, sous le grand arbre. Rose apporte les gâteaux, Paulo s'occupe de la musique. Il ne manque plus que toi.",
    ),
    (
        "Jeanne, l'encre bleue",
        "Demain, je viens. J'ai sorti ma plus belle robe bleue. Paulo dit qu'il apportera des fleurs de la rivière.",
    ),
    (
        "Biscuit, le chien",
        "Il y a beaucoup de monde dans le jardin. Je garde mon os bien caché. Et je surveille la croix rouge. Wouf !",
    ),
    (
        "Rose, la boulangère",
        "Quelle fête ! Les décorations ont tenu toute la soirée, Paulo et Jeanne ont dansé, et Léo a mangé trois parts de gâteau. Moi, j'ai dansé tout près des enceintes, là où la musique se sent le mieux. Merci d'avoir ouvert ton jardin.",
    ),
    (
        "Jeanne, l'encre bleue",
        "Nous avons ouvert le coffre au pied de l'arbre. Il est encore là, plein de souvenirs. La lettre qui t'était destinée dit simplement : prends soin de cette maison, elle te le rendra.",
    ),
    (
        "Tout le village",
        "Merci d'avoir lu nos lettres. Rose fait ses croissants, Paulo pêche avec Jeanne, Léo saute dans les flaques et Gaston passe toujours. Notre histoire se termine, la tienne continue. À bientôt !",
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

pub struct Letter {
    pub sender: String,
    pub body: String,
}

/// Built-in village Lettres, before any `lettres.txt` override.
pub fn defaults() -> Vec<Letter> {
    DEFAULT_LETTERS
        .iter()
        .map(|(sender, body)| Letter {
            sender: sender.to_string(),
            body: body.to_string(),
        })
        .collect()
}

/// Village Lettres, with `lettres.txt` overrides applied (written with defaults when missing).
pub fn load() -> Vec<Letter> {
    let mut letters = defaults();

    let Some(path) = letters_path() else {
        return letters;
    };
    match fs::read_to_string(&path) {
        Ok(content) => apply_overrides(&mut letters, &content),
        Err(_) => {
            let _ = write_defaults(&path, &letters);
        }
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
