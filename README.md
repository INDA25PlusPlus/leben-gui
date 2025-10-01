# leben-chess-gui

Chess GUI application written in Rust, using the [ggez](https://crates.io/crates/ggez) game framework and [rsoderh_chess](https://github.com/INDA25PlusPlus/rsoderh-chess) chess library.

## Features

- Move pieces by clicking their square, and then clicking their destination square
- Current game state is displayed underneath the board
- Supports promotion and castling moves
- Supports remote play via a TCP connection

## Usage

Run using `cargo run` or build using `cargo build`.

### Local play

To run the game locally, simply run the executable without passing any arguments.

### Remote play

To host a game server, run `EXE host [OPTIONS] <ADDRESS>`.
- Available `[OPTIONS]`: `-s` - Enforce a strict rule policy and reject invalid moves from the opponent
- `<ADDRESS>`: The local IPv4/IPv6 address to bind the server to

To join a remote game server, run `EXE join [OPTIONS] <ADDRESS>`.
- Available `[OPTIONS]`: `-s` - Enforce a strict rule policy and reject invalid moves from the opponent
- `<ADDRESS>`: The IPv4/IPv6 address of the server to join
