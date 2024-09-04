mod game;

use macroquad::prelude::*;
use game::asteroid::Asteroid;
use game::bullet::Bullet;
use game::player::Player;

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
    let mut player = Player::new();
    let mut bullets = Vec::new();
    let mut asteroids = Vec::new();
    let mut last_shot = get_time();

    loop {
        player.move_from_keys(&mut bullets, &mut last_shot);
        player.draw();

        let frame_t = get_time();

        bullets.retain(|bullet| bullet.shot_at + 1.5 > frame_t);

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
        
        for bullet in bullets.iter() {
            bullet.draw();
        }
        for bullet in bullets.iter_mut() {
            bullet.update_position();
        }

        next_frame().await;
    }
}
