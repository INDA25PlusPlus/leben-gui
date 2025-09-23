#![allow(dead_code)]
#![allow(warnings)]

use ggez::conf::{WindowMode, WindowSetup};
use crate::game::GameContainer;

mod game;

const APP_ID: &str = "leben-gui-chess";
const AUTHOR: &str = "Leonard Bengtsson";
const WINDOW_TITLE: &str = "And he sacrifices... THE ROOK";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let window_mode = WindowMode::default()
        .resizable(true);
    let window_setup = WindowSetup::default()
        .title(WINDOW_TITLE);

    let (mut ctx, event_loop) = ggez::ContextBuilder::new(APP_ID, AUTHOR)
        .window_mode(window_mode)
        .window_setup(window_setup)
        .build()
        .expect("failed to create game context");

    let game_container = GameContainer::new(&mut ctx)?;
    ggez::event::run(ctx, event_loop, game_container);
}
