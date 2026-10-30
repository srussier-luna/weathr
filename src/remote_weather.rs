use crate::badges;
use crate::story::content::WEEKDAYS;
use crate::weather::WeatherCondition;
use crate::weather::normalizer::WeatherNormalizer;
use chrono::{Days, NaiveDate};
use serde::Deserialize;
use std::time::Duration;
use tokio::sync::mpsc;

const FORECAST_URL: &str = "https://api.open-meteo.com/v1/forecast";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
const RAINY_DAY_MM: f64 = 1.0;
const HEAT_WAVE_CELSIUS: f64 = 35.0;
const SEVERE_COLD_CELSIUS: f64 = -10.0;
const STRONG_GUSTS_KMH: f64 = 70.0;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Request {
    Postcard(usize),
    WeekForecast { week: usize, monday: NaiveDate },
}

pub struct Reply {
    pub request: Request,
    pub text: String,
}

#[derive(Deserialize)]
struct CurrentResponse {
    current: Current,
}

#[derive(Deserialize)]
struct Current {
    temperature_2m: f64,
    weather_code: i32,
}

#[derive(Deserialize)]
struct DailyResponse {
    daily: Daily,
}

#[derive(Deserialize)]
struct Daily {
    temperature_2m_max: Vec<Option<f64>>,
    temperature_2m_min: Vec<Option<f64>>,
    rain_sum: Vec<Option<f64>>,
    showers_sum: Vec<Option<f64>>,
    snowfall_sum: Vec<Option<f64>>,
    weather_code: Vec<Option<i32>>,
    wind_gusts_10m_max: Vec<Option<f64>>,
}

pub fn spawn(request: Request, latitude: f64, longitude: f64, sender: mpsc::Sender<Reply>) {
    tokio::spawn(async move {
        let Ok(client) = reqwest::Client::builder().timeout(REQUEST_TIMEOUT).build() else {
            return;
        };
        let text = match request {
            Request::Postcard(_) => current_weather(&client, latitude, longitude).await,
            Request::WeekForecast { monday, .. } => {
                week_forecast(&client, latitude, longitude, monday).await
            }
        };
        if let Some(text) = text {
            let _ = sender.send(Reply { request, text }).await;
        }
    });
}

async fn current_weather(
    client: &reqwest::Client,
    latitude: f64,
    longitude: f64,
) -> Option<String> {
    let url = format!(
        "{FORECAST_URL}?latitude={latitude}&longitude={longitude}&current=temperature_2m,weather_code&timezone=auto"
    );
    let response: CurrentResponse = client.get(&url).send().await.ok()?.json().await.ok()?;
    let condition = WeatherNormalizer::wmo_code_to_condition(response.current.weather_code);
    let label = badges::name(badges::weather_badge(condition)).to_lowercase();
    Some(format!(
        "Ici, aujourd'hui : {:.0} °C, {label}.",
        response.current.temperature_2m
    ))
}

async fn week_forecast(
    client: &reqwest::Client,
    latitude: f64,
    longitude: f64,
    monday: NaiveDate,
) -> Option<String> {
    let url = week_forecast_url(latitude, longitude, monday);
    let response: DailyResponse = client.get(&url).send().await.ok()?.json().await.ok()?;
    describe_week(&response.daily)
}

fn week_forecast_url(latitude: f64, longitude: f64, monday: NaiveDate) -> String {
    let sunday = monday + Days::new(6);
    format!(
        "{FORECAST_URL}?latitude={latitude}&longitude={longitude}&daily=temperature_2m_max,temperature_2m_min,rain_sum,showers_sum,snowfall_sum,weather_code,wind_gusts_10m_max&start_date={monday}&end_date={sunday}&timezone=auto"
    )
}

