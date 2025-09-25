mod util;
pub mod colors;

use ggez::graphics;
use rsoderh_chess::Position;
use colors::BOARD_BORDER_COLOR;
use crate::gui::SquareSelection;
use crate::resources::ImageResources;

const SQUARE_BORDER_THICKNESS: f32 = 0.05;
const TARGET_CIRCLE_RADIUS: f32 = 0.4;
const TARGET_CIRCLE_THICKNESS: f32 = 0.05;

struct SquareDrawState {
    hovered: bool,
    selected: bool,
    targeted: bool,
}

fn get_square_render_state(square: Position, selected_square: Option<&SquareSelection>,
                           hovered_square: Option<Position>) -> SquareDrawState
{
    let targeted = selected_square.as_ref()
        .is_some_and(|sel|
            sel.available_moves.contains(&square));
    SquareDrawState {
        hovered: hovered_square.is_some_and(|s| s == square),
        selected: selected_square.as_ref().is_some_and(|sel| sel.pos == square),
        targeted
    }
}

pub fn draw_board(ctx: &mut ggez::Context, canvas: &mut graphics::Canvas,
                  resources: &ImageResources, board: &rsoderh_chess::Board,
                  selected_square: Option<&SquareSelection>,
                  hovered_square: Option<Position>) -> ggez::GameResult
{
    let square_params = util::square_draw_param(ctx);
    // board border
    util::draw_rect(
        ctx, canvas, square_params, BOARD_BORDER_COLOR, -4.15_f32, -4.15_f32, 8.3_f32, 8.3_f32,
    )?;
    // board squares
    for i in 0..8_u8 {
        for j in 0..8_u8 {
            let pos = Position::new(i, j).unwrap();
            let square_render_state = get_square_render_state(pos, selected_square,
                                                              hovered_square);
            draw_board_square(ctx, canvas, square_params, resources, (i, j),
                              board.at_position(pos), square_render_state)?;
        }
    }
    Ok(())
}

fn draw_board_square(ctx: &mut ggez::Context, canvas: &mut graphics::Canvas,
                     square_params: graphics::DrawParam, resources: &ImageResources,
                     index: (u8, u8), slot: rsoderh_chess::Slot,
                     draw_state: SquareDrawState) -> ggez::GameResult
{
    let x = index.0 as f32 - 4_f32;
    let y = 3_f32 - index.1 as f32;
    let is_dark_square = util::is_dark_square(index);
    let square_color = if draw_state.selected {
        colors::selected_square_color(is_dark_square)
    } else {
        colors::square_color(is_dark_square)
    };
    if draw_state.hovered {
        let border_color = if draw_state.selected {
            colors::selected_square_border_color(is_dark_square)
        } else {
            colors::square_border_color(is_dark_square)
        };
        util::draw_rect(
            ctx, canvas, square_params, border_color,
            x, y, 1_f32, 1_f32,
        )?;
        util::draw_rect(
            ctx, canvas, square_params, square_color,
            x + SQUARE_BORDER_THICKNESS, y + SQUARE_BORDER_THICKNESS,
            1_f32 - 2_f32 * SQUARE_BORDER_THICKNESS, 1_f32 - 2_f32 * SQUARE_BORDER_THICKNESS,
        )?;
    } else {
        util::draw_rect(
            ctx, canvas, square_params, square_color,
            x, y, 1_f32, 1_f32,
        )?;
    }
    if draw_state.targeted {
        let color = if draw_state.selected {
            colors::selected_square_border_color(is_dark_square)
        } else {
            colors::square_border_color(is_dark_square)
        };
        util::draw_circle(
            ctx, canvas, square_params, color,
            x + 0.5_f32, y + 0.5_f32, TARGET_CIRCLE_RADIUS,
        )?;
        util::draw_circle(
            ctx, canvas, square_params, square_color,
            x + 0.5_f32, y + 0.5_f32, TARGET_CIRCLE_RADIUS - TARGET_CIRCLE_THICKNESS,
        )?;
    }
    if let Some(piece) = slot.as_piece() {
        let image = &resources.get_piece(*piece).resource;
        let image_size = (image.width(), image.height());
        let image_params = util::board_image_draw_param(
            ctx, image_size, (x + 0.5_f32, y + 0.5_f32), 0.9_f32
        );
        canvas.draw(image, image_params);
    }
    Ok(())
}
