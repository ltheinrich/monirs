//! Monirs main
#![cfg(target_os = "linux")]

mod sys;

use kern::Fail;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
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
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        log.write_all(
            format!(
                "{:02}:{:02}:{:02} -> CPU usage ={:>6}%\n",
                (time.as_secs() / (60 * 60)) % 24,
                (time.as_secs() / 60) % 60,
                time.as_secs() % 60,
                format!("{:.2}", perc)
            )
            .as_bytes(),
        )
        .unwrap();
    }
}
