use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::io::AsRawFd;

// I2C_SLAVE_FORCE bypasses the kernel driver lock (same as i2cset -f)
const I2C_SLAVE_FORCE: libc::c_ulong = 0x0706;

const REGISTER_SEQUENCE: &[(u8, u8)] = &[
    (0x00, 0x00), 
    (0x7f, 0x00), 
    (0x01, 0x01), 
    (0x0e, 0xc4),
    (0x0f, 0x40),
    (0x5c, 0xd9),
    (0x60, 0x10),
    (0x0a, 0x00), 
    (0x0d, 0x01),
    (0x16, 0x40), // speaker protection configuration
    (0x00, 0x01), 
    (0x17, 0xc8),
    (0x00, 0x04), 
    (0x30, 0x00),
    (0x31, 0x00),
    (0x32, 0x00),
    (0x33, 0x01),
    (0x00, 0x08), 
    (0x18, 0x00),
    (0x19, 0x00),
    (0x1a, 0x00),
    (0x1b, 0x00),
    (0x28, 0x40),
    (0x29, 0x00),
    (0x2a, 0x00),
    (0x2b, 0x00),
    (0x00, 0x0a), 
    (0x48, 0x00),
    (0x49, 0x00),
    (0x4a, 0x00),
    (0x4b, 0x00),
    (0x58, 0x40),
    (0x59, 0x00),
    (0x5a, 0x00),
    (0x5b, 0x00),
    (0x00, 0x00), 
    (0x02, 0x00), 
];

// 0x38 = left channel, 0x3d = right channel (TP3407SA board wiring)
const CHIP_ADDRESSES: &[u16] = &[0x38, 0x3d];

const MAX_RETRIES: u32 = 5;
const RETRY_DELAY: std::time::Duration = std::time::Duration::from_secs(1);

pub fn configure(bus: u8) {
    let dev_path = format!("/dev/i2c-{}", bus);

    for &addr in CHIP_ADDRESSES {
        let mut configured = false;

        for attempt in 1..=MAX_RETRIES {
            if attempt > 1 {
                // wait before retrying 
                std::thread::sleep(RETRY_DELAY);
            }

            if try_configure_chip(&dev_path, addr) {
                configured = true;
                break;
            }

            println!(
                "Attempt {}/{} failed for chip 0x{:02x}, retrying...",
                attempt, MAX_RETRIES, addr
            );
        }

        if configured {
            println!("Configured chip 0x{:02x}", addr);
        } else {
            println!(
                "Failed to configure chip 0x{:02x} after {} attempts",
                addr, MAX_RETRIES
            );
        }
    }
}

// Returns true if the chip was opened, addressed, and all registers written successfully.
fn try_configure_chip(dev_path: &str, addr: u16) -> bool {
    let mut file = match OpenOptions::new().read(true).write(true).open(dev_path) {
        Ok(f) => f,
        Err(_) => return false,
    };

    let ret = unsafe {
        libc::ioctl(file.as_raw_fd(), I2C_SLAVE_FORCE, addr as libc::c_ulong)
    };
    if ret < 0 {
        return false;
    }

    for &(register, value) in REGISTER_SEQUENCE {
        // 0x0a selects the TDM audio channel, left chip uses 0x1e, right chip uses 0x2e
        let actual_value = if register == 0x0a {
            if addr == 0x38 { 0x1e } else { 0x2e }
        } else {
            value
        };

        if file.write_all(&[register, actual_value]).is_err() {
            return false;
        }
    }

    true
}
