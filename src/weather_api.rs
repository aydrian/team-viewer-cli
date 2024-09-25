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

pub async fn get_weather(city: &str, api_key: &str) -> Result<Weather, Box<dyn std::error::Error>> {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );

    let response = reqwest::get(&url)
        .await?
        .json::<Weather>()
        .await
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    Ok(response)
}
