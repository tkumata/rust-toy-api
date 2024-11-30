use serde::Serialize;
use sysinfo::System;

use crate::application::usecases::metrics_service::CpuLoad;
use crate::application::usecases::metrics_service::DiskInfo;
use crate::application::usecases::metrics_service::MemInfo;
use crate::application::usecases::metrics_service::MetricsService;

#[derive(Serialize)]
pub struct Metrics {
    kernel_info: String,
    cpu_load: CpuLoad,
    memory_usage: ConvertedMemoryInfo,
    disk_info: Vec<ConvertedDiskInfo>,
}

#[derive(Serialize)]
pub struct ConvertedDiskInfo {
    mount_point: String,
    available_space: String,
    total_space: String,
}

#[derive(Serialize)]
pub struct ConvertedMemoryInfo {
    memory_usage: String,
    memory_total: String,
}

pub struct MetricsController;

impl MetricsController {
    pub fn new() -> Self {
        MetricsController
    }

    pub async fn get_metrics(&self) -> Metrics {
        let service = MetricsService::new();

        let kernel = self.get_kernel().await;
        let load_avg = service.get_cpu().await;
        let used_mem = converted_memory_info(service.get_memory().await);
        let diskinfo = converted_disks_info(service.get_storage().await);

        Metrics {
            kernel_info: kernel,
            cpu_load: load_avg,
            memory_usage: used_mem,
            disk_info: diskinfo,
        }
    }

    pub async fn get_kernel(&self) -> String {
        format!(
            "{} {}",
            System::long_os_version().unwrap(),
            System::kernel_version().unwrap()
        )
    }

    pub async fn get_cpu(&self) -> CpuLoad {
        let service = MetricsService::new();
        service.get_cpu().await
    }

    pub async fn get_memory(&self) -> ConvertedMemoryInfo {
        let service = MetricsService::new();
        converted_memory_info(service.get_memory().await)
    }

    pub async fn get_storage(&self) -> Vec<ConvertedDiskInfo> {
        let service = MetricsService::new();
        converted_disks_info(service.get_storage().await)
    }
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

fn converted_memory_info(memory: MemInfo) -> ConvertedMemoryInfo {
    ConvertedMemoryInfo {
        memory_usage: format_bytes(memory.memory_usage),
        memory_total: format_bytes(memory.memory_total),
    }
}