fn describe_week(daily: &Daily) -> Option<String> {
    let maximum = daily
        .temperature_2m_max
        .iter()
        .flatten()
        .copied()
        .reduce(f64::max)?;
    let minimum = daily
        .temperature_2m_min
        .iter()
        .flatten()
        .copied()
        .reduce(f64::min)?;
    let condition =
        |day| at(&daily.weather_code, day).map(WeatherNormalizer::wmo_code_to_condition);
    let rainy = days_where(|day| {
        rain_at(daily, day).is_some_and(|millimeters| millimeters >= RAINY_DAY_MM)
    });
    let skies: Vec<_> = (0..WEEKDAYS.len()).filter_map(condition).collect();
    let sunny = days_where(|day| condition(day).is_some_and(is_sunny));
    let snowy = days_where(|day| {
        at(&daily.snowfall_sum, day).is_some_and(|centimeters| centimeters > 0.0)
            || condition(day).is_some_and(|sky| sky.is_snowing())
    });
    let stormy = days_where(|day| condition(day).is_some_and(|sky| sky.is_thunderstorm()));
    let hot = days_where(|day| {
        at(&daily.temperature_2m_max, day).is_some_and(|celsius| celsius >= HEAT_WAVE_CELSIUS)
    });
    let cold = days_where(|day| {
        at(&daily.temperature_2m_min, day).is_some_and(|celsius| celsius <= SEVERE_COLD_CELSIUS)
    });
    let windy = days_where(|day| {
        at(&daily.wind_gusts_10m_max, day).is_some_and(|kmh| kmh >= STRONG_GUSTS_KMH)
    });

    let lines = [
        Some(format!(
            "Prévisions de la semaine\u{202F}: {} au plus, {} au moins.",
            degrees(maximum),
            degrees(minimum)
        )),
        best_day(daily, &[&sunny, &snowy], &[&stormy, &hot, &cold, &windy])
            .map(|day| format!("Plus beau jour\u{202F}: {}.", WEEKDAYS[day])),
        days_line("Pluie", &rainy, None).or_else(|| dry_week(&skies)),
        days_line("Neige", &snowy, None),
        days_line("⚠ Alerte orage", &stormy, None),
        days_line(
            "⚠ Alerte canicule",
            &hot,
            worst(&daily.temperature_2m_max, &hot, f64::max).map(degrees),
        ),
        days_line(
            "⚠ Alerte grand froid",
            &cold,
            worst(&daily.temperature_2m_min, &cold, f64::min).map(degrees),
        ),
        days_line(
            "⚠ Alerte vent fort",
            &windy,
            worst(&daily.wind_gusts_10m_max, &windy, f64::max)
                .map(|kmh| format!("rafales {kmh:.0} km/h")),
        ),
    ];
    Some(lines.into_iter().flatten().collect::<Vec<_>>().join("\n"))
}

fn best_day(daily: &Daily, preferred: &[&[usize]], alerts: &[&[usize]]) -> Option<usize> {
    (0..WEEKDAYS.len())
        .filter(|day| !alerts.iter().any(|alert| alert.contains(day)))
        .filter_map(|day| {
            let tier = preferred
                .iter()
                .position(|days| days.contains(&day))
                .unwrap_or(preferred.len());
            Some((
                day,
                tier,
                rain_at(daily, day)?,
                at(&daily.temperature_2m_max, day)?,
            ))
        })
        .min_by(
            |(_, tier, rain, warmth), (_, other_tier, other_rain, other_warmth)| {
                tier.cmp(other_tier)
                    .then(rain.total_cmp(other_rain))
                    .then(other_warmth.total_cmp(warmth))
            },
        )
        .map(|(day, ..)| day)
}

fn dry_week(skies: &[WeatherCondition]) -> Option<String> {
    if skies.is_empty() {
        return None;
    }
    let sunny_days = skies.iter().filter(|sky| is_sunny(**sky)).count();
    let recap = if sunny_days * 2 > skies.len() {
        "Semaine ensoleillée, sans pluie."
    } else {
        "Semaine nuageuse, sans pluie."
    };
    Some(recap.to_string())
}

fn is_sunny(sky: WeatherCondition) -> bool {
    matches!(
        sky,
        WeatherCondition::Clear | WeatherCondition::PartlyCloudy
    )
}

fn at<T: Copy>(values: &[Option<T>], day: usize) -> Option<T> {
    values.get(day).copied().flatten()
}

