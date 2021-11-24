use crate::{
    background::Background, constants::*, obstacles::ObstaclePool, player::Player,
    shaders::get_post_processing_material,
};
use macroquad::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum GameState {
    Running,
    Paused,
    GameOver,
}

#[derive(Debug, Clone)]
pub struct Game {
    pub distance: f32,
    pub time: f64,
    pub delta_time: f32,
    pub background: Background,
    pub player: Player,
    pub obstacles: ObstaclePool,
    pub spawn_time: f64,
    pub state: GameState,
    pub round_time: f32,
    pub size: Vec2,
    pub post_processing_material: Material,
}

impl Game {
    pub fn tick(&mut self) {
        if !self.is_game_over() {
            self.time = get_time();
            self.delta_time = get_frame_time();
        }

        if self.is_running() {
            self.player.tick();
            self.obstacles.tick();
            self.round_time += self.delta_time;
        }

        let size = vec2(screen_width(), screen_height());
        if self.size != size {
            self.size = size;
        }
    }

    pub fn half_size(&self) -> Vec2 {
        return self.size * 0.5;
    }

    fn is_running(&self) -> bool {
        self.state == GameState::Running
    }

    fn is_paused(&self) -> bool {
        self.state == GameState::Paused
    }

    fn is_game_over(&self) -> bool {
        self.state == GameState::GameOver
    }

    pub fn step(&mut self) {
        if self.is_paused() || self.is_game_over() {
            return;
        }
        self.player.step(self.round_time);
        self.obstacles.step(self.player.speed);
        self.spawn_attempt();
        self.distance += self.player.speed;

        if self.obstacles.has_collision(&self.player) {
            self.state = GameState::GameOver;
            return;
        }
    }

    fn render_post_processing(&self, texture: Texture2D) {
        set_default_camera();
        self.post_processing_material
            .set_uniform("iTime", self.time as f32);
        self.post_processing_material
            .set_uniform("iResolution", vec2(self.size.x as f32, self.size.y as f32));

        gl_use_material(self.post_processing_material);
        draw_texture_ex(
            texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                flip_y: true,
                ..Default::default()
            },
        );
        gl_use_default_material();
    }

    pub fn render(&mut self, _alpha: f32) {
        let rect = Rect::new(0., 0., self.size.x, self.size.y);
        let mut camera = Camera2D::from_display_rect(rect);
        let game_render_target = render_target(self.size.x as u32, self.size.y as u32);
        game_render_target.texture.set_filter(FilterMode::Linear);
        camera.render_target = Some(game_render_target);
        set_camera(&camera);

        if self.is_paused() {
            self.background.render(self.time as f32, self.size);
            self.player.render();
            self.state = if self.render_menu() {
                GameState::Paused
            } else {
                GameState::Running
            };
            self.render_post_processing(game_render_target.texture);
            return;
        }

        if self.is_game_over() {
            self.background.render(self.time as f32, self.size);
            self.player.render();
            self.obstacles.render();
            let restart = self.render_game_over();
            if restart {
                self.reset();
                self.state = GameState::Running;
            }
            self.render_post_processing(game_render_target.texture);
            return;
        }

        self.background.render(self.time as f32, self.size);
        self.player.render();
        self.obstacles.render();
        self.render_post_processing(game_render_target.texture);
        self.render_info();
    }

    fn render_info(&self) {
        let mut x = 32.;
        let y = 64.;
        let font_size = 64.;
        let distance_text = format!("{}m", self.distance.round() as i32);
        let distance_text_measure = measure_text(&distance_text, None, font_size as u16, 1.);
        draw_text(&distance_text, x, y, font_size, PALETTE[15]);
        x += distance_text_measure.width + font_size * 0.5;

        draw_text(
            &format!("{}s", self.round_time.round() as i32),
            x,
            y,
            font_size,
            PALETTE[15],
        );
    }

    fn render_menu(&self) -> bool {
        let half_width = screen_width() * 0.5;
        let mut x;
        let mut y = screen_height() * 0.5;

        // DRAW TITLE
        let title = "JOS HARINK'S";
        let title_measure = measure_text(title, None, 64, 1.);
        x = half_width - title_measure.width * 0.5;
        y -= title_measure.height * 0.5;
        draw_text(title, x, y, 64., PALETTE[15]);
        y += title_measure.height * 1.8;

        //DRAW SUBTITLE
        let subtitle = "MTB MASTER";
        let subtitle_measure = measure_text(subtitle, None, 96, 1.);
        x = half_width - subtitle_measure.width * 0.5;
        draw_text(subtitle, x, y, 96., PALETTE[15]);
        y += subtitle_measure.height * 2.;

        // DRAW INSTRUCTIONS
        let press_to_start = "CLICK/TOUCH TO START";
        let press_to_start_measure = measure_text(press_to_start, None, 64, 1.);
        x = half_width - press_to_start_measure.width * 0.5;
        draw_text(press_to_start, x, y, 64., PALETTE[15]);

        !is_mouse_button_pressed(MouseButton::Left)
    }

    fn render_game_over(&self) -> bool {
        let half_width = screen_width() * 0.5;
        let mut x;
        let mut y = screen_height() * 0.5;

        // DRAW TITLE
        let title = "GAME OVER";
        let title_measure = measure_text(title, None, 96, 1.);
        x = half_width - title_measure.width * 0.5;
        y -= title_measure.height * 0.5;
        draw_text(title, x, y, 96., PALETTE[15]);
        y += title_measure.height * 1.4;

        //DRAW SCORE
        let score = format!(
            "YOU BIKED {:?} METERS IN {:?} SECONDS!",
            self.distance.round() as i32,
            self.round_time.round() as i32
        );
        let score_measure = measure_text(&score, None, 64, 1.);
        x = half_width - score_measure.width * 0.5;
        draw_text(&score, x, y, 64., PALETTE[15]);
        y += score_measure.height * 2.;

        // DRAW INSTRUCTIONS
        let press_to_start = "CLICK/TOUCH TO RESTART";
        let press_to_start_measure = measure_text(press_to_start, None, 48, 1.);
        x = half_width - press_to_start_measure.width * 0.5;
        draw_text(press_to_start, x, y, 48., PALETTE[15]);

        is_mouse_button_pressed(MouseButton::Left)
    }

    fn spawn_attempt(&mut self) {
        if self.round_time % self.spawn_time as f32 <= TIMESTEP {
            self.obstacles.spawn();
        }
    }

    fn reset(&mut self) {
        self.round_time = 0.;
        self.spawn_time = 2.;
        self.player.reset();
        self.obstacles.reset();
        self.distance = 0.;
    }
}

impl Default for Game {
    fn default() -> Self {
        let size = vec2(screen_width(), screen_height());
        Self {
            time: 0.,
            delta_time: 0.,
            round_time: 0.,
            background: Background::default(),
            player: Player::new(32., 32.),
            obstacles: ObstaclePool::new(10),
            distance: 0.,
            spawn_time: 2.,
            state: GameState::Paused,
            size,
            post_processing_material: get_post_processing_material(),
        }
    }
}