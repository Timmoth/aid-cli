use serde_derive::Serialize;
use sysinfo::Networks;

use crate::format_utils;
use std::thread::sleep;
use std::time::{Duration, Instant};

// Helper function to format bytes to human-readable units (kB, MB, GB)
fn format_bytes(bytes: u64) -> String {
    let kilobyte = 1024.0;
    let megabyte = kilobyte * 1024.0;
    let gigabyte = megabyte * 1024.0;

    if bytes as f64 >= gigabyte {
        format!("{:.2} GB/s", bytes as f64 / gigabyte)
    } else if bytes as f64 >= megabyte {
        format!("{:.2} MB/s", bytes as f64 / megabyte)
    } else if bytes as f64 >= kilobyte {
        format!("{:.2} kB/s", bytes as f64 / kilobyte)
    } else {
        format!("{} B/s", bytes)
    }
}

pub fn system_network_usage(watch: bool) {

    let mut networks = Networks::new_with_refreshed_list();

    let mut last_instant = Instant::now();

    loop {
        sleep(Duration::from_secs(1));

        networks.refresh();
        let current_instant = Instant::now();
        let elapsed_time = current_instant.duration_since(last_instant).as_secs_f64();

        format_utils::clear_terminal();
       for (interface_name, data) in networks.iter() {
            let received = data.received();
            let transmitted = data.transmitted();

            // Calculate speeds
            let receive_speed = received as f64 / elapsed_time; // Bytes per second
            let transmit_speed = transmitted as f64 / elapsed_time; // Bytes per second

            // Output the receive/transmit speeds in a human-readable format
            println!("{}: rx {}, tx {}", interface_name, format_bytes(receive_speed as u64), format_bytes(transmit_speed as u64));
        }

        last_instant = current_instant;
        if !watch{
            return;
        }
    }
}

#[derive(Serialize, Debug)]
struct NetworkInfoOutput {
    name: String,
    transmitted: f64,
    received: f64,
    mac: String,
}

pub async fn system_network_info(json: bool) {

    let networks = Networks::new_with_refreshed_list();

    let mut outputs: Vec<NetworkInfoOutput> = Vec::new();
    for (interface_name, data) in &networks {
        outputs.push(NetworkInfoOutput{
            name: interface_name.clone(),
            transmitted: format_utils::bytes_to_gb(data.total_transmitted()),
            received: format_utils::bytes_to_gb(data.total_received()),
            mac: data.mac_address().to_string(),
                });
    }

     if json {
        format_utils::print_json(&outputs);
    }else{
        for o in outputs{
            println!("name: {}, transmitted: {}GB, received: {}GB, mac: {}", o.name, o.transmitted, o.received, o.mac);
        }
    }
}