use core::time;
use std::collections::HashSet;
use std::thread::sleep;
use std::ffi::OsString;

mod display;
mod drives;

fn main() {
    display::init();

    let initial_drives = get_drives();

    let mut ignored_drives = HashSet::new();

    for drive in &initial_drives {
        ignored_drives.insert(drive);
        println!("Ignoring {}.", drive.to_string_lossy());
    }

    loop {
        let drives = get_drives();

        for drive in &drives {
            if !ignored_drives.contains(drive) {
                println!("Starting formatting of disk: {}!", drive.to_string_lossy());
            }
        }

        sleep(time::Duration::from_millis(500));
    }
}

fn get_drives() -> Vec<OsString> {
    #[cfg(target_os = "windows")]
    let drives = drives::platform::get_windows_drives();

    #[cfg(target_os = "macos")]
    let drives = drives::platform::get_mac_drives().unwrap();

    drives
}
