use serde_derive::Serialize;
use std::time::Duration;
use sysinfo::System;

use crate::{format_utils, graph_utils};

pub async fn system_mem_usage(watch: bool, json: bool, plot: bool) {
    let mut system = System::new_all();

    if watch {
        let mut points: Vec<(f32, f32)> = Vec::new();
        let mut i:f32 = 0.0;
        loop {
            system.refresh_memory();
            let used_memory = system.used_memory();
            let total_memory = system.total_memory();
            let free_memory = system.free_memory();
            let used_percentage = format_utils::percent(used_memory, total_memory);

            if points.len() >= 30 {
                // Remove the oldest point (optional, only if you want a rolling window)
                points.remove(0); // or points.drain(..1); for potentially more efficient operation
            }
            points.push((i, used_percentage as f32));
            i += 1.0;
 

            format_utils::clear_terminal();
            if json {
                let mem_usage = MemUsage {
                    total_gb: format_utils::bytes_to_gb(total_memory),
                    used_gb: format_utils::bytes_to_gb(used_memory),
                    used_percent: used_percentage,
                    free_gb: format_utils::bytes_to_gb(free_memory),
                    free_percent: format_utils::percent(free_memory, total_memory),
                };
                format_utils::print_json(&mem_usage);
            } else {
                println!(
                    "total: {}, used: {} ({}), free: {} ({})",
                    format_utils::format_bytes(total_memory),
                    format_utils::format_bytes(used_memory),
                    format_utils::calculate_format_percent(used_memory, total_memory),
                    format_utils::format_bytes(free_memory),
                    format_utils::calculate_format_percent(free_memory, total_memory)
                );
            }

            if plot{
                // Plot the chart
                let title: &str = "used memory %";
                graph_utils::plot_chart(&points, String::from(title));
            }

            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    } else {
        system.refresh_memory();
        let used_memory = system.used_memory();
        let total_memory = system.total_memory();
        let free_memory = system.free_memory();

        if json {
            let mem_usage = MemUsage {
                total_gb: format_utils::bytes_to_gb(total_memory),
                used_gb: format_utils::bytes_to_gb(used_memory),
                used_percent: format_utils::percent(used_memory, total_memory),
                free_gb: format_utils::bytes_to_gb(free_memory),
                free_percent: format_utils::percent(free_memory, total_memory),
            };
            format_utils::print_json(&mem_usage);
        } else {
            println!(
                "total: {}, used: {} ({}), free: {} ({})",
                format_utils::format_bytes(total_memory),
                format_utils::format_bytes(used_memory),
                format_utils::calculate_format_percent(used_memory, total_memory),
                format_utils::format_bytes(free_memory),
                format_utils::calculate_format_percent(free_memory, total_memory)
            );
        }
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
