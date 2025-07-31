use iced::Color;
use plotters::style::RGBColor;

pub fn to_rgb_color(color: Color) -> RGBColor {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    if color.r <= 1.0
        && color.r >= 0.0
        && color.g <= 1.0
        && color.g >= 0.0
        && color.b <= 1.0
        && color.b >= 0.0
    {
        RGBColor(
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
        )
    } else {
        RGBColor(0, 0, 0) // Black
    }
}

/// Returns the average of two colors; color intensity is fixed to 100%
pub fn mix_colors(color_1: Color, color_2: Color) -> Color {
    Color {
        r: f32::midpoint(color_1.r, color_2.r),
        g: f32::midpoint(color_1.g, color_2.g),
        b: f32::midpoint(color_1.b, color_2.b),
        a: 1.0,
    }
}
