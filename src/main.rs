use std::env;

pub mod logger;
pub mod socket_handler;
pub mod table_manager;
pub mod types;

fn main() {
    let mut config_file: String = "/etc/portector.toml".to_string();
    let arg = env::args().nth(1);
    if arg.is_some() {
        config_file = arg.unwrap();
    }

    let config: types::Config = match std::fs::read_to_string(config_file) {
        Ok(o) => toml::from_str(o.as_str()).unwrap(),
        Err(_) => types::Config::default()
    };

    println!("{}", toml::to_string_pretty(&config).unwrap());
}
