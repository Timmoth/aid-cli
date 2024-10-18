use reqwest;
use serde_derive::Serialize;
use tokio::task::JoinHandle;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::time::Duration;
use std::time::Instant;

use crate::format_utils;
use crate::ip_utils;

#[derive(Serialize, Debug)]
struct IpResponse {
    ip: String,
}

#[derive(Serialize, Debug)]
struct MultiIpResponse {
    ips: Vec<String>,
}

#[derive(Serialize, Debug)]
struct OnlineResponse {
    online: bool,
}

pub async fn ip_local(json: bool) {
    match ip_utils::get_local_ip().await {
        Ok(addr) => {
            if json {
                format_utils::print_json(&IpResponse {
                    ip: addr.to_string(),
                });
            } else {
                println!("{}", addr)
            }
        }
        Err(e) => eprintln!("Failed to get local address: {}", e),
    }
}

pub async fn ip_public(json: bool) {
    // Send a request to an external service that returns the public IP
    let response = match reqwest::get("https://api.ipify.org").await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Http request failed: {}", e);
            return;
        }
    };

    match response.text().await {
        Ok(ip_addr) => {
            if json {
                format_utils::print_json(&IpResponse { ip: ip_addr });
            } else {
                println!("{}", ip_addr)
            }
        }
        Err(e) => println!("{}", e),
    }
}

pub async fn ip_scan(ip: Option<String>, json: bool) {
    let timeout = Duration::from_millis(100);

    let target = if let Some(t) = ip {
        if !t.is_empty() {
            // If the provided IP is not empty, use it
            t
        } else {
            // If the provided IP is empty, fetch the local IP
            match ip_utils::get_local_ip().await {
                Ok(addr) => addr.to_string(),
                Err(e) => {
                    eprintln!("Failed to get local address: {}", e);
                    return;
                }
            }
        }
    } else {
        // If ip is None, fetch the local IP
        match ip_utils::get_local_ip().await {
            Ok(addr) => addr.to_string(),
            Err(e) => {
                eprintln!("Failed to get local address: {}", e);
                return;
            }
        }
    };

  let ping_tasks: Vec<JoinHandle<Option<(Ipv4Addr, Duration)>>> = (0..255)
        .filter_map(|i| {
            // Split the IP into segments
            let mut segments: Vec<String> = target.split('.').map(|s| s.to_string()).collect();

            // Make sure it's a valid IPv4 address (i.e., has 4 segments)
            if segments.len() != 4 {
                eprintln!("Invalid IP format: {}", target);
                return None;
            }
            
            // Replace the last segment with `i`
            segments[3] = i.to_string();

            // Join the segments back into a string
            let target_ip_str = segments.join(".");

            // Parse the modified string back into an Ipv4Addr
            let target_ip = match Ipv4Addr::from_str(&target_ip_str) {
                Ok(ip) => ip,
                Err(_) => return None, // Skip invalid IPs
            };

            let start = Instant::now();

            // Create a task for each ping
            Some(tokio::spawn(async move {
                match ip_utils::ping(target_ip, timeout).await {
                    Ok(_) => Some((target_ip, start.elapsed())),
                    Err(_) => None,
                }
            }))
        })
        .collect();

    let mut ips: Vec<String> = Vec::new();
    // Await all tasks and filter successful pings
    for task in ping_tasks {
        if let Ok(Some(result)) = task.await {
            ips.push(result.0.to_string());
        }
    }

    if json {
        format_utils::print_json(&MultiIpResponse { ips: ips });
    } else {
        println!("{}", ips.join("\n\r"))
    }
}

pub async fn ip_status(ip_str: String, json: bool) {
    let ip = match Ipv4Addr::from_str(&ip_str) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to bind udp socket: {}", e);
            return;
        }
    };

    let timeout = Duration::from_millis(100); // Timeout for each port scan

    if json {
        match ip_utils::ping(ip, timeout).await {
            Ok(_) => format_utils::print_json(&OnlineResponse { online: true }),
            Err(_) => format_utils::print_json(&OnlineResponse { online: false }),
        }
    } else {
        match ip_utils::ping(ip, timeout).await {
            Ok(_) => println!("online"),
            Err(_) => println!("offline"),
        }
    }
}
