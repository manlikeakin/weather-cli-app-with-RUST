use serde::Deserialize;

use crate::weather::Weather;


// ========================================
// GEOCODING RESPONSE
// ========================================

#[derive(Debug, Deserialize)]
struct GeocodingResponse {
    results: Option<Vec<Location>>,
}


// ========================================
// LOCATION
// ========================================

#[derive(Debug, Deserialize)]
struct Location {
    name: String,
    latitude: f64,
    longitude: f64,
}


// ========================================
// WEATHER API RESPONSE
// ========================================

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    current: CurrentWeather,
}


// ========================================
// CURRENT WEATHER
// ========================================

#[derive(Debug, Deserialize)]
struct CurrentWeather {
    temperature_2m: f64,
    relative_humidity_2m: f64,
    apparent_temperature: f64,
    wind_speed_10m: f64,
    weather_code: i32,
}


// ========================================
// GET WEATHER
// ========================================

pub fn get_weather(city: &str) -> Result<Weather, String> {

    // ------------------------------------
    // STEP 1: FIND CITY COORDINATES
    // ------------------------------------

    let encoded_city =
        city.replace(' ', "%20");

    let geocoding_url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1&language=en&format=json",
        encoded_city
    );


    let response =
        reqwest::blocking::get(&geocoding_url)
        .map_err(|error| {
            format!(
                "Failed to connect to geocoding API: {}",
                error
            )
        })?;


    if !response.status().is_success() {

        return Err(format!(
            "Geocoding API returned status: {}",
            response.status()
        ));
    }


    let location_data =
        response
            .json::<GeocodingResponse>()
            .map_err(|error| {
                format!(
                    "Failed to read location data: {}",
                    error
                )
            })?;


    // ------------------------------------
    // STEP 2: GET FIRST LOCATION
    // ------------------------------------

    let location =
        match location_data.results {

            Some(results) => {

                match results.into_iter().next() {

                    Some(location) => location,

                    None => {
                        return Err(
                            format!(
                                "Could not find city: {}",
                                city
                            )
                        );
                    }
                }
            }

            None => {

                return Err(
                    format!(
                        "Could not find city: {}",
                        city
                    )
                );
            }
        };


    // ------------------------------------
    // STEP 3: BUILD WEATHER URL
    // ------------------------------------

    let weather_url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,relative_humidity_2m,apparent_temperature,wind_speed_10m,weather_code",
        location.latitude,
        location.longitude
    );


    // ------------------------------------
    // STEP 4: REQUEST WEATHER
    // ------------------------------------

    let response =
        reqwest::blocking::get(&weather_url)
        .map_err(|error| {
            format!(
                "Failed to connect to weather API: {}",
                error
            )
        })?;


    if !response.status().is_success() {

        return Err(format!(
            "Weather API returned status: {}",
            response.status()
        ));
    }


    // ------------------------------------
    // STEP 5: CONVERT JSON
    // ------------------------------------

    let weather_data =
        response
            .json::<WeatherResponse>()
            .map_err(|error| {
                format!(
                    "Failed to read weather data: {}",
                    error
                )
            })?;


    // ------------------------------------
    // STEP 6: CONVERT WEATHER CODE
    // ------------------------------------

    let condition =
        weather_condition(
            weather_data.current.weather_code
        );


    // ------------------------------------
    // STEP 7: CREATE OUR WEATHER STRUCT
    // ------------------------------------

    let weather =
        Weather {
            city: location.name,
            temperature:
                weather_data.current.temperature_2m,

            feels_like:
                weather_data.current.apparent_temperature,

            humidity:
                weather_data.current.relative_humidity_2m as u32,

            wind_speed:
                weather_data.current.wind_speed_10m,

            condition,
        };


    Ok(weather)
}


// ========================================
// WEATHER CODE → DESCRIPTION
// ========================================

fn weather_condition(code: i32) -> String {

    match code {

        0 =>
            "Clear sky".to_string(),

        1 | 2 | 3 =>
            "Partly cloudy".to_string(),

        45 | 48 =>
            "Fog".to_string(),

        51 | 53 | 55 =>
            "Drizzle".to_string(),

        56 | 57 =>
            "Freezing drizzle".to_string(),

        61 | 63 | 65 =>
            "Rain".to_string(),

        66 | 67 =>
            "Freezing rain".to_string(),

        71 | 73 | 75 =>
            "Snow fall".to_string(),

        77 =>
            "Snow grains".to_string(),

        80 | 81 | 82 =>
            "Rain showers".to_string(),

        85 | 86 =>
            "Snow showers".to_string(),

        95 =>
            "Thunderstorm".to_string(),

        96 | 99 =>
            "Thunderstorm with hail".to_string(),

        _ =>
            "Unknown weather condition".to_string(),
    }
}