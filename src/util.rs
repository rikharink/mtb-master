use macroquad::{prelude::*, rand::gen_range};

pub fn rgba_texture(width: u16, height: u16) -> Texture2D {
    let n = width as usize * height as usize * 4;
    let mut bytes: Vec<u8> = Vec::with_capacity(n);
    for _ in 0..n {
        bytes.push(gen_range(0, 255));
    }
    let texture = Texture2D::from_rgba8(width, height, &bytes);
    texture.set_filter(FilterMode::Nearest);
    texture
}
