use std::collections::HashMap;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4};
use std::path::Path;
use std::sync::Mutex;

pub mod logger;
pub mod socket_handler;
pub mod table_manager;
pub mod types;

use state::Storage;
static STATE: Storage<Mutex<HashMap<IpAddr, usize>>> = Storage::new();

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
            /*logger::log_error*/println!("configuration file argument failed to load or did not exist, loading default configuration");
            types::Config::default()
        }
    };
    //println!("{}", toml::to_string_pretty(&config).unwrap());
    let state_path = Path::new(&config.state_file_path);
    match state_path.parent() {
        Some(path) => {
            if !path.is_dir() {
                match std::fs::create_dir(path) {
                    Ok(_) => (),
                    Err(e) => logger::log_error(&config, format!("failed to create portector state directory: {}", e))
                }
            }
        },
        None => logger::log_error(&config, "failed to parse provided state file path".into())
    }

    let mut state: HashMap<IpAddr, usize> = HashMap::new();
    match File::open(&config.state_file_path) {
        Ok(file_handle) => {
            let mut reader = snap::read::FrameDecoder::new(file_handle);
            let mut bytes = Vec::new();
            match reader.read_to_end(&mut bytes) {
                Ok(_) => {
                    state = match toml::from_slice(bytes.as_slice()) {
                        Ok(o) => o,
                        Err(e) => {
                            logger::log_error(&config, format!("failed to parse state file: {}", e));
                            HashMap::new()
                        }
                    };
                },
                Err(e) => logger::log_error(&config, format!("failed to read state file: {}", e))
            }
        },
        Err(e) => logger::log_error(&config, format!("failed to open state file: {}", e))
    }

    STATE.set(Mutex::new(state));
    let mut active_threads: Vec<std::thread::JoinHandle<()>> = vec!();
    for port in config.ports.clone() {
        active_threads.push(socket_handler::create_listener(config.clone(), SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port)));
    }
    for thread in active_threads {
        _ = thread.join();
        logger::log_error(&config, "closing thread".into());
    }

    //TODO: Handle kill signals from systemd: https://rust-cli.github.io/book/in-depth/signals.html
    save_state(&config);
}

pub fn save_state(config: &types::Config) {
    let held_state = STATE.get().lock().unwrap();
    match OpenOptions::new().create(true).append(true).open(&config.state_file_path) {
        Ok(state_file) => {
            let mut writer = snap::write::FrameEncoder::new(state_file);
            match writer.write_all(&toml::to_string(&*held_state).unwrap().as_bytes()) {
                Ok(_) => (),
                Err(e) => logger::log_error(&config, format!("failed to write state file: {}", e))
            };
        },
        Err(e) => logger::log_error(&config, format!("failed to open state file: {}", e))
    }
}
