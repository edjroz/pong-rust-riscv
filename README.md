# RISC-V Pong

A bare-metal Pong game written in Rust (`#![no_std]`) targeting RISC-V. Renders via UART using VT100 escape codes.

## Features

- Player vs AI — left paddle (you) vs right paddle (computer)
- Text-based rendering over serial console at 30 FPS
- Custom boot assembly, linker script, and UART driver

## Controls

- `W` / `S` — move paddle up / down

## Build & Run

Requires a RISC-V Rust toolchain (`riscv64gc-unknown-none-elf` or similar) and QEMU.

```
make build   # cargo build --release
make run     # cargo run --release
```
