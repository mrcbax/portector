pub fn create_socket(address: std::net::SocketAddrV4) -> std::net::TcpListener {
    return match std::net::TcpListener::bind(address) {
        Ok(o) => o,
        Err(e) => {
            super::logger::log_io_fatal(e);
            std::process::exit(1);
        }
    };
}
