use std::f32::consts::{PI, TAU};

use crate::{constants::*, geometry::Rectangle, util::*};
use macroquad::{prelude::*, audio::{Sound, play_sound_once, stop_sound}};

#[derive(Debug, Clone)]
pub struct Player {
    pub center: Vec2,
    pub size: Vec2,
    pub speed: f32,
    pub ground_height: f32,
    pub velocity: Vec2,
    pub position: Vec2,
    pub acceleration: Vec2,
    pub is_jumping: bool,
    pub can_jump: bool,
    pub is_moving: bool,
    pedal_theta: f32,
    previous_pedal_theta: f32,
    wheel_theta: f32,
    previous_wheel_theta: f32,
    pub headlight: Vec2,
    pub taillight: Vec2,
    jump_sound: Sound,
    land_sound: Sound,
}

impl Player {
    pub fn new(size: Vec2, resolution: Vec2, jump: Sound, land: Sound) -> Self {
        Self {
            center: vec2((resolution.x - size.x) * 0.5, resolution.y - size.y),
            size,
            speed: DEFAULT_PLAYER_SPEED,
            ground_height: 0.,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            position: Vec2::ZERO,
            is_moving: true,
            is_jumping: false,
            can_jump: false,
            pedal_theta: 0.,
            previous_pedal_theta: 0.,
            wheel_theta: 0.,
            previous_wheel_theta: 0.,
            headlight: vec2(0., 0.),
            taillight: vec2(0., 0.),
            jump_sound: jump,
            land_sound: land,
        }
    }

    pub fn reset(&mut self) {
        self.speed = DEFAULT_PLAYER_SPEED;
        self.ground_height = 0.;
        self.velocity = Vec2::ZERO;
        self.acceleration = Vec2::ZERO;
        self.position = Vec2::ZERO;
        self.is_jumping = false;
        self.is_moving = true;
        self.can_jump = false;
    }

    pub fn render(&mut self) {
        let line_thickness = 8.;
        let half_line_thickness = line_thickness * 0.5;
        let wheel_radius = 24.;
        let center = self.center - self.position
            + vec2(
                self.size.x * 0.5,
                self.size.y - wheel_radius - line_thickness,
            );
        let wheel_1 = center - vec2(40., -line_thickness);
        let wheel_2 = center + vec2(40., line_thickness);
        let bottom_bracket = wheel_1 + vec2((wheel_2.x - wheel_1.x) * 0.5, 0.);

        let seat_post = wheel_1 + vec2((bottom_bracket.x - wheel_1.x) * 0.5, -wheel_radius * 1.5);
        let seat_start = seat_post - vec2(line_thickness * 2., line_thickness * 0.75);
        let seat_end = seat_post + vec2(line_thickness * 1.7, -line_thickness * 0.75 * 1.4);
        let steering_tube = wheel_2 - vec2(2. * line_thickness, wheel_radius * 1.6);
        let steer = steering_tube + vec2(line_thickness, -line_thickness);

        let crank_length = wheel_radius * 0.5;
        

        let crank_1 = point_on_circle(bottom_bracket, crank_length, self.pedal_theta);
        let crank_2 = point_on_circle(bottom_bracket, crank_length, self.pedal_theta + PI);

        let pedal_length = 8.;
        let pedal_vec = vec2(pedal_length, 0.);
        let pedal_1_start = crank_1 - pedal_vec;
        let pedal_1_end = crank_1 + pedal_vec;

        let pedal_2_start = crank_2 - pedal_vec;
        let pedal_2_end = crank_2 + pedal_vec;
        draw_line(
            bottom_bracket.x,
            bottom_bracket.y,
            crank_2.x,
            crank_2.y,
            line_thickness * 0.5,
            PALETTE[12],
        );
        draw_line(
            pedal_2_start.x,
            pedal_2_start.y,
            pedal_2_end.x,
            pedal_2_end.y,
            line_thickness * 0.75,
            PALETTE[0],
        );

        draw_circle_lines(
            wheel_1.x,
            wheel_1.y,
            wheel_radius,
            line_thickness,
            PALETTE[0],
        );
        draw_circle_lines(
            wheel_2.x,
            wheel_2.y,
            wheel_radius,
            line_thickness,
            PALETTE[0],
        );
        let spokes = 16.;

        

        let increment = TAU / spokes;
        let mut theta = self.wheel_theta;
        for _i in 0..(spokes as usize) {
            let point_1 =
                point_on_circle(wheel_1, wheel_radius - line_thickness * 0.5, theta % TAU);
            let point_2 =
                point_on_circle(wheel_2, wheel_radius - line_thickness * 0.5, theta % TAU);
            draw_line(wheel_1.x, wheel_1.y, point_1.x, point_1.y, 1., PALETTE[12]);
            draw_line(wheel_2.x, wheel_2.y, point_2.x, point_2.y, 1., PALETTE[12]);
            theta += increment;
        }

        draw_line(
            wheel_1.x - half_line_thickness,
            wheel_1.y,
            bottom_bracket.x,
            bottom_bracket.y,
            line_thickness,
            PALETTE[1],
        );
        draw_line(
            wheel_1.x - half_line_thickness,
            wheel_1.y,
            seat_post.x,
            seat_post.y,
            line_thickness,
            PALETTE[1],
        );
        draw_line(
            seat_post.x,
            seat_post.y,
            bottom_bracket.x,
            bottom_bracket.y,
            line_thickness,
            PALETTE[1],
        );
        draw_line(
            bottom_bracket.x,
            bottom_bracket.y,
            steering_tube.x,
            steering_tube.y,
            line_thickness,
            PALETTE[1],
        );
        draw_line(
            seat_post.x,
            seat_post.y,
            steering_tube.x,
            steering_tube.y,
            line_thickness,
            PALETTE[1],
        );
        draw_line(
            steering_tube.x,
            steering_tube.y,
            wheel_2.x,
            wheel_2.y,
            line_thickness + 2.,
            PALETTE[1],
        );

        let lamp_front = steering_tube + vec2(line_thickness, line_thickness * 0.75);
        let lamp_back = vec2(center.x - 28., lamp_front.y - line_thickness * 0.3);

        self.headlight = lamp_front;
        self.taillight = lamp_front;
        draw_circle(
            lamp_front.x,
            lamp_front.y,
            line_thickness * 0.5,
            PALETTE[14],
        );
        draw_circle(lamp_back.x, lamp_back.y, line_thickness * 0.5, PALETTE[8]);

        draw_line(
            seat_start.x,
            seat_start.y,
            seat_end.x,
            seat_end.y,
            line_thickness + 4.,
            PALETTE[4],
        );
        draw_circle(wheel_1.x, wheel_1.y, line_thickness, PALETTE[9]);
        draw_circle(wheel_2.x, wheel_2.y, line_thickness, PALETTE[9]);
        draw_circle(
            bottom_bracket.x,
            bottom_bracket.y,
            line_thickness * 0.5,
            PALETTE[1],
        );
        draw_circle(
            steering_tube.x,
            steering_tube.y,
            line_thickness * 0.5,
            PALETTE[1],
        );
        draw_line(
            steering_tube.x,
            steering_tube.y,
            steer.x,
            steer.y,
            line_thickness,
            PALETTE[1],
        );
        draw_circle(steer.x, steer.y, 0.75 * line_thickness, PALETTE[0]);

        draw_line(
            bottom_bracket.x,
            bottom_bracket.y,
            crank_1.x,
            crank_1.y,
            line_thickness * 0.5,
            PALETTE[12],
        );
        draw_circle(bottom_bracket.x, bottom_bracket.y, 4., PALETTE[12]);
        draw_line(
            pedal_1_start.x,
            pedal_1_start.y,
            pedal_1_end.x,
            pedal_1_end.y,
            line_thickness * 0.75,
            PALETTE[0],
        );
    }

