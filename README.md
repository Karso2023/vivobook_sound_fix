### Vivobook S14 Flip TP3407SA Speaker no sound FIXED

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)


### Problem
Vivobook S14 Flip no sound, only works if I use bluetooth speaker or headphone.

I found one temporary fix (link is provided in reference and code is in legacy_attempt directory). 
It works but the sound is really low + it might fail again after package update.


### Solution

A Rust binary that:
- Auto-detects the correct I2C bus by scanning sysfs for the `TIAS2781` ACPI entry
- Configures both TAS2781 amplifier chips (left + right channels) at full volume
- Uses `I2C_SLAVE_FORCE` to bypass the kernel driver lock (equivalent to `i2cset -f`)

A systemd service runs it automatically on every boot, and survives `pacman -Syu` upgrades.

**Install via AUR (CachyOS / Arch Linux):**
```bash
paru -S asus-sound-fix
```


<br>

**If use directly:**
```bash
git clone https://github.com/Karso2023/vivobook_sound_fix.git

cd vivobook_sound_fix/sound-fix

cargo build --release

sudo cp target/release/sound-fix /usr/bin/asus-sound-fix

sudo cp ../systemd/vivobook-sound-fix.service /usr/lib/systemd/system/

sudo systemctl enable --now asus-sound-fix.service
```

Confirmed working on **TP3407SA**. Should work on other ASUS laptops with the same TAS2781 no sound issue. Open an issue if it works (or doesn't) on your model.


### Reference

- [A cool tutorial I found if you want to learn how to build your own package](https://youtu.be/_OgJhPR_CJo?si=jLLWZpZQbJQ59NaB)
- [Linux Mint Forums discussion](https://forums.linuxmint.com/viewtopic.php?t=445025&sid=d2698ea759a6284263a1a88a4aad0ca8)
- [Pretty useful temporary fix from other people](https://gist.github.com/rraks/4edddb99b50b94fe6298adbf3c9f43eb)
- [TAS2781 product details](https://www.ti.com/product/TAS2781)
- [TAS2781 documentation](https://www.ti.com/lit/ds/symlink/tas2781.pdf)
- [SmartPA speaker protection algorithm](https://www.ti.com/lit/an/slaa857/slaa857.pdf)
