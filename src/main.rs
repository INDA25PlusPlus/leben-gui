#![allow(dead_code)]
#![allow(warnings)]

use std::error::Error;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use ggez::conf::{NumSamples, WindowMode, WindowSetup};
use crate::args::{Cli, Commands};
use crate::gui::GuiState;
use crate::network::GameConnection;
use rsoderh_chess::Color;

mod gui;
mod resources;
mod util;
mod network;
mod args;

const APP_ID: &str = "leben-chess-gui";
const AUTHOR: &str = "Leonard Bengtsson";
const WINDOW_TITLE: &str = "And he sacrifices... THE ROOK";
const RESOURCES_PATH: &str = "res";

fn main() {
    let cli = <Cli as clap::Parser>::parse();
    let connection = if let Some(subcommand) = cli.command {
        match subcommand {
            Commands::Host {
                strict,
                bind_address
            } => {
                let listener = TcpListener::bind(&bind_address)
                    .expect("failed to bind address");
                println!("Waiting for other player...");
                let (stream, other_address) = listener.accept()
                    .expect("failed to accept connection");
                stream.set_nonblocking(true)
                    .expect("failed to set stream to non-blocking");
                println!("Connected to {other_address}");
                Some(GameConnection::new(stream, Color::Black, strict))
            },
            Commands::Join { strict, address } => {
                println!("Connecting to {}...", &address);
                let stream = TcpStream::connect(&address)
                    .expect("failed to connect to remote host");
                stream.set_nonblocking(true)
                    .expect("failed to set stream to non-blocking");
                println!("Connected to {}", &address);
                Some(GameConnection::new(stream, Color::White, strict))
            },
        }
    } else {
        None
    };

    let resource_path = std::env::var("CARGO_MANIFEST_DIR").map(|manifest_dir| {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push(RESOURCES_PATH);
        path
    }).expect("env variable `CARGO_MANIFEST_DIR` should be present");

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

    let gui_state = if let Some(connection) = connection {
        GuiState::new_remote(&mut ctx, connection).expect("failed to setup GUI")
    } else {
        GuiState::new_local(&mut ctx).expect("failed to setup GUI")
    };
    ggez::event::run(ctx, event_loop, gui_state);
}
