use std::env;
use std::net::{Ipv4Addr, SocketAddrV4};

pub mod logger;
pub mod socket_handler;
pub mod table_manager;
pub mod types;

fn main() {
    let mut config_file: String = "/etc/portector.toml".to_string();
    let arg = env::args().nth(1);
    if let Some(path) = arg {
        config_file = path;
        logger::log_notice(format!("using configuration file: {}", &config_file));
    } else {
        logger::log_notice(format!("using default configuration file: {}", &config_file));
    }

    let config: types::Config = match std::fs::read_to_string(config_file) {
        Ok(o) => toml::from_str(o.as_str()).unwrap(),
        Err(_) => {
            logger::log_error("failed to load provided configuration file, loading default configuration".into());
            types::Config::default()
        }
    };
    let mut active_threads: Vec<std::thread::JoinHandle<()>> = vec!();
    for port in config.ports.clone() {
        active_threads.push(socket_handler::create_listener(config.clone(), SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port)));
    }

    for thread in active_threads {
        _ = thread.join();
        logger::log_error("closing thread".into());
    }

    println!("{}", toml::to_string_pretty(&config).unwrap());
}
