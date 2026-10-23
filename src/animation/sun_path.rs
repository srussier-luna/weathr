use chrono::{DateTime, Datelike, Timelike, Utc};

const SUN_ALTITUDE_AT_HORIZON: f64 = -0.833;
const MINUTES_PER_DEGREE: f64 = 4.0;

pub fn day_progress(latitude: f64, longitude: f64, now: DateTime<Utc>) -> Option<f64> {
    let (sunrise, sunset) = sunrise_and_sunset(latitude, longitude, now.ordinal() as f64)?;
    let minutes = now.hour() as f64 * 60.0 + now.minute() as f64 + now.second() as f64 / 60.0;
    let progress = (minutes - sunrise) / (sunset - sunrise);
    (0.0..=1.0).contains(&progress).then_some(progress)
}

fn sunrise_and_sunset(latitude: f64, longitude: f64, day_of_year: f64) -> Option<(f64, f64)> {
    let angle = (360.0 / 365.0 * (day_of_year - 81.0)).to_radians();
    let equation_of_time = 9.87 * (2.0 * angle).sin() - 7.53 * angle.cos() - 1.5 * angle.sin();
    let declination = 23.44_f64.to_radians() * angle.sin();
    let latitude = latitude.to_radians();

    let cos_hour_angle = (SUN_ALTITUDE_AT_HORIZON.to_radians().sin()
        - latitude.sin() * declination.sin())
        / (latitude.cos() * declination.cos());
    if cos_hour_angle >= 1.0 {
        return None;
    }
    let hour_angle = cos_hour_angle.max(-1.0).acos().to_degrees();
    let solar_noon = 720.0 - MINUTES_PER_DEGREE * longitude - equation_of_time;

    Some((
        solar_noon - MINUTES_PER_DEGREE * hour_angle,
        solar_noon + MINUTES_PER_DEGREE * hour_angle,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_sun_times_in_chambery() {
        let (sunrise, sunset) = sunrise_and_sunset(45.57, 5.92, 295.0).unwrap();
        assert!((353.0..369.0).contains(&sunrise), "sunrise {sunrise}");
        assert!((990.0..1006.0).contains(&sunset), "sunset {sunset}");
    }

    #[test]
    fn test_day_progress() {
        let noon = Utc.with_ymd_and_hms(2026, 10, 22, 11, 20, 0).unwrap();
        let night = Utc.with_ymd_and_hms(2026, 10, 22, 22, 0, 0).unwrap();
        let progress = day_progress(45.57, 5.92, noon).unwrap();
        assert!((0.45..0.55).contains(&progress), "progress {progress}");
        assert_eq!(day_progress(45.57, 5.92, night), None);
    }
}
