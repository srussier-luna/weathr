use chrono::{Datelike, Month, NaiveDate};
use std::collections::BTreeSet;

pub const ROSE_SENDER: &str = "Rose, la boulangère";

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tag {
    Meat,
    Pork,
    Fish,
    Vegetables,
    Fruit,
    AnimalProducts,
    Protein,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Diet {
    Vegetarian,
    Vegan,
    NoVegetable,
    NoMeat,
    NoFish,
    NoPork,
    Protein,
    NoFruit,
}

pub const DIETS: [Diet; 8] = [
    Diet::Vegetarian,
    Diet::Vegan,
    Diet::NoVegetable,
    Diet::NoMeat,
    Diet::NoFish,
    Diet::NoPork,
    Diet::Protein,
    Diet::NoFruit,
];

impl Diet {
    pub fn label(self) -> &'static str {
        match self {
            Self::Vegetarian => "végétarien",
            Self::Vegan => "vegan",
            Self::NoVegetable => "sans légume",
            Self::NoMeat => "sans viande",
            Self::NoFish => "sans poisson",
            Self::NoPork => "sans porc",
            Self::Protein => "full protéine",
            Self::NoFruit => "sans fruit",
        }
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Vegetarian => "vegetarian",
            Self::Vegan => "vegan",
            Self::NoVegetable => "no_vegetable",
            Self::NoMeat => "no_meat",
            Self::NoFish => "no_fish",
            Self::NoPork => "no_pork",
            Self::Protein => "protein",
            Self::NoFruit => "no_fruit",
        }
    }

    fn allows(self, recipe: &Recipe) -> bool {
        let has_meat = recipe.has(Tag::Meat) || recipe.has(Tag::Pork);
        match self {
            Self::Vegetarian => !has_meat && !recipe.has(Tag::Fish),
            Self::Vegan => Self::Vegetarian.allows(recipe) && !recipe.has(Tag::AnimalProducts),
            Self::NoVegetable => !recipe.has(Tag::Vegetables),
            Self::NoMeat => !has_meat,
            Self::NoFish => !recipe.has(Tag::Fish),
            Self::NoPork => !recipe.has(Tag::Pork),
            Self::Protein => recipe.has(Tag::Protein),
            Self::NoFruit => !recipe.has(Tag::Fruit),
        }
    }
}

pub fn diets_from_ids<'a>(ids: impl IntoIterator<Item = &'a String>) -> BTreeSet<Diet> {
    ids.into_iter()
        .filter_map(|id| DIETS.into_iter().find(|diet| diet.id() == id))
        .collect()
}

pub fn diet_ids(diets: &BTreeSet<Diet>) -> BTreeSet<String> {
    diets.iter().map(|diet| diet.id().to_string()).collect()
}

