use axum::{http::StatusCode, response::IntoResponse, response::Json};
use serde::Serialize;
use serde_json::json;
use sysinfo::System;

use crate::models;

#[derive(Serialize)]
struct Metrics {
    kernel_name: Option<String>,
    cpu_load: String,
    memory_usage: u64,
    disk_info: Vec<String>,
}

pub async fn get_metrics() -> impl IntoResponse {
    let kernel_name = models::metrics::get_kernelname();
    let cpu_load = models::metrics::get_load();
    let mem_usage = models::metrics::get_mem();
    let disk_info = models::metrics::get_storage();

    let metrics: Metrics = Metrics {
        kernel_name: kernel_name.await,
        cpu_load: cpu_load.await,
        memory_usage: mem_usage.await,
        disk_info: disk_info.await,
    };

    (StatusCode::OK, Json(json!(metrics)))
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
