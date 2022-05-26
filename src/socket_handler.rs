pub fn create_socket(address: std::net::SocketAddrV4) -> Option<std::net::TcpListener> {
    return match std::net::TcpListener::bind(address) {
        Ok(o) => {
            super::logger::log_notice(format!("binding to port: {}", address));
            Some(o)
        },
        Err(_) => {
            super::logger::log_error(format!("failed to bind to port, ignoring: {}", address));
            return None;
        }
    };
}

pub fn create_listener(address: std::net::SocketAddrV4) -> std::thread::JoinHandle<()> {
    return std::thread::spawn( move || {
        let socket = create_socket(address);
        if socket.is_some() {
            for stream in socket.unwrap().incoming() {
                match stream {
                    Ok(successful_stream) => {
                        match successful_stream.peer_addr() {
                            Ok(peer_addr) => {
                                super::logger::log_notice(format!("hit from: {}", peer_addr));
                                //println!("hit from: {}", peer_addr);
                            },
                            Err(e) => super::logger::log_error(format!("failed to parse hit address: {}", e))
                        }
                    },
                    Err(e) => super::logger::log_error(format!("failed to open stream: {}", e))
                }
            }
        }
    });
}
