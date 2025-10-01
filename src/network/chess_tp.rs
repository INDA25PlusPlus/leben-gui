use rsoderh_chess::{Board, PieceKind, Position};

mod util;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameStateType {
    Normal,
    WhiteWon,
    BlackWon,
    Draw,
}

#[derive(Clone, Debug)]
struct MessageBuilder<const N: usize> {
    buffer: [u8; N],
    index: usize,
}

impl<const N: usize> MessageBuilder<N> {
    pub fn new(padding: Option<u8>) -> Result<MessageBuilder<N>, ()> {
        Ok(MessageBuilder { buffer: [padding.unwrap_or(0); N], index: 0, })
    }

    pub fn skip(mut self, steps: usize) -> Result<Self, ()> {
        if self.index + steps > N { return Err(()); }
        self.index += steps;
        Ok(self)
    }

    pub fn write(mut self, value: u8) -> Result<Self, ()> {
        if self.index + 1 > N { return Err(()); }
        self.buffer[self.index] = value;
        self.index += 1;
        Ok(self)
    }

    pub fn write_slice(mut self, section: &[u8]) -> Result<Self, ()> {
        if self.index + section.len() > N { return Err(()); }
        &self.buffer[self.index..(self.index + section.len())].copy_from_slice(section);
        self.index += section.len();
        Ok(self)
    }

    pub fn write_pos(mut self, pos: Position) -> Result<Self, ()> {
        let file = b'A' + pos.column.get();
        let rank = b'1' + pos.row.get();
        self.write_slice(&[file, rank])
    }

    pub fn write_promotion_type(mut self, promotion_type: Option<PieceKind>) -> Result<Self, ()> {
        match promotion_type {
            None => self.skip(1),
            Some(promotion_type) => {
                let ch = match promotion_type {
                    PieceKind::Pawn => b'p',
                    PieceKind::Knight => b'n',
                    PieceKind::Bishop => b'b',
                    PieceKind::Rook => b'r',
                    PieceKind::Queen => b'q',
                    PieceKind::King => b'k',
                };
                self.write(ch)
            }
        }
    }

    pub fn write_game_state(mut self, game_state: GameStateType) -> Result<Self, ()> {
        let state = match game_state {
            GameStateType::Normal => b"0-0",
            GameStateType::WhiteWon => b"1-0",
            GameStateType::BlackWon => b"0-1",
            GameStateType::Draw => b"1-1",
        };
        self.write_slice(state)
    }

    pub fn write_board(mut self, board: &Board) -> Result<Self, ()> {
        self.write_slice(&util::board_to_fen(board))
    }

    pub fn build(self) -> [u8; N] {
        self.buffer
    }
}

#[derive(Clone, Debug)]
pub struct MessageReader<'a> {
    buffer: &'a [u8],
}

impl<'a> MessageReader<'a> {
    pub fn new(buffer: &'a [u8]) -> MessageReader<'a> {
        MessageReader { buffer }
    }

    pub fn skip(&mut self, len: usize) -> Result<(), ()> {
        if self.buffer.len() < len { return Err(()); }
        self.buffer = &self.buffer[len..];
        Ok(())
    }

    pub fn read(&mut self) -> Result<u8, ()> {
        if self.buffer.len() < 1 { return Err(()); }
        let value = self.buffer[0];
        self.buffer = &self.buffer[1..];
        Ok(value)
    }

    pub fn check_and_skip(&mut self, value: u8) -> Result<(), ()> {
        if self.read()? == value { Ok(()) } else { Err(()) }
    }

    pub fn check_rest(&mut self, value: u8) -> Result<(), ()> {
        if self.buffer.iter().all(|b| *b == value) { Ok(()) } else { Err(()) }
    }

    pub fn read_slice(&mut self, len: usize) -> Result<&'a [u8], ()> {
        if self.buffer.len() < len { return Err(()); }
        let (before, after) = self.buffer.split_at(len);
        self.buffer = after;
        Ok(before)
    }

    pub fn read_up_to(&mut self, byte: u8) -> Result<&'a [u8], ()> {
        let len = self.buffer.iter().position(|b| *b == byte).ok_or(())?;
        self.read_slice(len)
    }

    pub fn read_pos(&mut self) -> Result<Position, ()> {
        let file = self.read()?;
        let rank = self.read()?;
        if !(b'A'..=b'H').contains(&file) { return Err(()); }
        if !(b'1'..=b'8').contains(&rank) { return Err(()); }
        Position::new(file - b'A', rank - b'1').ok_or(())
    }

    pub fn read_promotion_type(&mut self) -> Result<Option<PieceKind>, ()> {
        match self.read()? {
            b'p' | b'P' => Ok(Some(PieceKind::Pawn)),
            b'n' | b'N' => Ok(Some(PieceKind::Knight)),
            b'b' | b'B' => Ok(Some(PieceKind::Bishop)),
            b'r' | b'R' => Ok(Some(PieceKind::Rook)),
            b'q' | b'Q' => Ok(Some(PieceKind::Queen)),
            b'k' | b'K' => Ok(Some(PieceKind::King)),
            b'0' => Ok(None),
            _ => Err(())
        }
    }

    pub fn read_game_state(&mut self) -> Result<GameStateType, ()> {
        match self.read_slice(3)? {
            b"0-0" => Ok(GameStateType::Normal),
            b"1-0" => Ok(GameStateType::WhiteWon),
            b"0-1" => Ok(GameStateType::BlackWon),
            b"1-1" => Ok(GameStateType::Draw),
            _ => Err(())
        }
    }

    pub fn read_board_argument(&mut self) -> Result<Board, ()> {
        let fen = self.read_up_to(b':')?;
        util::board_from_fen(fen).ok_or(())
    }
}
