#![no_std]
#![no_main]

use core::arch::global_asm;
mod uart;

global_asm!(include_str!("boot.S"));

#[no_mangle]
extern "C" fn kmain() -> ! {
    uart::Uart::init();
    println!("Hello, RISC-V!");
    loop {

    }
}

#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
