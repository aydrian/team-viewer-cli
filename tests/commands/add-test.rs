use team_viewer::commands::add_coworker;
use team_viewer::config::Config;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_coworker_success() {
        let mut config = Config::new("TestCity".to_string());
        let result = add_coworker("John Doe", "New York", &mut config);
        assert!(result.is_ok());
        assert_eq!(config.coworkers.len(), 1);
        assert_eq!(config.coworkers[0].name, "John Doe");
        assert_eq!(config.coworkers[0].city, "New York");
    }
}
