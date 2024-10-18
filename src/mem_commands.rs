use serde_derive::Serialize;
use sysinfo::System;
use std::time::Duration;

use crate::format_utils;

pub async fn system_mem_usage(watch: bool, json: bool) {
    let mut system = System::new_all();

    if watch {
        loop {
            system.refresh_memory();
            format_utils::clear_terminal();
            output_mem_usage(&system, json);
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    } else {
        system.refresh_memory();
        output_mem_usage(&system, json);
    }
}

#[derive(Serialize, Debug)]
struct MemUsage {
    total_gb: f64,
    used_gb: f64,
    used_percent: f64,
    free_gb: f64,
    free_percent: f64,
}

fn output_mem_usage(system: &System, json: bool){
    let used_memory = system.used_memory();
    let total_memory = system.total_memory();
    let free_memory = system.free_memory();

    if json {
        let mem_usage = MemUsage {
            total_gb: format_utils::bytes_to_gb(total_memory),
            used_gb: format_utils::bytes_to_gb(used_memory),
            used_percent: format_utils::percent(used_memory, total_memory),
            free_gb: format_utils::bytes_to_gb(free_memory),
            free_percent: format_utils::percent(free_memory, total_memory)
        };
        format_utils::print_json(&mem_usage);
    }else{
        println!("total: {}, used: {} ({}), free: {} ({})", 
        format_utils::format_bytes(total_memory), 
        format_utils::format_bytes(used_memory), 
        format_utils::calculate_format_percent(used_memory, total_memory),
        format_utils::format_bytes(free_memory),
        format_utils::calculate_format_percent(free_memory, total_memory));
    }
}