    pub fn step(&mut self, time: f32) {
        if self.position.y <= self.ground_height {
            self.position.y = self.ground_height;
            self.acceleration += *UP * *GRAVITY;

            if self.is_jumping {
                self.acceleration = *UP * *GRAVITY;
                self.velocity = Vec2::ZERO;
                self.is_jumping = false;
                stop_sound(self.jump_sound);
                play_sound_once(self.land_sound);
            }
            self.can_jump = true;

        }

        if time <= 0.1 {
            self.can_jump = false;
        }
        self.acceleration += *DOWN * *GRAVITY;
        self.velocity += self.acceleration * TIMESTEP;
        self.position += self.velocity * TIMESTEP;

        self.speed += 0.0001;

        self.previous_wheel_theta = self.wheel_theta;
        let wheel_speed = if self.is_jumping { 15. } else { 30. };
        self.wheel_theta = if self.is_moving {
            (self.previous_wheel_theta + (TAU / wheel_speed)) % TAU
        } else {
            self.previous_wheel_theta
        };

        self.previous_pedal_theta = self.pedal_theta;
        self.pedal_theta = if self.is_moving && !self.is_jumping {
            (self.previous_pedal_theta + (TAU / 30.)) % TAU
        } else {
            self.previous_pedal_theta
        };
    }

    pub fn tick(&mut self) {
        if is_mouse_button_down(MouseButton::Left) && self.can_jump {
            if !self.is_jumping {
                self.jump();
                play_sound_once(self.jump_sound);
            }
        }

        if is_mouse_button_released(MouseButton::Left) && self.is_jumping && self.can_jump {
            self.can_jump = false;
        }
    }

    fn jump(&mut self) {
        self.position.y = self.ground_height + 0.1;
        self.is_jumping = true;
        self.velocity += *UP * *JUMP_FORCE;
    }

    pub fn get_aabb(&self) -> Rectangle {
        Rectangle::new(self.origin(), self.size)
    }

    fn origin(&self) -> Vec2 {
        vec2(
            self.center.x,
            self.center.y - self.ground_height - self.position.y,
        )
    }
}
