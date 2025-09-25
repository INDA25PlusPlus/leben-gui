use std::num::ParseIntError;
use ggez::graphics::Color;
use crate::gui::drawing::{SquareDrawColor, SquareDrawState};

/// Convert hex code to `Color` object
const fn hex(s: &str) -> Color {
    let c = match u32::from_str_radix(s, 16) {
        Ok(c) => c,
        Err(_) => unreachable!(),
    };
    let r = (c >> 16) as u8;
    let g = (c >> 8) as u8;
    let b = c as u8;
    from_rgb(r, g, b)
}

const fn from_rgb(r: u8, g: u8, b: u8) -> Color {
    // why didn't they just make Color::from_rgb const :|
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    Color::new(r, g, b, 1_f32)
}

/// Multiply all components of the color object
const fn mult(c: Color, factor: f32) -> Color {
    Color::new(c.r * factor, c.g * factor, c.b * factor, c.a)
}

pub const BACKGROUND_COLOR: Color = from_rgb(30, 30, 30);
pub const BOARD_BORDER_COLOR: Color = hex("552C25");

pub const DARK_SQUARE_COLOR: Color = hex("3078BF");
pub const DARK_SQUARE_BORDER_COLOR: Color = mult(DARK_SQUARE_COLOR, 0.9);
pub const LIGHT_SQUARE_COLOR: Color = hex("85AEDB");
pub const LIGHT_SQUARE_BORDER_COLOR: Color = mult(LIGHT_SQUARE_COLOR, 0.9);

pub const DARK_SELECTED_SQUARE_COLOR: Color = hex("EFCA08");
pub const DARK_SELECTED_SQUARE_BORDER_COLOR: Color = mult(DARK_SELECTED_SQUARE_COLOR, 0.9);
pub const LIGHT_SELECTED_SQUARE_COLOR: Color = mult(DARK_SELECTED_SQUARE_COLOR, 1.1);
pub const LIGHT_SELECTED_SQUARE_BORDER_COLOR: Color = mult(LIGHT_SELECTED_SQUARE_COLOR, 0.9);

pub const PROMOTION_SELECTION_SQUARE_COLOR: Color = hex("FFEBD6");
pub const PROMOTION_SELECTION_SQUARE_BORDER_COLOR: Color = mult(PROMOTION_SELECTION_SQUARE_COLOR, 0.9);

pub const DARK_SQUARE_TEXT_COLOR: Color = LIGHT_SQUARE_COLOR;
pub const LIGHT_SQUARE_TEXT_COLOR: Color = DARK_SQUARE_COLOR;

pub fn square_colors(is_dark_square: bool, square_draw_color: SquareDrawColor) -> (Color, Color) {
    match square_draw_color {
        SquareDrawColor::Normal | SquareDrawColor::Targeted =>
            (normal_square_color(is_dark_square), normal_square_border_color(is_dark_square)),
        SquareDrawColor::Selected =>
            (selected_square_color(is_dark_square), selected_square_border_color(is_dark_square)),
        SquareDrawColor::PromotionSelection =>
            (PROMOTION_SELECTION_SQUARE_COLOR, PROMOTION_SELECTION_SQUARE_BORDER_COLOR)
    }
}

pub fn normal_square_color(is_dark_square: bool) -> Color {
    if is_dark_square { DARK_SQUARE_COLOR } else { LIGHT_SQUARE_COLOR }
}

pub fn normal_square_border_color(is_dark_square: bool) -> Color {
    if is_dark_square { DARK_SQUARE_BORDER_COLOR } else { LIGHT_SQUARE_BORDER_COLOR }
}

pub fn selected_square_color(is_dark_square: bool) -> Color {
    if is_dark_square { DARK_SELECTED_SQUARE_COLOR } else { LIGHT_SELECTED_SQUARE_COLOR }
}

pub fn selected_square_border_color(is_dark_square: bool) -> Color {
    if is_dark_square { DARK_SELECTED_SQUARE_BORDER_COLOR }
    else { LIGHT_SELECTED_SQUARE_BORDER_COLOR }
}

pub fn square_text_color(is_dark_square: bool) -> Color {
    if is_dark_square { DARK_SQUARE_TEXT_COLOR } else { LIGHT_SQUARE_TEXT_COLOR }
}
