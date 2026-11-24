use super::{Line, MUTED, Segment};
use crate::app_state::AppState;
use crate::badges;
use crate::config::LocationDisplay;
use crate::weather::types::{PrecipitationUnit, TemperatureUnit};
use crate::weather::{
    WeatherData, WeatherUnits, format_precipitation, format_temperature, format_wind_speed,
};
use crossterm::style::Color;

const UNIT_SPACE: char = '\u{202F}';
const TEXT: Color = Color::Reset;
const CLOCK: Color = Color::White;
const RAIN: Color = Color::Blue;
const ALERT: Color = Color::Yellow;
const TEMPERATURE_STEPS: [(i64, Color); 6] = [
    (
        0,
        Color::Rgb {
            r: 180,
            g: 225,
            b: 255,
        },
    ),
    (
        5,
        Color::Rgb {
            r: 80,
            g: 140,
            b: 255,
        },
    ),
    (
        11,
        Color::Rgb {
            r: 60,
            g: 210,
            b: 200,
        },
    ),
    (
        17,
        Color::Rgb {
            r: 110,
            g: 200,
            b: 80,
        },
    ),
    (
        24,
        Color::Rgb {
            r: 240,
            g: 210,
            b: 60,
        },
    ),
    (
        29,
        Color::Rgb {
            r: 255,
            g: 145,
            b: 40,
        },
    ),
];
const HOT: Color = Color::Rgb {
    r: 235,
    g: 65,
    b: 55,
};

#[derive(Debug, Default)]
pub struct WeatherBar {
    pub left: Vec<Line>,
    pub right: Vec<Line>,
}

impl WeatherBar {
    pub fn new(state: &AppState) -> Self {
        Self {
            left: left_block(state),
            right: right_block(state),
        }
    }
}

fn left_block(state: &AppState) -> Vec<Line> {
    let Some(weather) = &state.current_weather else {
        return vec![vec![Segment::new("Météo en route…", TEXT)]];
    };
    let rain_alert = state.umbrella.as_deref().map(|alert| (alert, ALERT));
    let offline = state.is_offline.then_some(("Hors ligne", ALERT));
    let demo = state.demo_label.as_deref().map(|label| (label, MUTED));
    let extras = [rain_alert, offline, demo]
        .into_iter()
        .flatten()
        .map(|(text, color)| vec![Segment::new(text, color)]);

    [Some(sky_line(weather, state.units.temperature))]
        .into_iter()
        .chain([
            wind_line(weather, &state.units),
            rain_line(weather, &state.units),
        ])
        .flatten()
        .chain(extras)
        .collect()
}

fn sky_line(weather: &WeatherData, unit: TemperatureUnit) -> Line {
    let sky = badges::name(badges::weather_badge(weather.condition));
    let (value, symbol) = format_temperature(weather.temperature, unit);
    vec![
        Segment::new(format!("{sky} "), TEXT),
        Segment::new(
            format!("{}{UNIT_SPACE}{symbol}", value.round() as i64),
            temperature_color(weather.temperature.round() as i64),
        ),
    ]
}

fn wind_line(weather: &WeatherData, units: &WeatherUnits) -> Option<Line> {
    let (value, symbol) = format_wind_speed(weather.wind_speed, units.wind_speed);
    let speed = value.round() as i64;
    (speed > 0).then(|| {
        vec![Segment::new(
            format!("Vent {speed}{UNIT_SPACE}{symbol}"),
            MUTED,
        )]
    })
}

fn rain_line(weather: &WeatherData, units: &WeatherUnits) -> Option<Line> {
    let (value, symbol) = format_precipitation(weather.precipitation, units.precipitation);
    let (decimals, scale) = match units.precipitation {
        PrecipitationUnit::Mm => (1, 10.0),
        PrecipitationUnit::Inch => (2, 100.0),
    };
    if (value * scale).round() == 0.0 {
        return None;
    }
    let amount = format!("{value:.decimals$}").replace('.', ",");
    Some(vec![Segment::new(
        format!("Pluie {amount}{UNIT_SPACE}{symbol}"),
        RAIN,
    )])
}

