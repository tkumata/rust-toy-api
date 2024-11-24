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
    disk_info: Vec<DiskInfo>,
}

pub async fn get_metrics() -> impl IntoResponse {
    let sys = System::new_all();

    let kernel = System::name();
    let load_avg = System::load_average();
    let used_mem = format_memory_size(sys.used_memory());
    let disk_info = disks_service::get_storage();

    let metrics: Metrics = Metrics {
        kernel_name: kernel,
        cpu_load: format!("{}, {}, {}", load_avg.one, load_avg.five, load_avg.fifteen),
        memory_usage: used_mem,
        disk_info: disk_info.await,
    };

    Json(json!(metrics))
}

pub async fn get_cpuload1() -> impl IntoResponse {
    let load_avg: sysinfo::LoadAvg = System::load_average();
    Json(json!(load_avg.one))
}

pub async fn get_memusage() -> impl IntoResponse {
    let sys = System::new_all();
    let mem = format_memory_size(sys.used_memory());
    Json(json!(mem))
}

pub async fn get_diskusage() -> impl IntoResponse {
    Json(json!(disks_service::get_storage().await))
}

fn format_memory_size(bytes: u64) -> String {
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
