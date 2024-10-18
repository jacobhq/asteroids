use macroquad::prelude::*;
use crate::game::asteroid::{Asteroid, AsteroidPosition};
use crate::game::bullet::Bullet;
use crate::game::player::{Player, PLAYER_HEIGHT};

pub mod utils;
pub mod asteroid;
pub mod player;
pub mod bullet;
mod collision_handler;

pub enum GameType {
    Zen,
    Live,
}

pub struct AsteroidsGame {
    id: String,
    player: Player,
    bullets: Vec<Bullet>,
    asteroids: Vec<Asteroid>,
    score: u16,
    game_type: GameType,
}

pub enum GameOutcome {
    Quit,
    Lose,
    Continue,
    ScorePoint
}

impl AsteroidsGame {
    pub fn new(id: String, game_type: GameType) -> Self {
        AsteroidsGame {
            id,
            player: Player::new(),
            bullets: Vec::new(),
            asteroids: Vec::new(),
            score: 0,
            game_type,
        }
    }

    pub fn play(&mut self) -> GameOutcome {
        if is_key_down(KeyCode::Escape) {
            return GameOutcome::Quit;
        }

        let mut last_shot = get_time();

        self.player.move_from_keys(&mut self.bullets, &mut last_shot);
        self.player.draw();

        let text: &str = &format!("{}", self.score);
        let font_size = 64.;

        let text_size = measure_text(text, None, font_size as _, 1.0);
        draw_text(text, 16., 16. + text_size.height, font_size, WHITE);

        let frame_t = get_time();

        self.bullets.retain(|bullet| bullet.shot_at + 1.5 > frame_t);

        if self.asteroids.len() < 10 {
            for _ in 0..10 {
                self.asteroids.push(Asteroid::new(AsteroidPosition::Center))
            }
        }
        for asteroid in self.asteroids.iter() {
            asteroid.draw()
        }
        for asteroid in self.asteroids.iter_mut() {
            asteroid.update_position()
        }

        for bullet in self.bullets.iter() {
            bullet.draw();
        }
        for bullet in self.bullets.iter_mut() {
            bullet.update_position();
        }

        let mut new_asteroids = Vec::new();
        for asteroid in self.asteroids.iter_mut() {
            // Asteroid/ship collision
            if (asteroid.position - self.player.position).length() < asteroid.size + PLAYER_HEIGHT / 3. {
                return GameOutcome::Lose;
            }

            // Asteroid/bullet collision
            for bullet in self.bullets.iter_mut() {
                if (asteroid.position - bullet.position).length() < asteroid.size {
                    asteroid.collided = true;
                    bullet.collided = true;

                    // Break the asteroid
                    asteroid.get_hit_and_split(bullet, &mut new_asteroids);
                    
                    // Remove the collided objects
                    self.bullets.retain(|bullet| bullet.shot_at + 1.5 > frame_t && !bullet.collided);
                    self.asteroids.retain(|asteroid| !asteroid.collided);
                    self.asteroids.append(&mut new_asteroids);

                    return GameOutcome::ScorePoint;
                }
            }
        }

        GameOutcome::Continue
    }

    pub fn increment_score(&mut self) {
        self.score += 1;
    }
}