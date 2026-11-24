use crate::weather::WeatherCondition;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BadgeKind {
    Weather,
    Holiday,
    Loyalty,
    Common,
    Rare,
    Epic,
    Secret,
    Demerit,
}

impl BadgeKind {
    pub fn title(&self) -> &'static str {
        match self {
            Self::Weather => "Météo",
            Self::Holiday => "Fêtes",
            Self::Loyalty => "Fidélité",
            Self::Common => "Commun",
            Self::Rare => "Rares",
            Self::Epic => "Épique",
            Self::Secret => "Secrets",
            Self::Demerit => "Mauvais points",
        }
    }
}

pub struct Badge {
    pub id: &'static str,
    pub name: &'static str,
    pub kind: BadgeKind,
    pub hint: Option<&'static str>,
    pub icon: [&'static str; 3],
}

const fn badge(
    id: &'static str,
    name: &'static str,
    kind: BadgeKind,
    icon: [&'static str; 3],
) -> Badge {
    Badge {
        id,
        name,
        kind,
        hint: None,
        icon,
    }
}

const fn hinted(
    id: &'static str,
    name: &'static str,
    kind: BadgeKind,
    hint: &'static str,
    icon: [&'static str; 3],
) -> Badge {
    Badge {
        id,
        name,
        kind,
        hint: Some(hint),
        icon,
    }
}

const fn secret(
    id: &'static str,
    name: &'static str,
    hint: &'static str,
    icon: [&'static str; 3],
) -> Badge {
    hinted(id, name, BadgeKind::Secret, hint, icon)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tier {
    Chocolate,
    Bronze,
    Silver,
    Gold,
    Platinum,
}

impl Tier {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Chocolate => "chocolat",
            Self::Bronze => "bronze",
            Self::Silver => "argent",
            Self::Gold => "or",
            Self::Platinum => "platine",
        }
    }

    pub fn threshold(&self) -> usize {
        match self {
            Self::Chocolate => 1,
            Self::Bronze => 5,
            Self::Silver => 15,
            Self::Gold => 30,
            Self::Platinum => 40,
        }
    }

    pub fn reached(sam_cities: usize) -> Option<Self> {
        TIERS
            .into_iter()
            .rev()
            .find(|tier| sam_cities >= tier.threshold())
    }

    pub fn crossed(before: usize, after: usize) -> Vec<Self> {
        TIERS
            .into_iter()
            .filter(|tier| (before + 1..=after).contains(&tier.threshold()))
            .collect()
    }

    fn next(&self) -> Option<Self> {
        TIERS
            .into_iter()
            .find(|tier| tier.threshold() > self.threshold())
    }
}

#[derive(Clone, Copy)]
pub struct Award {
    pub badge: &'static Badge,
    tier: Option<Tier>,
}

impl From<&'static Badge> for Award {
    fn from(badge: &'static Badge) -> Self {
        Self { badge, tier: None }
    }
}

impl Award {
    pub fn sam_trail(tier: Tier) -> Self {
        Self {
            badge: &SAM_TRAIL,
            tier: Some(tier),
        }
    }

    pub fn is_new_tier(&self) -> bool {
        self.tier.is_some_and(|tier| tier != Tier::Chocolate)
    }

    pub fn name(&self) -> String {
        self.tier
            .map_or_else(|| self.badge.name.to_string(), sam_trail_name)
    }

    pub fn hint(&self) -> Option<String> {
        match self.tier {
            Some(tier) if self.is_new_tier() => {
                Some(format!("{} villes de Sam visitées", tier.threshold()))
            }
            _ => self.badge.hint.map(str::to_string),
        }
    }
}

fn sam_trail_name(tier: Tier) -> String {
    format!("{} · {}", SAM_TRAIL.name, tier.name())
}

pub fn sam_trail_line(sam_cities: usize) -> Option<String> {
    let tier = Tier::reached(sam_cities)?;
    let cities = if sam_cities > 1 { "villes" } else { "ville" };
    let name = sam_trail_name(tier);
    Some(match tier.next() {
        Some(next) => format!(
            "{name} ({sam_cities} {cities}, prochain palier\u{202F}: {} à {})",
            next.name(),
            next.threshold()
        ),
        None => format!("{name} ({sam_cities} {cities})"),
    })
}

const SKY_LOOKOUT_NAME: &str = "Denis Richard";
pub const CHEATER: &str = "cheater";

const TIERS: [Tier; 5] = [
    Tier::Chocolate,
    Tier::Bronze,
    Tier::Silver,
    Tier::Gold,
    Tier::Platinum,
];

pub const SAM_TRAIL: Badge = secret(
    "sam_trail",
    "Sur les traces de Sam",
    "visité la même ville que Sam",
    ["   ::", " ::  ", "::   "],
);

const CONTRIBUTORS: [&str; 1] = ["bpisano"];

pub const KINDS: [BadgeKind; 8] = [
    BadgeKind::Weather,
    BadgeKind::Holiday,
    BadgeKind::Loyalty,
    BadgeKind::Common,
    BadgeKind::Rare,
    BadgeKind::Epic,
    BadgeKind::Secret,
    BadgeKind::Demerit,
];

pub const BADGES: &[Badge] = &[
    badge(
        "clear",
        "Ciel dégagé",
        BadgeKind::Weather,
        [" \\|/ ", "--O--", " /|\\ "],
    ),
    badge(
        "partly_cloudy",
        "Éclaircies",
        BadgeKind::Weather,
        ["\\|/  ", "-O.-.", " (__)"],
    ),
    badge(
        "cloudy",
        "Nuageux",
        BadgeKind::Weather,
        ["  _  ", " ( ) ", "(___)"],
    ),
    badge(
        "overcast",
        "Couvert",
        BadgeKind::Weather,
        [" _ _ ", "(_)_)", "(___)"],
    ),
    badge(
        "fog",
        "Brouillard",
        BadgeKind::Weather,
        ["~~~~~", " ~~~ ", "~~~~~"],
    ),
    badge(
        "drizzle",
        "Bruine",
        BadgeKind::Weather,
        ["(___)", " ' ' ", "' ' '"],
    ),
    badge(
        "rain",
        "Pluie",
        BadgeKind::Weather,
        ["(___)", " / / ", "/ / /"],
    ),
    badge(
        "freezing_rain",
        "Pluie verglaçante",
        BadgeKind::Weather,
        ["(___)", " /*/ ", "*/ /*"],
    ),
    badge(
        "rain_showers",
        "Averses",
        BadgeKind::Weather,
        ["(___)", "/////", "/////"],
    ),
    badge(
        "snow",
        "Neige",
        BadgeKind::Weather,
        ["  *  ", "* * *", "  *  "],
    ),
    badge(
        "snow_grains",
        "Grésil",
        BadgeKind::Weather,
        ["(___)", " .:. ", ".:.:."],
    ),
    badge(
        "snow_showers",
        "Averses de neige",
        BadgeKind::Weather,
        ["(___)", "* * *", " * * "],
    ),
    badge(
        "thunderstorm",
        "Orage",
        BadgeKind::Weather,
        ["(___)", " _/  ", " /   "],
    ),
    badge(
        "thunderstorm_hail",
        "Orage de grêle",
        BadgeKind::Weather,
        ["(___)", " _/ o", "o/ o "],
    ),
    badge(
        "christmas",
        "Noël",
        BadgeKind::Holiday,
        ["  *  ", " /^\\ ", "/___\\"],
    ),
    badge(
        "national_day",
        "14 juillet",
        BadgeKind::Holiday,
        ["\\ * /", "* + *", "/ * \\"],
    ),
    badge(
        "halloween",
        "Halloween",
        BadgeKind::Holiday,
        [" _|_ ", "(^ ^)", "(\\_/)"],
    ),
    badge(
        "new_year",
        "Nouvel An",
        BadgeKind::Holiday,
        [" \\ / ", "  |  ", " _|_ "],
    ),
    badge(
        "easter",
        "Pâques",
        BadgeKind::Holiday,
        [" .-. ", "(~~~)", " '-' "],
    ),
    badge(
        "neighbours_party",
        "Fête des voisins",
        BadgeKind::Holiday,
        ["o   o", "[===]", " | | "],
    ),
    badge(
        "perseids",
        "Perséides",
        BadgeKind::Holiday,
        ["  * *", " / / ", "/ /  "],
    ),
    badge(
        "music_day",
        "Fête de la musique",
        BadgeKind::Holiday,
        ["  |\\ ", "  |  ", "(_|  "],
    ),
    badge(
        "geminids",
        "Géminides",
        BadgeKind::Holiday,
        ["* *  ", " \\ \\ ", "  \\ \\"],
    ),
    badge(
        "candlemas",
        "Chandeleur",
        BadgeKind::Holiday,
        [" _._ ", "(o .)", " '-' "],
    ),
    badge(
        "may_day",
        "1er mai",
        BadgeKind::Holiday,
        [" .--.", " |o o", "\\|/  "],
    ),
    badge(
        "streak_3",
        "3 jours d'affilée",
        BadgeKind::Loyalty,
        ["[===]", "| 3 |", "|___|"],
    ),
    badge(
        "streak_7",
        "7 jours d'affilée",
        BadgeKind::Loyalty,
        ["[===]", "| 7 |", "|___|"],
    ),
    badge(
        "streak_30",
        "30 jours d'affilée",
        BadgeKind::Loyalty,
        ["[===]", "|30 |", "|___|"],
    ),
    badge(
        "traveller",
        "Voyageur",
        BadgeKind::Common,
        [" .-. ", "|===|", "|___|"],
    ),
    badge(
        "riddle_1",
        "Première énigme",
        BadgeKind::Common,
        [" .-. ", "  .' ", "  .  "],
    ),
    badge(
        "riddle_10",
        "Dix énigmes",
        BadgeKind::Common,
        ["?   ?", " 10  ", "?   ?"],
    ),
    badge(
        "postman",
        "Facteur",
        BadgeKind::Common,
        [" .-. ", "|__|>", "  |  "],
    ),
    badge(
        "snowman",
        "Bonhomme de neige",
        BadgeKind::Common,
        [" _|_ ", " (o) ", "(   )"],
    ),
    badge(
        "first_bottle",
        "Bouteille à la mer",
        BadgeKind::Common,
        ["  _  ", " | | ", "~[_]~"],
    ),
    badge(
        "rainbow",
        "Arc-en-ciel",
        BadgeKind::Rare,
        [" ___ ", "/ _ \\", "|/ \\|"],
    ),
    badge(
        "shooting_star",
        "Étoile filante",
        BadgeKind::Rare,
        ["    *", " .*' ", "*'   "],
    ),
    badge(
        "backpacker",
        "Baroudeur",
        BadgeKind::Rare,
        [" .-. ", "/[_]\\", "\\___/"],
    ),
    badge(
        "all_letters",
        "Toutes les lettres",
        BadgeKind::Epic,
        ["_____", "|\\_/|", "|___|"],
    ),
    badge(
        "all_objects",
        "Tous les objets",
        BadgeKind::Epic,
        ["_____", "|_$_|", "|___|"],
    ),
    badge(
        "all_riddles",
        "Toutes les énigmes",
        BadgeKind::Epic,
        ["?_?_?", "|???|", "|___|"],
    ),
    badge(
        "all_recipes",
        "Toutes les recettes",
        BadgeKind::Epic,
        [" ___ ", "(___)", " \\_/ "],
    ),
    secret(
        "night_owl",
        "Oiseau de nuit",
        "ouvert entre minuit et 5 h",
        [" ___ ", "(o,o)", "{`\"`}"],
    ),
    secret(
        "early_bird",
        "Lève-tôt",
        "ouvert entre 5 h et 7 h",
        [" \\|/ ", "--O--", "_____"],
    ),
    secret(
        "star_hunter",
        "Chasseur d'étoiles",
        "3 étoiles filantes sans fermer weathr",
        ["*  * ", "  *  ", " *  *"],
    ),
    secret(
        "full_weather",
        "Collection complète",
        "tous les badges météo",
        [" _ _ ", "|\\|/|", "|___|"],
    ),
    secret(
        "patience",
        "Patience",
        "weathr ouvert 1 h d'affilée",
        [" ___ ", " \\:/ ", " /.\\ "],
    ),
    secret(
        "night_reading",
        "Lecture nocturne",
        "une lettre lue entre 22 h et 6 h",
        ["  ,  ", " |=| ", " |_| "],
    ),
    secret(
        "summer_holidays",
        "Grandes vacances",
        "boîte aux lettres pleine",
        [" _n_ ", "| | |", "|_|_|"],
    ),
    secret(
        "sky_lookout",
        "Vigie du ciel",
        "a repéré les nuages pressés",
        [" .-. ", "(___)", "  \\o "],
    ),
    secret(
        "time_master",
        "Maître du temps",
        "a mis le village à l'heure",
        ["\\___/", " )_( ", "/___\\"],
    ),
    SAM_TRAIL,
    hinted(
        CHEATER,
        "Tricheur",
        BadgeKind::Demerit,
        "pris la main dans le sac",
        ["  ^  ", " /=\\ ", "(-_-)"],
    ),
];

pub fn collection() -> impl Iterator<Item = &'static Badge> {
    BADGES
        .iter()
        .filter(|badge| badge.kind != BadgeKind::Demerit)
}

pub fn weather_badges() -> impl Iterator<Item = &'static Badge> {
    BADGES
        .iter()
        .filter(|badge| badge.kind == BadgeKind::Weather)
}

