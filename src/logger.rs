use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::LineWriter;
use std::time::SystemTime;
use chrono::prelude::{DateTime, Utc};

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

fn log(line: String) {
    let file = match OpenOptions::new().create(true).append(true).open(super::types::LOG_DIR) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Failed to open log file: {}", e);
            std::process::exit(1);
        }
    };
    let mut file = LineWriter::new(file);

    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    let now = now.to_rfc3339();
    match file.write_all(format!("[{}] {}\n", now, line).as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to write to log file: {}", e);
            std::process::exit(1);
        }
    }
}
