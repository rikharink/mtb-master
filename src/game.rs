use crate::{
    background::Background, constants::*, obstacles::ObstaclePool, player::Player,
    shaders::get_post_processing_material,
};
use macroquad::prelude::*;

#[derive(PartialEq, Clone)]
pub enum GameState {
    Running,
    Paused,
    GameOver,
}

#[derive(Clone)]
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
    pub resolution: Vec2,
    pub post_processing_material: Material,
    pub game_render_target: RenderTarget,
    pub camera: Camera2D,
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
    }

    pub fn half_size(&self) -> Vec2 {
        return self.resolution * 0.5;
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
        self.post_processing_material
            .set_uniform("iTime", self.time as f32);
        self.post_processing_material
            .set_uniform("iResolution", vec2(self.resolution.x as f32, self.resolution.y as f32));

        gl_use_material(self.post_processing_material);
        
        let sw = screen_width();
        let sh = screen_height();

        let aspect: f32;
        let width: f32;
        let height: f32;
        let x: f32;
        let y: f32;
        if sh >= sw {
            aspect = self.resolution.y / self.resolution.x;
            width = sw;
            height = width * aspect;
            x = 0.0;
            y = (sh - height) * 0.5;
        } else {
            aspect = self.resolution.x / self.resolution.y;
            height = sh;
            width = height * aspect;
            y = 0.0;
            x = (sw - width) * 0.5;
        }

        draw_texture_ex(
            texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(width, height)),
                flip_y: true,
                ..Default::default()
            },
        );
        gl_use_default_material();
    }

    pub fn render(&mut self, _alpha: f32) {
        set_camera(&self.camera);
        self.background.render(self.time as f32, self.resolution);
        self.player.render();
        
        if self.is_paused() || self.is_running() {
            self.obstacles.render();
        }
        set_default_camera();

        self.render_post_processing(self.game_render_target.texture);
        if self.is_paused() {
            self.state = if self.render_menu() {
                GameState::Paused
            } else {
                GameState::Running
            };
        } else if self.is_game_over() {
            let restart = self.render_game_over();
            if restart {
                self.reset();
                self.state = GameState::Running;
            }
        }
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
            self.obstacles.spawn(self.resolution);
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
        let size = vec2(1920., 1080.);
        let rect = Rect::new(0., 0., size.x, size.y);
        let mut camera = Camera2D::from_display_rect(rect);
        let game_render_target = render_target(size.x as u32, size.y as u32);
        game_render_target.texture.set_filter(FilterMode::Linear);
        camera.render_target = Some(game_render_target);

        Self {
            time: 0.,
            delta_time: 0.,
            round_time: 0.,
            background: Background::default(),
            player: Player::new(vec2(32., 32.), size),
            obstacles: ObstaclePool::new(10),
            distance: 0.,
            spawn_time: 2.,
            state: GameState::Paused,
            resolution: size,
            post_processing_material: get_post_processing_material(),
            game_render_target,
            camera,
        }
    }
}
