use std::fs::OpenOptions;
use std::io::Write;

pub fn ban(config: &super::types::Config, address: std::net::SocketAddr) {
    let mut held_state = super::STATE.get().lock().unwrap();
    match held_state.remove(&address.ip()) {
        Some(s) => _ = held_state.insert(address.ip(), s + 1),
        None => _ = held_state.insert(address.ip(), 1)
    }
    //TODO: Handle ban time
    if held_state.get(&address.ip()).unwrap() >= &config.ban_parameters.hits_before_ban {
        _ = held_state.remove(&address.ip());
        if !config.ip_whitelist.clone().contains(&address.ip().to_string()) {
            let ipt = iptables::new(false).unwrap();
            //iptables -I INPUT -s ipaddr -j DROP
            match ipt.append("filter", "INPUT", format!("-s {} -j DROP", address.ip()).as_str()) {
                Ok(_) => super::logger::log_notice(config, format!("ban: {}", address)),
                Err(e) => super::logger::log_error(config, format!("failed to ban {}: {}", address, e))
            }
        }
    }
    match OpenOptions::new().create(true).append(true).open(&config.state_file_path) {
        Ok(state_file) => {
            let mut writer = snap::write::FrameEncoder::new(state_file);
            match writer.write_all(&toml::to_string(&*held_state).unwrap().as_bytes()) {
                Ok(_) => (),
                Err(e) => super::logger::log_error(&config, format!("failed to write state file: {}", e))
            };
        },
        Err(e) => super::logger::log_error(&config, format!("failed to open state file: {}", e))
    }

    //println!("{}", toml::to_string_pretty(&*held_state).unwrap());
}
