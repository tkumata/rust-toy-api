use axum::{response::IntoResponse, response::Json};
use serde::Serialize;
use serde_json::json;
use sysinfo::System;

use crate::application::disks_service;
use crate::application::disks_service::DiskInfo;
use crate::application::memory_service;
use crate::application::memory_service::MemInfo;

#[derive(Serialize)]
struct Metrics {
    kernel_name: Option<String>,
    cpu_load: String,
    memory_usage: Vec<ConvertedMemoryInfo>,
    disk_info: Vec<ConvertedDiskInfo>,
}

#[derive(Serialize)]
struct ConvertedDiskInfo {
    mount_point: String,
    available_space: String,
    total_space: String,
}

#[derive(Serialize)]
struct ConvertedMemoryInfo {
    memory_usage: String,
    memory_total: String,
}

pub async fn get_metrics() -> impl IntoResponse {
    let kernel = System::name();
    let load_avg = System::load_average();
    let used_mem = converted_memory_info(memory_service::get_memusage().await);
    let diskinfo = converted_disks_info(disks_service::get_storage().await);

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
    Json(json!(converted_memory_info(
        memory_service::get_memusage().await
    )))
}

pub async fn get_diskusage() -> impl IntoResponse {
    Json(json!(converted_disks_info(
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

fn converted_disks_info(disks: Vec<DiskInfo>) -> Vec<ConvertedDiskInfo> {
    disks
        .into_iter()
        .map(|disk| ConvertedDiskInfo {
            mount_point: disk.mount_point,
            available_space: format_bytes(disk.available_space),
            total_space: format_bytes(disk.total_space),
        })
        .collect()
}

fn converted_memory_info(memory: Vec<MemInfo>) -> Vec<ConvertedMemoryInfo> {
    memory
        .into_iter()
        .map(|mem| ConvertedMemoryInfo {
            memory_usage: format_bytes(mem.memory_usage),
            memory_total: format_bytes(mem.memory_total),
        })
        .collect()
}
