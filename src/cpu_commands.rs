use serde_derive::Serialize;
use std::{collections::HashSet, time::Duration};
use sysinfo::System;

use crate::format_utils;

#[derive(Serialize, Debug)]
struct CpuInfo {
    name: String,
}

pub fn system_cpu_info(json: bool) {
    let s = sysinfo::System::new_all();

    let unique_vendor_ids: HashSet<String> =
        s.cpus().iter().map(|c| c.brand().to_string()).collect();
    let unique_vendor_ids_vec: Vec<String> = unique_vendor_ids.into_iter().collect();

    if json {
        let output: Vec<CpuInfo> = unique_vendor_ids_vec.iter().map(|v| CpuInfo { name: v.to_string() }).collect();
        format_utils::print_json(&output);
    } else {
        let joined_vendor_ids = unique_vendor_ids_vec.join(", ");
        println!("{}", joined_vendor_ids);
    }
}

#[derive(Serialize, Debug)]
struct CpuUsage {
    total: f64,
    core_usage: Vec<f64>,
}

fn output_cpu_usage(system: &System, json: bool) {
    let total: f32 = system.global_cpu_usage();

    if json {
        let mem_usage = CpuUsage {
            total: format_utils::round_to_one_decimal(total),
            core_usage: system
                .cpus()
                .iter()
                .map(|c| format_utils::round_to_one_decimal(c.cpu_usage()))
                .collect(),
        };

        format_utils::print_json(&mem_usage);
    } else {
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
