use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>
}

#[derive(Clone, Debug, Subcommand)]
pub enum Commands {
    /// Host a game server, playing as black
    Host {
        /// The local IPv4 address to bind the server to
        bind_address: String,

        /// Enforce a strict rule policy and reject invalid moves from the opponent
        #[arg(short, long)]
        strict: bool,
    },

    /// Join a game server, playing as white
    Join {
        /// The IPv4/IPv6 address of the server to join
        address: String,

        /// If true, reject invalid moves from the opponent
        #[arg(short, long)]
        strict: bool,
    },
}
