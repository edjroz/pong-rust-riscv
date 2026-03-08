use core::fmt;

const UART_BASE: usize = 0x1000_0000;


// 16550 UART register offsets
const THR: usize = 0x00; // Transmit Holding Register
const RBR: usize = 0x00; // Receive Buffer Register
const IER: usize = 0x01; // Interrupt Enable Register
const FCR: usize = 0x02; // FIFO Control Register
const LCR: usize = 0x03; // Line Control Register
const LSR: usize = 0x05; // Line Status Register
//
// LSR bit flags
const LSR_TX_EMPTY: u8 = 1 << 5; // Transmitter Holding Register Empty
const LSR_RX_READY: u8 = 1 << 0; // Transmiter Holding Register Ready
//

pub struct Uart;

impl Uart {
    // Initialize the UART (16550 compatible): enable FIFO, set 8-N-1, nointerrupts
    pub fn init() {
        let base = UART_BASE as *mut u8;
        unsafe{
            base.add(IER).write_volatile(0x00); // Disable interrupts
            base.add(LCR).write_volatile(0x80); // Enable to sed baud rate
            base.add(0).write_volatile(0x01); // Divisor Low Rate (115200 baud) 
            base.add(1).write_volatile(0x00); // Divisor High byte 
            base.add(LCR).write_volatile(0x03); // 8 bits, no parity, 1 stop bit
            base.add(FCR).write_volatile(0xC7); // enable FIFO, clear them, with 14-byte threshold
        }
    }
    pub fn putc(c: u8) {
        let base = UART_BASE as *mut u8;
        unsafe {
            while base.add(LSR).read_volatile() & LSR_TX_EMPTY == 0 {} // Wait until THR is empty
            base.add(THR).write_volatile(c); // Write character to THR
        }
    }
    pub fn puts(s: &str) {
        for b in s.bytes() {
            Self::putc(b);
        }
    }
    pub fn try_getc()-> Option<u8> {
        let base = UART_BASE as *mut u8;
        unsafe {
            if base.add(LSR).read_volatile() & LSR_RX_READY != 0 {
                Some(base.add(RBR).read_volatile()) // Read character from RBR
            } else {
                None
            }
        }

    }
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        Self::puts(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let _ = write!($crate::uart::Uart, $($arg)*);
    });
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\r\n"));
    ($($arg:tt)*) => ({
        $crate::print!($($arg)*);
        $crate::print!("\r\n");
    });
}

