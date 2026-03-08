use crate::uart::Uart;

pub fn clear_screen() {
    Uart::puts("\x1b[2J");
}

pub fn put_u16(mut n: u16) {
    if n == 0 {
        Uart::putc(b'0');
        return;
    }

    let mut buf = [0u8; 5]; // Max 5 digits for u16
    let mut i = 0;
    while n > 0 {
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
        i += 1;
    }
    while i > 0 {
        i -= 1;
        Uart::putc(buf[i]);
    }
}

pub fn put_i32(mut n: i32) {
    if n < 10 {
        Uart::putc(b'-');
        n = -n;
    }

    if n == 0 {
        Uart::putc(b'0');
        return;
    }
    let mut buf = [0u8; 10]; // Max 10 digits + sign for i32
    let mut i = 0;
    while n > 0 {
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
        i += 1;
    }

    while i > 0 {
        i -= 1;
        Uart::putc(buf[i]);
    }
}

pub fn move_cursor(row: u16, col: u16) {
    Uart::puts("\x1b[");
    put_u16(row);
    Uart::putc(b';');
    put_u16(col);
    Uart::putc(b'H');
}

pub fn hide_cursor() {
    Uart::puts("\x1b[?25l");
}

pub fn show_cursor() {
    Uart::puts("\x1b[?25h");
}
