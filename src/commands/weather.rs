use std::env;

use crate::config::Config;
use crate::weather_api::get_weather;
// Import other necessary dependencies

pub fn get_api_key() -> Result<String, Box<dyn std::error::Error>> {
    env::var("OPENWEATHERMAP_API_KEY")
        .map_err(|_| "OPENWEATHERMAP_API_KEY not found in environment variables".into())
}

pub async fn print_weather(city: &str, api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let weather = get_weather(city, api_key).await?;
    let timezone_offset = weather.timezone;
    let local_time = chrono::Utc::now() + chrono::Duration::seconds(i64::from(timezone_offset));

    println!("Weather in {}: ", city);
    println!(
        "Current Date and Time: {}",
        local_time.format("%Y-%m-%d %H:%M:%S")
    );
    println!("Temperature: {}°C", weather.main.temp);
    println!("Humidity: {}%", weather.main.humidity);
    println!(
        "Description: {}",
        weather
            .weather
            .get(0)
            .map(|w| w.description.as_str())
            .unwrap_or("No description available")
    );
    println!();
    Ok(())
}

pub async fn print_coworkers_weather(
    config: &Config,
    api_key: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !config.coworkers.is_empty() {
        println!("Coworkers' weather:");
        for coworker in &config.coworkers {
            println!("{}:", coworker.name);
            print_weather(&coworker.city, api_key).await?;
        }
    }
    Ok(())
}
