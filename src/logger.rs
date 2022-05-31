use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::LineWriter;
use chrono::prelude::{Local, DateTime};

pub fn log_fatal(message: String) {
    log(format!("(fatal) {}", message));
    std::process::exit(1);
}

pub fn log_error(message: String) {
    log(format!("(error) {}", message));
}

pub fn log_notice(message: String) {
    log(format!("(notice) {}", message));
}

pub fn log_aipdb(ip_addr: std::net::IpAddr, port: u16) {
    let file = match OpenOptions::new().create(true).append(true).open(super::types::AIPDB_LOG_FILE) {
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

fn log(line: String) {
    let file = match OpenOptions::new().create(true).append(true).open(super::types::LOG_FILE) {
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
