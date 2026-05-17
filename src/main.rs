#![allow(warnings)]

use macroquad::prelude::*;

mod data;
mod engine;
mod game;
mod save;
mod state;
mod ui;

use game::Game;

fn window_conf() -> Conf {
    Conf {
        window_title: "Heavenly Mandate".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;

    loop {
        clear_background(Color::from_rgba(10, 10, 10, 255));

        game.update();
        game.draw();

        next_frame().await;
    }
}
