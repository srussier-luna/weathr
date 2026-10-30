mod city;

pub use city::{CitySearch, Status};

use crate::config::Setting;
use crate::weather::WeatherUnits;
use crate::weather::types::{PrecipitationUnit, TemperatureUnit, WindSpeedUnit};

const TEMPERATURES: [TemperatureUnit; 2] = [TemperatureUnit::Celsius, TemperatureUnit::Fahrenheit];
const WIND_SPEEDS: [WindSpeedUnit; 4] = [
    WindSpeedUnit::Ms,
    WindSpeedUnit::Kmh,
    WindSpeedUnit::Mph,
    WindSpeedUnit::Kn,
];
const PRECIPITATIONS: [PrecipitationUnit; 2] = [PrecipitationUnit::Mm, PrecipitationUnit::Inch];
const HOURS_PER_DAY: isize = 24;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Row {
    Temperature,
    WindSpeed,
    Precipitation,
    MailHour,
    Hud,
    City,
    AutoLocation,
}

pub const ROWS: [Row; 7] = [
    Row::Temperature,
    Row::WindSpeed,
    Row::Precipitation,
    Row::MailHour,
    Row::Hud,
    Row::City,
    Row::AutoLocation,
];

#[derive(Clone, Debug)]
pub struct Settings {
    pub units: WeatherUnits,
    pub mail_hour: u32,
    pub hide_hud: bool,
    pub city: String,
    pub auto_location: bool,
    pub is_locating: bool,
}

impl Settings {
    pub fn stepped(&self, row: Row, step: isize) -> Option<Setting> {
        Some(match row {
            Row::Temperature => {
                Setting::Temperature(cycle(&TEMPERATURES, self.units.temperature, step))
            }
            Row::WindSpeed => Setting::WindSpeed(cycle(&WIND_SPEEDS, self.units.wind_speed, step)),
            Row::Precipitation => {
                Setting::Precipitation(cycle(&PRECIPITATIONS, self.units.precipitation, step))
            }
            Row::MailHour => {
                Setting::MailHour((self.mail_hour as isize + step).rem_euclid(HOURS_PER_DAY) as u32)
            }
            Row::Hud => Setting::HideHud(!self.hide_hud),
            Row::City | Row::AutoLocation => return None,
        })
    }

    pub fn apply(&mut self, setting: Setting) {
        match setting {
            Setting::Temperature(unit) => self.units.temperature = unit,
            Setting::WindSpeed(unit) => self.units.wind_speed = unit,
            Setting::Precipitation(unit) => self.units.precipitation = unit,
            Setting::MailHour(hour) => self.mail_hour = hour,
            Setting::HideHud(hide) => self.hide_hud = hide,
        }
    }
}

fn cycle<T: Copy + PartialEq>(options: &[T], current: T, step: isize) -> T {
    let index = options
        .iter()
        .position(|option| *option == current)
        .unwrap_or(0);
    options[(index as isize + step).rem_euclid(options.len() as isize) as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settings() -> Settings {
        Settings {
            units: WeatherUnits::default(),
            mail_hour: 9,
            hide_hud: false,
            city: "Lyon".to_string(),
            auto_location: false,
            is_locating: false,
        }
    }

    #[test]
    fn test_units_cycle_both_ways() {
        let settings = settings();
        assert_eq!(
            settings.stepped(Row::Temperature, 1),
            Some(Setting::Temperature(TemperatureUnit::Fahrenheit))
        );
        assert_eq!(
            settings.stepped(Row::WindSpeed, 1),
            Some(Setting::WindSpeed(WindSpeedUnit::Mph))
        );
        assert_eq!(
            settings.stepped(Row::WindSpeed, -1),
            Some(Setting::WindSpeed(WindSpeedUnit::Ms))
        );
        assert_eq!(
            settings.stepped(Row::Precipitation, -1),
            Some(Setting::Precipitation(PrecipitationUnit::Inch))
        );
    }

    #[test]
    fn test_mail_hour_wraps_around_the_day() {
        let mut settings = settings();
        settings.mail_hour = 23;
        assert_eq!(
            settings.stepped(Row::MailHour, 1),
            Some(Setting::MailHour(0))
        );
        settings.mail_hour = 0;
        assert_eq!(
            settings.stepped(Row::MailHour, -1),
            Some(Setting::MailHour(23))
        );
    }

    #[test]
    fn test_location_rows_are_not_stepped() {
        assert_eq!(settings().stepped(Row::City, 1), None);
        assert_eq!(settings().stepped(Row::AutoLocation, -1), None);
    }

    #[test]
    fn test_apply_changes_only_its_setting() {
        let mut settings = settings();
        settings.apply(settings.stepped(Row::Hud, 1).unwrap());
        settings.apply(Setting::Precipitation(PrecipitationUnit::Inch));
        assert!(settings.hide_hud);
        assert_eq!(settings.units.precipitation, PrecipitationUnit::Inch);
        assert_eq!(settings.units.temperature, TemperatureUnit::Celsius);
        assert_eq!(settings.mail_hour, 9);
    }
}
