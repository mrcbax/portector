use std::net::IpAddr;
use std::sync::{Arc,Mutex};

pub fn create_socket(config: &super::types::Config, address: std::net::SocketAddrV4) -> Option<std::net::TcpListener> {
    return match std::net::TcpListener::bind(address) {
        Ok(o) => {
            super::logger::log_notice(&config, format!("binding to port: {}", address));
            Some(o)
        },
        Err(_) => {
            super::logger::log_error(&config, format!("failed to bind to port, ignoring: {}", address));
            return None;
        }
    };
}

pub fn create_listener(config: super::types::Config, state: &Arc<Mutex<std::collections::HashMap<IpAddr, usize>>>, address: std::net::SocketAddrV4) -> std::thread::JoinHandle<()> {
    return std::thread::spawn( move || {
        let socket = create_socket(&config, address);
        if socket.is_some() {
            for stream in socket.unwrap().incoming() {
                match stream {
                    Ok(successful_stream) => {
                        match successful_stream.peer_addr() {
                            Ok(peer_addr) => {
                                super::logger::log_notice(&config, format!("hit from: {} on: {}", peer_addr, successful_stream.local_addr().unwrap()));
                                if config.log_as_aipdb {
                                    super::logger::log_aipdb(&config, peer_addr.ip(), successful_stream.local_addr().unwrap().port());
                                }
                                super::table_manager::ban(&config, state, peer_addr);

                                //println!("hit from: {}", peer_addr);
                            },
                            Err(e) => super::logger::log_error(&config, format!("failed to parse hit address: {}", e))
                        }
                    },
                    Err(e) => super::logger::log_error(&config, format!("failed to open stream: {}", e))
                }
            }
        }
    });
}
