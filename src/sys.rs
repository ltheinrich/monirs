//! System data collector

use kern::Fail;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;

/// Sends CPU usage in percent via channel every duration (sends nothing during first duration)
pub fn cpu_usage(duration: Duration) -> (Receiver<f64>, JoinHandle<Result<(), Box<Fail>>>) {
    // create channel
    let (tx, rc): (Sender<f64>, Receiver<f64>) = channel();

    // spawn sender thread
    let thread = spawn(move || {
        // read first cpu times and wait first second
        let (mut prev_idle, mut prev_total) = read_cpu_times()?;
        sleep(duration);

        // send cpu usage continously
        loop {
            // read cpu times and calculate difference
            let (idle, total) = read_cpu_times()?;
            let dif_idle = (idle - prev_idle) as f64;
            let dif_total = (total - prev_total) as f64;

            // calculate cpu usage and send
            let cpu_usage = ((1.0 - dif_idle / dif_total) * 10000.0).round() / 100.0;
            tx.send(cpu_usage).map_err(Fail::new)?;

            // set previous times and wait next second
            prev_idle = idle;
            prev_total = total;
            sleep(duration);
        }
    });

    // return receiver and thread
    (rc, thread)
}

/// Get CPU idle and total time from /proc/stat
fn read_cpu_times() -> Result<(u64, u64), Box<Fail>> {
    // open file
    let mut file = OpenOptions::new()
        .read(true)
        .open("/proc/stat")
        .map_err(Fail::new)?;

    // read file
    let mut buf = String::new();
    file.read_to_string(&mut buf).map_err(Fail::new)?;

    // only first line
    buf = {
        // split lines and get first
        let line = buf
            .split('\n')
            .next()
            .ok_or_else(|| Fail::new("broken /proc/stat"))?;

        // line should be at least 10 characters
        if line.len() < 10 {
            return Err(Fail::new("broken /proc/stat"));
        }

        // cut "cpu  "
        line[5..line.len()].to_string()
    };

    // split by whitespace and get idle time
    let split: Vec<&str> = buf.split_ascii_whitespace().collect();
    let idle: u64 = split[3].parse().map_err(Fail::new)?;

    // calculate total from all times
    let mut total = 0u64;
    for s in split {
        total += s.parse::<u64>().map_err(Fail::new)?;
    }

    // return idle and total time
    Ok((idle, total))
}
