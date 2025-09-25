use ggez::{graphics};
use ggez::graphics::DrawParam;
use crate::gui::drawing::colors::*;
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

pub fn board_image_draw_param(ctx: &ggez::Context, image_size: (u32, u32), board_pos: (f32, f32),
                              relative_width: f32) -> DrawParam
{
    let image_size = (image_size.0 as f32, image_size.1 as f32);
    let square_size = square_size(ctx);
    let offset = (board_pos.0 / relative_width, board_pos.1 / relative_width);
    let scale = square_size * relative_width / image_size.0;
    let screen_size = ctx.gfx.drawable_size();
    DrawParam::new()
        .offset([0.5_f32 - offset.0, 0.5_f32 - offset.1])
        .scale([scale, scale])
        .dest([screen_size.0 * 0.5_f32, screen_size.1 * 0.5_f32])
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
