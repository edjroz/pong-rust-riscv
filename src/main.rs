#![no_std]
#![no_main]

use core::arch::global_asm;
mod uart;
mod timer;
mod vt100;
mod game;
mod render;


global_asm!(include_str!("boot.S"));

#[no_mangle]
extern "C" fn kmain() -> ! {
    uart::Uart::init();
    println!("Booting Text Pong on RISC-V");

    timer::delay_ticks(timer::TIMEBASE_FREQ / 2);
    vt100::clear_screen();
    let mut state = game::GameState::new();
    let frame_ticks = timer::ticks_per_frame(30); // 30fps

    loop {
        let frame_start = timer::mtime();

        state.handle_input();
        state.update_ai();
        state.update_ball();
        render::draw(&state);
        let deadline = frame_start + frame_ticks;
        timer::delay_until(deadline);

    }

}

#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("\r\n!!! PANIC !!!");
    if let Some(loc) = info.location() {
        println!("  at {}:{}", loc.file(), loc.line());
    }
    loop {}
}
