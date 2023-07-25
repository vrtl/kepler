use filesystem::{FileSystem, OsFileSystem};
use sysinfo::{DiskExt, System, SystemExt};

fn bytes_to_gigabytes(bytes: u64) -> u64 {
    bytes / (1024 * 1024 * 1024)
}

fn main() {
    let mut sys = System::new_all();

    sys.refresh_disks();

    let fs = OsFileSystem::new();

    sys.disks().iter().for_each(|disk| {
        println!("{:?}", disk.mount_point());
        println!(
            "{:?} / {:?}",
            bytes_to_gigabytes(disk.available_space()),
            bytes_to_gigabytes(disk.total_space())
        );

        let path = disk.mount_point().to_path_buf();
        fs.set_current_dir(path).unwrap();

        let a = fs.read_dir(".").unwrap();
        a.for_each(|b| {
            println!("{}", b.unwrap().file_name().to_str().unwrap());
        });
    })
}
