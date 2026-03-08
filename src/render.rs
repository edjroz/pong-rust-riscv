use crate::game::{GameState, FIELD_H, FIELD_W, PADDLE_H, PADDLE_X_LEFT, PADDLE_X_RIGHT};
use crate::uart::Uart;
use crate::vt100;

pub fn draw(state: &GameState) {
    vt100::hide_cursor();
    vt100::move_cursor(1, 1);

    // Top border
    draw_horizontal_border();

    // Field rows
    for row in 1..=FIELD_H {
        vt100::move_cursor(row as u16 + 1, 1);
        Uart::putc(b'|');

        for col in 1..=FIELD_W {
            let ch = if col == FIELD_W / 2 && row % 2 == 0 {
                b':'  // center line (dotted)
            } else if col == PADDLE_X_LEFT
                && row >= state.left_y
                && row < state.left_y + PADDLE_H
            {
                b'#'  // left paddle
            } else if col == PADDLE_X_RIGHT
                && row >= state.right_y
                && row < state.right_y + PADDLE_H
            {
                b'#'  // right paddle
            } else if col == state.ball_x && row == state.ball_y {
                b'O'  // ball
            } else {
                b' '
            };
            Uart::putc(ch);
        }

        Uart::putc(b'|');
    }

    // Bottom border
    vt100::move_cursor(FIELD_H as u16 + 2, 1);
    draw_horizontal_border();

    // Score line
    vt100::move_cursor(FIELD_H as u16 + 3, 1);
    Uart::puts("  Player: ");
    vt100::put_i32(state.left_score);
    Uart::puts("    CPU: ");
    vt100::put_i32(state.right_score);
    Uart::puts("    [W/S] Move  [Ctrl-A X] Quit QEMU   ");
}


fn draw_horizontal_border() {
    Uart::putc(b'+');
    for _ in 0..FIELD_W {
        Uart::putc(b'-');
    }
    Uart::putc(b'+');
}
