use sysinfo::{Networks};
use serde_derive::Serialize;

use crate::format_utils;

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