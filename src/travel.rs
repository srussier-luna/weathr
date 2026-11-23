use crate::geolocation::{self, GeoLocation};
use crate::progress::Progress;
use crate::story::content::chronicles::CITIES;

const SAM_CITY_RADIUS_KM: f64 = 10.0;

pub fn record(progress: &mut Progress, location: &GeoLocation) -> bool {
    let is_new_city = location
        .city
        .as_deref()
        .map(str::trim)
        .filter(|city| !city.is_empty())
        .is_some_and(|city| progress.visited_cities.insert(city.to_lowercase()));
    let is_new_sam_city = sam_city(location).is_some_and(|index| progress.sam_cities.insert(index));
    is_new_city || is_new_sam_city
}

fn sam_city(location: &GeoLocation) -> Option<usize> {
    CITIES.iter().position(|city| {
        geolocation::distance_km(
            (location.latitude, location.longitude),
            (city.latitude, city.longitude),
        ) <= SAM_CITY_RADIUS_KM
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::badges::Tier;

    const LISBON: usize = 0;

    fn detected(city: Option<&str>) -> GeoLocation {
        GeoLocation {
            latitude: 45.76,
            longitude: 4.84,
            city: city.map(str::to_string),
        }
    }

    fn detected_at(latitude: f64, longitude: f64, city: &str) -> GeoLocation {
        GeoLocation {
            latitude,
            longitude,
            city: Some(city.to_string()),
        }
    }

    #[test]
    fn test_same_city_counts_once_whatever_its_case() {
        let mut progress = Progress::default();
        assert!(record(&mut progress, &detected(Some("Lyon"))));
        assert!(!record(&mut progress, &detected(Some("lyon"))));
        assert!(!record(&mut progress, &detected(Some(" LYON "))));
        assert!(record(&mut progress, &detected(Some("Paris"))));
        assert_eq!(progress.visited_cities.len(), 2);
    }

    #[test]
    fn test_detection_without_city_name_counts_nothing() {
        let mut progress = Progress::default();
        assert!(!record(&mut progress, &detected(None)));
        assert!(!record(&mut progress, &detected(Some("  "))));
        assert!(progress.visited_cities.is_empty());
    }

    #[test]
    fn test_point_within_10_km_of_lisbon_is_a_sam_city() {
        assert_eq!(CITIES[LISBON].name, "Lisbonne");
        assert_eq!(
            sam_city(&detected_at(38.7223, -9.1393, "Lisbon")),
            Some(LISBON)
        );
        assert_eq!(sam_city(&detected_at(38.79, -9.14, "Loures")), Some(LISBON));
    }

    #[test]
    fn test_point_beyond_10_km_of_every_sam_city_counts_for_nothing() {
        assert_eq!(sam_city(&detected_at(38.82, -9.14, "Loures")), None);
        assert_eq!(sam_city(&detected(Some("Lyon"))), None);
    }

    #[test]
    fn test_sam_city_is_found_by_coordinates_not_name() {
        let mut progress = Progress::default();
        assert!(record(&mut progress, &detected_at(38.72, -9.14, "Lisbon")));
        assert_eq!(progress.sam_cities, [LISBON].into());
        assert_eq!(progress.visited_cities, ["lisbon".to_string()].into());
    }

    #[test]
    fn test_same_sam_city_counts_once() {
        let mut progress = Progress::default();
        assert!(record(&mut progress, &detected_at(38.72, -9.14, "Lisbon")));
        assert!(!record(&mut progress, &detected_at(38.73, -9.15, "Lisbon")));
        assert!(record(&mut progress, &detected_at(38.75, -9.20, "Amadora")));
        assert_eq!(progress.sam_cities.len(), 1);
    }

    #[test]
    fn test_sam_city_without_city_name_still_counts() {
        let mut progress = Progress::default();
        let lisbon = GeoLocation {
            latitude: 38.72,
            longitude: -9.14,
            city: None,
        };
        assert!(record(&mut progress, &lisbon));
        assert!(progress.visited_cities.is_empty());
        assert_eq!(progress.sam_cities.len(), 1);
    }

    #[test]
    fn test_every_sam_city_is_its_own_city() {
        let indices: Vec<_> = CITIES
            .iter()
            .map(|city| sam_city(&detected_at(city.latitude, city.longitude, city.name)))
            .collect();
        let expected: Vec<_> = (0..CITIES.len()).map(Some).collect();
        assert_eq!(indices, expected);
    }

    #[test]
    fn test_platinum_needs_every_sam_city() {
        assert_eq!(Tier::Platinum.threshold(), CITIES.len());
    }
}
