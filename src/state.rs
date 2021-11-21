use crate::{background::Background, constants::*, obstacles::ObstaclePool, player::Player};
use lerp::Lerp;
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct State {
    pub distance: f32,
    pub time: f64,
    pub frame_time: f32,
    pub background: Background,
    pub player: Player,
    pub obstacles: ObstaclePool,
    pub paused: bool,
    pub game_over: bool,
    pub spawn_time: f64,
}

impl State {
    pub fn tick(&mut self) {
        if !self.game_over {
            self.time = get_time();
            self.frame_time = get_frame_time();
        }

        if !self.paused {
            self.player.tick();
        }
    }

    pub fn step(&mut self) {
        if self.paused || self.game_over {
            return;
        }
        self.player.step();
        self.obstacles.step(self.player.speed);
        self.spawn_attempt();
        self.distance += self.player.speed;

        if self.obstacles.has_collision(&self.player) {
            self.game_over = true;
            return;
        }
    }

    pub fn render(&mut self, _alpha: f32) {
        if self.paused {
            self.background.render(self.time as f32);
            self.player.render();
            self.paused = self.render_menu();
            return;
        }

        if self.game_over {
            self.background.render(self.time as f32);
            self.player.render();
            self.obstacles.render();
            let restart = self.render_game_over();
            if restart {
                self.reset();
            }
            return;
        }

        self.background.render(self.time as f32);
        self.player.render();
        self.obstacles.render();
        self.render_distance();
    }

    fn render_distance(&self) {
        draw_text(
            &format!("{}m", self.distance.round() as i32),
            32.,
            64.,
            64.,
            PALETTE[15],
        );
    }

    fn render_menu(&self) -> bool {
        let half_width = screen_width() * 0.5;
        let mut x = half_width;
        let mut y = screen_height() * 0.5;

        // DRAW TITLE
        let title = "MTB MASTER";
        let title_measure = measure_text(title, None, 96, 1.);
        x -= title_measure.width * 0.5;
        y -= title_measure.height * 0.5;
        draw_text(title, x, y, 96., PALETTE[15]);

        // DRAW INSTRUCTIONS
        y += title_measure.height * 2.;
        let press_to_start = "CLICK/TOUCH TO START";
        let press_to_start_measure = measure_text(press_to_start, None, 64, 1.);
        x = half_width - press_to_start_measure.width * 0.5;
        draw_text(press_to_start, x, y, 64., PALETTE[15]);

        !is_mouse_button_pressed(MouseButton::Left)
    }

    fn render_game_over(&self) -> bool {
        let half_width = screen_width() * 0.5;
        let mut x = half_width;
        let mut y = screen_height() * 0.5;

        // DRAW TITLE
        let title = "GAME OVER";
        let title_measure = measure_text(title, None, 96, 1.);
        x -= title_measure.width * 0.5;
        y -= title_measure.height * 0.5;
        draw_text(title, x, y, 96., PALETTE[15]);
        y += title_measure.height * 2.;

        //DRAW SCORE
        let score = format!("YOU BIKED {:?} METERS!", self.distance.round() as i32);
        let score_measure = measure_text(&score, None, 64, 1.);
        x = half_width - score_measure.width * 0.5;
        draw_text(&score, x, y, 64., PALETTE[15]);
        y += score_measure.height * 2.;

        // DRAW INSTRUCTIONS
        let press_to_start = "CLICK/TOUCH TO RESTART";
        let press_to_start_measure = measure_text(press_to_start, None, 64, 1.);
        x = half_width - press_to_start_measure.width * 0.5;
        draw_text(press_to_start, x, y, 64., PALETTE[15]);

        is_mouse_button_pressed(MouseButton::Left)
    }

    fn spawn_attempt(&mut self) {
        if self.time % self.spawn_time <= TIMESTEP as f64 {
            self.obstacles.spawn();
        }
    }

    fn reset(&mut self) {
        self.time = 0.;
        self.frame_time = 0.;
        self.background = Background::default();
        self.player.reset();
        self.obstacles.reset();
        self.distance = 0.;
        self.game_over = false;
        self.paused = false;
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            time: 0.,
            frame_time: 0.,
            background: Background::default(),
            player: Player::new(32., 32.),
            obstacles: ObstaclePool::new(10),
            paused: true,
            distance: 0.,
            game_over: false,
            spawn_time: 2.,
        }
    }
}
