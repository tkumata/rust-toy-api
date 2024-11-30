use serde::Serialize;
use sysinfo::Disks;
use sysinfo::System;

#[derive(Serialize)]
pub(crate) struct CpuLoad {
    pub(crate) load_01: f64,
    pub(crate) load_05: f64,
    pub(crate) load_15: f64,
}

#[derive(Serialize)]
pub(crate) struct DiskInfo {
    pub(crate) mount_point: String,
    pub(crate) available_space: u64,
    pub(crate) total_space: u64,
}

#[derive(Serialize)]
pub(crate) struct MemInfo {
    pub(crate) memory_usage: u64,
    pub(crate) memory_total: u64,
}

pub struct MetricsService;

impl MetricsService {
    pub fn new() -> Self {
        MetricsService
    }

    pub async fn get_cpu(&self) -> CpuLoad {
        let load_avg = System::load_average();

        CpuLoad {
            load_01: load_avg.one,
            load_05: load_avg.five,
            load_15: load_avg.fifteen,
        }
    }

    pub async fn get_memory(&self) -> MemInfo {
        let sys = System::new_all();
        let memu = sys.used_memory();
        let memt = sys.total_memory();

        MemInfo {
            memory_usage: memu,
            memory_total: memt,
        }
    }

    pub async fn get_storage(&self) -> Vec<DiskInfo> {
        let mut disk_info = Vec::new();
        let disks = Disks::new_with_refreshed_list();

        for disk in disks.list() {
            let diskinfo = DiskInfo {
                mount_point: disk
                    .mount_point()
                    .to_path_buf()
                    .to_string_lossy()
                    .to_string(),
                available_space: disk.available_space(),
                total_space: disk.total_space(),
            };
            disk_info.push(diskinfo);
        }

        disk_info
    }
}
