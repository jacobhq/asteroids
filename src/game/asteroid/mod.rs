use macroquad::prelude::*;
use super::utils::wrap_around;

pub struct Asteroid {
    pub(crate) position: Vec2,
    pub(crate) rotation: f32,
    rotation_speed: f32,
    velocity: Vec2,
    pub(crate) sides: u8,
    pub(crate) size: f32
}

impl Asteroid {
    pub(crate) fn new() -> Self {
        let screen_center = Vec2::new(screen_width() / 2., screen_height() / 2.);

        Asteroid {
            position: screen_center
                + Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.))
                .normalize()
                * screen_width().min(screen_height())
                / 2.,
            rotation: 0.0,
            rotation_speed: rand::gen_range(-2., 2.),
            velocity: Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)),
            sides: rand::gen_range(3, 8),
            size: screen_width().min(screen_height()) / 10.
        }
    }

    pub(crate) fn update_position(&mut self) {
        self.position += self.velocity;
        self.position = wrap_around(&self.position);
        self.rotation += self.rotation_speed;
    }
}