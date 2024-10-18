use sysinfo::Disks;
use serde_derive::Serialize;

use crate::format_utils;

#[derive(Serialize, Debug)]
struct DiskInfoOutput {
    available_gb: f64,
    total_gb: f64,
    percent_free: f64,
    file_system: String,
    mount_point: String,
    removable: bool,
    kind: String,
}

pub async fn system_disk_info(json: bool) {

    let disks = Disks::new_with_refreshed_list();
    let mut outputs: Vec<DiskInfoOutput> = Vec::new();
    for disk in &disks {
        outputs.push(DiskInfoOutput{
            available_gb: format_utils::bytes_to_gb(disk.available_space()),
            total_gb: format_utils::bytes_to_gb(disk.total_space()),
            percent_free: format_utils::percent(disk.available_space(), disk.total_space()),
            file_system: String::from(disk.file_system().to_str().unwrap_or_default()),
            mount_point: String::from(disk.mount_point().to_str().unwrap_or_default()),
            removable: disk.is_removable(),
            kind: disk.kind().to_string(),
        });
    }

     if json {
        format_utils::print_json(&outputs);
    }else{
        for o in outputs{
            println!("available: {}GB ({}%), total: {}GB, file_system: {}, mount_point: {}, removable: {}, king: {}", o.available_gb, o.percent_free, o.total_gb, o.file_system, o.mount_point, o.removable, o.kind)
        }
    }
}