use ggez::graphics::Color;

const fn from_rgb(r: u8, g: u8, b: u8) -> Color {
    // why didn't they just make Color::from_rgb const :|
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    Color::new(r, g, b, 1_f32)
}

pub const DARK_SQUARE_COLOR: Color = from_rgb(184, 135, 98);
pub const LIGHT_SQUARE_COLOR: Color = from_rgb(237, 214, 176);
pub const BOARD_BORDER_COLOR: Color = from_rgb(116, 80, 53);
pub const BACKGROUND_COLOR: Color = from_rgb(30, 30, 30);