pub fn diet_labels(diets: &BTreeSet<Diet>) -> String {
    diets
        .iter()
        .map(|diet| diet.label())
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn diet_for_key(key: char) -> Option<Diet> {
    let index = key.to_digit(10)?.checked_sub(1)?;
    DIETS.get(usize::try_from(index).ok()?).copied()
}

pub fn has_every_recipe(diets: &BTreeSet<Diet>, received: &BTreeSet<usize>) -> bool {
    RECIPES
        .iter()
        .enumerate()
        .filter(|(_, recipe)| recipe.suits(diets))
        .all(|(index, _)| received.contains(&index))
}

pub fn toggle_diet(diets: &mut BTreeSet<Diet>, diet: Diet) {
    if !diets.remove(&diet) {
        diets.insert(diet);
    }
}

pub struct Recipe {
    pub name: &'static str,
    pub body: &'static str,
    month: Option<Month>,
    tags: &'static [Tag],
}

impl Recipe {
    fn has(&self, tag: Tag) -> bool {
        self.tags.contains(&tag)
    }

    pub fn suits(&self, diets: &BTreeSet<Diet>) -> bool {
        diets.iter().all(|diet| diet.allows(self))
    }

    pub fn is_in_season(&self, date: NaiveDate) -> bool {
        self.month
            .is_none_or(|month| month.number_from_month() == date.month())
    }
}

const fn recipe(
    name: &'static str,
    month: Option<Month>,
    tags: &'static [Tag],
    body: &'static str,
) -> Recipe {
    Recipe {
        name,
        body,
        month,
        tags,
    }
}

pub const RECIPES: [Recipe; 40] = [
    recipe(
        "Galette des rois",
        Some(Month::January),
        &[],
        "Deux pâtes feuilletées (au beurre ou à l'huile), 100 g de poudre d'amande, 80 g de sucre, 60 g de beurre mou (ou de margarine), 1 œuf (ou une cuillère de fécule). Mélange le tout, étale sur la première pâte, cache la fève, couvre avec la seconde. Dore à l'œuf (ou au lait végétal), 30 min à 200 °C. Qui trouve la fève fait la vaisselle.",
    ),
    recipe(
        "Crêpes de la Chandeleur",
        Some(Month::February),
        &[],
        "250 g de farine, 4 œufs (ou 2 cuillères de fécule), 50 cl de lait (ou de lait végétal), une pincée de sel, 50 g de beurre fondu (ou d'huile). Fouette, laisse reposer une heure si tu as la patience. Fais sauter la crêpe avec une pièce dans l'autre main, pour la chance.",
    ),
    recipe(
        "Pain perdu",
        None,
        &[],
        "Du pain rassis, 2 œufs (ou une cuillère de fécule), 25 cl de lait (ou de lait végétal), du sucre, de la cannelle. Trempe les tranches, dore-les au beurre (ou à l'huile). Rien ne se perd chez Rose, surtout pas le pain.",
    ),
    recipe(
        "Gâteau au yaourt",
        None,
        &[],
        "1 pot de yaourt (ou de yaourt au soja), 2 pots de sucre, 3 pots de farine, 3 œufs (ou un second pot de yaourt au soja), un demi-pot d'huile, 1 sachet de levure. Mélange dans l'ordre, 35 min à 180 °C. Le pot sert de mesure\u{202F}: c'est tout le secret.",
    ),
    recipe(
        "Clafoutis aux cerises",
        Some(Month::May),
        &[Tag::Fruit],
        "500 g de cerises avec leurs noyaux (ça parfume), 3 œufs (ou 3 cuillères de fécule), 100 g de sucre, 80 g de farine, 30 cl de lait (ou de lait végétal). Verse la pâte sur les cerises, 40 min à 180 °C.",
    ),
    recipe(
        "Tarte aux fraises",
        Some(Month::June),
        &[Tag::Fruit, Tag::AnimalProducts],
        "Une pâte sablée cuite à blanc, de la crème pâtissière, des fraises coupées en deux et rangées en rosace. À manger le jour même, ou pas du tout.",
    ),
    recipe(
        "Sorbet citron",
        None,
        &[Tag::Fruit],
        "4 citrons, 200 g de sucre, 50 cl d'eau. Fais un sirop, ajoute le jus, mets au congélateur et gratte à la fourchette toutes les heures. Léo appelle ça « la neige d'été ».",
    ),
    recipe(
        "Tarte aux mirabelles",
        Some(Month::August),
        &[Tag::Fruit],
        "Une pâte brisée (au beurre ou à l'huile), 1 kg de mirabelles dénoyautées et bien serrées, deux cuillères de sucre. 35 min à 200 °C. Paulo en a déjà mangé trois parts.",
    ),
    recipe(
        "Tarte aux pommes",
        None,
        &[Tag::Fruit],
        "Une pâte brisée (au beurre ou à l'huile), de la compote, des pommes en fines lamelles, une noisette de beurre (ou de margarine), du sucre. 40 min à 180 °C. Jeanne dit que la mienne est presque aussi bonne que la sienne.",
    ),
    recipe(
        "Soupe de potiron",
        Some(Month::October),
        &[Tag::Vegetables],
        "Un potiron, une pomme de terre, un oignon, un litre de bouillon de légumes, un peu de crème (ou de crème d'avoine). Cuis 30 min, mixe. Garde l'écorce pour la lanterne d'Halloween.",
    ),
    recipe(
        "Gâteau de Savoie",
        None,
        &[Tag::AnimalProducts],
        "4 œufs séparés, 125 g de sucre, 60 g de farine, 60 g de fécule. Blanchis les jaunes avec le sucre, ajoute farine et fécule, puis les blancs montés. 35 min à 170 °C. Léger comme un nuage de chez nous.",
    ),
    recipe(
        "Bûche de Noël",
        Some(Month::December),
        &[Tag::AnimalProducts],
        "Un biscuit roulé, une crème au chocolat, du sucre glace pour la neige. Roule, couvre, strie à la fourchette pour faire l'écorce. Un champignon en meringue pour faire rire les enfants.",
    ),
    recipe(
        "Soupe poireaux-pommes de terre",
        Some(Month::March),
        &[Tag::Vegetables],
        "3 poireaux, 3 pommes de terre, un litre de bouillon de légumes, un peu de crème (ou de crème d'avoine). Émince, cuis 25 min, mixe ou pas, selon ton humeur. C'est la soupe des giboulées\u{202F}: elle mijote pendant que la grêle tape au carreau.",
    ),
    recipe(
        "Gaspacho",
        Some(Month::July),
        &[Tag::Vegetables],
        "1 kg de tomates bien mûres, un concombre, un poivron rouge, une gousse d'ail, une tranche de pain rassis, un filet d'huile d'olive. Mixe tout, sale, deux heures au frais. Ça se boit dans un verre, à l'ombre du grand arbre.",
    ),
    recipe(
        "Châtaignes grillées",
        Some(Month::November),
        &[],
        "Un kilo de châtaignes, une poêle percée, un torchon humide. Fends les écorces en croix, grille 20 min en secouant souvent, puis laisse-les 5 min dans le torchon. Elles s'épluchent alors toutes seules, ou presque. Les doigts noirs, c'est le prix à payer.",
    ),
    recipe(
        "Pain de campagne",
        None,
        &[],
        "500 g de farine, 35 cl d'eau tiède, 10 g de sel, 10 g de levure. Pétris dix minutes, laisse lever deux heures sous un torchon, façonne une boule. 40 min à 230 °C. Ne le répète à personne\u{202F}: c'est mon gagne-pain.",
    ),
    recipe(
        "Omelette au fromage",
        None,
        &[Tag::AnimalProducts, Tag::Protein],
        "3 œufs, 50 g de comté râpé, une noix de beurre. Bats les œufs, verse-les dans le beurre mousseux, ajoute le fromage et plie quand le dessus tremble encore. Gaston la mange debout, entre deux boîtes aux lettres.",
    ),
    recipe(
        "Houmous",
        None,
        &[Tag::Fruit, Tag::Protein],
        "400 g de pois chiches cuits, 2 cuillères de purée de sésame, le jus d'un citron, un filet d'huile d'olive, du cumin. Mixe jusqu'à ce que ce soit lisse, et trempe-y mon pain. Une gousse d'ail en plus, si personne ne doit t'embrasser ce soir.",
    ),
    recipe(
        "Poulet rôti",
        None,
        &[Tag::Meat, Tag::AnimalProducts, Tag::Protein],
        "Un poulet fermier, 50 g de beurre mou, du thym, quelques gousses d'ail en chemise. Masse-le au beurre, 1 h 15 à 200 °C, arrose-le de son jus toutes les vingt minutes. Biscuit attendra sous la table\u{202F}: fais semblant de ne pas le voir.",
    ),
    recipe(
        "Cookies dattes et avoine",
        None,
        &[Tag::Fruit, Tag::Protein],
        "150 g de dattes dénoyautées, 200 g de flocons d'avoine, 2 cuillères de purée de cacahuète, un peu d'eau chaude. Mixe les dattes avec l'eau, mélange avec l'avoine et la cacahuète, forme des palets. 12 min à 180 °C. Gaston en garde deux dans sa sacoche, pour les côtes.",
    ),
    recipe(
        "Truite aux amandes",
        None,
        &[Tag::Fish, Tag::AnimalProducts, Tag::Protein],
        "2 truites, un peu de farine, 40 g de beurre, 50 g d'amandes effilées, du persil. Dore les truites farinées 5 min de chaque côté, puis les amandes dans le même beurre. Pour les truites, demande à Paulo\u{202F}: il en a toujours une de trop, dit-il.",
    ),
    recipe(
        "Salade de lentilles",
        None,
        &[Tag::Vegetables, Tag::Protein],
        "250 g de lentilles vertes, une carotte, une échalote, une vinaigrette bien moutardée. Cuis les lentilles 25 min avec la carotte en rondelles, sale à la fin, mélange tiède avec l'échalote et la vinaigrette. Odile en apporte un saladier à chaque conseil municipal.",
    ),
    recipe(
        "Œufs au lait",
        None,
        &[Tag::AnimalProducts, Tag::Protein],
        "4 œufs, 50 cl de lait, 100 g de sucre, une gousse de vanille. Fais tiédir le lait avec la vanille, verse-le sur les œufs battus avec le sucre. 45 min à 150 °C, au bain-marie. Il tremble quand on le pose sur la table, comme Biscuit les soirs d'orage.",
    ),
    recipe(
        "Quiche lorraine",
        None,
        &[Tag::Pork, Tag::AnimalProducts, Tag::Protein],
        "Une pâte brisée, 200 g de lardons, 3 œufs, 20 cl de crème, 20 cl de lait. Dore les lardons, étale-les sur la pâte, verse les œufs battus avec la crème et le lait. 35 min à 180 °C. Pas de fromage dedans\u{202F}: en Lorraine, on se fâche pour moins que ça.",
    ),
    recipe(
        "Compote de pommes",
        None,
        &[Tag::Fruit],
        "1 kg de pommes, un fond d'eau, un bâton de cannelle. Épluche, coupe en morceaux, laisse compoter 20 min à couvert, écrase à la fourchette. Prends les pommes tombées\u{202F}: ce sont les plus sucrées, et personne n'en veut.",
    ),
    recipe(
        "Sardines grillées",
        None,
        &[Tag::Fish, Tag::Protein],
        "Une douzaine de sardines, du gros sel, un filet d'huile d'olive. Sale-les une demi-heure avant, puis 3 min de chaque côté sur un gril bien chaud. Fais-les dehors, et surveille le toit\u{202F}: Minouche a déjà senti l'odeur.",
    ),
    recipe(
        "Tofu doré au sésame",
        None,
        &[Tag::Protein],
        "400 g de tofu ferme, 2 cuillères de fécule, 3 cuillères de sauce soja, du sésame. Roule les cubes dans la fécule, dore-les à la poêle, sauce soja et sésame à la fin. Paulo a cru que c'était du fromage. Il en a repris trois fois quand même.",
    ),
    recipe(
        "Croque-monsieur",
        None,
        &[Tag::Pork, Tag::AnimalProducts, Tag::Protein],
        "Du pain de mie, du beurre, 4 tranches de jambon, 100 g de gruyère râpé. Jambon et fromage entre deux tranches beurrées, gruyère par-dessus, 10 min à 200 °C. Le pain de mie, c'est moi qui le fais\u{202F}: ne va pas l'acheter ailleurs.",
    ),
    recipe(
        "Crème au chocolat",
        None,
        &[Tag::Protein],
        "400 g de tofu soyeux, 150 g de chocolat noir fondu, 2 cuillères de sucre. Mixe le tout jusqu'à ce que ce soit bien lisse, verse en ramequins, deux heures au frais. Ne dis pas à Paulo ce qu'il y a dedans\u{202F}: il en est à son troisième.",
    ),
    recipe(
        "Ratatouille",
        None,
        &[Tag::Vegetables],
        "2 courgettes, une aubergine, 2 poivrons, 4 tomates, 2 oignons, de l'ail, du thym, de l'huile d'olive. Cuis chaque légume à part, puis réunis-les et laisse mijoter une heure à feu doux. C'est long, oui. C'est pour ça que c'est bon.",
    ),
    recipe(
        "Soufflé au fromage",
        None,
        &[Tag::AnimalProducts, Tag::Protein],
        "40 g de beurre, 40 g de farine, 25 cl de lait, 4 œufs séparés, 100 g de comté râpé. Fais une béchamel épaisse, ajoute les jaunes, le fromage, puis les blancs en neige. 25 min à 200 °C sans ouvrir le four. Un soufflé n'attend personne\u{202F}: tout le monde à table avant.",
    ),
    recipe(
        "Chili sin carne",
        None,
        &[Tag::Vegetables, Tag::Protein],
        "100 g de protéines de soja texturées, 250 g de haricots rouges cuits, un oignon, un poivron, 400 g de tomates concassées, du cumin, du piment. Réhydrate le soja dans du bouillon, ajoute le reste, 30 min à feu doux. «\u{202F}Sin carne\u{202F}», ça veut dire sans viande. Biscuit a quand même fouillé la casserole.",
    ),
    recipe(
        "Hachis parmentier",
        None,
        &[Tag::Meat, Tag::AnimalProducts, Tag::Protein],
        "500 g de bœuf haché, 1 kg de pommes de terre, 20 cl de lait, 50 g de beurre, du gruyère. Une purée bien beurrée sur la viande dorée, gruyère dessus, 20 min à 200 °C. Le bord qui gratine revient à qui a épluché.",
    ),
    recipe(
        "Fougasse aux olives",
        None,
        &[],
        "500 g de farine, 30 cl d'eau, 10 g de levure, 10 g de sel, un filet d'huile d'olive, une poignée d'olives noires. Laisse lever une heure, étale, fais de grandes entailles, 20 min à 220 °C. Elle se déchire à la main, jamais au couteau\u{202F}: c'est fait pour partager.",
    ),
    recipe(
        "Tarte au fromage blanc",
        None,
        &[Tag::AnimalProducts, Tag::Protein],
        "Une pâte brisée, 500 g de fromage blanc, 3 œufs, 100 g de sucre, 2 cuillères de fécule. Bats le tout, verse sur la pâte, 45 min à 180 °C. Elle gonfle au four puis retombe, c'est normal. Odile en prend toujours «\u{202F}juste une miette\u{202F}», trois fois.",
    ),
    recipe(
        "Pot-au-feu",
        None,
        &[Tag::Meat, Tag::Vegetables, Tag::Protein],
        "1,5 kg de bœuf à bouillir, un os à moelle, 4 carottes, 2 poireaux, 2 navets, 4 pommes de terre, un oignon piqué de clous de girofle. Couvre d'eau froide, laisse frémir trois heures. C'est la recette des dimanches de pluie.",
    ),
    recipe(
        "Soupe de pois cassés",
        None,
        &[Tag::Vegetables, Tag::Protein],
        "250 g de pois cassés, une carotte, un oignon, un litre et demi d'eau, du thym. Une heure à petit feu, puis mixe. Elle épaissit en refroidissant\u{202F}: rallonge-la d'eau le lendemain. Les jours de grand froid, elle réchauffe jusqu'au bout des doigts.",
    ),
    recipe(
        "Poires pochées au vin",
        None,
        &[Tag::Fruit],
        "4 poires, 75 cl de vin rouge, 150 g de sucre, un bâton de cannelle. Épluche les poires en gardant la queue, fais-les frémir 30 min dans le vin sucré. Laisse-les refroidir dedans toute une nuit\u{202F}: au matin, elles ont rougi jusqu'au cœur.",
    ),
    recipe(
        "Papillote de cabillaud",
        None,
        &[Tag::Fish, Tag::Vegetables, Tag::Protein],
        "2 filets de cabillaud, 10 tomates cerises, un filet d'huile d'olive, de l'aneth. Ferme le tout dans du papier cuisson, bien serré, comme une enveloppe. 15 min à 200 °C. On l'ouvre à table, et tout le monde se penche pour sentir.",
    ),
    recipe(
        "Taboulé",
        None,
        &[Tag::Vegetables, Tag::Fruit],
        "250 g de semoule, 3 tomates, un demi-concombre, un bouquet de persil et de menthe, le jus de 2 citrons, un filet d'huile d'olive. Mélange tout, une nuit au frais\u{202F}: la semoule gonfle toute seule. À manger dehors, sur le banc devant ta maison.",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;

    fn compatible(diets: &[Diet]) -> Vec<usize> {
        let diets: BTreeSet<Diet> = diets.iter().copied().collect();
        RECIPES
            .iter()
            .enumerate()
            .filter(|(_, recipe)| recipe.suits(&diets))
            .map(|(index, _)| index + 1)
            .collect()
    }

    fn date(month: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2027, month, 15).unwrap()
    }

    #[test]
    fn test_every_diet_keeps_at_least_twelve_recipes() {
        let counts: Vec<(Diet, usize, usize)> = DIETS
            .into_iter()
            .map(|diet| {
                let recipes = compatible(&[diet]);
                let all_year = recipes
                    .iter()
                    .filter(|number| RECIPES[*number - 1].month.is_none())
                    .count();
                (diet, recipes.len(), all_year)
            })
            .collect();
        assert_eq!(
            counts,
            [
                (Diet::Vegetarian, 32, 22),
                (Diet::Vegan, 25, 17),
                (Diet::NoVegetable, 30, 23),
                (Diet::NoMeat, 35, 25),
                (Diet::NoFish, 37, 27),
                (Diet::NoPork, 38, 28),
                (Diet::Protein, 19, 19),
                (Diet::NoFruit, 30, 23),
            ]
        );
    }

    #[test]
    fn test_every_diet_must_be_met_at_once() {
        assert_eq!(compatible(&[]).len(), RECIPES.len());
        assert_eq!(
            compatible(&[Diet::Vegan, Diet::NoVegetable, Diet::Protein]),
            [18, 20, 27, 29]
        );
        assert_eq!(
            compatible(&[Diet::Vegan, Diet::Protein, Diet::NoFruit]),
            [22, 27, 29, 32, 37]
        );
        assert_eq!(
            compatible(&[Diet::Vegan, Diet::NoVegetable, Diet::NoFruit]),
            [1, 2, 3, 4, 15, 16, 27, 29, 34]
        );
        assert_eq!(
            compatible(&[Diet::Vegetarian, Diet::NoVegetable, Diet::Protein]),
            [17, 18, 20, 23, 27, 29, 31, 35]
        );
        assert_eq!(
            compatible(&[Diet::NoVegetable, Diet::Protein, Diet::NoFruit]),
            [17, 19, 21, 23, 24, 26, 27, 28, 29, 31, 33, 35]
        );
    }

    #[test]
    fn test_pork_and_meat_rule_out_vegetarian_and_no_meat() {
        let quiche = &RECIPES[23];
        let pot_au_feu = &RECIPES[35];
        for recipe in [quiche, pot_au_feu] {
            assert!(!Diet::Vegetarian.allows(recipe));
            assert!(!Diet::NoMeat.allows(recipe));
            assert!(Diet::NoFish.allows(recipe));
        }
        assert!(!Diet::NoPork.allows(quiche));
        assert!(Diet::NoPork.allows(pot_au_feu));
    }

    #[test]
    fn test_seasonal_recipes_come_only_in_their_month() {
        let seasonal: Vec<(&str, u32)> = RECIPES
            .iter()
            .filter_map(|recipe| Some((recipe.name, recipe.month?.number_from_month())))
            .collect();
        assert_eq!(
            seasonal,
            [
                ("Galette des rois", 1),
                ("Crêpes de la Chandeleur", 2),
                ("Clafoutis aux cerises", 5),
                ("Tarte aux fraises", 6),
                ("Tarte aux mirabelles", 8),
                ("Soupe de potiron", 10),
                ("Bûche de Noël", 12),
                ("Soupe poireaux-pommes de terre", 3),
                ("Gaspacho", 7),
                ("Châtaignes grillées", 11),
            ]
        );
        assert!(RECIPES[0].is_in_season(date(1)));
        assert!(!RECIPES[0].is_in_season(date(2)));
        assert!((1..=12).all(|month| RECIPES[2].is_in_season(date(month))));
    }

    #[test]
    fn test_monthly_recipes_open_the_reserve_in_calendar_order() {
        let names: Vec<&str> = RECIPES[..12].iter().map(|recipe| recipe.name).collect();
        assert_eq!(
            names,
            [
                "Galette des rois",
                "Crêpes de la Chandeleur",
                "Pain perdu",
                "Gâteau au yaourt",
                "Clafoutis aux cerises",
                "Tarte aux fraises",
                "Sorbet citron",
                "Tarte aux mirabelles",
                "Tarte aux pommes",
                "Soupe de potiron",
                "Gâteau de Savoie",
                "Bûche de Noël",
            ]
        );
    }

    #[test]
    fn test_every_recipe_means_every_one_that_suits_the_diets() {
        let vegan = BTreeSet::from([Diet::Vegan]);
        let mut received: BTreeSet<usize> = compatible(&[Diet::Vegan])
            .into_iter()
            .map(|number| number - 1)
            .collect();
        assert!(has_every_recipe(&vegan, &received));
        let last = *received.iter().next_back().unwrap();
        received.remove(&last);
        assert!(!has_every_recipe(&vegan, &received));
    }

    #[test]
    fn test_recipes_outside_the_diets_are_not_needed() {
        let vegan = BTreeSet::from([Diet::Vegan]);
        let all: BTreeSet<usize> = (0..RECIPES.len()).collect();
        assert!(has_every_recipe(&vegan, &all));
        assert!(!has_every_recipe(
            &BTreeSet::new(),
            &all.iter().copied().skip(1).collect()
        ));
    }

    #[test]
    fn test_diets_round_trip_through_their_ids() {
        let diets = BTreeSet::from([Diet::Vegan, Diet::NoFruit]);
        let ids = diet_ids(&diets);
        assert_eq!(ids, BTreeSet::from(["vegan".into(), "no_fruit".into()]));
        assert_eq!(diets_from_ids(&ids), diets);
        assert_eq!(
            diets_from_ids(&BTreeSet::from(["paleo".into()])),
            BTreeSet::new()
        );
        assert_eq!(diet_labels(&diets), "vegan, sans fruit");
    }

    #[test]
    fn test_keys_check_then_uncheck_their_diet() {
        assert_eq!(diet_for_key('1'), Some(Diet::Vegetarian));
        assert_eq!(diet_for_key('8'), Some(Diet::NoFruit));
        for key in ['0', '9', 'a'] {
            assert_eq!(diet_for_key(key), None);
        }
        let mut diets = BTreeSet::new();
        toggle_diet(&mut diets, Diet::Vegetarian);
        toggle_diet(&mut diets, Diet::NoPork);
        assert_eq!(diets, BTreeSet::from([Diet::Vegetarian, Diet::NoPork]));
        toggle_diet(&mut diets, Diet::Vegetarian);
        assert_eq!(diets, BTreeSet::from([Diet::NoPork]));
    }
}
