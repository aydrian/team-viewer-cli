use crate::config::Config;

pub fn list_coworkers(config: &Config) {
    if config.coworkers.is_empty() {
        println!("Your team is currently empty.");
    } else {
        println!("Your team:");
        for coworker in &config.coworkers {
            println!("- {} ({})", coworker.name, coworker.city);
        }
    }
}
