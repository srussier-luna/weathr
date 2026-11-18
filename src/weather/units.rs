use super::types::{PrecipitationUnit, TemperatureUnit, WindSpeedUnit};

pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

pub fn ms_to_kmh(ms: f64) -> f64 {
    ms * 3.6
}

pub fn kmh_to_ms(kmh: f64) -> f64 {
    kmh / 3.6
}

pub fn ms_to_mph(ms: f64) -> f64 {
    ms * 2.236936
}

pub fn mph_to_ms(mph: f64) -> f64 {
    mph / 2.236936
}

pub fn ms_to_kn(ms: f64) -> f64 {
    ms * 1.943844
}

pub fn kn_to_ms(kn: f64) -> f64 {
    kn / 1.943844
}

pub fn mm_to_inch(mm: f64) -> f64 {
    mm / 25.4
}

pub fn inch_to_mm(inch: f64) -> f64 {
    inch * 25.4
}

pub fn temperature_symbol(unit: TemperatureUnit) -> &'static str {
    match unit {
        TemperatureUnit::Celsius => "°C",
        TemperatureUnit::Fahrenheit => "°F",
    }
}

pub fn wind_speed_symbol(unit: WindSpeedUnit) -> &'static str {
    match unit {
        WindSpeedUnit::Ms => "m/s",
        WindSpeedUnit::Kmh => "km/h",
        WindSpeedUnit::Mph => "mph",
        WindSpeedUnit::Kn => "kn",
    }
}

pub fn precipitation_symbol(unit: PrecipitationUnit) -> &'static str {
    match unit {
        PrecipitationUnit::Mm => "mm",
        PrecipitationUnit::Inch => "in",
    }
}

pub fn format_temperature(celsius: f64, unit: TemperatureUnit) -> (f64, &'static str) {
    let value = match unit {
        TemperatureUnit::Celsius => celsius,
        TemperatureUnit::Fahrenheit => celsius_to_fahrenheit(celsius),
    };
    (value, temperature_symbol(unit))
}

pub fn format_wind_speed(ms: f64, unit: WindSpeedUnit) -> (f64, &'static str) {
    let value = match unit {
        WindSpeedUnit::Ms => ms,
        WindSpeedUnit::Kmh => ms_to_kmh(ms),
        WindSpeedUnit::Mph => ms_to_mph(ms),
        WindSpeedUnit::Kn => ms_to_kn(ms),
    };
    (value, wind_speed_symbol(unit))
}

pub fn format_precipitation(mm: f64, unit: PrecipitationUnit) -> (f64, &'static str) {
    let value = match unit {
        PrecipitationUnit::Mm => mm,
        PrecipitationUnit::Inch => mm_to_inch(mm),
    };
    (value, precipitation_symbol(unit))
}

pub fn normalize_temperature(value: f64, unit: TemperatureUnit) -> f64 {
    match unit {
        TemperatureUnit::Celsius => value,
        TemperatureUnit::Fahrenheit => fahrenheit_to_celsius(value),
    }
}

pub fn normalize_wind_speed(value: f64, unit: WindSpeedUnit) -> f64 {
    match unit {
        WindSpeedUnit::Ms => value,
        WindSpeedUnit::Kmh => kmh_to_ms(value),
        WindSpeedUnit::Mph => mph_to_ms(value),
        WindSpeedUnit::Kn => kn_to_ms(value),
    }
}

pub fn normalize_precipitation(value: f64, unit: PrecipitationUnit) -> f64 {
    match unit {
        PrecipitationUnit::Mm => value,
        PrecipitationUnit::Inch => inch_to_mm(value),
    }
}
