# Arduino Uno R4 WiFi Rust HAL (RA4M1)

This repository contains a Hardware Abstraction Layer (HAL) for the **Renesas RA4M1** microcontroller, specifically tailored for the **Arduino Uno R4 WiFi**. It provides a type-safe, memory-secure, and efficient way to develop embedded firmware using the Rust programming language.

## 🛠 Features
* **Zero-Cost Abstractions:** Built directly on top of the PAC (Peripheral Access Crate).
* **Type-Safe GPIO:** Prevents hardware resource conflicts at compile time.
* **Pin Mapping:** Pre-configured for Arduino Uno R4 WiFi (e.g., Onboard LED on `P102`).
* **Optimized for Security:** Ideal for projects requiring high reliability like smart door locks.

## 🚀 Getting Started

### Prerequisites
1. **Rust Toolchain:** Install `thumbv4t-none-eabi` or the relevant target for RA4M1.
2. **Cargo-binutils:** For generating binary files.
   ```powershell
   cargo install cargo-binutils
   rustup component add llvm-tools-preview
3. **Arduino CLI:** Required for uploading the firmware.
   ```powershell
   .\arduino-cli.exe core install arduino:renesas_uno

## 📦 Building and Flashing
To build the project and flash it to your board, follow these steps:
1. **Compile to ELF and Convert to BIN:**
   ```powershell
   cargo build --release
   cargo objcopy --release -- -O binary uno_r4_wifi.bin
2. **Upload to Board:**
Put your board in Bootloader Mode (Double-tap the Reset button) and run:
   ```powershell
   .\arduino-cli.exe upload -p COM6 --fqbn arduino:renesas_uno:unor4wifi --input-file uno_r4_wifi.bin
Note: Change COM6 to your actual port found via:
   ```powershell
   .\arduino-cli.exe board list
