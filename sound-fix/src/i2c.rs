use std::fs;
use std::path::Path;

use i2cdev::linux::LinuxI2CDevice;


pub fn find_tias2781_device() -> Option<u8> {
    let path = Path::new("/sys/bus/i2c/devices/");

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();

            if let Some(name) = path.file_name() {
                if name.to_string_lossy().contains("TIAS2781") {

                    // read the symlink target + find the right bus
                    if let Ok(target) = fs::read_link(&path) {
                        for component in target.components() {
                            if let Some(s) = component.as_os_str().to_str() {
                                if s.starts_with("i2c-") && s[4..].chars().all(|x| x.is_ascii_digit()) {
                                    if let Some(bus) = s.strip_prefix("i2c-").and_then(|n| n.parse::<u8>().ok()) {
                                        return Some(bus);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // open bus directly if cant find sysfs entry 
    for bus_num in 0u8..= 9 {
        let dev_path = format!("/dev/i2c-{}", bus_num);
        if LinuxI2CDevice::new(&dev_path, 0x38).is_ok() {
            return Some(bus_num);
        }
        if LinuxI2CDevice::new(&dev_path, 0x3d).is_ok() {
            return Some(bus_num);
        }
    }
    
    None
}
