use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::io::AsRawFd;


// the previous version kept failing with EBUSY if kernel claimed it 
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
    (0x16, 0x40), 
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
    (0x02, 0x00), // power up starts the amplifier running
];

// 0x38 = left channel, 0x3d = right channel (specific to TP3407SA board wiring)
const CHIP_ADDRESSES: &[u16] = &[0x38, 0x3d];

pub fn configure(bus: u8) {
    let dev_path = format!("/dev/i2c-{}", bus);

    for &addr in CHIP_ADDRESSES {
        // Open the raw I2C bus file. We need read+write access.
        let mut file = match OpenOptions::new().read(true).write(true).open(&dev_path) {
            Ok(f) => f,
            Err(e) => {
                println!("Cannot open {}: {}", dev_path, e);
                continue;
            }
        };


        let ret = unsafe {
            libc::ioctl(file.as_raw_fd(), I2C_SLAVE_FORCE, addr as libc::c_ulong)
        };
        if ret < 0 {
            println!("Failed to force-set address 0x{:02x} on {}", addr, dev_path);
            continue;
        }

        // Write each (register, value) pair as a 2-byte I2C write transaction.
        // Sending [register, value] over the raw file is identical to what
        // i2cset and smbus_write_byte_data do, the kernel I2C layer handles framing.
        for &(register, value) in REGISTER_SEQUENCE {
            if let Err(e) = file.write_all(&[register, value]) {
                println!("Write failed reg 0x{:02x} on chip 0x{:02x}: {}", register, addr, e);
            }
        }

        println!("Configured chip 0x{:02x}", addr);
    }
}
