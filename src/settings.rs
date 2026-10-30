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
}

pub const ROWS: [Row; 5] = [
    Row::Temperature,
    Row::WindSpeed,
    Row::Precipitation,
    Row::MailHour,
    Row::Hud,
];

#[derive(Clone, Copy, Debug)]
pub struct Settings {
    pub units: WeatherUnits,
    pub mail_hour: u32,
    pub hide_hud: bool,
}

impl Settings {
    pub fn stepped(&self, row: Row, step: isize) -> Setting {
        match row {
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
        }
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
        }
    }

    #[test]
    fn test_units_cycle_both_ways() {
        let settings = settings();
        assert_eq!(
            settings.stepped(Row::Temperature, 1),
            Setting::Temperature(TemperatureUnit::Fahrenheit)
        );
        assert_eq!(
            settings.stepped(Row::WindSpeed, 1),
            Setting::WindSpeed(WindSpeedUnit::Mph)
        );
        assert_eq!(
            settings.stepped(Row::WindSpeed, -1),
            Setting::WindSpeed(WindSpeedUnit::Ms)
        );
        assert_eq!(
            settings.stepped(Row::Precipitation, -1),
            Setting::Precipitation(PrecipitationUnit::Inch)
        );
    }

    #[test]
    fn test_mail_hour_wraps_around_the_day() {
        let mut settings = settings();
        settings.mail_hour = 23;
        assert_eq!(settings.stepped(Row::MailHour, 1), Setting::MailHour(0));
        settings.mail_hour = 0;
        assert_eq!(settings.stepped(Row::MailHour, -1), Setting::MailHour(23));
    }

    #[test]
    fn test_apply_changes_only_its_setting() {
        let mut settings = settings();
        settings.apply(settings.stepped(Row::Hud, 1));
        settings.apply(Setting::Precipitation(PrecipitationUnit::Inch));
        assert!(settings.hide_hud);
        assert_eq!(settings.units.precipitation, PrecipitationUnit::Inch);
        assert_eq!(settings.units.temperature, TemperatureUnit::Celsius);
        assert_eq!(settings.mail_hour, 9);
    }
}
