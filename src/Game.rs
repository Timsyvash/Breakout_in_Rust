use piston_window::{clear, rectangle};
use piston_window::*;

pub struct Blocks {
    pub pos_x: f64,
    pub pos_y: f64,
    pub is_exists: bool,
    pub width: f64,
    pub height: f64,
}

pub struct Game {
    pub pause: bool,
    pub pos_x: f64,
    pub pos_y: f64,
    pub ball_x: f64,
    pub ball_y: f64,
    pub ball_dx: f64,
    pub ball_dy: f64,
    pub width_r: f64,
    pub height_r: f64,
    pub ball_width: f64,
    pub ball_height: f64,
    pub blocks: Vec<Blocks>
}

impl Game {
    pub fn render(&mut self, c: Context, g: &mut G2d) {
        const COLOR1: [f32; 4] = [0.0, 0.9, 0.8, 1.0];
        const COLOR2: [f32; 4] = [0.8, 0.2, 1.0, 1.0];
        const COLOR3: [f32; 4] = [0.2, 0.1, 0.0, 1.0];
        clear(COLOR1, g);

        for block in &self.blocks {
            if block.is_exists {
                rectangle(
                    [1.0, 0.8, 0.6, 1.0],
                    [block.pos_x, block.pos_y, block.width, block.height],
                    c.transform,
                    g,
                );
            }
        }

        rectangle(
            COLOR2,
            [self.pos_x, self.pos_y, self.width_r, self.height_r],
            c.transform,
            g,
        );

        rectangle(
            COLOR3,
            [self.ball_x, self.ball_y, self.ball_width, self.ball_height],
            c.transform,
            g,
        );
    }

    pub fn game_update(&mut self) {
        if self.pause {
            return;
        }

        self.ball_x += self.ball_dx;
        self.ball_y += self.ball_dy;

        let r_x = self.ball_x + self.ball_width >= self.pos_x &&
            self.ball_x <= self.pos_x + self.width_r;
        let r_y = self.ball_y + self.ball_height >= self.pos_y &&
            self.ball_y <= self.pos_y + self.height_r;

        if r_x && r_y && self.ball_dy > 0.0 {
            self.ball_y = self.pos_y - self.ball_height;
            self.ball_dy = -self.ball_dy;
        }

        if self.ball_x <= 0.0 || self.ball_x + self.ball_width >= 400.0 {
            self.ball_dx = -self.ball_dx;
        }

        if self.ball_y <= 0.0 {
            self.ball_dy = -self.ball_dy;
        }

        for block in &mut self.blocks {
            if block.is_exists
                && Game::collision_ball(
                self.ball_x,
                self.ball_y,
                self.ball_width,
                self.ball_height,
                block,
            )
            {
                block.is_exists = false;
                self.ball_dy = -self.ball_dy;
                break;
            }
        }

        if self.ball_y >= 300.0 {
            self.ball_y = 250.0;
            self.ball_x = 200.0;
            self.ball_dx = 2.0;
            self.ball_dy = -2.0;
        }
    }

    pub fn move_keys(&mut self, key: Key) {
        if key == Key::D {
            self.pos_x += 10.0;
        } else if key == Key::A {
            self.pos_x -= 10.0;
        } else {
            return;
        }
    }

    fn collision_ball(
        ball_x: f64,
        ball_y: f64,
        ball_w: f64,
        ball_h: f64,
        b: &Blocks,
    ) -> bool {
        let cx = ball_x + ball_w / 2.0;
        let cy = ball_y + ball_h / 2.0;

        let closest_x = cx.clamp(b.pos_x, b.pos_x + b.width);
        let closest_y = cy.clamp(b.pos_y, b.pos_y + b.height);

        let dx = cx - closest_x;
        let dy = cy - closest_y;

        let radius = ball_w / 2.0;
        (dx * dx + dy * dy) < radius * radius
    }
}
