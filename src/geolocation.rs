use crate::cache;
use crate::error::{GeolocationError, NetworkError};
use serde::{Deserialize, Serialize};
use std::time::Duration;

const IPINFO_URL: &str = "https://ipinfo.io/json";
const NOMINATIM_REVERSE_URL: &str = "https://nominatim.openstreetmap.org/reverse";
const NOMINATIM_SEARCH_URL: &str = "https://nominatim.openstreetmap.org/search";
const NOMINATIM_TIMEOUT_SECS: u64 = 5;
const NOMINATIM_CONNECT_TIMEOUT: Duration = Duration::from_secs(3);
const MAX_CITY_RESULTS: usize = 5;
const MAX_RETRIES: u32 = 3;
const INITIAL_RETRY_DELAY_MS: u64 = 500;
const CORE_LOCATION_CLI: &str = "CoreLocationCLI";
const CORE_LOCATION_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Deserialize, Debug)]
struct IpInfoResponse {
    loc: String,
    city: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub city: Option<String>,
}

pub async fn detect_location() -> Result<GeoLocation, GeolocationError> {
    if let Some(location) = detect_device_location().await {
        return Ok(location);
    }

    if let Some(cached) = cache::load_cached_location().await {
        return Ok(cached);
    }

    detect_location_with_retry().await
}

async fn detect_device_location() -> Option<GeoLocation> {
    let command = tokio::process::Command::new(CORE_LOCATION_CLI)
        .args(["--format", "%latitude %longitude %locality"])
        .kill_on_drop(true)
        .output();
    let output = tokio::time::timeout(CORE_LOCATION_TIMEOUT, command)
        .await
        .ok()?
        .ok()?;

    if !output.status.success() {
        return None;
    }

    parse_device_location(&String::from_utf8_lossy(&output.stdout))
}

fn parse_device_location(output: &str) -> Option<GeoLocation> {
    let mut parts = output.trim().splitn(3, ' ');
    let latitude = parts.next()?.parse().ok()?;
    let longitude = parts.next()?.parse().ok()?;
    let city = parts
        .next()
        .map(str::trim)
        .filter(|city| !city.is_empty())
        .map(str::to_string);

    Some(GeoLocation {
        latitude,
        longitude,
        city,
    })
}

async fn detect_location_with_retry() -> Result<GeoLocation, GeolocationError> {
    let mut last_error = None;

    for attempt in 1..=MAX_RETRIES {
        match fetch_location().await {
            Ok(location) => return Ok(location),
            Err(e) => {
                let should_retry = matches!(
                    e,
                    GeolocationError::Unreachable(ref net_err) if net_err.is_retryable()
                );

                if !should_retry || attempt == MAX_RETRIES {
                    return Err(e);
                }

                let delay_ms = INITIAL_RETRY_DELAY_MS * 2_u64.pow(attempt - 1);
                tokio::time::sleep(Duration::from_millis(delay_ms)).await;
                last_error = Some(e);
            }
        }
    }

    Err(
        last_error.unwrap_or_else(|| GeolocationError::RetriesExhausted {
            attempts: MAX_RETRIES,
        }),
    )
}

async fn fetch_location() -> Result<GeoLocation, GeolocationError> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .connect_timeout(Duration::from_secs(5))
        .build()
        .map_err(|e| GeolocationError::Unreachable(NetworkError::ClientCreation(e)))?;

    let response = client.get(IPINFO_URL).send().await.map_err(|e| {
        GeolocationError::Unreachable(NetworkError::from_reqwest(e, IPINFO_URL, 10))
    })?;

    let ip_info: IpInfoResponse = response.json().await.map_err(|e| {
        GeolocationError::Unreachable(NetworkError::from_reqwest(e, IPINFO_URL, 10))
    })?;

    let coords: Vec<&str> = ip_info.loc.split(',').collect();
    if coords.len() != 2 {
        return Err(GeolocationError::ParseError(
            "Invalid location format from ipinfo.io".to_string(),
        ));
    }

    let latitude = coords[0]
        .parse::<f64>()
        .map_err(|_| GeolocationError::ParseError("Invalid latitude format".to_string()))?;

    let longitude = coords[1]
        .parse::<f64>()
        .map_err(|_| GeolocationError::ParseError("Invalid longitude format".to_string()))?;

    let location = GeoLocation {
        latitude,
        longitude,
        city: ip_info.city,
    };

    cache::save_location_cache(&location);

    Ok(location)
}

#[derive(Deserialize, Debug)]
struct NominatimAddress {
    city: Option<String>,
    town: Option<String>,
    village: Option<String>,
    state_district: Option<String>,
    county: Option<String>,
    state: Option<String>,
    country: Option<String>,
}

#[derive(Deserialize, Debug)]
struct NominatimResponse {
    address: Option<NominatimAddress>,
}

/// Best-effort reverse geocode: returns a city/town/village name for the given
/// coordinates, or `None` if the lookup fails or the location doesn't map to a
/// meaningful settlement (e.g. open sea, administrative-only regions).
pub async fn reverse_geocode(latitude: f64, longitude: f64, language: &str) -> Option<String> {
    if let Some(cached) = cache::load_cached_geocode(latitude, longitude, language).await {
        return Some(cached);
    }

    let city = fetch_reverse_geocode(latitude, longitude, language).await?;
    cache::save_geocode_cache(&city, latitude, longitude, language);
    Some(city)
}

async fn fetch_reverse_geocode(latitude: f64, longitude: f64, language: &str) -> Option<String> {
    let url = format!(
        "{}?lat={}&lon={}&format=json&zoom=10",
        NOMINATIM_REVERSE_URL, latitude, longitude
    );
    let resp = nominatim_get(&url, language).ok()?.send().await.ok()?;

    let data: NominatimResponse = resp.json().await.ok()?;

    let addr = data.address?;
    addr.city.or(addr.town).or(addr.village)
}