pub fn weather_badge(condition: WeatherCondition) -> &'static str {
    match condition {
        WeatherCondition::Clear => "clear",
        WeatherCondition::PartlyCloudy => "partly_cloudy",
        WeatherCondition::Cloudy => "cloudy",
        WeatherCondition::Overcast => "overcast",
        WeatherCondition::Fog => "fog",
        WeatherCondition::Drizzle => "drizzle",
        WeatherCondition::Rain => "rain",
        WeatherCondition::FreezingRain => "freezing_rain",
        WeatherCondition::RainShowers => "rain_showers",
        WeatherCondition::Snow => "snow",
        WeatherCondition::SnowGrains => "snow_grains",
        WeatherCondition::SnowShowers => "snow_showers",
        WeatherCondition::Thunderstorm => "thunderstorm",
        WeatherCondition::ThunderstormHail => "thunderstorm_hail",
    }
}

pub fn find(id: &str) -> Option<&'static Badge> {
    BADGES.iter().find(|badge| badge.id == id)
}

pub fn name(id: &str) -> &'static str {
    find(id).map_or("?", |badge| badge.name)
}

pub fn is_sky_lookout(user_name: Option<&str>) -> bool {
    is_listed(user_name, &[SKY_LOOKOUT_NAME])
}

pub fn is_time_master(user_name: Option<&str>) -> bool {
    is_listed(user_name, &CONTRIBUTORS)
}

