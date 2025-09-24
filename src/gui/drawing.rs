mod util;
pub mod colors;

use ggez::graphics;
use rsoderh_chess::Position;
use crate::gui::drawing::colors::BOARD_BORDER_COLOR;
use crate::resources::ImageResources;

fn draw_board_square(ctx: &mut ggez::Context, canvas: &mut graphics::Canvas,
                     resources: &ImageResources, index: (u8, u8),
                     slot: rsoderh_chess::Slot) -> ggez::GameResult
{
    let pos = util::center_pos_offset(
        ctx, ((index.0 as f32 - 4_f32) * 50_f32, (3_f32 - index.1 as f32) * 50_f32)
    );
    let rect = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(pos.0, pos.1, 50_f32, 50_f32),
        util::board_square_color(index)
    )?;
    canvas.draw(&rect, graphics::DrawParam::new());
    if let Some(piece) = slot.as_piece() {
        let image = &resources.get_piece(*piece).resource;
        canvas.draw(image, graphics::DrawParam::new().dest([pos.0 + 2.5_f32, pos.1 + 2.5_f32]));
    }
    Ok(())
}

pub fn draw_board(ctx: &mut ggez::Context, canvas: &mut graphics::Canvas,
                  resources: &ImageResources, board: &rsoderh_chess::Board) -> ggez::GameResult
{
    // drawing border
    let pos = util::center_pos_offset(ctx, (-207_f32, -207_f32));
    let rect = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(pos.0, pos.1, 414_f32, 414_f32),
        BOARD_BORDER_COLOR
    )?;
    canvas.draw(&rect, graphics::DrawParam::new());
    for i in 0..8_u8 {
        for j in 0..8_u8 {
            let pos = Position::new(i, j).unwrap();
            draw_board_square(ctx, canvas, resources, (i, j), board.at_position(pos))?;
        }
    }
    Ok(())
}
