use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;
use std::io::LineWriter;

pub fn log_io_fatal(error: std::io::Error) {
    log(format!("{}", error));
    std::process::exit(1);
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

    match file.write_all(format!("{}\n", line).as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to write to log file: {}", e);
            std::process::exit(1);
        }
    }
}