fn nominatim_get(url: &str, language: &str) -> Result<reqwest::RequestBuilder, reqwest::Error> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(NOMINATIM_TIMEOUT_SECS))
        .connect_timeout(NOMINATIM_CONNECT_TIMEOUT)
        .build()?;

    let mut req = client.get(url).header(
        "User-Agent",
        format!("weathr/{}", env!("CARGO_PKG_VERSION")),
    );

    if language != "auto" {
        req = req.header("Accept-Language", language);
    }

    Ok(req)
}

pub async fn detect_location_with_city(language: &str) -> Result<GeoLocation, GeolocationError> {
    let mut location = detect_location().await?;
    if location.city.is_none() {
        location.city = reverse_geocode(location.latitude, location.longitude, language).await;
    }
    Ok(location)
}

#[derive(Debug, Clone)]
pub struct Place {
    pub label: String,
    pub location: GeoLocation,
}

#[derive(Deserialize, Debug)]
struct NominatimPlace {
    lat: String,
    lon: String,
    name: String,
    address: Option<NominatimAddress>,
}

pub async fn search_cities(query: &str, language: &str) -> Result<Vec<Place>, GeolocationError> {
    let unreachable = |e| {
        GeolocationError::Unreachable(NetworkError::from_reqwest(
            e,
            NOMINATIM_SEARCH_URL,
            NOMINATIM_TIMEOUT_SECS,
        ))
    };
    let limit = MAX_CITY_RESULTS.to_string();
    let url = reqwest::Url::parse_with_params(
        NOMINATIM_SEARCH_URL,
        [
            ("q", query),
            ("format", "jsonv2"),
            ("addressdetails", "1"),
            ("featureType", "city"),
            ("limit", &limit),
        ],
    )
    .map_err(|e| GeolocationError::ParseError(e.to_string()))?;

    let request = nominatim_get(url.as_str(), language)
        .map_err(|e| GeolocationError::Unreachable(NetworkError::ClientCreation(e)))?;
    let results: Vec<NominatimPlace> = request
        .send()
        .await
        .and_then(reqwest::Response::error_for_status)
        .map_err(unreachable)?
        .json()
        .await
        .map_err(unreachable)?;

    Ok(places_from(results))
}

fn places_from(results: Vec<NominatimPlace>) -> Vec<Place> {
    let mut places: Vec<Place> = Vec::new();
    for result in results {
        let (Ok(latitude), Ok(longitude)) = (result.lat.parse(), result.lon.parse()) else {
            continue;
        };
        let label = place_label(&result.name, result.address.as_ref());
        if result.name.is_empty() || places.iter().any(|place| place.label == label) {
            continue;
        }
        places.push(Place {
            label,
            location: GeoLocation {
                latitude,
                longitude,
                city: Some(result.name),
            },
        });
    }
    places.truncate(MAX_CITY_RESULTS);
    places
}

fn place_label(name: &str, address: Option<&NominatimAddress>) -> String {
    let region = address.and_then(|address| {
        address
            .state_district
            .as_deref()
            .or(address.county.as_deref())
            .or(address.state.as_deref())
    });
    let country = address.and_then(|address| address.country.as_deref());
    let mut parts = vec![name];
    parts.extend(region.filter(|region| *region != name));
    parts.extend(country);
    parts.join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    const LYON_RESULTS: &str = r#"[
        {"lat": "45.7578137", "lon": "4.8320114", "name": "Lyon",
         "address": {"city": "Lyon", "county": "Métropole de Lyon", "state_district": "Rhône",
                     "state": "Auvergne-Rhône-Alpes", "country": "France"}},
        {"lat": "45.7600000", "lon": "4.8400000", "name": "Lyon",
         "address": {"municipality": "Lyon", "county": "Rhône", "state_district": "Rhône",
                     "state": "Auvergne-Rhône-Alpes", "country": "France"}},
        {"lat": "34.2120", "lon": "-90.5432", "name": "Lyon",
         "address": {"village": "Lyon", "county": "Coahoma County", "state": "Mississippi",
                     "country": "United States"}},
        {"lat": "not a number", "lon": "0", "name": "Broken", "address": null},
        {"lat": "48.8566", "lon": "2.3522", "name": "Paris",
         "address": {"state": "Paris", "country": "France"}}
    ]"#;

    fn parsed(json: &str) -> Vec<Place> {
        places_from(serde_json::from_str(json).unwrap())
    }

    #[test]
    fn test_places_are_labelled_with_region_and_country() {
        let places = parsed(LYON_RESULTS);
        let labels: Vec<&str> = places.iter().map(|place| place.label.as_str()).collect();
        assert_eq!(
            labels,
            [
                "Lyon, Rhône, France",
                "Lyon, Coahoma County, United States",
                "Paris, France"
            ]
        );
        assert_eq!(places[0].location.latitude, 45.7578137);
        assert_eq!(places[0].location.longitude, 4.8320114);
        assert_eq!(places[0].location.city.as_deref(), Some("Lyon"));
    }

    #[test]
    fn test_no_result_gives_no_place() {
        assert!(parsed("[]").is_empty());
    }

    #[test]
    fn test_places_stop_at_five() {
        let results: Vec<String> = (0..8)
            .map(|index| {
                format!(r#"{{"lat": "1", "lon": "2", "name": "Ville {index}", "address": null}}"#)
            })
            .collect();
        assert_eq!(
            parsed(&format!("[{}]", results.join(","))).len(),
            MAX_CITY_RESULTS
        );
    }
}