fn is_listed(user_name: Option<&str>, logins: &[&str]) -> bool {
    user_name.is_some_and(|name| logins.contains(&name.trim()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_every_badge_icon_is_three_lines_of_five_chars() {
        for badge in BADGES {
            assert!(
                badge.icon.iter().all(|line| line.chars().count() == 5),
                "{}",
                badge.id
            );
        }
    }

    #[test]
    fn test_cheater_badge_is_alone_in_its_family_and_out_of_the_collection() {
        let demerits: Vec<_> = BADGES
            .iter()
            .filter(|badge| badge.kind == BadgeKind::Demerit)
            .map(|badge| badge.id)
            .collect();
        assert_eq!(demerits, [CHEATER]);
        assert!(find(CHEATER).is_some_and(|badge| badge.hint.is_some()));
        assert_eq!(collection().count(), BADGES.len() - 1);
        assert!(collection().all(|badge| badge.id != CHEATER));
    }

    #[test]
    fn test_tier_follows_sam_cities_count() {
        let tiers: Vec<_> = [0, 1, 4, 5, 14, 15, 29, 30, 39, 40]
            .into_iter()
            .map(Tier::reached)
            .collect();
        assert_eq!(
            tiers,
            [
                None,
                Some(Tier::Chocolate),
                Some(Tier::Chocolate),
                Some(Tier::Bronze),
                Some(Tier::Bronze),
                Some(Tier::Silver),
                Some(Tier::Silver),
                Some(Tier::Gold),
                Some(Tier::Gold),
                Some(Tier::Platinum),
            ]
        );
    }

    #[test]
    fn test_crossed_tiers_are_the_new_ones_only() {
        assert_eq!(Tier::crossed(0, 1), [Tier::Chocolate]);
        assert_eq!(Tier::crossed(4, 5), [Tier::Bronze]);
        assert!(Tier::crossed(5, 5).is_empty());
        assert!(Tier::crossed(5, 6).is_empty());
        assert_eq!(Tier::crossed(0, 5), [Tier::Chocolate, Tier::Bronze]);
        assert_eq!(Tier::crossed(39, 40), [Tier::Platinum]);
    }

    #[test]
    fn test_sam_trail_line_shows_tier_and_next_one() {
        assert_eq!(sam_trail_line(0), None);
        assert_eq!(
            sam_trail_line(1).as_deref(),
            Some("Sur les traces de Sam · chocolat (1 ville, prochain palier\u{202F}: bronze à 5)")
        );
        assert_eq!(
            sam_trail_line(12).as_deref(),
            Some(
                "Sur les traces de Sam · bronze (12 villes, prochain palier\u{202F}: argent à 15)"
            )
        );
        assert_eq!(
            sam_trail_line(20).as_deref(),
            Some("Sur les traces de Sam · argent (20 villes, prochain palier\u{202F}: or à 30)")
        );
        assert_eq!(
            sam_trail_line(40).as_deref(),
            Some("Sur les traces de Sam · platine (40 villes)")
        );
    }

    #[test]
    fn test_first_tier_is_a_new_badge_then_new_tiers() {
        let chocolate = Award::sam_trail(Tier::Chocolate);
        assert!(!chocolate.is_new_tier());
        assert_eq!(chocolate.name(), "Sur les traces de Sam · chocolat");
        assert_eq!(
            chocolate.hint().as_deref(),
            Some("visité la même ville que Sam")
        );

        let bronze = Award::sam_trail(Tier::Bronze);
        assert!(bronze.is_new_tier());
        assert_eq!(bronze.name(), "Sur les traces de Sam · bronze");
        assert_eq!(bronze.hint().as_deref(), Some("5 villes de Sam visitées"));
    }

    #[test]
    fn test_plain_badge_award_keeps_its_name_and_hint() {
        let patience = Award::from(find("patience").unwrap());
        assert!(!patience.is_new_tier());
        assert_eq!(patience.name(), "Patience");
        assert_eq!(
            patience.hint().as_deref(),
            Some("weathr ouvert 1 h d'affilée")
        );
    }

    #[test]
    fn test_sam_trail_is_one_secret_badge() {
        let sam_trail = BADGES.iter().filter(|badge| badge.id == SAM_TRAIL.id);
        assert_eq!(sam_trail.count(), 1);
        assert!(find(SAM_TRAIL.id).is_some_and(|badge| badge.kind == BadgeKind::Secret));
    }

    fn row(kind: BadgeKind) -> Vec<&'static str> {
        BADGES
            .iter()
            .filter(|badge| badge.kind == kind)
            .map(|badge| badge.name)
            .collect()
    }

    #[test]
    fn test_rows_are_ordered_from_weather_to_secrets() {
        let titles: Vec<_> = KINDS.iter().map(BadgeKind::title).collect();
        assert_eq!(
            titles,
            [
                "Météo",
                "Fêtes",
                "Fidélité",
                "Commun",
                "Rares",
                "Épique",
                "Secrets",
                "Mauvais points",
            ]
        );
    }

    #[test]
    fn test_loyalty_keeps_only_the_streaks() {
        assert_eq!(
            row(BadgeKind::Loyalty),
            [
                "3 jours d'affilée",
                "7 jours d'affilée",
                "30 jours d'affilée"
            ]
        );
    }

    #[test]
    fn test_common_row_holds_the_easiest_badges() {
        assert_eq!(
            row(BadgeKind::Common),
            [
                "Voyageur",
                "Première énigme",
                "Dix énigmes",
                "Facteur",
                "Bonhomme de neige",
                "Bouteille à la mer",
            ]
        );
    }

    #[test]
    fn test_backpacker_joins_the_rare_row() {
        assert_eq!(
            row(BadgeKind::Rare),
            ["Arc-en-ciel", "Étoile filante", "Baroudeur"]
        );
        assert!(find("backpacker").is_some_and(|badge| badge.kind == BadgeKind::Rare));
    }

    #[test]
    fn test_epic_row_collects_everything() {
        assert_eq!(
            row(BadgeKind::Epic),
            [
                "Toutes les lettres",
                "Tous les objets",
                "Toutes les énigmes",
                "Toutes les recettes",
            ]
        );
    }

    #[test]
    fn test_candlemas_badge_is_a_fete() {
        let badge = find("candlemas").unwrap();
        assert_eq!(badge.name, "Chandeleur");
        assert!(badge.kind == BadgeKind::Holiday);
        assert!(badge.hint.is_none());
    }

    #[test]
    fn test_may_day_badge_is_a_fete() {
        let badge = find("may_day").unwrap();
        assert_eq!(badge.name, "1er mai");
        assert!(badge.kind == BadgeKind::Holiday);
        assert!(badge.hint.is_none());
    }

    #[test]
    fn test_sky_lookout_matches_exact_name() {
        assert!(is_sky_lookout(Some("Denis Richard")));
    }

    #[test]
    fn test_sky_lookout_ignores_surrounding_whitespace() {
        assert!(is_sky_lookout(Some("Denis Richard\n")));
        assert!(is_sky_lookout(Some(" Denis Richard ")));
    }

    #[test]
    fn test_sky_lookout_rejects_other_name() {
        assert!(!is_sky_lookout(Some("srussier-luna")));
        assert!(!is_sky_lookout(Some("drichard-luna")));
    }

    #[test]
    fn test_sky_lookout_does_not_grant_time_master() {
        assert!(!is_time_master(Some("Denis Richard")));
    }

    #[test]
    fn test_sky_lookout_rejects_missing_name() {
        assert!(!is_sky_lookout(None));
        assert!(!is_sky_lookout(Some("")));
    }

    #[test]
    fn test_time_master_matches_each_contributor() {
        assert!(is_time_master(Some("bpisano")));
    }

    #[test]
    fn test_time_master_ignores_surrounding_whitespace() {
        assert!(is_time_master(Some(" bpisano\n")));
    }

    #[test]
    fn test_time_master_rejects_other_name() {
        assert!(!is_time_master(Some("srussier-luna")));
        assert!(!is_time_master(Some("drichard-luna")));
    }

    #[test]
    fn test_time_master_rejects_missing_name() {
        assert!(!is_time_master(None));
        assert!(!is_time_master(Some("")));
    }
}
