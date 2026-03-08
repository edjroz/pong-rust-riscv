use crate::uart::Uart;

// Field dimensions
pub const FIELD_W: i32 = 60;
pub const FIELD_H: i32 = 20;
pub const PADDLE_H: i32 = 4;
pub const PADDLE_X_LEFT: i32 = 2;
pub const PADDLE_X_RIGHT: i32 = FIELD_W -1;


pub struct GameState {
    // Ball 
    pub ball_x: i32,
    pub ball_y: i32,
    pub ball_dx: i32,
    pub ball_dy: i32,
    
    //Paddles
    pub left_y: i32,
    pub right_y: i32,
    
    // Scores
    pub left_score: i32,
    pub right_score: i32,

    // Ball speed control
    tick: u32,

}

impl GameState {
    pub fn new() -> Self {
        Self {
            ball_x: FIELD_W / 2,
            ball_y: FIELD_H / 2,
            ball_dx: 1,
            ball_dy: 1,
            left_y: FIELD_H / 2 - PADDLE_H / 2,
            right_y: FIELD_H / 2 - PADDLE_H / 2,
            left_score: 0,
            right_score: 0,
            tick: 0,
        }
    }

    pub fn handle_input(&mut self) {
        while let Some(input) = Uart::try_getc() {
            match input {
              b'w'|b'W' => {
                    if self.left_y > 1 {
                        self.left_y -= 1;
                    }
                }        
              b's'|b'S' => {
                    if self.left_y + PADDLE_H < FIELD_H {
                        self.left_y += 1;
                    }
                }
            _ => {}
            }
        }
    }
    pub fn update_ball(&mut self) {
        self.tick += 1;
        if self.tick % 3 != 0 {
            return;
        }

        let next_x = self.ball_x + self.ball_dx;
        let next_y = self.ball_y + self.ball_dy;

        // Top/Bottom Wall Bounce
        if next_y <= 1 || next_y >= FIELD_H {
            self.ball_dy = -self.ball_dy;
        }

        // Left paddle Bounce
        if next_x <= PADDLE_X_LEFT +1  && self.ball_y >= self.left_y && self.ball_y < self.left_y + PADDLE_H {
            self.ball_dx = self.ball_dx.abs();
            let hit_pos = self.ball_y - self.left_y;
            self.ball_dy = hit_pos - PADDLE_H / 2;
            if self.ball_dy == 0 {self.ball_dy = 1;}
        }

        //Right Paddle Bounce
        if next_x >= PADDLE_X_RIGHT-1 && self.ball_y >= self.right_y && self.ball_y <self.right_y + PADDLE_H {
            self.ball_dx = -(self.ball_dx.abs());
            let hit_pos = self.ball_y - self.right_y;
            self.ball_dy = hit_pos - PADDLE_H / 2;
            if self.ball_dy == 0 { self.ball_dy = -1;}
        }

        // Scoring - ball passed a paddle
        if next_x <= 0 {
            self.right_score += 1;
            self.reset_ball(-1);
            return;
        }
        if next_x >= FIELD_W +1 {
            self.left_score += 1;
            self.reset_ball(1);
            return;
        }

        self.ball_x += self.ball_dx;
        self.ball_y += self.ball_dy;

        // Clamp Bounds
        if self.ball_y < 1 { self.ball_y = 1;}
        if self.ball_y > FIELD_H { self.ball_y = FIELD_H;}
    }

    fn reset_ball(&mut self, direction: i32) {
        self.ball_x = FIELD_W / 2;
        self.ball_y = FIELD_H / 2;
        self.ball_dx = direction;
        self.ball_dy = 1;
    }
    pub fn update_ai(&mut self) {
        let paddle_center = self.right_y + PADDLE_H / 2;
        if self.ball_y < paddle_center && self.right_y > 1 {
            self.right_y -= 1;
        } else if self.ball_y > paddle_center && self.right_y + PADDLE_H < FIELD_H {
            self.right_y += 1;
        }
    }

}


