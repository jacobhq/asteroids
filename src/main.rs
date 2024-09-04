mod game;

use macroquad::prelude::*;
use game::asteroid::Asteroid;

struct Player {
    position: Vec2,
    rotation: f32,
    velocity: Vec2
}

struct Bullet {
    position: Vec2,
    velocity: Vec2,
    shot_at: f64,
    collided: bool,
}

enum CollisionTypes {
    Player(Player),
    Asteroid(Asteroid),
    Bullet(Bullet)
}

struct Collision {
    objects: Vec<CollisionTypes>
}

#[macroquad::main("asteroids")]
async fn main() {
    let mut asteroids = Vec::new();
    loop {
        if asteroids.len() < 10 {
            for _ in 0..10 {
                asteroids.push(Asteroid::new())
            }
        }
        for asteroid in asteroids.iter() {
            draw_poly_lines(
                asteroid.position.x,
                asteroid.position.y,
                asteroid.sides,
                asteroid.size,
                asteroid.rotation,
                2.,
                WHITE,
            )
        }
        for asteroid in asteroids.iter_mut() {
            asteroid.update_position()
        }
        next_frame().await;
    }
}
