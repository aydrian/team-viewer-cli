use clap::Parser;
use dotenv::dotenv;
use team_viewer::cli::{Cli, Commands};
use team_viewer::commands::{
    add_coworker, get_api_key, list_coworkers, print_coworkers_weather, print_weather,
    remove_coworker,
};
use team_viewer::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let cli = Cli::parse();

    let mut config = match Config::read_config() {
        Ok(config) => config,
        Err(_) => {
            println!("No configuration found. Let's set up a new one.");
            Config::setup()?
        }
    };

    match &cli.command {
        Some(Commands::Add { name, city }) => add_coworker(name, city, &mut config)?,
        Some(Commands::Remove { name }) => remove_coworker(name, &mut config)?,
        Some(Commands::List) => list_coworkers(&config),
        None => {
            let user_city = cli.city.unwrap_or_else(|| config.user_city.clone());
            let api_key = get_api_key()?;

            println!("Your weather:");
            print_weather(&user_city, &api_key).await?;

            print_coworkers_weather(&config, &api_key).await?;
        }
    }

    Ok(())
}
