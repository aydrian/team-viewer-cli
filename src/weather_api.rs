use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Weather {
    pub main: MainWeather,
    pub weather: Vec<WeatherDescription>,
    pub timezone: i32,
}

#[derive(Deserialize, Debug)]
pub struct MainWeather {
    pub temp: f32,
    pub humidity: i32,
}

#[derive(Deserialize, Debug)]
pub struct WeatherDescription {
    pub description: String,
}

use std::env;

/// Fetches weather information for a given city using the OpenWeatherMap API.
///
/// # Arguments
///
/// * `city` - A string slice that holds the name of the city.
/// * `api_key` - A string slice that holds the API key for authentication.
///
/// # Returns
///
/// Returns a `Result` which is:
/// - `Ok(Weather)` containing the weather information if the API call is successful.
/// - `Err(Box<dyn std::error::Error>)` if there's an error during the API call or data parsing.
///
/// # Errors
///
/// This function will return an error if:
/// - The API request fails.
/// - The response cannot be parsed into the `Weather` struct.
pub async fn get_weather(city: &str, api_key: &str) -> Result<Weather, Box<dyn std::error::Error>> {
    // Retrieve the base URL from environment variables or use the default
    let base_url = env::var("WEATHER_API_BASE_URL")
        .unwrap_or_else(|_| "http://api.openweathermap.org".to_string());

    // Construct the full URL for the API request
    let url = format!(
        "{}/data/2.5/weather?q={}&appid={}&units=metric",
        base_url, city, api_key
    );

    // Send GET request to the API
    let response = reqwest::get(&url).await?;

    // Parse the JSON response into our Weather struct
    let weather: Weather = response.json().await?;

    // Return the weather information
    Ok(weather)
}
