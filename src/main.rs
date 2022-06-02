use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4};
use std::sync::{Arc,Mutex};

pub mod logger;
pub mod socket_handler;
pub mod table_manager;
pub mod types;

fn main() {
    let mut config_file: String = "/etc/portector.toml".to_string();
    let arg = env::args().nth(1);
    if let Some(path) = arg {
        config_file = path;
        /*logger::log_notice*/println!("using configuration file: {}", &config_file);
    } else {
        /*logger::log_notice*/println!("using default configuration file: {}", &config_file);
    }

    let config: types::Config = match std::fs::read_to_string(config_file) {
        Ok(o) => toml::from_str(o.as_str()).unwrap(),
        Err(_) => {
            /*logger::log_error*/println!("failed to load provided configuration file, loading default configuration");
            types::Config::default()
        }
    };
    let mut state: Arc<Mutex<HashMap<IpAddr, usize>>>= Arc::new(Mutex::new(HashMap::new()));
    match File::open(&config.state_file_path) {
        Ok(file_handle) => {
            let mut reader = snap::read::FrameDecoder::new(file_handle);
            let mut bytes = Vec::new();
            match reader.read_to_end(&mut bytes) {
                Ok(_) => {
                    state = match toml::from_slice(bytes.as_slice()) {
                        Ok(o) => Arc::new(Mutex::new(o)),
                        Err(e) => {
                            logger::log_error(&config, format!("failed to parse state file: {}", e));
                            Arc::new(Mutex::new(HashMap::new()))
                        }
                    };
                },
                Err(e) => logger::log_error(&config, format!("failed to read state file: {}", e))
            }
        },
        Err(e) => logger::log_error(&config, format!("failed to open state file: {}", e))
    }
    let shared_state = Arc::clone(&state);
    let mut active_threads: Vec<std::thread::JoinHandle<()>> = vec!();
    for port in config.ports.clone() {
        active_threads.push(socket_handler::create_listener(config.clone(), &shared_state, SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port)));
    }

    for thread in active_threads {
        _ = thread.join();
        logger::log_error(&config, "closing thread".into());
    }

    let held_state = shared_state.lock().unwrap();
    match File::create(config.state_file_path) {
        Ok(state_file) => {
            let mut writer = snap::write::FrameEncoder::new(state_file);
            match writer.write_all(&toml::to_string(&*held_state).unwrap().as_bytes()) {
                Ok(_) => (),
                Err(e) => logger::log_error(&config, format!("failed to write state file: {}", e))
            };
        },
        Err(e) => logger::log_error(&config, format!("failed to open state file: {}", e))
    }
    println!("{}", toml::to_string_pretty(&config).unwrap());
}
