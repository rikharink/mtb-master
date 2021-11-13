use macroquad::prelude::Color;

pub const TPS: f32 = 60.;

lazy_static! {
    pub static ref PALETTE: [Color; 16] = [
        Color::from_rgba(29, 24, 25, 255),
        Color::from_rgba(72, 38, 50, 255),
        Color::from_rgba(39, 54, 53, 255),
        Color::from_rgba(77, 61, 47, 255),
        Color::from_rgba(147, 54, 51, 255),
        Color::from_rgba(49, 100, 54, 255),
        Color::from_rgba(130, 92, 58, 255),
        Color::from_rgba(185, 83, 88, 255),
        Color::from_rgba(199, 115, 49, 255),
        Color::from_rgba(97, 122, 111, 255),
        Color::from_rgba(127, 165, 51, 255),
        Color::from_rgba(202, 152, 100, 255),
        Color::from_rgba(175, 170, 148, 255),
        Color::from_rgba(125, 207, 168, 255),
        Color::from_rgba(231, 220, 88, 255),
        Color::from_rgba(247, 246, 219, 255),
    ];
}