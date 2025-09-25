use ggez::graphics::Color;

const fn from_rgb(r: u8, g: u8, b: u8) -> Color {
    // why didn't they just make Color::from_rgb const :|
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    Color::new(r, g, b, 1_f32)
}

pub const BACKGROUND_COLOR: Color = from_rgb(30, 30, 30);
pub const BOARD_BORDER_COLOR: Color = from_rgb(116, 80, 53);

pub const DARK_SQUARE_COLOR: Color = from_rgb(48, 120, 191);
pub const DARK_SQUARE_BORDER_COLOR: Color = from_rgb(40, 103, 166);
pub const LIGHT_SQUARE_COLOR: Color = from_rgb(133, 174, 219);
pub const LIGHT_SQUARE_BORDER_COLOR: Color = from_rgb(40, 103, 166);

pub const DARK_SELECTED_SQUARE_COLOR: Color = from_rgb(134, 134, 36);
pub const DARK_SELECTED_SQUARE_BORDER_COLOR: Color = from_rgb(115, 115, 26);
pub const LIGHT_SELECTED_SQUARE_COLOR: Color = from_rgb(181, 181, 53);
pub const LIGHT_SELECTED_SQUARE_BORDER_COLOR: Color = from_rgb(155, 155, 44);

pub const DARK_SQUARE_TEXT_COLOR: Color = LIGHT_SQUARE_COLOR;
pub const LIGHT_SQUARE_TEXT_COLOR: Color = DARK_SQUARE_COLOR;

pub fn square_color(is_dark_square: bool) -> Color {
    if is_dark_square { DARK_SQUARE_COLOR } else { LIGHT_SQUARE_COLOR }
}

pub fn square_border_color(is_dark_square: bool) -> Color {
    if is_dark_square { DARK_SQUARE_BORDER_COLOR } else { LIGHT_SQUARE_BORDER_COLOR }
}

pub fn selected_square_color(is_dark_square: bool) -> Color {
    if is_dark_square { DARK_SELECTED_SQUARE_COLOR } else { LIGHT_SELECTED_SQUARE_COLOR }
}

pub fn selected_square_border_color(is_dark_square: bool) -> Color {
    if is_dark_square { DARK_SELECTED_SQUARE_BORDER_COLOR }
    else { LIGHT_SELECTED_SQUARE_BORDER_COLOR }
}
