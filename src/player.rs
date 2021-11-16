use crate::constants::*;
use macroquad::prelude::*;

#[derive(Debug)]
pub struct Player {
    pub center: Vec2,
    pub size: Vec2,
    pub distance: f32,
    pub speed: f32,
    pub ground_height: f32,
    pub velocity: Vec2,
    pub position: Vec2,
    pub acceleration: Vec2,
    pub is_jumping: bool,
    pub can_jump: bool,
}

impl Player {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            center: Vec2::new(screen_width() * 0.5 - width * 0.5, screen_height() - height),
            size: Vec2::new(width, height),
            distance: 0.,
            speed: 0.14,
            ground_height: 0.,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            position: Vec2::ZERO,
            is_jumping: false,
            can_jump: true,
        }
    }

    pub fn render(&self) {
        draw_rectangle(
            self.center.x,
            self.center.y - self.ground_height - self.position.y,
            32.,
            32.,
            PALETTE[8],
        );

        draw_text(
            &format!("{}m", self.distance.round() as i32),
            32.,
            64.,
            64.,
            PALETTE[15],
        );
    }

    pub fn step(&mut self) {
        if self.position.y <= self.ground_height {
            self.position.y = self.ground_height;
            self.acceleration += *UP * *GRAVITY;

            if self.is_jumping {
                self.acceleration = *UP * *GRAVITY;
                self.velocity = Vec2::ZERO;
                self.can_jump = true;
                self.is_jumping = false;
            }
        }

        self.acceleration += *DOWN * *GRAVITY;
        self.distance += self.speed;
        self.velocity += self.acceleration * TIMESTEP;
        self.position += self.velocity * TIMESTEP;
    }

    pub fn tick(&mut self) {
        self.center = Vec2::new(
            screen_width() * 0.5 - self.size.x * 0.5,
            screen_height() - self.size.y,
        );

        if is_mouse_button_down(MouseButton::Left) && self.can_jump {
            if !self.is_jumping {
                self.jump();
            }
        }
        if is_mouse_button_released(MouseButton::Left) && self.is_jumping && self.can_jump {
            self.can_jump = false;
        }
    }

    fn jump(&mut self) {
        self.position.y = self.ground_height + 0.1;
        self.is_jumping = true;
        self.velocity += *UP * 150.;
    }
}