fn rain_at(daily: &Daily, day: usize) -> Option<f64> {
    [at(&daily.rain_sum, day), at(&daily.showers_sum, day)]
        .into_iter()
        .flatten()
        .reduce(|rain, showers| rain + showers)
}

fn days_where(is_met: impl Fn(usize) -> bool) -> Vec<usize> {
    (0..WEEKDAYS.len()).filter(|day| is_met(*day)).collect()
}

fn worst(values: &[Option<f64>], days: &[usize], pick: fn(f64, f64) -> f64) -> Option<f64> {
    days.iter().filter_map(|day| at(values, *day)).reduce(pick)
}

fn days_line(label: &str, days: &[usize], detail: Option<String>) -> Option<String> {
    if days.is_empty() {
        return None;
    }
    let names = days
        .iter()
        .map(|day| WEEKDAYS[*day])
        .collect::<Vec<_>>()
        .join(", ");
    let detail = detail.map_or_else(String::new, |detail| format!(" ({detail})"));
    Some(format!("{label}\u{202F}: {names}{detail}."))
}

fn degrees(celsius: f64) -> String {
    let rounded = celsius.round();
    let sign = if rounded < 0.0 { "\u{2212}" } else { "" };
    format!("{sign}{:.0} °C", rounded.abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calm_week() -> Daily {
        Daily {
            temperature_2m_max: vec![
                Some(12.4),
                Some(11.0),
                Some(14.2),
                Some(17.6),
                Some(15.0),
                Some(13.1),
                Some(12.0),
            ],
            temperature_2m_min: vec![
                Some(5.0),
                Some(3.2),
                Some(1.9),
                Some(6.0),
                Some(7.5),
                Some(4.0),
                Some(3.0),
            ],
            rain_sum: vec![
                Some(4.0),
                Some(0.5),
                Some(0.9),
                Some(0.0),
                Some(0.2),
                Some(0.0),
                Some(0.4),
            ],
            showers_sum: vec![
                Some(0.0),
                Some(0.5),
                Some(0.0),
                Some(0.0),
                Some(0.0),
                Some(0.0),
                Some(0.0),
            ],
            snowfall_sum: vec![Some(0.0); 7],
            weather_code: vec![
                Some(61),
                Some(61),
                Some(3),
                Some(0),
                Some(2),
                Some(1),
                Some(3),
            ],
            wind_gusts_10m_max: vec![Some(30.0); 7],
        }
    }

    fn line(daily: &Daily, label: &str) -> Option<String> {
        describe_week(daily)?
            .lines()
            .find(|line| line.starts_with(label))
            .map(str::to_string)
    }

    #[test]
    fn test_calm_week_text() {
        assert_eq!(
            describe_week(&calm_week()).as_deref(),
            Some(
                "Prévisions de la semaine\u{202F}: 18 °C au plus, 2 °C au moins.\n\
                 Plus beau jour\u{202F}: jeudi.\n\
                 Pluie\u{202F}: lundi, mardi."
            )
        );
    }

    #[test]
    fn test_dry_week_recap() {
        let mut daily = calm_week();
        daily.rain_sum[0] = Some(0.0);
        daily.rain_sum[1] = Some(0.0);
        daily.showers_sum[1] = Some(0.0);
        daily.weather_code = vec![
            Some(0),
            Some(1),
            Some(2),
            Some(3),
            Some(0),
            Some(1),
            Some(3),
        ];
        assert_eq!(
            describe_week(&daily).as_deref(),
            Some(
                "Prévisions de la semaine\u{202F}: 18 °C au plus, 2 °C au moins.\n\
                 Plus beau jour\u{202F}: samedi.\n\
                 Semaine ensoleillée, sans pluie."
            )
        );

        daily.weather_code = vec![
            Some(3),
            Some(3),
            Some(45),
            Some(0),
            Some(3),
            Some(2),
            Some(3),
        ];
        assert_eq!(
            line(&daily, "Semaine").as_deref(),
            Some("Semaine nuageuse, sans pluie.")
        );

        daily.weather_code = vec![Some(0), Some(1), Some(2), Some(3), Some(3), Some(45), None];
        assert_eq!(
            line(&daily, "Semaine").as_deref(),
            Some("Semaine nuageuse, sans pluie.")
        );

        daily.weather_code = vec![None; 7];
        assert_eq!(line(&daily, "Semaine"), None);

        assert_eq!(line(&calm_week(), "Semaine"), None);
    }

    #[test]
    fn test_snow_days() {
        let mut daily = calm_week();
        daily.snowfall_sum[5] = Some(1.2);
        assert_eq!(
            line(&daily, "Neige").as_deref(),
            Some("Neige\u{202F}: samedi.")
        );

        let mut daily = calm_week();
        daily.weather_code[5] = Some(73);
        assert_eq!(
            line(&daily, "Neige").as_deref(),
            Some("Neige\u{202F}: samedi.")
        );
    }

    #[test]
    fn test_snow_is_not_rain() {
        let mut daily = calm_week();
        daily.snowfall_sum[5] = Some(4.2);
        daily.weather_code[5] = Some(73);
        assert_eq!(
            line(&daily, "Pluie").as_deref(),
            Some("Pluie\u{202F}: lundi, mardi.")
        );
        assert_eq!(
            line(&daily, "Neige").as_deref(),
            Some("Neige\u{202F}: samedi.")
        );

        daily.rain_sum = vec![Some(0.0); 7];
        daily.showers_sum = vec![Some(0.0); 7];
        daily.snowfall_sum = vec![Some(2.0); 7];
        daily.weather_code = vec![Some(71); 7];
        assert_eq!(line(&daily, "Pluie"), None);
        assert_eq!(
            line(&daily, "Neige").as_deref(),
            Some("Neige\u{202F}: lundi, mardi, mercredi, jeudi, vendredi, samedi, dimanche.")
        );
        assert_eq!(
            line(&daily, "Semaine").as_deref(),
            Some("Semaine nuageuse, sans pluie.")
        );
        assert_eq!(
            line(&daily, "Plus beau jour").as_deref(),
            Some("Plus beau jour\u{202F}: jeudi.")
        );
    }

    #[test]
    fn test_thunderstorm_alert() {
        for code in [95, 96, 99] {
            let mut daily = calm_week();
            daily.weather_code[4] = Some(code);
            assert_eq!(
                line(&daily, "⚠ Alerte orage").as_deref(),
                Some("⚠ Alerte orage\u{202F}: vendredi.")
            );
        }
    }

    #[test]
    fn test_heat_wave_alert() {
        let mut daily = calm_week();
        daily.temperature_2m_max[1] = Some(36.0);
        assert_eq!(
            line(&daily, "⚠ Alerte canicule").as_deref(),
            Some("⚠ Alerte canicule\u{202F}: mardi (36 °C).")
        );

        daily.temperature_2m_max[3] = Some(38.2);
        assert_eq!(
            line(&daily, "⚠ Alerte canicule").as_deref(),
            Some("⚠ Alerte canicule\u{202F}: mardi, jeudi (38 °C).")
        );

        let mut daily = calm_week();
        daily.temperature_2m_max[1] = Some(35.0);
        assert_eq!(
            line(&daily, "⚠ Alerte canicule").as_deref(),
            Some("⚠ Alerte canicule\u{202F}: mardi (35 °C).")
        );

        daily.temperature_2m_max[1] = Some(34.9);
        assert_eq!(line(&daily, "⚠ Alerte canicule"), None);
    }

    #[test]
    fn test_severe_cold_alert() {
        let mut daily = calm_week();
        daily.temperature_2m_min[2] = Some(-12.0);
        assert_eq!(
            line(&daily, "⚠ Alerte grand froid").as_deref(),
            Some("⚠ Alerte grand froid\u{202F}: mercredi (\u{2212}12 °C).")
        );
        assert_eq!(
            line(&daily, "Prévisions").as_deref(),
            Some("Prévisions de la semaine\u{202F}: 18 °C au plus, \u{2212}12 °C au moins.")
        );

        daily.temperature_2m_min[2] = Some(-10.0);
        assert_eq!(
            line(&daily, "⚠ Alerte grand froid").as_deref(),
            Some("⚠ Alerte grand froid\u{202F}: mercredi (\u{2212}10 °C).")
        );

        daily.temperature_2m_min[2] = Some(-9.9);
        assert_eq!(line(&daily, "⚠ Alerte grand froid"), None);
    }

    #[test]
    fn test_strong_wind_alert() {
        let mut daily = calm_week();
        daily.wind_gusts_10m_max[2] = Some(85.0);
        assert_eq!(
            line(&daily, "⚠ Alerte vent fort").as_deref(),
            Some("⚠ Alerte vent fort\u{202F}: mercredi (rafales 85 km/h).")
        );

        daily.wind_gusts_10m_max[2] = Some(70.0);
        assert_eq!(
            line(&daily, "⚠ Alerte vent fort").as_deref(),
            Some("⚠ Alerte vent fort\u{202F}: mercredi (rafales 70 km/h).")
        );

        daily.wind_gusts_10m_max[2] = Some(69.9);
        assert_eq!(line(&daily, "⚠ Alerte vent fort"), None);
    }

    #[test]
    fn test_best_day() {
        let best_day = |daily: &Daily| line(daily, "Plus beau jour");

        let mut daily = calm_week();
        daily.rain_sum[3] = Some(0.3);
        daily.rain_sum[5] = Some(0.1);
        daily.rain_sum[6] = Some(0.0);
        assert_eq!(
            best_day(&daily).as_deref(),
            Some("Plus beau jour\u{202F}: samedi.")
        );

        daily.rain_sum[4] = Some(0.1);
        assert_eq!(
            best_day(&daily).as_deref(),
            Some("Plus beau jour\u{202F}: vendredi.")
        );

        daily.temperature_2m_max[5] = Some(15.0);
        assert_eq!(
            best_day(&daily).as_deref(),
            Some("Plus beau jour\u{202F}: vendredi.")
        );

        daily.wind_gusts_10m_max[4] = Some(80.0);
        assert_eq!(
            best_day(&daily).as_deref(),
            Some("Plus beau jour\u{202F}: samedi.")
        );

        let mut daily = calm_week();
        daily.weather_code = vec![
            Some(61),
            Some(61),
            Some(3),
            Some(3),
            Some(3),
            Some(1),
            Some(71),
        ];
        daily.rain_sum[5] = Some(0.3);
        daily.rain_sum[6] = Some(0.0);
        daily.snowfall_sum[6] = Some(2.0);
        daily.temperature_2m_max[6] = Some(20.0);
        assert_eq!(
            best_day(&daily).as_deref(),
            Some("Plus beau jour\u{202F}: samedi.")
        );

        let mut daily = calm_week();
        daily.weather_code = vec![Some(3); 7];
        daily.snowfall_sum[4] = Some(1.5);
        assert_eq!(
            best_day(&daily).as_deref(),
            Some("Plus beau jour\u{202F}: vendredi.")
        );

        daily.snowfall_sum[4] = Some(0.0);
        assert_eq!(
            best_day(&daily).as_deref(),
            Some("Plus beau jour\u{202F}: jeudi.")
        );

        daily.temperature_2m_max[3] = Some(36.0);
        assert_eq!(
            best_day(&daily).as_deref(),
            Some("Plus beau jour\u{202F}: samedi.")
        );

        daily.wind_gusts_10m_max = vec![Some(90.0); 7];
        assert_eq!(best_day(&daily), None);
    }

    #[test]
    fn test_week_forecast_url_covers_the_gazette_week() {
        let monday = NaiveDate::from_ymd_opt(2026, 11, 2).unwrap();
        assert_eq!(
            week_forecast_url(48.85, 2.35, monday),
            "https://api.open-meteo.com/v1/forecast?latitude=48.85&longitude=2.35\
             &daily=temperature_2m_max,temperature_2m_min,rain_sum,showers_sum,snowfall_sum,weather_code,wind_gusts_10m_max\
             &start_date=2026-11-02&end_date=2026-11-08&timezone=auto"
        );
    }
}
