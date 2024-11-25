use axum::{response::IntoResponse, response::Json};
use serde::Serialize;
use serde_json::json;
use sysinfo::System;

use crate::application::disks_service;
use crate::application::disks_service::DiskInfo;

#[derive(Serialize)]
struct Metrics {
    kernel_name: Option<String>,
    cpu_load: String,
    memory_usage: String,
    disk_info: Vec<ConvertedDiskInfo>,
}

#[derive(Serialize)]
struct ConvertedDiskInfo {
    mount_point: String,
    available_space: String,
    total_space: String,
}

pub async fn get_metrics() -> impl IntoResponse {
    let sys = System::new_all();

    let kernel = System::name();
    let load_avg = System::load_average();
    let used_mem = format_bytes(sys.used_memory());
    let diskinfo = convert_disks_info(disks_service::get_storage().await);

    let metrics: Metrics = Metrics {
        kernel_name: kernel,
        cpu_load: format!("{}, {}, {}", load_avg.one, load_avg.five, load_avg.fifteen),
        memory_usage: used_mem,
        disk_info: diskinfo,
    };

    Json(json!(metrics))
}

pub async fn get_cpuload() -> impl IntoResponse {
    let load_avg: sysinfo::LoadAvg = System::load_average();
    Json(json!(load_avg.one))
}

pub async fn get_memusage() -> impl IntoResponse {
    let sys = System::new_all();
    let mem = format_bytes(sys.used_memory());
    Json(json!(mem))
}

pub async fn get_diskusage() -> impl IntoResponse {
    Json(json!(convert_disks_info(
        disks_service::get_storage().await
    )))
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: &[(&str, u64)] = &[
        ("GB", 1024 * 1024 * 1024),
        ("MB", 1024 * 1024),
        ("KB", 1024),
    ];

    for &(unit, size) in UNITS {
        if bytes >= size {
            return format!("{:.2} {}", bytes as f64 / size as f64, unit);
        }
    }

    format!("{} Bytes", bytes)
}

fn convert_disks_info(disks: Vec<DiskInfo>) -> Vec<ConvertedDiskInfo> {
    disks
        .into_iter()
        .map(|disk| ConvertedDiskInfo {
            mount_point: disk.mount_point,
            available_space: format_bytes(disk.available_space),
            total_space: format_bytes(disk.total_space),
        })
        .collect()
}
