//! Monirs main
#![cfg(target_os = "linux")]

mod sys;

use chrono::{Datelike, Local, Timelike};
use kern::Fail;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::time::Duration;
use sys::cpu_usage;

fn main() -> Result<(), Fail> {
    let mut log = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("log.txt")
        .unwrap();
    let (rc, _) = cpu_usage(Duration::from_secs(60));
    loop {
        let perc = rc.recv().unwrap();
        let time = Local::now();
        log.write_all(
            format!(
                "{:02}.{:02}.{:04} - {:02}:{:02}:{:02} -> CPU usage ={:>6}%\n",
                time.day(),
                time.month(),
                time.year(),
                time.hour(),
                time.minute(),
                time.second(),
                format!("{:.2}", perc)
            )
            .as_bytes(),
        )
        .unwrap();
    }
}
