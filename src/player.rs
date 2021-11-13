use crate::constants::*;
use macroquad::prelude::*;

pub struct Player {
    pub center: Vec2,
    pub size: Vec2,
    pub distance: f32,
    pub speed: f32,
    pub ground_height: f32,
    pub y_offset: f32,
}

impl Player {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            center: Vec2::new(screen_width() * 0.5 - width * 0.5, screen_height() - height),
            size: Vec2::new(width, height),
            distance: 0.,
            speed: 0.14,
            ground_height: 0.,
            y_offset: 0.,
        }
    }

    pub fn render(&self) {
        draw_rectangle(
            self.center.x,
            self.center.y - self.ground_height - self.y_offset,
            32.,
            32.,
            PALETTE[8],
        );
    }

    pub fn step(&mut self) {
        self.distance += self.speed;
    }

    pub fn tick(&mut self) {
        self.center = Vec2::new(
            screen_width() * 0.5 - self.size.x * 0.5,
            screen_height() - self.size.y,
        );

        if is_mouse_button_pressed(MouseButton::Left) && !self.is_jumping() {
            self.jump();
        }
    }

    pub fn is_jumping(&self) -> bool {
        self.y_offset > 0.
    }

    fn jump(&mut self) {
        self.y_offset += self.size.y * 2.;
    }
}
