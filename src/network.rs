pub mod chess_tp;

use std::io::{Read, Write};
use std::net::TcpStream;
use rsoderh_chess::Color;
use crate::network::chess_tp::Message;

#[derive(Debug)]
pub struct GameConnection {
    connection: TcpStream,
    local_player: Color,
    strict_rule_policy: bool,
}

impl GameConnection {
    pub fn new(connection: TcpStream, local_player: Color, strict_rule_policy: bool) -> GameConnection {
        GameConnection {
            connection,
            local_player,
            strict_rule_policy,
        }
    }

    pub fn send_message(&mut self, message: Message) -> Result<(), ()> {
        let message = message.encode()?;
        self.connection.write(&message).map(|_| ()).map_err(|_| ())
    }

    pub fn read_message(&mut self) -> Result<Message, ()> {
        let mut buffer = [0u8; chess_tp::BUFFER_SIZE];
        self.connection.read(&mut buffer).map_err(|_| ())?;
        Message::decode(&buffer)
    }

    pub fn local_player(&self) -> Color {
        self.local_player
    }

    pub fn strict_rule_policy(&self) -> bool {
        self.strict_rule_policy
    }
}
