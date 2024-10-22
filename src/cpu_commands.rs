use serde_derive::Serialize;
use std::{collections::HashSet, time::Duration};
use sysinfo::System;

use crate::{format_utils, graph_utils};

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
        let output: Vec<CpuInfo> = unique_vendor_ids_vec
            .iter()
            .map(|v| CpuInfo {
                name: v.to_string(),
            })
            .collect();
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

pub async fn system_cpu_usage(watch: bool, json: bool, plot: bool) {
    let mut system = System::new_all();

    if watch {
        let mut points: Vec<(f32, f32)> = Vec::new();
        let mut i:f32 = 0.0;
        loop {
            system.refresh_cpu_all();
            let total: f32 = system.global_cpu_usage();
            if points.len() >= 30 {
                // Remove the oldest point (optional, only if you want a rolling window)
                points.remove(0); // or points.drain(..1); for potentially more efficient operation
            }
            points.push((i, total));
            i += 1.0;

            let usage = CpuUsage {
                total: format_utils::round_to_one_decimal(total),
                core_usage: system
                    .cpus()
                    .iter()
                    .map(|c| format_utils::round_to_one_decimal(c.cpu_usage()))
                    .collect(),
            };

            format_utils::clear_terminal();
            if json {
                format_utils::print_json(&usage);
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

            if plot{
                // Plot the chart
                let title: &str = "total cpu %";
                graph_utils::plot_chart(&points, String::from(title));
            }

            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    } else {
        system.refresh_cpu_all();
        let total: f32 = system.global_cpu_usage();

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
