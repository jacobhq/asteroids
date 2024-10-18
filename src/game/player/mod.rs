use macroquad::prelude::*;
use crate::game::utils::wrap_around;
use crate::game::bullet::Bullet;

pub const PLAYER_HEIGHT: f32 = 25.;
pub const PLAYER_BASE: f32 = 22.;

pub struct Player {
    pub position: Vec2,
    pub rotation: f32,
    pub velocity: Vec2
}

impl Player {
    pub fn new() -> Self {
        Player {
            position: Vec2::new(screen_width() / 2., screen_height() / 2.),
            rotation: 0.,
            velocity: Vec2::new(0., 0.),
        }
    }

    pub fn draw(&self) {
        let rotation = self.rotation.to_radians();
        let v1 = Vec2::new(
            self.position.x + rotation.sin() * PLAYER_HEIGHT / 2.,
            self.position.y - rotation.cos() * PLAYER_HEIGHT / 2.,
        );
        let v2 = Vec2::new(
            self.position.x - rotation.cos() * PLAYER_BASE / 2. - rotation.sin() * PLAYER_HEIGHT / 2.,
            self.position.y - rotation.sin() * PLAYER_BASE / 2. + rotation.cos() * PLAYER_HEIGHT / 2.,
        );
        let v3 = Vec2::new(
            self.position.x + rotation.cos() * PLAYER_BASE / 2. - rotation.sin() * PLAYER_HEIGHT / 2.,
            self.position.y + rotation.sin() * PLAYER_BASE / 2. + rotation.cos() * PLAYER_HEIGHT / 2.,
        );
        draw_triangle_lines(v1, v2, v3, 2., WHITE);
    }

    pub fn move_from_keys(&mut self, bullets: &mut Vec<Bullet>, last_shot: &mut f64) {
        let mut acceleration = -self.velocity / 50.;
        let rotation = self.rotation.to_radians();
        let frame_t = get_time();

        if is_key_down(KeyCode::Up) {
            acceleration = Vec2::new(rotation.sin(), -rotation.cos()) / 3.;
        }
        
        if is_key_down(KeyCode::Space) && frame_t - *last_shot > 0.000001 {
            let rot_vec = Vec2::new(rotation.sin(), -rotation.cos());
            bullets.push(Bullet::new(self.position, rot_vec, frame_t, PLAYER_HEIGHT));
            *last_shot = frame_t;
        }

        if is_key_down(KeyCode::Right) {
            self.rotation += 5.;
        } else if is_key_down(KeyCode::Left) {
            self.rotation -= 5.;
        }

        // Euler integration
        self.velocity += acceleration;
        if self.velocity.length() > 5. {
            self.velocity = self.velocity.normalize() * 5.;
        }
        self.position += self.velocity;
        self.position = wrap_around(&self.position);
    }

    pub fn reset(&mut self) {
        self.position = Vec2::new(screen_width() / 2., screen_height() / 2.);
        self.rotation = 0.;
        self.velocity = Vec2::new(0., 0.);
    }
}