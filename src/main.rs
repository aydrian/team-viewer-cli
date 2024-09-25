use clap::{Parser, Subcommand};
use dotenv::dotenv;
use std::env;
mod weather_api;
use chrono;
use weather_api::get_weather;
mod config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    city: Option<String>,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    AddCoworker {
        #[arg(short, long)]
        first_name: String,
        #[arg(short, long)]
        last_name: String,
        #[arg(short, long)]
        city: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let args = Args::parse();
    let config_path = config::get_config_path()?;
    let mut config = config::read_config(&config_path)?;

    if let Some(command) = args.command {
        return handle_command(command, &config_path, &mut config);
    }

    let user_city = get_user_city(&args, &mut config, &config_path)?;
    let api_key = get_api_key()?;

    println!("Your weather:");
    print_weather(&user_city, &api_key).await?;

    print_coworkers_weather(&config, &api_key).await?;

    Ok(())
}

fn handle_command(
    command: Command,
    config_path: &std::path::Path,
    config: &mut config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Command::AddCoworker {
            first_name,
            last_name,
            city,
        } => {
            config.add_coworker(first_name.clone(), last_name.clone(), city.clone());
            config::write_config(&config_path.to_path_buf(), &config)?;
            println!("Added coworker: {} {} from {}", first_name, last_name, city);
        }
    }
    Ok(())
}

fn get_user_city(
    args: &Args,
    config: &mut config::Config,
    config_path: &std::path::Path,
) -> Result<String, Box<dyn std::error::Error>> {
    Ok(match &args.city {
        Some(c) => c.clone(),
        None => {
            if config.user_city.is_empty() {
                let mut input = String::new();
                println!("Enter your city name:");
                std::io::stdin().read_line(&mut input)?;
                let city = input.trim().to_string();
                config.user_city = city.clone();
                config::write_config(&config_path.to_path_buf(), &config)?;
                city
            } else {
                config.user_city.clone()
            }
        }
    })
}

fn get_api_key() -> Result<String, Box<dyn std::error::Error>> {
    env::var("OPENWEATHERMAP_API_KEY")
        .map_err(|_| "OPENWEATHERMAP_API_KEY not found in environment variables".into())
}

async fn print_weather(city: &str, api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
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

async fn print_coworkers_weather(
    config: &config::Config,
    api_key: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !config.coworkers.is_empty() {
        println!("Coworkers' weather:");
        for coworker in &config.coworkers {
            println!("{} {}:", coworker.first_name, coworker.last_name);
            print_weather(&coworker.city, api_key).await?;
        }
    }
    Ok(())
}
