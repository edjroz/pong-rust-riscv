#![no_std]
#![no_main]

use core::arch::global_asm;
mod uart;
mod timer;
mod vt100;

global_asm!(include_str!("boot.S"));

#[no_mangle]
extern "C" fn kmain() -> ! {
    uart::Uart::init();
    vt100::clear_screen();
    vt100::hide_cursor();

    vt100::move_cursor(1, 1);
    uart::Uart::putc(b'+');
    for _ in 0..60 {
        uart::Uart::putc(b'-');
    }
    uart::Uart::putc(b'+');
    for row in 2..=21 {
        vt100::move_cursor(row, 1);
        uart::Uart::putc(b'|');
        vt100::move_cursor(row, 62);
        uart::Uart::putc(b'|');
    }

    vt100::move_cursor(22, 1);
    uart::Uart::putc(b'+');
    for _ in 0..60 {
        uart::Uart::putc(b'-');
    }
    uart::Uart::putc(b'+');

    loop {
    }
}

#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
