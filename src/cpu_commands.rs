use sysinfo::System;
use std::time::Duration;
use serde_derive::Serialize;

use crate::format_utils;

pub fn system_cpu_details(){
 // Create a System object
    let mut system = System::new_all();

    // Refresh system information
    system.refresh_all();

    for cpu in system.cpus() {
        println!("{}%", cpu.cpu_usage());
        println!("{}%", cpu.name());
    }

    println!("Total CPU usage: {}%", system.global_cpu_usage());
    println!("Total CPU usage: {}%", system.available_memory());
    println!("Total CPU usage: {:?}%", system.physical_core_count());
    println!("Total CPU usage: {}%", system.total_memory());
    println!("Total CPU usage: {}%", system.used_memory());
}

#[derive(Serialize, Debug)]
struct CpuUsage {
    total: f64,
    cores: Vec<f64>,
}

fn output_cpu_usage(system: &System, json: bool){
    let total: f32 = system.global_cpu_usage();

    if json {

        let mem_usage = CpuUsage {
            total: format_utils::round_to_one_decimal(total),
            cores: system.cpus().iter().map(|c| format_utils::round_to_one_decimal(c.cpu_usage())).collect(),
        };

        format_utils::print_json(&mem_usage);
    }else{
        println!(
            "total: {}, cpus: {}",
            format_utils::format_percent(total),
            system
                .cpus()
                .iter()
                .map(|c| format_utils::format_percent(c.cpu_usage()))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
}

pub async fn system_cpu_usage(watch: bool, json: bool) {
    let mut system = System::new_all();

    if watch {
        loop {
            system.refresh_cpu_all();
            format_utils::clear_terminal();
            output_cpu_usage(&system, json);
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    } else {
        system.refresh_cpu_all();
        output_cpu_usage(&system, json)
    }
}