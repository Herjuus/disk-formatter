use core::time;
use std::collections::HashSet;
use std::thread::sleep;

mod display;
mod drives;
mod format;

fn main() {
    display::init();

    let initial_drives = drives::get_drives();

    let mut ignored_drives = HashSet::new();

    for drive in &initial_drives {
        ignored_drives.insert(drive);
        println!("Ignoring {}.", drive.to_string_lossy());
    }

    loop {
        let drives = drives::get_drives();

        for drive in &drives {
            if !ignored_drives.contains(drive) {
                println!("Starting formatting of disk: {}!", drive.to_string_lossy());
                match format::formatter::format_drive(drive.to_owned()) {
                    Ok(()) => println!("Drive formatted successfully."),
                    Err(err) => eprintln!("Error formatting drive: {}", err),
                }
            }
        }

        sleep(time::Duration::from_millis(500));
    }
}
