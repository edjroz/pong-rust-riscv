/// CLINT base address on QEMU virt machine
const CLINT_BASE: usize = 0x0200_0000;

/// mtime register offset within CLINT
const MTIME_OFFSET: usize = 0xBFF8;

/// QEMU virt timebase frequency: 10 MHz
pub const TIMEBASE_FREQ: u64 = 10_000_000;


/// Read the current mtime value (64-bit tick counter).
pub fn mtime() -> u64 {
    let mtime_addr = (MTIME_OFFSET + CLINT_BASE) as *const u64;
    unsafe { mtime_addr.read_volatile() }
}


/// Busy-wait until mtime reaches `target`.
pub fn delay_until(target: u64) {
    while mtime() < target {
        // Optionally, we could add a hint to the CPU here (e.g., `core::hint::spin_loop()`)
    }
}

pub fn delay_ticks(ticks: u64) {
    let target = mtime() + ticks;
    delay_until(target);
}

pub const fn ticks_per_frame(fps: u64) -> u64 {
    TIMEBASE_FREQ / fps
}
