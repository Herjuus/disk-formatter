pub mod formatter {
    use std::ffi::OsString;
    use std::fs::{OpenOptions};
    use std::io::{Write, Result, SeekFrom, Seek};
    use std::path::{Path, PathBuf};
    
    const BUFFER_SIZE: usize = 4096;
    
    fn overwrite_bytes(drive_path: &Path, passes: usize) -> Result<()> {
        let mut buffer = vec![0u8; BUFFER_SIZE];
        
        let mut dir = OpenOptions::new()
            .write(true)
            .open(drive_path)?;
        
        let drive_size = dir.metadata()?.len();
        
        for i in 0..passes {
            println!("Disk overwrite pass: {}.", i);
            
            dir.seek(SeekFrom::Start(0))?;
            
            let mut bytes_written: usize = 0;
            
            while bytes_written < drive_size as usize {
                let bytes_to_write = BUFFER_SIZE.min((drive_size - bytes_written as u64) as usize);
                dir.write_all(&buffer[..bytes_to_write])?;
                bytes_written += bytes_to_write;
            }
            
            dir.flush()?;
        }
        
        Ok(())
    }

    pub fn format_drive(drive: OsString) -> Result<()> {
        let passes = 3;
        
        #[cfg(target_os = "windows")]
        let drive_path = PathBuf::from(drive);
        
        #[cfg(target_os = "macos")]
        let drive_path = PathBuf::from(format!("/Volumes/{}/", drive.to_string_lossy()));
        println!("{}", drive_path.display());
        
        println!("Starting disk overwrite.");
        match overwrite_bytes(&drive_path, passes) {
            Ok(()) => println!("Data overwritten successfully."),
            Err(err) => eprintln!("Error: {}", err),
        }
        
        Ok(())
    }
}