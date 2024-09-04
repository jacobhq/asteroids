use macroquad::prelude::*;

pub struct Bullet {
    position: Vec2,
    velocity: Vec2,
    pub shot_at: f64,
    collided: bool,
}

impl Bullet {
    pub fn new(position: Vec2, rot_vec: Vec2, frame_t: f64, player_height: f32) -> Self {
        Bullet {
            position: position + rot_vec * player_height / 2.,
            velocity: rot_vec * 7.,
            shot_at: frame_t,
            collided: false,
        }
    }

    pub fn update_position(&mut self) {
        self.position += self.velocity;
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, 2., WHITE);
    }
}