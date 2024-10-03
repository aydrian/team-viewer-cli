use crate::config::Config;

pub fn add_coworker(
    name: &str,
    city: &str,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    match config.add_coworker(name.to_string(), city.to_string()) {
        Ok(_) => {
            config.write_config()?;
            println!("Added {} from {} to your team.", name, city);
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}
