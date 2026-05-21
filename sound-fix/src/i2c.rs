use std::fs;
use std::path::Path;

pub fn find_tias2781_device() -> Option<String> {
    let path = Path::new("/sys/bus/i2c/devices/");

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();

            if let Some(name) = path.file_name() {
                if name.to_string_lossy().contains("TIAS2781") {
                    return Some(path.to_string_lossy().into_owned());
                }
            }
        }
    }
    None 
}