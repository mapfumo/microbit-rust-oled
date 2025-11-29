# micro:bit v2 OLED Display with Embedded Rust

A simple "Hello World" example for controlling an SSD1306 OLED display (128x32, I2C interface) using embedded Rust on the BBC micro:bit v2.

![micro:bit with OLED display](https://img.shields.io/badge/micro:bit-v2-blue) ![Rust](https://img.shields.io/badge/rust-embedded-orange)

## Overview

### About the micro:bit v2

The [BBC micro:bit v2](https://microbit.org/) is an ARM-based embedded development board featuring:

- **Nordic nRF52833 microcontroller** (Cortex-M4, 64MHz, 512KB Flash, 128KB RAM)
- **Built-in sensors**: accelerometer, magnetometer, temperature sensor, microphone
- **5×5 LED matrix display** and speaker
- **Edge connector** with 25 pins for external peripherals
- **Bluetooth Low Energy** support
- Excellent for learning embedded systems and IoT projects

### Why Embedded Rust?

Rust brings memory safety and zero-cost abstractions to embedded development:

- **Memory safety without garbage collection** - catch bugs at compile time
- **No undefined behavior** - eliminates entire classes of embedded bugs
- **Excellent tooling** - cargo, rustfmt, clippy work seamlessly
- **Growing ecosystem** - mature HAL crates for most microcontrollers
- **Performance** - same speed as C/C++, but safer

The micro:bit has excellent Rust support through the `microbit-v2` crate, making it an ideal platform for learning embedded Rust.

### Expansion Boards

This project uses a micro:bit expansion board (like the IO BIT V2.0) which:

- **Breaks out all edge connector pins** to convenient screw terminals or headers
- **Provides stable power** via USB or battery for external peripherals
- **Makes prototyping easier** - no need for alligator clips or breadboard adapters
- **Protects the micro:bit** - prevents accidental damage to the edge connector

### This Project as a Foundation

This simple OLED "Hello World" serves as a foundation for more complex projects:

**Sensor Data Display:**

- Show accelerometer readings (tilt angle, gesture detection)
- Display temperature and compass heading from built-in sensors
- Add external I2C sensors (BME280 for humidity/pressure, MPU6050 for gyroscope)
- Graph real-time data with scrolling charts

**User Interface:**

- Create menus navigated with the A/B buttons
- Display status messages and error codes
- Show WiFi/Bluetooth connection status
- Build data logging interfaces

**Multi-Device Projects:**

- Combine multiple I2C devices (sensors + OLED) on the same bus
- Add SPI displays or SD card storage
- Create weather stations, motion monitors, or game consoles
- Build robotics projects with motor controllers

The patterns learned here (I2C communication, display management, error handling) apply directly to more advanced embedded Rust projects.

## Hardware Requirements

- **BBC micro:bit v2** (nRF52833)
- **OLED Display Module** - SSD1306, 128x32 resolution, I2C interface, white font
- **micro:bit Expansion Board** (e.g., IO BIT V2.0 Horizontal Adapter Plate)
- **4 female-to-female jumper wires** (for VCC, GND, SCL, SDA)
- **2 USB cables** (one for programming micro:bit, one for powering expansion board)

## Wiring

Connect the OLED display to the expansion board's edge connector pins:

| OLED Pin | Expansion Board Pin | Description  |
| -------- | ------------------- | ------------ |
| VCC      | 3V                  | Power (3.3V) |
| GND      | GND                 | Ground       |
| SCL      | Pin 19 (SCL)        | I2C Clock    |
| SDA      | Pin 20 (SDA)        | I2C Data     |

**Important:** The expansion board must be powered separately via its USB port to provide stable 3.3V to the OLED.

## Software Setup

### Prerequisites

1. **Install Rust and tools:**

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add thumbv7em-none-eabihf
   cargo install probe-rs --features cli
   ```

2. **Install objcopy (for creating hex files):**
   ```bash
   # Already included with Rust toolchain as rust-objcopy
   ```

### Project Structure

```
microbit-oled/
├── Cargo.toml
├── .cargo/
│   └── config.toml
└── src/
    └── main.rs
```

### Dependencies (Cargo.toml)

```toml
[package]
name = "microbit-oled"
version = "0.1.0"
edition = "2021"

[dependencies]
cortex-m = "0.7"
cortex-m-rt = { version = "0.7", features = ["device"] }
panic-halt = "0.2"
microbit-v2 = "0.13"
ssd1306 = "0.8"
embedded-graphics = "0.8"

[profile.release]
codegen-units = 1
debug = true
lto = true
```

### Configuration (.cargo/config.toml)

```toml
[target.thumbv7em-none-eabihf]
runner = "probe-rs run --chip nRF52833_xxAA"
rustflags = [
  "-C", "link-arg=-Tlink.x",
]

[build]
target = "thumbv7em-none-eabihf"
```

## Building and Flashing

Simply run:

```bash
cargo run --release
```

The program will automatically build, flash to the micro:bit, and run.

### Alternative: Drag-and-Drop Method

If you prefer manual flashing or `cargo run` has issues:

```bash
# Build the project
cargo build --release

# Convert to Intel HEX format
rust-objcopy target/thumbv7em-none-eabihf/release/microbit-oled -O ihex microbit.hex

# Copy to micro:bit (it appears as USB drive)
cp microbit.hex /media/$USER/MICROBIT/
```

The micro:bit will automatically reset and run the program.

## Features

The example program:

1. Shows a **smiley face** on the LED matrix when starting
2. Initializes the I2C connection to the OLED
3. If initialization fails, shows a blinking **X pattern** on the LED matrix
4. On success, shows a **checkmark** on the LED matrix
5. Displays "Hello World!" on the OLED screen
6. Shows a **heart pattern** on the LED matrix to confirm completion

## Common Issues and Gotchas

### 1. ❌ No Display Output - Wrong I2C Bus

**Problem:** Using `board.i2c_internal` instead of `board.i2c_external`

**Solution:** The edge connector pins (19/20) are on the external I2C bus:

```rust
// ❌ Wrong - this is for internal sensors
let i2c = twim::Twim::new(board.TWIM0, board.i2c_internal.into(), twim::Frequency::K100);

// ✅ Correct - use external I2C for edge connector
let i2c = twim::Twim::new(board.TWIM0, board.i2c_external.into(), twim::Frequency::K100);
```

### 2. ❌ Program Compiles but Nothing Runs

**Problem:** Using individual LED pins incorrectly

**Solution:** Use the Display API properly:

```rust
// ❌ Wrong - doesn't work reliably
let mut led = board.display_pins.col1.into_push_pull_output(Level::Low);
led.set_high().ok();

// ✅ Correct - use the Display API
let mut display = Display::new(board.display_pins);
display.show(&mut timer, pattern, 1000);
```

### 3. ❌ OLED Not Responding (Blinking X on LED Matrix)

**Possible causes:**

- **Expansion board not powered:** Connect a separate USB cable to the expansion board's USB port
- **Wrong wiring:** Double-check connections to pins 19 (SCL) and 20 (SDA)
- **micro:bit inserted upside down:** LED matrix should face UP, buttons on top
- **Wrong I2C address:** Try 0x3D if 0x3C doesn't work
- **Loose connections:** Ensure jumper wires are firmly connected

### 4. ❌ Linker Errors: "No loadable segments"

**Problem:** Missing linker script configuration

**Solution:** Ensure `.cargo/config.toml` has the correct rustflags:

```toml
rustflags = [
  "-C", "link-arg=-Tlink.x",
]
```

And `cortex-m-rt` has the device feature:

```toml
cortex-m-rt = { version = "0.7", features = ["device"] }
```

### 5. ❌ Binary Only Contains Debug Sections

**Problem:** Code isn't being linked properly

**Solution:** Clean and rebuild:

```bash
cargo clean
cargo build --release
```

Verify sections exist:

```bash
rust-objdump -h target/thumbv7em-none-eabihf/release/microbit-oled
```

You should see `.text`, `.rodata`, `.data`, and `.bss` sections.

### 6. ⚠️ Pin Naming Confusion

The micro:bit documentation refers to edge connector pins (like "Pin 19"), but these map to specific nRF52833 GPIO pins (like "P0.16"). The `microbit-v2` crate provides convenient abstractions:

- ✅ Use `board.i2c_external` for edge connector I2C
- ❌ Don't try to manually specify GPIO pin numbers

## Troubleshooting Checklist

- [ ] micro:bit is V2 (not V1)
- [ ] micro:bit is seated correctly in expansion board (LED matrix facing up)
- [ ] Expansion board has separate USB power connected
- [ ] OLED wiring: VCC→3V, GND→GND, SCL→19, SDA→20
- [ ] Code uses `board.i2c_external` (not `i2c_internal`)
- [ ] Binary has `.text` section (check with `rust-objcopy`)
- [ ] Used drag-and-drop hex file method for flashing

## Extending the Project

Once you have the basic example working, you can:

- Display sensor data (temperature, accelerometer, etc.)
- Show animations or scrolling text
- Draw shapes and graphics using `embedded-graphics`
- Add button controls to change what's displayed
- Connect multiple I2C devices

## Resources

- [micro:bit Hardware](https://tech.microbit.org/hardware/)
- [microbit-v2 Rust Crate](https://docs.rs/microbit-v2/)
- [embedded-graphics Documentation](https://docs.rs/embedded-graphics/)
- [SSD1306 Driver](https://docs.rs/ssd1306/)
- [probe-rs Documentation](https://probe.rs/)

## Acknowledgments

Thanks to the Rust Embedded community and the maintainers of the `microbit-v2`, `ssd1306`, and `embedded-graphics` crates.

---
