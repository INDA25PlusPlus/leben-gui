use std::vec;

use rsoderh_chess::{Board, Color, Piece, PieceKind, Position, Slot};

fn piece_to_char(piece: Piece) -> u8 {
    match (piece.color, piece.kind) {
        (Color::White, PieceKind::Pawn) => b'P',
        (Color::White, PieceKind::Knight) => b'N',
        (Color::White, PieceKind::Bishop) => b'B',
        (Color::White, PieceKind::Rook) => b'R',
        (Color::White, PieceKind::Queen) => b'Q',
        (Color::White, PieceKind::King) => b'K',
        (Color::Black, PieceKind::Pawn) => b'p',
        (Color::Black, PieceKind::Knight) => b'n',
        (Color::Black, PieceKind::Bishop) => b'b',
        (Color::Black, PieceKind::Rook) => b'r',
        (Color::Black, PieceKind::Queen) => b'q',
        (Color::Black, PieceKind::King) => b'k',
    }
}

fn piece_from_char(byte: u8) -> Option<Piece> {
    match byte {
        b'P' => Some(Piece { kind: PieceKind::Pawn, color: Color::White }),
        b'N' => Some(Piece { kind: PieceKind::Knight, color: Color::White }),
        b'B' => Some(Piece { kind: PieceKind::Bishop, color: Color::White }),
        b'R' => Some(Piece { kind: PieceKind::Rook, color: Color::White }),
        b'Q' => Some(Piece { kind: PieceKind::Queen, color: Color::White }),
        b'K' => Some(Piece { kind: PieceKind::King, color: Color::White }),
        b'p' => Some(Piece { kind: PieceKind::Pawn, color: Color::Black }),
        b'n' => Some(Piece { kind: PieceKind::Knight, color: Color::Black }),
        b'b' => Some(Piece { kind: PieceKind::Bishop, color: Color::Black }),
        b'r' => Some(Piece { kind: PieceKind::Rook, color: Color::Black }),
        b'q' => Some(Piece { kind: PieceKind::Queen, color: Color::Black }),
        b'k' => Some(Piece { kind: PieceKind::King, color: Color::Black }),
        _ => None,
    }
}

pub fn board_to_fen(board: &Board) -> Vec<u8> {
    let mut empty_counter = 0;
    let mut vec = Vec::new();
    for rank in (0..8).rev() {
        for file in 0..8 {
            let pos = Position::new(file, rank).unwrap();
            let square = board.at_position(pos);
            match square {
                Slot::Empty => {
                    empty_counter += 1;
                },
                Slot::Occupied(piece) => {
                    if empty_counter > 0 {
                        vec.push(char::from_digit(empty_counter, 9).unwrap() as u8)
                    }
                    empty_counter = 0;
                    vec.push(piece_to_char(piece));
                },
            }
        }
        if empty_counter > 0 {
            vec.push(char::from_digit(empty_counter, 9).unwrap() as u8)
        }
        empty_counter = 0;
        if rank != 0 {
            vec.push(b'/');
        }
    }
    vec
}

pub fn board_from_fen(fen: &[u8]) -> Option<Board> {
    let mut board = Board::new_empty();
    let mut file = 0;
    let mut rank = 0;
    for byte in fen {
        if let Some(piece) = piece_from_char(*byte) {
            if file >= 8 || rank >= 8 {
                return None;
            }
            let pos = Position::new(file, 7 - rank).unwrap();
            *board.at_position_mut(pos) = Slot::Occupied(piece);
            file += 1;
        } else if (b'1'..=b'8').contains(byte) {
            let digit = byte - b'1' + 1;
            if digit as u8 + file > 8 {
                return None;
            }
            file += digit as u8;
        } else if *byte == b'/' {
            if file != 8 || rank > 6 {
                return None;
            }
            file = 0;
            rank += 1;
        } else {
            return None;
        }
    }
    if file != 8 || rank != 7 {
        return None;
    }
    Some(board)
}