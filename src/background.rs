use crate::constants::*;
use arrayvec::ArrayVec;
use macroquad::prelude::*;
use noise::NoiseFn;

#[derive(Debug)]
pub enum BackgroundType {
    Mountains,
}

#[derive(Debug)]
pub struct Background {
    pub style: BackgroundType,
    pub speed: f32,
    pub color: Color,
    pub size: UVec2,
}

impl Background {
    pub fn new(style: BackgroundType) -> Self {
        let size: UVec2 = UVec2::new(screen_width() as u32, screen_height() as u32);
        Self {
            style,
            size,
            speed: 0.14,
            color: PALETTE[2],
        }
    }

    pub fn tick(&mut self) {
        let size: UVec2 = UVec2::new(screen_width() as u32, screen_height() as u32);
        if self.size != size {
            self.size = size;
        }
    }

    pub fn render(&mut self) {}
}
