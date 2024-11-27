use serde::Serialize;
use sysinfo::System;

#[derive(Serialize)]
pub(crate) struct MemInfo {
    pub(crate) memory_usage: u64,
    pub(crate) memory_total: u64,
}

pub async fn get_memusage() -> Vec<MemInfo> {
    let mut mem_info = Vec::new();

    let sys = System::new_all();
    let memu = sys.used_memory();
    let memt = sys.total_memory();

    let meminfo = MemInfo {
        memory_usage: memu,
        memory_total: memt,
    };
    mem_info.push(meminfo);

    mem_info
}
