use serde::Serialize;
use sysinfo::Disks;

#[derive(Serialize)]
pub(crate) struct DiskInfo {
    mount_point: String,
    spaces: Spaces,
}

#[derive(Serialize)]
struct Spaces {
    available_space: u64,
    total_space: u64,
}

pub async fn get_storage() -> Vec<DiskInfo> {
    let mut disk_info = Vec::new();
    let disks = Disks::new_with_refreshed_list();

    for disk in disks.list() {
        let diskinfo = DiskInfo {
            mount_point: disk
                .mount_point()
                .to_path_buf()
                .to_string_lossy()
                .to_string(),
            spaces: Spaces {
                available_space: disk.available_space(),
                total_space: disk.total_space(),
            },
        };
        disk_info.push(diskinfo);
    }

    disk_info
}