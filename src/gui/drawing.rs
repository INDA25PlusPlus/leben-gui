mod util;
pub mod colors;

use ggez::graphics;
use ggez::graphics::PxScale;
use rsoderh_chess::{Color, Piece, Position};
use colors::BOARD_BORDER_COLOR;
use crate::gui;
use crate::gui::SquareSelection;
use crate::resources::ImageResources;

const SQUARE_BORDER_THICKNESS: f32 = 0.05;
const TARGET_CIRCLE_RADIUS: f32 = 0.4;
const TARGET_CIRCLE_THICKNESS: f32 = 0.05;

#[derive(Copy, Clone, Debug)]
pub enum TextAlignHorizontal {
    Left, Middle, Right,
}

#[derive(Copy, Clone, Debug)]
pub enum TextAlignVertical {
    Top, Middle, Bottom,
}

#[derive(Copy, Clone, Debug)]
pub struct TextAlign {
    pub horizontal: TextAlignHorizontal,
    pub vertical: TextAlignVertical,
}

#[derive(Copy, Clone, Debug)]
enum SquareDrawColor {
    /// Normal state
    Normal,
    /// Targeted by the selected piece
    Targeted,
    /// Currently selected square
    Selected,
    /// Displays options for performing a promotion move
    PromotionSelection,
}

#[derive(Copy, Clone, Debug)]
struct SquareDrawState {
    hovered: bool,
    color: SquareDrawColor,
    piece: Option<Piece>,
}

fn get_square_render_state(square: Position, piece: Option<Piece>,
                           selected_square: Option<&SquareSelection>,
                           hovered_square: Option<Position>,
                           turn: Option<Color>,
                           promotion_selection: Option<Position>) -> SquareDrawState
{
    let hovered = hovered_square.is_some_and(|s| s == square);

    let Some(turn) = turn else {
        return SquareDrawState {
            hovered,
            color: SquareDrawColor::Normal,
            piece,
        };
    };

    if let Some(promotion_type) = promotion_selection
        .map(|promotion_square|
            gui::util::promotion_selection_type(turn, promotion_square, square))
        .flatten()
    {
        return SquareDrawState {
            hovered,
            color: SquareDrawColor::PromotionSelection,
            piece: Some(Piece { kind: promotion_type, color: turn }),
        };
    }

    let is_selected = selected_square.as_ref().is_some_and(|sel| sel.pos == square);
    if is_selected {
        return SquareDrawState {
            hovered,
            color: SquareDrawColor::Selected,
            piece,
        };
    }

    let is_targeted = selected_square.as_ref()
        .is_some_and(|sel|
            sel.available_moves.contains(&square));

    return SquareDrawState {
        hovered,
        color: if is_targeted { SquareDrawColor::Targeted } else { SquareDrawColor::Normal },
        piece,
    };


    // let mut promotion_selection_iter = turn.map(
    //     |turn| selected_square.map(
    //         |selected_square| gui::util::promotion_selection_iter(turn, selected_square.pos)
    // )).flatten();
    // let promotion_selection = promotion_selection_iter.map(
    //     |mut iter| iter.find_map(
    //         |(pos, piece_type)| (pos == square).then_some(piece_type))
    // ).flatten()
    // let is_selected = selected_square.as_ref().is_some_and(|sel| sel.pos == square);
    // let targeted = selected_square.as_ref()
    //     .is_some_and(|sel|
    //         sel.available_moves.contains(&square));
    // SquareDrawState {
    //     hovered: hovered_square.is_some_and(|s| s == square),
    // }
}

pub fn draw_board(ctx: &mut ggez::Context, canvas: &mut graphics::Canvas,
                  resources: &ImageResources, board: &rsoderh_chess::Board,
                  selected_square: Option<&SquareSelection>,
                  hovered_square: Option<Position>, turn: Option<Color>,
                  promotion_selection: Option<Position>) -> ggez::GameResult
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
            let piece = board.at_position(pos).as_piece().map(|p| *p);
            let square_render_state = get_square_render_state(
                pos, piece, selected_square, hovered_square, turn, promotion_selection);
            draw_board_square(ctx, canvas, square_params, resources, (i, j),
                              square_render_state)?;
        }
    }
    Ok(())
}

fn draw_board_square(ctx: &mut ggez::Context, canvas: &mut graphics::Canvas,
                     square_params: graphics::DrawParam, resources: &ImageResources,
                     index: (u8, u8), draw_state: SquareDrawState) -> ggez::GameResult
{
    let x = index.0 as f32 - 4_f32;
    let y = 3_f32 - index.1 as f32;
    let is_dark_square = util::is_dark_square(index);

    let (square_color, border_color) = colors::square_colors(is_dark_square, draw_state.color);

    if draw_state.hovered {
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

    if matches!(draw_state.color, SquareDrawColor::Targeted) {
        util::draw_circle(
            ctx, canvas, square_params, border_color,
            x + 0.5_f32, y + 0.5_f32, TARGET_CIRCLE_RADIUS,
        )?;
        util::draw_circle(
            ctx, canvas, square_params, square_color,
            x + 0.5_f32, y + 0.5_f32, TARGET_CIRCLE_RADIUS - TARGET_CIRCLE_THICKNESS,
        )?;
    }

    if let Some(piece) = draw_state.piece {
        let image = &resources.get_piece(piece).resource;
        let image_size = (image.width() as f32, image.height() as f32);
        let image_params = util::board_relative_draw_param(
            ctx, image_size, (x + 0.5_f32, y + 0.5_f32), 0.9_f32
        );
        canvas.draw(image, image_params);
    }
    Ok(())
}

pub fn draw_status_text(ctx: &mut ggez::Context, canvas: &mut graphics::Canvas,
                        text: &str) -> ggez::GameResult
{
    let mut text = graphics::Text::new(text);
    text.set_scale(PxScale::from(24_f32));
    let [w, h] = text.measure(ctx)?.into();
    let text_align = TextAlign {
        horizontal: TextAlignHorizontal::Middle,
        vertical: TextAlignVertical::Top,
    };
    let relative_pos = (0_f32, 4.5_f32);
    let params = util::board_relative_text_param(ctx, (w, h), text_align, relative_pos);
    canvas.draw(&text, params);
    Ok(())
}
