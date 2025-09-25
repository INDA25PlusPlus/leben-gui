use ggez::{graphics};
use ggez::graphics::DrawParam;
use crate::gui::drawing::colors::*;
use crate::gui::drawing::{TextAlign, TextAlignHorizontal, TextAlignVertical};
use crate::gui::util::square_size;

const CIRCLE_TOLERANCE: f32 = 0.002_f32;

pub fn is_dark_square(square_index: (u8, u8)) -> bool {
    (square_index.0 + square_index.1) % 2 == 0
}

pub fn square_draw_param(ctx: &ggez::Context) -> DrawParam {
    let square_size = square_size(ctx);
    let screen_size = ctx.gfx.drawable_size();
    DrawParam::new()
        .scale([square_size, square_size])
        .dest([screen_size.0 * 0.5_f32, screen_size.1 * 0.5_f32])
}

pub fn board_relative_draw_param(ctx: &ggez::Context, element_size: (f32, f32),
                                 board_pos: (f32, f32), relative_height: f32) -> DrawParam
{
    let square_size = square_size(ctx);
    let offset = (board_pos.0 / relative_height, board_pos.1 / relative_height);
    let scale = square_size * relative_height / element_size.1;
    let screen_size = ctx.gfx.drawable_size();
    DrawParam::new()
        .offset([0.5_f32 - offset.0, 0.5_f32 - offset.1])
        .scale([scale, scale])
        .dest([screen_size.0 * 0.5_f32, screen_size.1 * 0.5_f32])
}

pub fn board_relative_text_param(ctx: &ggez::Context, element_size: (f32, f32),
                                 text_align: TextAlign, board_pos: (f32, f32)) -> DrawParam
{
    let square_size = square_size(ctx);
    let screen_size = ctx.gfx.drawable_size();

    let horizontal_offset = match text_align.horizontal {
        TextAlignHorizontal::Left => 0_f32,
        TextAlignHorizontal::Middle => element_size.0 * 0.5_f32,
        TextAlignHorizontal::Right => element_size.0,
    };
    let vertical_offset = match text_align.vertical {
        TextAlignVertical::Top => 0_f32,
        TextAlignVertical::Middle => element_size.1 * 0.5_f32,
        TextAlignVertical::Bottom => element_size.1,
    };

    let screen_offset = (screen_size.0 * 0.5_f32, screen_size.1 * 0.5_f32);
    let pos_offset = (board_pos.0 * square_size, board_pos.1 * square_size);
    DrawParam::new()
        .offset([
            horizontal_offset - screen_offset.0 - pos_offset.0,
            vertical_offset - screen_offset.1 - pos_offset.1
        ])
}

pub fn draw_rect(ctx: &ggez::Context, canvas: &mut graphics::Canvas, params: graphics::DrawParam,
                 color: graphics::Color, x: f32, y: f32, w: f32, h: f32) -> ggez::GameResult
{
    let rect = graphics::Mesh::new_rectangle(
        ctx, graphics::DrawMode::fill(), graphics::Rect::new(x, y, w, h), color
    )?;
    canvas.draw(&rect, params);
    Ok(())
}

pub fn draw_circle(ctx: &ggez::Context, canvas: &mut graphics::Canvas, params: graphics::DrawParam,
                   color: graphics::Color, x: f32, y: f32, r: f32) -> ggez::GameResult
{
    let circle = graphics::Mesh::new_circle(
        ctx, graphics::DrawMode::fill(), [x, y], r, CIRCLE_TOLERANCE, color
    )?;
    canvas.draw(&circle, params);
    Ok(())
}
