use serde::Deserialize;
use std::time::Duration;
use tokio::sync::mpsc;

const FORECAST_URL: &str = "https://api.open-meteo.com/v1/forecast";
const REFRESH_INTERVAL: Duration = Duration::from_secs(600);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
const MIN_PRECIPITATION_MM: f64 = 0.1;
const MIN_PROBABILITY: f64 = 50.0;

#[derive(Deserialize)]
struct ForecastResponse {
    minutely_15: Option<QuarterHours>,
    hourly: Option<Hours>,
}

#[derive(Deserialize)]
struct QuarterHours {
    time: Vec<String>,
    precipitation: Vec<Option<f64>>,
}

#[derive(Deserialize)]
struct Hours {
    precipitation_probability: Vec<Option<f64>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RainAlert {
    At(String),
    Soon,
}

impl RainAlert {
    pub fn message(&self) -> String {
        match self {
            Self::At(time) => format!("Pluie vers {time} : prends ton parapluie !"),
            Self::Soon => "Pluie probable dans l'heure : prends ton parapluie !".to_string(),
        }
    }
}

pub fn spawn(latitude: f64, longitude: f64) -> mpsc::Receiver<Option<RainAlert>> {
    let (sender, receiver) = mpsc::channel(1);
    tokio::spawn(async move {
        let Ok(client) = reqwest::Client::builder().timeout(REQUEST_TIMEOUT).build() else {
            return;
        };
        loop {
            if let Some(forecast) = fetch(&client, latitude, longitude).await
                && sender.send(alert_from(forecast)).await.is_err()
            {
                break;
            }
            tokio::time::sleep(REFRESH_INTERVAL).await;
        }
    });
    receiver
}

async fn fetch(
    client: &reqwest::Client,
    latitude: f64,
    longitude: f64,
) -> Option<ForecastResponse> {
    let url = format!(
        "{FORECAST_URL}?latitude={latitude}&longitude={longitude}&minutely_15=precipitation&forecast_minutely_15=4&hourly=precipitation_probability&forecast_hours=1&timezone=auto"
    );
    client.get(&url).send().await.ok()?.json().await.ok()
}

fn alert_from(forecast: ForecastResponse) -> Option<RainAlert> {
    if let Some(quarters) = &forecast.minutely_15 {
        let rainy_quarter = quarters
            .time
            .iter()
            .zip(&quarters.precipitation)
            .find(|(_, amount)| amount.unwrap_or(0.0) >= MIN_PRECIPITATION_MM);
        if let Some((time, _)) = rainy_quarter {
            return Some(RainAlert::At(time.get(11..16).unwrap_or(time).to_string()));
        }
    }

    let probability = forecast
        .hourly?
        .precipitation_probability
        .first()
        .copied()
        .flatten()?;
    (probability >= MIN_PROBABILITY).then_some(RainAlert::Soon)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn forecast(amounts: [f64; 4], probability: f64) -> ForecastResponse {
        ForecastResponse {
            minutely_15: Some(QuarterHours {
                time: ["12:15", "12:30", "12:45", "13:00"]
                    .iter()
                    .map(|time| format!("2026-10-22T{time}"))
                    .collect(),
                precipitation: amounts.iter().map(|amount| Some(*amount)).collect(),
            }),
            hourly: Some(Hours {
                precipitation_probability: vec![Some(probability)],
            }),
        }
    }

    #[test]
    fn test_alert_uses_first_rainy_quarter() {
        let alert = alert_from(forecast([0.0, 0.0, 0.4, 1.0], 80.0));
        assert_eq!(alert, Some(RainAlert::At("12:45".to_string())));
    }

    #[test]
    fn test_alert_falls_back_on_probability() {
        assert_eq!(alert_from(forecast([0.0; 4], 60.0)), Some(RainAlert::Soon));
        assert_eq!(alert_from(forecast([0.0; 4], 20.0)), None);
    }
}
