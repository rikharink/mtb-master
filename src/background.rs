use crate::constants::*;
use macroquad::prelude::Color;

#[derive(Debug)]
pub struct Background {
    pub speed: f32,
    pub color: Color,
}

impl Background {
    pub fn new() -> Self {
        Self {
            speed: 0.14,
            color: PALETTE[2],
        }
    }
    pub fn tick(&self) {}

    pub fn render(&self) {}
}
