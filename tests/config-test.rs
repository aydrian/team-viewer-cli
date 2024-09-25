use crate::config::get_config_path;
use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use super::*;
}
#[test]
fn test_get_config_path_valid() {
    let result = get_config_path();
    assert!(result.is_ok());
    let path = result.unwrap();
    assert!(path.is_absolute());
    assert_eq!(path.file_name().unwrap(), ".team_view_config.json");
    assert!(path.parent().unwrap() == dirs::home_dir().unwrap());
}
#[test]
fn test_get_config_path_no_home_dir() {
    // Mock the dirs::home_dir() function to return None
    let original_home_dir = dirs::home_dir;
    dirs::home_dir = || None;

    let result = get_config_path();

    // Restore the original dirs::home_dir function
    dirs::home_dir = original_home_dir;

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Unable to determine home directory"
    );
}
