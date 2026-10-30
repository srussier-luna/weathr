use crate::weather::WeatherCondition;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BadgeKind {
    Weather,
    Holiday,
    Rare,
    Loyalty,
    Secret,
}

impl BadgeKind {
    pub fn title(&self) -> &'static str {
        match self {
            Self::Weather => "Météo",
            Self::Holiday => "Fêtes",
            Self::Rare => "Rares",
            Self::Loyalty => "Fidélité",
            Self::Secret => "Secrets",
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

const fn secret(
    id: &'static str,
    name: &'static str,
    hint: &'static str,
    icon: [&'static str; 3],
) -> Badge {
    Badge {
        id,
        name,
        kind: BadgeKind::Secret,
        hint: Some(hint),
        icon,
    }
}

pub const KINDS: [BadgeKind; 5] = [
    BadgeKind::Weather,
    BadgeKind::Holiday,
    BadgeKind::Rare,
    BadgeKind::Loyalty,
    BadgeKind::Secret,
];

pub const BADGES: [Badge; 40] = [
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
        "perseids",
        "Perséides",
        BadgeKind::Holiday,
        ["  * *", " / / ", "/ /  "],
    ),
    badge(
        "geminids",
        "Géminides",
        BadgeKind::Holiday,
        ["* *  ", " \\ \\ ", "  \\ \\"],
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
        "postman",
        "Facteur",
        BadgeKind::Rare,
        [" .-. ", "|__|>", "  |  "],
    ),
    badge(
        "snowman",
        "Bonhomme de neige",
        BadgeKind::Rare,
        [" _|_ ", " (o) ", "(   )"],
    ),
    badge(
        "first_bottle",
        "Bouteille à la mer",
        BadgeKind::Rare,
        ["  _  ", " | | ", "~[_]~"],
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
        "all_letters",
        "Toutes les lettres",
        BadgeKind::Loyalty,
        ["_____", "|\\_/|", "|___|"],
    ),
    badge(
        "riddle_1",
        "Première énigme",
        BadgeKind::Loyalty,
        [" .-. ", "  .' ", "  .  "],
    ),
    badge(
        "riddle_10",
        "Dix énigmes",
        BadgeKind::Loyalty,
        ["?   ?", " 10  ", "?   ?"],
    ),
    badge(
        "all_objects",
        "Tous les objets",
        BadgeKind::Loyalty,
        ["_____", "|_$_|", "|___|"],
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
];

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_every_badge_icon_is_three_lines_of_five_chars() {
        for badge in &BADGES {
            assert!(
                badge.icon.iter().all(|line| line.chars().count() == 5),
                "{}",
                badge.id
            );
        }
    }
}
