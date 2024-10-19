use std::net::Ipv4Addr;
use std::str::FromStr;
use std::time::Duration;
use serde_derive::Serialize;
use crate::ip_utils;

use crate::format_utils;

#[derive(Serialize, Debug)]
struct OpenResponse {
    open: bool,
}

#[derive(Serialize, Debug)]
struct MultiPortResponse {
    ports: Vec<u16>,
}

pub async fn port_scan(ip_str: Option<String>, json: bool) {

    let ip_str = ip_utils::to_ip_or_local(ip_str).await;

    let ip = match Ipv4Addr::from_str(&ip_str) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed parsing Ip address {}", e);
            return;
        }
    };

    let timeout = Duration::from_millis(100); // Timeout for each port scan

    let ping_tasks: Vec<_> = (1..9999)
        .map(|i: u16| {
            tokio::spawn(async move {
                match ip_utils::can_connect(ip, i, timeout).await {
                    Ok(is_open) => {
                        if is_open {
                            Some(i)
                        } else {
                            None
                        }
                    }
                    Err(_) => None,
                }
            })
        })
        .collect();

    let mut ports: Vec<u16> = Vec::new();
    for task in ping_tasks {
        if let Ok(Some(result)) = task.await {
            ports.push(result);
        }
    }

    if json {
        format_utils::print_json(&MultiPortResponse { ports: ports });
    } else {
        let port_strings: Vec<String> = ports.iter().map(|p| p.to_string()).collect();
        println!("{}", port_strings.join("\n\r"))
    }
}

pub async fn port_status(ip_str:  Option<String>, port: u16, json: bool) {

    let ip_str = ip_utils::to_ip_or_local(ip_str).await;

    let ip = match Ipv4Addr::from_str(&ip_str) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to bind udp socket: {}", e);
            return;
        }
    };

    let timeout = Duration::from_millis(100);

    if json {
        match ip_utils::can_connect(ip, port, timeout).await {
            Ok(is_open) => format_utils::print_json(&OpenResponse { open: is_open }),
            Err(_) => format_utils::print_json(&OpenResponse { open: false }),
        }
    } else {
        match ip_utils::can_connect(ip, port, timeout).await {
            Ok(_) => println!("open"),
            Err(_) => println!("closed"),
        }
    }
}
