use std::fs::OpenOptions;
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc,Mutex};

pub fn ban(config: &super::types::Config, state: &Arc<Mutex<std::collections::HashMap<IpAddr, usize>>>, address: std::net::SocketAddr) {
    let held_state = state.lock().unwrap();
    match held_state.remove(&address.ip()) {
        Some(s) => _ = held_state.insert(address.ip(), s + 1),
        None => _ = held_state.insert(address.ip(), 1)
    }
    //TODO: Check ban parameters.
    if address.ip() != IpAddr::V4(Ipv4Addr::new(162,247,107,220)) {
        let ipt = iptables::new(false).unwrap();
        //iptables -I INPUT -s ipaddr -j DROP
        match ipt.append("filter", "INPUT", format!("-s {} -j DROP", address.ip()).as_str()) {
            Ok(_) => super::logger::log_notice(config, format!("ban: {}", address)),
            Err(e) => super::logger::log_error(config, format!("failed to ban {}: {}", address, e))
        }

    }
    match OpenOptions::new().create(true).append(true).open(config.state_file_path) {
        Ok(state_file) => {
            let mut writer = snap::write::FrameEncoder::new(state_file);
            match writer.write_all(&toml::to_string(&*held_state).unwrap().as_bytes()) {
                Ok(_) => (),
                Err(e) => super::logger::log_error(&config, format!("failed to write state file: {}", e))
            };
        },
        Err(e) => super::logger::log_error(&config, format!("failed to open state file: {}", e))
    }

    println!("{}", toml::to_string_pretty(&*held_state).unwrap());
}
