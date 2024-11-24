use serde::Serialize;
use sysinfo::Disks;

#[derive(Serialize)]
struct DiskInfo {
    mount_point: String,
    spaces: Spaces,
}
#[derive(Serialize)]
struct Spaces {
    available_space: u64,
    total_space: u64,
}

pub async fn get_storage() -> Vec<String> {
    let mut disk_info: Vec<String> = Vec::new();
    let disks = Disks::new_with_refreshed_list();

    for disk in disks.list() {
        let diskinfo: DiskInfo = DiskInfo {
            mount_point: disk
                .mount_point()
                .to_path_buf()
                .to_string_lossy()
                .into_owned(),
            spaces: Spaces {
                available_space: disk.available_space(),
                total_space: disk.total_space(),
            },
        };
        let serialized: String = serde_json::to_string(&diskinfo).unwrap();
        disk_info.push(serialized);
    }

    // return Vec<String>
    disk_info
}
