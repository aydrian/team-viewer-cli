mod add;
mod list;
mod remove;
mod weather;

pub use add::add_coworker;
pub use list::list_coworkers;
pub use remove::remove_coworker;
pub use weather::{get_api_key, print_coworkers_weather, print_weather};