fn temperature_color(celsius: i64) -> Color {
    TEMPERATURE_STEPS
        .iter()
        .find(|(ceiling, _)| celsius <= *ceiling)
        .map_or(HOT, |(_, color)| *color)
}

fn right_block(state: &AppState) -> Vec<Line> {
    let day = format!("Jour {}", state.story_day);
    let place_and_day = match location_label(state) {
        Some(place) => format!("{place} • {day}"),
        None => day,
    };
    vec![
        vec![Segment::new(state.displayed_clock.clone(), CLOCK)],
        vec![Segment::new(place_and_day, MUTED)],
    ]
}

fn location_label(state: &AppState) -> Option<String> {
    if state.hide_location {
        return None;
    }
    let location = &state.location;
    let (lat_value, lat_dir) = if location.latitude >= 0.0 {
        (location.latitude, "N")
    } else {
        (-location.latitude, "S")
    };
    let (lon_value, lon_dir) = if location.longitude >= 0.0 {
        (location.longitude, "E")
    } else {
        (-location.longitude, "W")
    };
    let coords = format!("{lat_value:.2}°{lat_dir}, {lon_value:.2}°{lon_dir}");
    let label = match (state.location_display, &state.city_name) {
        (LocationDisplay::Coordinates, _) | (_, None) => coords,
        (LocationDisplay::City, Some(city)) => city.clone(),
        (LocationDisplay::Mixed, Some(city)) => format!("{city} ({coords})"),
    };
    Some(label)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hud::text;
    use crate::weather::types::WindSpeedUnit;
    use crate::weather::{WeatherCondition, WeatherLocation};

    fn kmh_to_ms(kmh: f64) -> f64 {
        kmh / 3.6
    }

    fn weather(
        condition: WeatherCondition,
        celsius: f64,
        wind_kmh: f64,
        rain_mm: f64,
    ) -> WeatherData {
        WeatherData {
            condition,
            temperature: celsius,
            apparent_temperature: celsius,
            humidity: 60.0,
            precipitation: rain_mm,
            wind_speed: kmh_to_ms(wind_kmh),
            wind_direction: 0.0,
            cloud_cover: 0.0,
            pressure: 1013.0,
            visibility: Some(10.0),
            is_day: true,
            moon_phase: Some(0.5),
            timestamp: "2024-01-01T12:00:00Z".to_string(),
        }
    }

    fn state_at(lat: f64, lon: f64, city: Option<&str>, display: LocationDisplay) -> AppState {
        let location = WeatherLocation {
            latitude: lat,
            longitude: lon,
            elevation: None,
        };
        let mut state = AppState::new(
            location,
            city.map(str::to_string),
            display,
            false,
            WeatherUnits::default(),
        );
        state.displayed_clock = "14:32".to_string();
        state.set_story_day(12);
        state
    }

    fn state_with(weather: WeatherData) -> AppState {
        let mut state = state_at(45.76, 4.84, Some("Lyon"), LocationDisplay::City);
        state.update_weather(weather);
        state
    }

    fn left_texts(state: &AppState) -> Vec<String> {
        WeatherBar::new(state)
            .left
            .iter()
            .map(|line| text(line))
            .collect()
    }

    fn place(state: &AppState) -> String {
        text(&WeatherBar::new(state).right[1])
    }

    #[test]
    fn test_first_line_shows_french_sky_and_rounded_temperature() {
        let state = state_with(weather(WeatherCondition::PartlyCloudy, 12.4, 0.0, 0.0));

        assert_eq!(left_texts(&state), ["Éclaircies 12\u{202F}°C"]);
    }

    #[test]
    fn test_right_block_shows_clock_then_city_and_day() {
        let bar = WeatherBar::new(&state_with(weather(
            WeatherCondition::Clear,
            20.0,
            0.0,
            0.0,
        )));

        assert_eq!(bar.right[0], vec![Segment::new("14:32", CLOCK)]);
        assert_eq!(bar.right[1], vec![Segment::new("Lyon • Jour 12", MUTED)]);
    }

    #[test]
    fn test_calm_dry_weather_has_no_wind_or_rain_line() {
        let state = state_with(weather(WeatherCondition::Overcast, 9.0, 0.0, 0.0));

        assert_eq!(left_texts(&state).len(), 1);
    }

    #[test]
    fn test_wind_and_rain_lines_follow_the_sky_in_their_colors() {
        let bar = WeatherBar::new(&state_with(weather(WeatherCondition::Rain, 9.0, 8.0, 1.2)));

        assert_eq!(bar.left[1], vec![Segment::new("Vent 8\u{202F}km/h", MUTED)]);
        assert_eq!(bar.left[2], vec![Segment::new("Pluie 1,2\u{202F}mm", RAIN)]);
    }

    #[test]
    fn test_amounts_that_round_to_zero_are_hidden() {
        let state = state_with(weather(WeatherCondition::Drizzle, 9.0, 0.4, 0.04));

        assert_eq!(left_texts(&state).len(), 1);
    }

    #[test]
    fn test_rain_alert_and_offline_lines_are_yellow() {
        let mut state = state_with(weather(WeatherCondition::Clear, 20.0, 0.0, 0.0));
        state.set_umbrella(Some("Pluie vers 16:00".to_string()));
        state.set_offline_mode(true);
        let bar = WeatherBar::new(&state);

        assert_eq!(bar.left[1], vec![Segment::new("Pluie vers 16:00", ALERT)]);
        assert_eq!(bar.left[2], vec![Segment::new("Hors ligne", ALERT)]);
    }

    #[test]
    fn test_demo_label_gets_its_own_line() {
        let mut state = state_with(weather(WeatherCondition::Clear, 20.0, 0.0, 0.0));
        state.set_demo_label(Some("Demo 1/8".to_string()));

        assert_eq!(left_texts(&state).last().unwrap(), "Demo 1/8");
    }

    #[test]
    fn test_loading_shows_only_the_waiting_line_with_clock_and_day() {
        let state = state_at(45.76, 4.84, Some("Lyon"), LocationDisplay::City);
        let bar = WeatherBar::new(&state);

        assert_eq!(left_texts(&state), ["Météo en route…"]);
        assert_eq!(text(&bar.right[0]), "14:32");
        assert_eq!(text(&bar.right[1]), "Lyon • Jour 12");
    }

    #[test]
    fn test_temperature_takes_the_color_of_its_step() {
        let steps = [
            (-8.0, TEMPERATURE_STEPS[0].1),
            (3.0, TEMPERATURE_STEPS[1].1),
            (9.0, TEMPERATURE_STEPS[2].1),
            (14.0, TEMPERATURE_STEPS[3].1),
            (21.0, TEMPERATURE_STEPS[4].1),
            (27.0, TEMPERATURE_STEPS[5].1),
            (34.0, HOT),
        ];
        for (celsius, color) in steps {
            let bar = WeatherBar::new(&state_with(weather(
                WeatherCondition::Clear,
                celsius,
                0.0,
                0.0,
            )));
            assert_eq!(bar.left[0][1].color, color, "{celsius} °C");
        }
        let colors: Vec<Color> = steps.iter().map(|(_, color)| *color).collect();
        for (index, color) in colors.iter().enumerate() {
            assert!(!colors[index + 1..].contains(color));
        }
    }

    #[test]
    fn test_step_bounds_are_inclusive() {
        for (celsius, step) in [
            (0, 0),
            (1, 1),
            (5, 1),
            (6, 2),
            (11, 2),
            (12, 3),
            (17, 3),
            (18, 4),
            (24, 4),
            (25, 5),
            (29, 5),
        ] {
            assert_eq!(
                temperature_color(celsius),
                TEMPERATURE_STEPS[step].1,
                "{celsius} °C"
            );
        }
        assert_eq!(temperature_color(30), HOT);
    }

    #[test]
    fn test_fahrenheit_shows_converted_value_with_celsius_step() {
        let mut state = state_with(weather(WeatherCondition::Clear, 20.0, 10.0, 0.0));
        state.set_units(WeatherUnits {
            temperature: TemperatureUnit::Fahrenheit,
            wind_speed: WindSpeedUnit::Ms,
            precipitation: PrecipitationUnit::Inch,
        });
        let bar = WeatherBar::new(&state);

        assert_eq!(
            bar.left[0][1],
            Segment::new("68\u{202F}°F", TEMPERATURE_STEPS[4].1)
        );
        assert_eq!(text(&bar.left[1]), "Vent 3\u{202F}m/s");
    }

    #[test]
    fn test_inches_keep_two_decimals() {
        let mut state = state_with(weather(WeatherCondition::Rain, 9.0, 0.0, 5.0));
        state.set_units(WeatherUnits {
            precipitation: PrecipitationUnit::Inch,
            ..WeatherUnits::default()
        });

        assert_eq!(left_texts(&state)[1], "Pluie 0,20\u{202F}in");
    }

    #[test]
    fn test_no_english_label_left() {
        let mut state = state_with(weather(WeatherCondition::Thunderstorm, 20.0, 8.0, 1.2));
        state.set_offline_mode(true);
        let bar = WeatherBar::new(&state);
        let all: String = bar
            .left
            .iter()
            .chain(&bar.right)
            .map(|line| text(line))
            .collect();

        for label in [
            "Weather:",
            "Temp:",
            "Wind:",
            "Precip:",
            "Location:",
            "Press 'q' to quit",
            "OFFLINE",
        ] {
            assert!(!all.contains(label), "{label}");
        }
    }

    #[test]
    fn test_coordinates_in_every_quadrant() {
        for (lat, lon, expected) in [
            (40.7128, -74.0060, "40.71°N, 74.01°W"),
            (-33.8688, 151.2093, "33.87°S, 151.21°E"),
            (51.5074, -0.1278, "51.51°N, 0.13°W"),
            (-23.5505, -46.6333, "23.55°S, 46.63°W"),
            (35.6762, 139.6503, "35.68°N, 139.65°E"),
            (0.0, 0.0, "0.00°N, 0.00°E"),
        ] {
            let state = state_at(lat, lon, None, LocationDisplay::Coordinates);
            assert_eq!(place(&state), format!("{expected} • Jour 12"));
        }
    }

    #[test]
    fn test_place_follows_the_display_mode() {
        let city = Some("Alpharetta");
        let coords = "34.08°N, 84.29°W";
        let cases = [
            (city, LocationDisplay::Coordinates, coords.to_string()),
            (city, LocationDisplay::City, "Alpharetta".to_string()),
            (
                city,
                LocationDisplay::Mixed,
                format!("Alpharetta ({coords})"),
            ),
            (None, LocationDisplay::City, coords.to_string()),
            (None, LocationDisplay::Mixed, coords.to_string()),
        ];
        for (city, display, expected) in cases {
            let state = state_at(34.0754, -84.2941, city, display);
            assert_eq!(place(&state), format!("{expected} • Jour 12"));
        }
    }

    #[test]
    fn test_switching_to_city_display_shows_the_name_at_once() {
        let mut state = state_at(
            34.0754,
            -84.2941,
            Some("Alpharetta"),
            LocationDisplay::Coordinates,
        );
        state.set_location_display(LocationDisplay::City);

        assert_eq!(place(&state), "Alpharetta • Jour 12");
    }

    #[test]
    fn test_hidden_location_leaves_only_the_day() {
        let mut state = state_at(34.0754, -84.2941, Some("Alpharetta"), LocationDisplay::City);
        state.hide_location = true;

        assert_eq!(place(&state), "Jour 12");
    }
}
