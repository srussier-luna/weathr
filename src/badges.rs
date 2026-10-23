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
}

const fn badge(id: &'static str, name: &'static str, kind: BadgeKind) -> Badge {
    Badge { id, name, kind }
}

pub const KINDS: [BadgeKind; 5] = [
    BadgeKind::Weather,
    BadgeKind::Holiday,
    BadgeKind::Rare,
    BadgeKind::Loyalty,
    BadgeKind::Secret,
];

pub const BADGES: [Badge; 37] = [
    badge("clear", "Ciel dégagé", BadgeKind::Weather),
    badge("partly_cloudy", "Éclaircies", BadgeKind::Weather),
    badge("cloudy", "Nuageux", BadgeKind::Weather),
    badge("overcast", "Couvert", BadgeKind::Weather),
    badge("fog", "Brouillard", BadgeKind::Weather),
    badge("drizzle", "Bruine", BadgeKind::Weather),
    badge("rain", "Pluie", BadgeKind::Weather),
    badge("freezing_rain", "Pluie verglaçante", BadgeKind::Weather),
    badge("rain_showers", "Averses", BadgeKind::Weather),
    badge("snow", "Neige", BadgeKind::Weather),
    badge("snow_grains", "Grésil", BadgeKind::Weather),
    badge("snow_showers", "Averses de neige", BadgeKind::Weather),
    badge("thunderstorm", "Orage", BadgeKind::Weather),
    badge("thunderstorm_hail", "Orage de grêle", BadgeKind::Weather),
    badge("christmas", "Noël", BadgeKind::Holiday),
    badge("national_day", "14 juillet", BadgeKind::Holiday),
    badge("halloween", "Halloween", BadgeKind::Holiday),
    badge("new_year", "Nouvel An", BadgeKind::Holiday),
    badge("easter", "Pâques", BadgeKind::Holiday),
    badge("rainbow", "Arc-en-ciel", BadgeKind::Rare),
    badge("shooting_star", "Étoile filante", BadgeKind::Rare),
    badge("postman", "Facteur", BadgeKind::Rare),
    badge("snowman", "Bonhomme de neige", BadgeKind::Rare),
    badge("first_bottle", "Bouteille à la mer", BadgeKind::Rare),
    badge("streak_3", "3 jours d'affilée", BadgeKind::Loyalty),
    badge("streak_7", "7 jours d'affilée", BadgeKind::Loyalty),
    badge("streak_30", "30 jours d'affilée", BadgeKind::Loyalty),
    badge("all_letters", "Toutes les lettres", BadgeKind::Loyalty),
    badge("riddle_1", "Première énigme", BadgeKind::Loyalty),
    badge("riddle_10", "Dix énigmes", BadgeKind::Loyalty),
    badge("all_objects", "Tous les objets", BadgeKind::Loyalty),
    badge("night_owl", "Oiseau de nuit", BadgeKind::Secret),
    badge("early_bird", "Lève-tôt", BadgeKind::Secret),
    badge("star_hunter", "Chasseur d'étoiles", BadgeKind::Secret),
    badge("full_weather", "Collection complète", BadgeKind::Secret),
    badge("patience", "Patience", BadgeKind::Secret),
    badge("night_reading", "Lecture nocturne", BadgeKind::Secret),
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

pub fn name(id: &str) -> &'static str {
    BADGES
        .iter()
        .find(|badge| badge.id == id)
        .map_or("?", |badge| badge.name)
}
