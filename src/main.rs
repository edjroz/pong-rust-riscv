#![no_std]
#![no_main]

use core::arch::global_asm;
mod uart;
mod timer;

global_asm!(include_str!("boot.S"));

#[no_mangle]
extern "C" fn kmain() -> ! {
    uart::Uart::init();
    let mut count = 0u32;
    loop {
        println!("Tick : {}", count);
        count +=1;
        timer::delay_ticks(timer::TIMEBASE_FREQ);
    }
}

#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
