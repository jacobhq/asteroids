mod game;

use std::{thread, time};
use macroquad::prelude::*;
use game::asteroid::{Asteroid, AsteroidPosition};
use game::player::Player;
use crate::game::player::PLAYER_HEIGHT;
use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Group},
    Drag, Ui,
};
use macroquad::ui::widgets::Button;

enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver,
}

# [macroquad::main("asteroids")]
async fn main() {
    let mut player = Player::new();
    let mut bullets = Vec::new();
    let mut asteroids = Vec::new();
    let mut last_shot = get_time();
    let mut score: u32 = 0;
    let mut game_state: GameState = GameState::MainMenu;

    loop {
        match game_state {
            GameState::Playing => {
                player.move_from_keys(&mut bullets, &mut last_shot);
                player.draw();

                {
                    let text: &str = &format!("{}", score);
                    let font_size = 64.;

                    let text_size = measure_text(text, None, font_size as _, 1.0);
                    draw_text(text, 16., 16. + text_size.height, font_size, WHITE);
                }

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
                        game_state = GameState::GameOver;
                        break;
                    }

                    // Asteroid/bullet collision
                    for bullet in bullets.iter_mut() {
                        if (asteroid.position - bullet.position).length() < asteroid.size {
                            asteroid.collided = true;
                            bullet.collided = true;

                            score += 1;

                            // Break the asteroid
                            asteroid.get_hit_and_split(bullet, &mut new_asteroids);
                            break;
                        }
                    }
                }

                // Remove the collided objects
                bullets.retain(|bullet| bullet.shot_at + 1.5 > frame_t && !bullet.collided);
                asteroids.retain(|asteroid| !asteroid.collided);
                asteroids.append(&mut new_asteroids);

            }
            GameState::GameOver => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::MainMenu;
                }

                clear_background(LIGHTGRAY);
                let text = "You Died!. Press [space] to play again.";
                let font_size = 30.;

                let text_size = measure_text(text, None, font_size as _, 1.0);
                draw_text(
                    text,
                    screen_width() / 2. - text_size.width / 2.,
                    screen_height() / 2. - text_size.height / 2.,
                    font_size,
                    DARKGRAY,
                );
            }
            _ => {
                widgets::Window::new(hash!(), vec2(100., 220.), vec2(542., 430.))
                    .label("Asteroids Rust Implementation - Jacob Marshall")
                    .titlebar(true)
                    .ui(&mut root_ui(), |ui| {
                        if Button::new("Play Game").ui(ui) {
                            asteroids.clear();
                            bullets.clear();
                            player.reset();
                            score = 0;
                            game_state = GameState::Playing;
                        }
                    });
            }
        }

        next_frame().await;
    }
}
