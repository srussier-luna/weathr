mod city;

pub use city::{CitySearch, SearchStatus};

use crate::config::{Setting, clamp_mail_hour};
use crate::story::content::recipes::Diet;
use crate::weather::WeatherUnits;
use crate::weather::types::{PrecipitationUnit, TemperatureUnit, WindSpeedUnit};
use std::collections::BTreeSet;
use tokio::sync::oneshot;

const TEMPERATURES: [TemperatureUnit; 2] = [TemperatureUnit::Celsius, TemperatureUnit::Fahrenheit];
const WIND_SPEEDS: [WindSpeedUnit; 4] = [
    WindSpeedUnit::Ms,
    WindSpeedUnit::Kmh,
    WindSpeedUnit::Mph,
    WindSpeedUnit::Kn,
];
const PRECIPITATIONS: [PrecipitationUnit; 2] = [PrecipitationUnit::Mm, PrecipitationUnit::Inch];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Row {
    Temperature,
    WindSpeed,
    Precipitation,
    MailHour,
    Hud,
    City,
    AutoLocation,
    Diet,
}

const ROWS: [Row; 8] = [
    Row::Temperature,
    Row::WindSpeed,
    Row::Precipitation,
    Row::MailHour,
    Row::Hud,
    Row::City,
    Row::AutoLocation,
    Row::Diet,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutoLocation {
    Off,
    Locating,
    On,
}

#[derive(Clone, Debug)]
pub struct Settings {
    pub units: WeatherUnits,
    pub mail_hour: u32,
    pub hide_hud: bool,
    pub city: String,
    pub auto_location: AutoLocation,
    pub diets: Option<BTreeSet<Diet>>,
}

impl Settings {
    pub fn rows(&self) -> &'static [Row] {
        match self.diets {
            Some(_) => &ROWS,
            None => &ROWS[..ROWS.len() - 1],
        }
    }

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
                let hour = clamp_mail_hour(self.mail_hour.saturating_add_signed(step as i32));
                if hour == self.mail_hour {
                    return None;
                }
                Setting::MailHour(hour)
            }
            Row::Hud => Setting::HideHud(!self.hide_hud),
            Row::City | Row::AutoLocation | Row::Diet => return None,
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

pub async fn send_unless_cancelled<T>(
    mut sender: oneshot::Sender<T>,
    work: impl Future<Output = T>,
) {
    tokio::select! {
        biased;
        () = sender.closed() => {}
        result = work => {
            sender.send(result).ok();
        }
    }
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
            auto_location: AutoLocation::Off,
            diets: None,
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
    fn test_mail_hour_stops_at_14() {
        let mut settings = settings();
        settings.mail_hour = 14;
        assert_eq!(settings.stepped(Row::MailHour, 1), None);
        assert_eq!(
            settings.stepped(Row::MailHour, -1),
            Some(Setting::MailHour(13))
        );
    }

    #[test]
    fn test_mail_hour_stops_at_6() {
        let mut settings = settings();
        settings.mail_hour = 6;
        assert_eq!(settings.stepped(Row::MailHour, -1), None);
        assert_eq!(
            settings.stepped(Row::MailHour, 1),
            Some(Setting::MailHour(7))
        );
    }

    #[test]
    fn test_location_and_diet_rows_are_not_stepped() {
        assert_eq!(settings().stepped(Row::City, 1), None);
        assert_eq!(settings().stepped(Row::AutoLocation, -1), None);
        assert_eq!(settings().stepped(Row::Diet, 1), None);
    }

    #[test]
    fn test_diet_row_appears_once_rose_has_an_answer() {
        let mut settings = settings();
        assert_eq!(settings.rows().last(), Some(&Row::AutoLocation));
        settings.diets = Some(BTreeSet::new());
        assert_eq!(settings.rows().last(), Some(&Row::Diet));
        assert_eq!(settings.rows().len(), 8);
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

    #[tokio::test]
    async fn test_cancelled_work_sends_nothing() {
        let (sender, receiver) = oneshot::channel::<u32>();
        drop(receiver);
        let mut worked = false;
        send_unless_cancelled(sender, async {
            worked = true;
            1
        })
        .await;
        assert!(!worked);

        let (sender, receiver) = oneshot::channel::<u32>();
        let waiting = tokio::spawn(send_unless_cancelled(sender, std::future::pending()));
        drop(receiver);
        tokio::time::timeout(std::time::Duration::from_secs(1), waiting)
            .await
            .unwrap()
            .unwrap();

        let (sender, mut receiver) = oneshot::channel();
        send_unless_cancelled(sender, async { 1 }).await;
        assert_eq!(receiver.try_recv(), Ok(1));
    }
}
