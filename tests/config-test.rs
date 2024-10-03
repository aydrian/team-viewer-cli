use team_viewer::config::Config;

#[test]
fn test_new_config() {
    let user_city = "New York".to_string();
    let config = Config::new(user_city.clone());

    assert_eq!(config.user_city, user_city);
    assert!(config.coworkers.is_empty());
}
#[test]
fn test_add_valid_coworker() {
    let mut config = Config::new("New York".to_string());
    let result = config.add_coworker("John Doe".to_string(), "London".to_string());

    assert!(result.is_ok());
    assert_eq!(config.coworkers.len(), 1);
    assert_eq!(config.coworkers[0].name, "John Doe");
    assert_eq!(config.coworkers[0].city, "London");
}
#[test]
fn test_get_config_path() {
    let home_dir = dirs::home_dir().unwrap();
    let expected_path = home_dir.join(".team_view_config.json");

    let result = Config::get_config_path();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_path);
}
