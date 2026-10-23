use crate::badges;
use crate::weather::normalizer::WeatherNormalizer;
use serde::Deserialize;
use std::time::Duration;
use tokio::sync::mpsc;

const FORECAST_URL: &str = "https://api.open-meteo.com/v1/forecast";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
const RAINY_DAY_MM: f64 = 1.0;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Request {
    Postcard(usize),
    WeekRecap(usize),
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
    precipitation_sum: Vec<Option<f64>>,
}

pub fn spawn(request: Request, latitude: f64, longitude: f64, sender: mpsc::Sender<Reply>) {
    tokio::spawn(async move {
        let Ok(client) = reqwest::Client::builder().timeout(REQUEST_TIMEOUT).build() else {
            return;
        };
        let text = match request {
            Request::Postcard(_) => current_weather(&client, latitude, longitude).await,
            Request::WeekRecap(_) => week_recap(&client, latitude, longitude).await,
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

async fn week_recap(client: &reqwest::Client, latitude: f64, longitude: f64) -> Option<String> {
    let url = format!(
        "{FORECAST_URL}?latitude={latitude}&longitude={longitude}&daily=temperature_2m_max,temperature_2m_min,precipitation_sum&past_days=7&forecast_days=0&timezone=auto"
    );
    let response: DailyResponse = client.get(&url).send().await.ok()?.json().await.ok()?;
    describe_week(&response.daily)
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
    let rainy_days = daily
        .precipitation_sum
        .iter()
        .flatten()
        .filter(|amount| **amount >= RAINY_DAY_MM)
        .count();
    let plural = if rainy_days > 1 { "s" } else { "" };
    Some(format!(
        "Ta semaine : {rainy_days} jour{plural} de pluie, maximum {maximum:.0} °C, minimum {minimum:.0} °C."
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_week_summary() {
        let daily = Daily {
            temperature_2m_max: vec![Some(12.4), Some(17.6), None],
            temperature_2m_min: vec![Some(3.2), Some(5.0), Some(1.9)],
            precipitation_sum: vec![Some(0.2), Some(4.0), Some(1.0)],
        };
        assert_eq!(
            describe_week(&daily).as_deref(),
            Some("Ta semaine : 2 jours de pluie, maximum 18 °C, minimum 2 °C.")
        );
    }
}
