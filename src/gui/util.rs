use rsoderh_chess::Position;

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
