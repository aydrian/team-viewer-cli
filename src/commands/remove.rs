use crate::config::Config;

pub fn remove_coworker(name: &str, config: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
    let initial_len = config.coworkers.len();
    config.coworkers.retain(|coworker| coworker.name != name);

    if config.coworkers.len() < initial_len {
        config.write_config()?;
        println!("Removed {} from your team.", name);
    } else {
        println!("Coworker {} not found in your team.", name);
    }

    Ok(())
}
