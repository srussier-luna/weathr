use crate::config::LocationDisplay;
use crate::holiday::Holiday;
use crate::weather::{
    WeatherCondition, WeatherConditions, WeatherData, WeatherLocation, WeatherUnits,
};
use std::time::{Duration, Instant};

const RAINBOW_DURATION: Duration = Duration::from_secs(30 * 60);

pub struct AppState {
    pub current_weather: Option<WeatherData>,
    pub is_offline: bool,
    pub weather_conditions: WeatherConditions,
    pub weather_info_needs_update: bool,
    pub displayed_clock: String,
    pub story_day: usize,
    pub demo_label: Option<String>,
    pub is_mail_waiting: bool,
    pub umbrella: Option<String>,
    pub finds_hint: Option<&'static str>,
    pub badges_earned: usize,
    pub badges_total: usize,
    pub has_new_badge: bool,
    pub holiday: Holiday,
    pub rainbow_until: Option<Instant>,
    pub location: WeatherLocation,
    pub city_name: Option<String>,
    pub location_display: LocationDisplay,
    pub hide_location: bool,
    pub units: WeatherUnits,
}

impl AppState {
    pub fn new(
        location: WeatherLocation,
        city_name: Option<String>,
        location_display: LocationDisplay,
        hide_location: bool,
        units: WeatherUnits,
    ) -> Self {
        Self {
            current_weather: None,
            is_offline: false,
            weather_conditions: WeatherConditions::default(),
            weather_info_needs_update: true,
            displayed_clock: String::new(),
            story_day: 1,
            demo_label: None,
            is_mail_waiting: false,
            umbrella: None,
            finds_hint: None,
            badges_earned: 0,
            badges_total: 0,
            has_new_badge: false,
            holiday: Holiday::default(),
            rainbow_until: None,
            location,
            city_name,
            location_display,
            hide_location,
            units,
        }
    }

    pub fn update_weather(&mut self, weather: WeatherData) {
        let was_raining =
            self.weather_conditions.is_raining || self.weather_conditions.is_thunderstorm;
        self.weather_conditions.is_thunderstorm = weather.condition.is_thunderstorm();
        self.weather_conditions.is_snowing = weather.condition.is_snowing();
        self.weather_conditions.is_raining =
            weather.condition.is_raining() && !self.weather_conditions.is_thunderstorm;
        self.weather_conditions.is_cloudy = weather.condition.is_cloudy();
        self.weather_conditions.is_overcast = weather.condition.is_overcast();
        self.weather_conditions.is_foggy = weather.condition.is_foggy();
        self.weather_conditions.is_day = weather.is_day;

        let is_raining =
            self.weather_conditions.is_raining || self.weather_conditions.is_thunderstorm;
        if was_raining && !is_raining && !self.weather_conditions.is_snowing {
            self.rainbow_until = Some(Instant::now() + RAINBOW_DURATION);
        }

        self.current_weather = Some(weather);
        self.is_offline = false;
        self.weather_info_needs_update = true;
    }

    pub fn set_rainbow(&mut self, until: Option<Instant>) {
        self.rainbow_until = until;
    }

    pub fn shows_rainbow(&self) -> bool {
        let conditions = &self.weather_conditions;
        self.rainbow_until
            .is_some_and(|until| Instant::now() < until)
            && conditions.is_day
            && !conditions.is_raining
            && !conditions.is_thunderstorm
            && !conditions.is_snowing
    }

    pub fn set_finds_hint(&mut self, hint: Option<&'static str>) {
        if hint != self.finds_hint {
            self.finds_hint = hint;
            self.weather_info_needs_update = true;
        }
    }

    pub fn set_new_badge(&mut self, has_new_badge: bool) {
        if has_new_badge != self.has_new_badge {
            self.has_new_badge = has_new_badge;
            self.weather_info_needs_update = true;
        }
    }

    pub fn set_umbrella(&mut self, message: Option<String>) {
        if message != self.umbrella {
            self.umbrella = message;
            self.weather_info_needs_update = true;
        }
    }

    pub fn set_story_day(&mut self, day: usize) {
        if day != self.story_day {
            self.story_day = day;
            self.weather_info_needs_update = true;
        }
    }

    pub fn set_progress_summary(&mut self, is_mail_waiting: bool, earned: usize, total: usize) {
        let summary = (is_mail_waiting, earned, total);
        if summary != (self.is_mail_waiting, self.badges_earned, self.badges_total) {
            self.is_mail_waiting = is_mail_waiting;
            self.badges_earned = earned;
            self.badges_total = total;
            self.weather_info_needs_update = true;
        }
    }

    pub fn set_location(&mut self, location: WeatherLocation, city_name: Option<String>) {
        self.location = location;
        self.city_name = city_name;
        self.weather_info_needs_update = true;
    }

    pub fn set_location_display(&mut self, display: LocationDisplay) {
        self.location_display = display;
        self.weather_info_needs_update = true;
    }

    pub fn set_units(&mut self, units: WeatherUnits) {
        self.units = units;
        self.weather_info_needs_update = true;
    }

    pub fn set_demo_label(&mut self, label: Option<String>) {
        self.demo_label = label;
        self.weather_info_needs_update = true;
    }

    pub fn set_offline_mode(&mut self, offline: bool) {
        self.is_offline = offline;
        self.weather_info_needs_update = true;
    }

    pub fn update_clock(&mut self) {
        let clock = chrono::Local::now().format("%H:%M").to_string();
        if clock != self.displayed_clock {
            self.displayed_clock = clock;
            self.weather_info_needs_update = true;
        }
    }

    pub fn take_weather_info_update(&mut self) -> bool {
        std::mem::take(&mut self.weather_info_needs_update)
    }

    pub fn should_show_sun(&self) -> bool {
        if !self.weather_conditions.is_day {
            return false;
        }

        if let Some(ref weather) = self.current_weather {
            matches!(
                weather.condition,
                WeatherCondition::Clear | WeatherCondition::PartlyCloudy | WeatherCondition::Cloudy
            )
        } else {
            false
        }
    }

    pub fn should_show_fireflies(&self) -> bool {
        if self.weather_conditions.is_day {
            return false;
        }

        if let Some(ref weather) = self.current_weather {
            let is_warm = weather.temperature > 15.0;
            let is_clear_night = matches!(
                weather.condition,
                WeatherCondition::Clear | WeatherCondition::PartlyCloudy
            );
            is_warm
                && is_clear_night
                && !self.weather_conditions.is_raining
                && !self.weather_conditions.is_thunderstorm
                && !self.weather_conditions.is_snowing
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state() -> AppState {
        let location = WeatherLocation {
            latitude: 0.0,
            longitude: 0.0,
            elevation: None,
        };
        AppState::new(
            location,
            None,
            LocationDisplay::default(),
            false,
            WeatherUnits::default(),
        )
    }

    #[test]
    fn test_story_day_change_asks_for_a_redraw_once() {
        let mut state = state();
        state.set_story_day(7);
        assert!(state.take_weather_info_update());
        assert!(!state.take_weather_info_update());

        state.set_story_day(7);
        assert!(!state.take_weather_info_update());
        state.set_story_day(8);
        assert!(state.take_weather_info_update());
    }

    #[test]
    fn test_units_change_asks_for_a_redraw() {
        let mut state = state();
        state.take_weather_info_update();

        state.set_units(WeatherUnits::default());
        assert!(state.take_weather_info_update());
    }
}
