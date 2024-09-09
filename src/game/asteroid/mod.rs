use macroquad::prelude::*;
use crate::game::bullet::Bullet;
use super::utils::wrap_around;

pub struct Asteroid {
    pub(crate) position: Vec2,
    pub(crate) rotation: f32,
    rotation_speed: f32,
    velocity: Vec2,
    pub(crate) sides: u8,
    pub(crate) size: f32,
    pub(crate) collided: bool
}

pub enum AsteroidPosition {
    Center,
    Arbitrary(Vec2)
}

impl Asteroid {
    pub(crate) fn new(position: AsteroidPosition) -> Self {
        let screen_center = Vec2::new(screen_width() / 2., screen_height() / 2.);

        Asteroid {
            position: match position {
                AsteroidPosition::Center => screen_center
                    + Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.))
                    .normalize()
                    * screen_width().min(screen_height())
                    / 2.,
                AsteroidPosition::Arbitrary(position) => position
            },
            rotation: 0.0,
            rotation_speed: rand::gen_range(-2., 2.),
            velocity: Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)),
            sides: rand::gen_range(3, 8),
            size: screen_width().min(screen_height()) / 10.,
            collided: false
        }
    }
    
    pub(crate) fn draw(&self) {
        draw_poly_lines(
            self.position.x,
            self.position.y,
            self.sides,
            self.size,
            self.rotation,
            2.,
            WHITE,
        )
    }
    
    pub(crate) fn get_hit_and_split(&mut self, bullet: &mut Bullet, new_asteroids: &mut Vec<Asteroid>) {
        if (self.position - bullet.position).length() < self.size {
            self.collided = true;
            bullet.collided = true;

            // Break the asteroid
            if self.sides > 3 {
                new_asteroids.push(Asteroid {
                    position: self.position,
                    velocity: Vec2::new(bullet.velocity.y, -bullet.velocity.x).normalize()
                        * rand::gen_range(1., 3.),
                    rotation: rand::gen_range(0., 360.),
                    rotation_speed: rand::gen_range(-2., 2.),
                    size: self.size * 0.8,
                    sides: self.sides - 1,
                    collided: false,
                });
                new_asteroids.push(Asteroid {
                    position: self.position,
                    velocity: Vec2::new(-bullet.velocity.y, bullet.velocity.x).normalize()
                        * rand::gen_range(1., 3.),
                    rotation: rand::gen_range(0., 360.),
                    rotation_speed: rand::gen_range(-2., 2.),
                    size: self.size * 0.8,
                    sides: self.sides - 1,
                    collided: false,
                })
            }
        }
    }

    pub(crate) fn update_position(&mut self) {
        self.position += self.velocity;
        self.position = wrap_around(&self.position);
        self.rotation += self.rotation_speed;
    }
}