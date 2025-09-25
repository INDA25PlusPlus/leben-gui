use ggez::conf::{NumSamples, WindowMode, WindowSetup};
use crate::gui::GuiState;

mod gui;
mod resources;
mod util;

const APP_ID: &str = "leben-gui-chess";
const AUTHOR: &str = "Leonard Bengtsson";
const WINDOW_TITLE: &str = "And he sacrifices... THE ROOK";
const RESOURCES_PATH: &str = "res";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resource_path = std::env::var("CARGO_MANIFEST_DIR").map(|manifest_dir| {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push(RESOURCES_PATH);
        path
    })?;

    let window_mode = WindowMode::default()
        .dimensions(1200_f32, 800_f32)
        .resizable(true);
    let window_setup = WindowSetup::default()
        .title(WINDOW_TITLE)
        .icon(resources::WHITE_KING_IMAGE)
        .samples(NumSamples::Four);

    let (mut ctx, event_loop) = ggez::ContextBuilder::new(APP_ID, AUTHOR)
        .window_mode(window_mode)
        .window_setup(window_setup)
        .add_resource_path(resource_path)
        .build()
        .expect("failed to create game context");

    let gui_container = GuiState::new(&mut ctx)?;
    ggez::event::run(ctx, event_loop, gui_container);
}
