use rsoderh_chess::{Color, PieceKind, Position};

const MIN_SIZE_IN_SQUARES: f32 = 12_f32;
const MIN_SQUARE_SIZE: f32 = 40_f32;
const MAX_SQUARE_SIZE: f32 = 100_f32;

/// returns: The calculated current board square size, in pixels
pub fn square_size(ctx: &ggez::Context) -> f32 {
    let screen_size = ctx.gfx.drawable_size();
    let square_size = f32::min(
        screen_size.0 / MIN_SIZE_IN_SQUARES,
        screen_size.1 / MIN_SIZE_IN_SQUARES,
    );
    f32::min(f32::max(square_size, MIN_SQUARE_SIZE), MAX_SQUARE_SIZE)
}

/// returns: A position relative to the center of the screen, converted to screen coordinate space
fn center_to_global_pos(ctx: &ggez::Context, offset: (f32, f32)) -> (f32, f32) {
    let screen_size = ctx.gfx.drawable_size();
    (offset.0 + screen_size.0 * 0.5_f32, offset.1 + screen_size.1 * 0.5_f32)
}

/// returns: A position in screen coordinate space, relative to the center of the screen
fn global_to_center_pos(ctx: &ggez::Context, pos: (f32, f32)) -> (f32, f32) {
    let screen_size = ctx.gfx.drawable_size();
    (pos.0 - screen_size.0 * 0.5_f32, pos.1 - screen_size.1 * 0.5_f32)
}

/// returns: A position in screen coordinate space, converted to board coordinate space, if within
///          the range -128 to 127 (inclusive)
pub fn global_to_board_offset_pos(ctx: &ggez::Context, mouse_pos: (f32, f32),
                                  flipped_board: bool) -> Option<(i8, i8)>
{
    let relative = global_to_center_pos(ctx, mouse_pos);
    let square_size = square_size(ctx);
    let flip_factor = if flipped_board { -1_f32 } else { 1_f32 };
    let x = flip_factor * relative.0 / square_size + 4_f32;
    let y = flip_factor * relative.1 / -square_size + 4_f32;
    let valid_range = -128_f32..=127_f32;
    if valid_range.contains(&x) && valid_range.contains(&y) {
        Some((x.floor() as i8, y.floor() as i8))
    } else {
        None
    }
}

/// returns: A position in screen coordinate space, converted to board coordinate space and
///          converted to a board [Position] if valid
pub fn global_to_board_pos(ctx: &ggez::Context, pos: (f32, f32),
                           flipped_board: bool) -> Option<Position>
{
    let relative = global_to_board_offset_pos(ctx, pos, flipped_board)?;
    let index_range = 0i8..=7;
    if index_range.contains(&relative.0) && index_range.contains(&relative.1) {
        Position::new(relative.0 as u8, relative.1 as u8)
    } else {
        None
    }
}

/// returns: An [Iterator] containing tuples with a [Position] and [PieceKind] representing what
///          piece should be displayed at that position in the promotion type selection menu
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

/// returns: Which piece type should be displayed at the provided `square`, if it is part of the
///          promotion type selection menu for the given `promotion_square`
pub fn promotion_selection_type(turn: Color, promotion_square: Position,
                                square: Position) -> Option<PieceKind>
{
    let mut iter = promotion_selection_iter(turn, promotion_square);
    iter.find_map(|(pos, piece_type)| (pos == square).then_some(piece_type))
}
