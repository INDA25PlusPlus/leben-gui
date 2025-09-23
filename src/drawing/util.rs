use ggez::graphics;
use crate::drawing::colors::*;

pub fn center_pos_offset(ctx: &ggez::Context, pos: (f32, f32)) -> (f32, f32) {
    let screen_size = ctx.gfx.drawable_size();
    (pos.0 + screen_size.0 * 0.5_f32, pos.1 + screen_size.1 * 0.5_f32)
}

pub fn board_square_color(square_index: (u8, u8)) -> graphics::Color {
    if (square_index.0 + square_index.1) % 2 == 0 {
        DARK_SQUARE_COLOR
    } else {
        LIGHT_SQUARE_COLOR
    }
}
