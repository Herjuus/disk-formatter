use std::ffi::OsString;

#[cfg(target_os = "windows")]
pub mod platform {
    use std::ffi::OsString;
    use std::fs;
    use std::io;
    
    pub fn get_windows_drives() -> io::Result<Vec<OsString>> {
        let mut drives = Vec::new();
        for letter in b'A'..=b'Z' {
            let root = format!("{}", letter as char);
            if fs::metadata(&root).is_ok() {
                drives.push(OsString::from(root));
            }
        }
        Ok(drives)
    }
}

#[cfg(target_os = "macos")]
pub mod platform {
    use std::ffi::OsString;
    use std::fs;
    use std::io;
    
    pub fn get_mac_drives() -> io::Result<Vec<OsString>> {
        let mut drives = Vec::new();
        for entry in fs::read_dir("/Volumes")? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name() {
                    drives.push(name.to_owned());
                }
            }
        }
        Ok(drives)
    }
}

pub fn get_drives() -> Vec<OsString> {
    #[cfg(target_os = "windows")]
    let drives = platform::get_windows_drives();

    #[cfg(target_os = "macos")]
    let drives = platform::get_mac_drives().unwrap();

    drives
}
