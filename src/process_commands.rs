use regex::Regex;
use std::thread::sleep;
use std::time::{Duration, Instant};
use sysinfo::{Pid, System};

use crate::format_utils;

pub fn system_process_info(
    name_filter: Option<String>,
    sort_by: Option<String>,
    limit: Option<usize>,
    watch: bool,
) {
    let mut sys = System::new_all();

    let name_regex = name_filter
        .as_ref()
        .map(|filter| Regex::new(filter).unwrap());

    loop {
        sys.refresh_all();

        // Collect process info
        let mut process_info: Vec<(Pid, &sysinfo::Process)> = sys
            .processes()
            .iter()
            .filter(|(_, process)| {
                // Filter processes by name if regex is provided
                if let Some(ref regex) = name_regex {
                    regex.is_match(&process.name().to_str().unwrap_or_default())
                } else {
                    true
                }
            })
            .map(|(pid, process)| (*pid, process))
            .collect();

        // Sort processes based on the specified criteria
        match sort_by.as_deref() {
            Some("cpu") => {
                process_info.sort_by(|a, b| b.1.cpu_usage().partial_cmp(&a.1.cpu_usage()).unwrap())
            }
            Some("mem") => {
                process_info.sort_by(|a, b| b.1.memory().partial_cmp(&a.1.memory()).unwrap())
            }
            Some("disk") => process_info.sort_by(|a, b| {
                let disk_a = a.1.disk_usage();
                let disk_b = b.1.disk_usage();
                (disk_b.read_bytes + disk_b.written_bytes)
                    .partial_cmp(&(disk_a.read_bytes + disk_a.written_bytes))
                    .unwrap()
            }),
            Some("name") | None => process_info.sort_by(|a, b| a.1.name().cmp(b.1.name())),
            _ => eprintln!("Invalid sort option provided. Defaulting to sort by name."),
        }

        let last_instant = Instant::now();

        format_utils::clear_terminal();
        println!(
            "{:<5}\t{:>6}\t{:>10}\t{:>12}\t{:>12}\tname",
            "[pid]", "[cpu %]", "[mem]", "[disk read]", "[disk write]"
        );

        for (pid, process) in process_info.iter().take(limit.unwrap_or(usize::MAX)) {
            let disk_usage = process.disk_usage();

            let elapsed_time = last_instant.elapsed().as_secs_f64();
            let read_speed = if elapsed_time > 0.0 {
                disk_usage.read_bytes as f64 / elapsed_time // Bytes per second
            } else {
                0.0
            };
            let write_speed = if elapsed_time > 0.0 {
                disk_usage.written_bytes as f64 / elapsed_time // Bytes per second
            } else {
                0.0
            };

            // Display process information
            println!(
                "[{:>5}]\t{:<6.2}%\t{:>10}\t{:>12}\t{:>12}\t{}\t",
                pid,
                process.cpu_usage(),
                format_memory(process.memory()),
                format_disk_speed(read_speed),
                format_disk_speed(write_speed),
                process.name().to_str().unwrap()
            );
        }

        // If not watching, break out of the loop after one display
        if !watch {
            break;
        }

        sleep(Duration::from_secs(1));
    }
}

fn format_memory(bytes: u64) -> String {
    let kilobyte = 1024.0;
    let megabyte = kilobyte * 1024.0;
    let gigabyte = megabyte * 1024.0;

    if bytes as f64 >= gigabyte {
        format!("{:.0} GB", bytes as f64 / gigabyte)
    } else if bytes as f64 >= megabyte {
        format!("{:.0} MB", bytes as f64 / megabyte)
    } else if bytes as f64 >= kilobyte {
        format!("{:.0} kB", bytes as f64 / kilobyte)
    } else {
        format!("{} B", bytes)
    }
}

fn format_disk_speed(bytes_per_sec: f64) -> String {
    let kilobyte = 1024.0;
    let megabyte = kilobyte * 1024.0;
    let gigabyte = megabyte * 1024.0;

    if bytes_per_sec >= gigabyte {
        format!("{:.0} GB/s", bytes_per_sec / gigabyte)
    } else if bytes_per_sec >= megabyte {
        format!("{:.0} MB/s", bytes_per_sec / megabyte)
    } else if bytes_per_sec >= kilobyte {
        format!("{:.0} kB/s", bytes_per_sec / kilobyte)
    } else {
        format!("{:.0} B/s", bytes_per_sec)
    }
}
