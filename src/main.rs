mod game;

use macroquad::prelude::*;
use game::asteroid::{Asteroid, AsteroidPosition};
use game::player::Player;
use crate::game::player::PLAYER_HEIGHT;

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
                asteroids.push(Asteroid::new(AsteroidPosition::Center))
            }
        }
        for asteroid in asteroids.iter() {
            asteroid.draw()
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

        let mut new_asteroids = Vec::new();
        for asteroid in asteroids.iter_mut() {
            // Asteroid/ship collision
            if (asteroid.position - player.position).length() < asteroid.size + PLAYER_HEIGHT / 3. {
                player.die();
                break;
            }

            // Asteroid/bullet collision
            for bullet in bullets.iter_mut() {
                if (asteroid.position - bullet.position).length() < asteroid.size {
                    asteroid.collided = true;
                    bullet.collided = true;

                    // Break the asteroid
                    asteroid.get_hit_and_split(bullet, &mut new_asteroids);
                    break;
                }
            }
        }

        // Remove the collided objects
        bullets.retain(|bullet| bullet.shot_at + 5.0 > frame_t && !bullet.collided);
        asteroids.retain(|asteroid| !asteroid.collided);
        asteroids.append(&mut new_asteroids);


        next_frame().await;
    }
}
