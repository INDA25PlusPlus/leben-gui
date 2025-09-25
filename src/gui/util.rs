use rsoderh_chess::{Color, PieceKind, Position};

const MIN_SIZE_IN_SQUARES: f32 = 12_f32;
const MIN_SQUARE_SIZE: f32 = 40_f32;
const MAX_SQUARE_SIZE: f32 = 100_f32;

pub fn square_size(ctx: &ggez::Context) -> f32 {
    let screen_size = ctx.gfx.drawable_size();
    let square_size = f32::min(
        screen_size.0 / MIN_SIZE_IN_SQUARES,
        screen_size.1 / MIN_SIZE_IN_SQUARES,
    );
    f32::min(f32::max(square_size, MIN_SQUARE_SIZE), MAX_SQUARE_SIZE)
}

fn center_to_global_pos(ctx: &ggez::Context, offset: (f32, f32)) -> (f32, f32) {
    let screen_size = ctx.gfx.drawable_size();
    (offset.0 + screen_size.0 * 0.5_f32, offset.1 + screen_size.1 * 0.5_f32)
}

fn global_to_center_pos(ctx: &ggez::Context, pos: (f32, f32)) -> (f32, f32) {
    let screen_size = ctx.gfx.drawable_size();
    (pos.0 - screen_size.0 * 0.5_f32, pos.1 - screen_size.1 * 0.5_f32)
}

pub fn global_to_board_offset_pos(ctx: &ggez::Context, mouse_pos: (f32, f32)) -> Option<(i8, i8)> {
    let relative = global_to_center_pos(ctx, mouse_pos);
    let square_size = square_size(ctx);
    let x = relative.0 / square_size + 4_f32;
    let y = relative.1 / -square_size + 4_f32;
    let valid_range = -128_f32..=127_f32;
    if valid_range.contains(&x) && valid_range.contains(&y) {
        Some((x.floor() as i8, y.floor() as i8))
    } else {
        None
    }
}

pub fn global_to_board_pos(ctx: &ggez::Context, mouse_pos: (f32, f32)) -> Option<Position> {
    let relative = global_to_board_offset_pos(ctx, mouse_pos)?;
    let index_range = 0i8..=7;
    if index_range.contains(&relative.0) && index_range.contains(&relative.1) {
        Position::new(relative.0 as u8, relative.1 as u8)
    } else {
        None
    }
}

pub fn promotion_selection_iter(to_play: Color, square: Position)
                                -> impl Iterator<Item=(Position, PieceKind)>
{
    use PieceKind::*;
    let piece_types = [Queen, Knight, Rook, Bishop];

    let ranks = match to_play {
        Color::White => [7, 6, 5, 4],
        Color::Black => [0, 1, 2, 3],
    };
    let file = square.column.get();
    ranks.into_iter()
        .map(move |rank| Position::from_pair((file, rank)).unwrap())
        .zip(piece_types.into_iter())
}

pub fn promotion_selection_type(turn: Color, promotion_square: Position,
                                square: Position) -> Option<PieceKind>
{
    let mut iter = promotion_selection_iter(turn, promotion_square);
    iter.find_map(|(pos, piece_type)| (pos == square).then_some(piece_type))
}
