#![allow(warnings)]

use macroquad::prelude::*;
use macroquad_toolkit::capture;

mod data;
mod engine;
mod game;
mod save;
mod state;
mod ui;

use game::Game;

fn window_conf() -> Conf {
    capture::capture_window_conf("CULTIVATION", "Heavenly Mandate", 1280, 720)
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;

    // Screenshot harness: when CULTIVATION_CAPTURE_PATH is set, seed a scene,
    // simulate deterministic frames, write a PNG, and exit.
    if let Some(config) = capture::CaptureConfig::from_env("CULTIVATION") {
        game.begin_capture_scene(&config.scene);
        capture::run_capture(&config, |_dt| {
            clear_background(Color::from_rgba(10, 10, 10, 255));
            game.update();
            game.draw();
        })
        .await;
        return;
    }

    loop {
        clear_background(Color::from_rgba(10, 10, 10, 255));

        game.update();
        game.draw();

        next_frame().await;
    }
}
