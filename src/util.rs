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

pub fn color_to_vector(color: Color) -> Vec3 {
    vec3(color.r, color.g, color.b)
}

pub fn point_on_circle(origin: Vec2, radius: f32, theta: f32) -> Vec2 {
    let x = origin.x + radius * theta.cos();
    let y = origin.y + radius * theta.sin();
    vec2(x, y)
}
