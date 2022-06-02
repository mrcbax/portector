use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::LineWriter;
use chrono::prelude::{Local, DateTime};

pub fn log_fatal(config: &super::types::Config, message: String) {
    log(config, format!("(fatal) {}", message));
    std::process::exit(1);
}

pub fn log_error(config: &super::types::Config, message: String) {
    log(config, format!("(error) {}", message));
}

pub fn log_notice(config: &super::types::Config, message: String) {
    log(config, format!("(notice) {}", message));
}

pub fn log_aipdb(config: &super::types::Config, ip_addr: std::net::IpAddr, port: u16) {
    let file = match OpenOptions::new().create(true).append(true).open(&config.aipdb_log_file_path) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Failed to open log file: {}", e);
            std::process::exit(1);
        }
    };
    let mut file = LineWriter::new(file);

    let now: DateTime<Local> = Local::now();
    let now = now.to_rfc3339_opts(chrono::SecondsFormat::Secs, false);
    match file.write_all(format!("{},\"14\",{},\"Attempted to access trap port {}\"\n", ip_addr, now, port).as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to write to log file: {}", e);
            std::process::exit(1);
        }
    }
}

fn log(config: &super::types::Config, line: String) {
    if config.logging_enabled {
        let file = match OpenOptions::new().create(true).append(true).open(&config.log_file_path) {
            Ok(o) => o,
            Err(e) => {
                eprintln!("Failed to open log file: {}", e);
                std::process::exit(1);
            }
        };
        let mut file = LineWriter::new(file);

        let now: DateTime<Local> = Local::now();
        let now = now.to_rfc3339();
        match file.write_all(format!("[{}] {}\n", now, line).as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Failed to write to log file: {}", e);
                std::process::exit(1);
            }
        }
    }
}
