mod game;
mod api;

use std::{thread, time};
use macroquad::prelude::*;
use game::asteroid::{Asteroid, AsteroidPosition};
use game::player::Player;
use crate::game::player::PLAYER_HEIGHT;
use macroquad::ui::{hash, root_ui, widgets::{self, Group}, Drag, Skin, Ui};
use macroquad::ui::widgets::Button;
use crate::api::score_point;
use crate::game::{AsteroidsGame, GameOutcome, GameType};

enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver,
    LiveMode,
}

#[macroquad::main("asteroids")]
async fn main() {
    let mut game_state: GameState = GameState::MainMenu;
    let mut game_id: String = "".to_string();
    let mut player_id: String = "".to_string();
    let mut game: Option<AsteroidsGame> = None;

    loop {
        match game_state {
            GameState::MainMenu => {
                let window_size = vec2(542., 430.);
                let button_size = vec2(window_size.x - 20., 50.);

                widgets::Window::new(hash!(), vec2(
                    screen_width() / 2.0 - window_size.x / 2.0,
                    screen_height() / 2.0 - window_size.y / 2.0,
                ), window_size)
                    .label("Asteroids Rust Implementation - Jacob Marshall")
                    .titlebar(true)
                    .ui(&mut root_ui(), |ui| {
                        if Button::new("Play Game").size(button_size).position(vec2(
                            10., 10.,
                        )).ui(ui) {
                            game = Some(AsteroidsGame::new(game_id.clone(), GameType::Zen));
                            game_state = GameState::Playing;
                        }
                    });
            },
            GameState::Playing => {
                if let Some(ref mut game_instance) = game {
                    match AsteroidsGame::play(game_instance) {
                        GameOutcome::Lose => game_state = GameState::GameOver,
                        GameOutcome::Quit => game_state = GameState::MainMenu,
                        GameOutcome::ScorePoint => {
                            score_point(&game_id, &player_id);
                            game_instance.increment_score()
                        },
                        GameOutcome::Continue => ()
                    }
                } else {
                    // This should never happen, but just in case:
                    game_state = GameState::MainMenu;
                }
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
            _ => ()
        }

        next_frame().await;
    }
}
