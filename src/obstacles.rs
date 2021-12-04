use crate::{constants::*, geometry::Rectangle, player::Player};
use macroquad::{prelude::*, rand::gen_range};

#[derive(Debug, Clone)]
struct Obstacle {
    size: Vec2,
    position: Vec2,
}

impl Obstacle {
    pub fn new(size: Vec2, resolution: Vec2) -> Self {
        Self {
            size,
            position: vec2(resolution.x + size.x, resolution.y - size.y),
        }
    }

    pub fn tick(&mut self) {}

    pub fn step(&mut self, player_speed: f32) {
        self.position.x -= player_speed * PLAYER_SPEED_TO_OBSTACLE_SPEED;
    }

    pub fn render(&self, rock: &Texture2D) {
        let texture_pos = self.position;
        draw_texture_ex(
            *rock,
            texture_pos.x,
            texture_pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.size),
                ..Default::default()
            },
        );
    }

    fn get_aabb(&self) -> Rectangle {
        Rectangle::new(self.position, self.size)
    }
}

#[derive(Clone)]
pub struct ObstaclePool {
    obstacles: Vec<Obstacle>,
    base_chance: f32,
    spawn_chance: f32,
    spawn_interval: f32,
    last_spawn: f32,
    obstacle_size: Vec2,
    max_obstacle_size: Vec2,
    settings: ObstaclePoolSettings,
}

#[derive(Clone)]
pub struct ObstaclePoolSettings {
    pub base_spawn_chance: f32,
    pub spawn_interval: f32,
    pub base_size: Vec2,
    pub max_size: Vec2,
}

impl Default for ObstaclePoolSettings {
    fn default() -> Self {
        Self {
            base_spawn_chance: 0.1,
            spawn_interval: 2.,
            base_size: vec2(64., 44.),
            max_size: vec2(128., 64.),
        }
    }
}

impl ObstaclePool {
    pub fn new(capacity: usize, settings: Option<ObstaclePoolSettings>) -> ObstaclePool {
        let settings = match settings {
            Some(x) => x,
            None => ObstaclePoolSettings::default(),
        };

        ObstaclePool {
            obstacles: Vec::with_capacity(capacity),
            base_chance: settings.base_spawn_chance,
            spawn_chance: settings.base_spawn_chance,
            spawn_interval: settings.spawn_interval,
            last_spawn: 0.,
            obstacle_size: settings.base_size,
            max_obstacle_size: settings.max_size,
            settings,
        }
    }

    pub fn spawn_attempt(&mut self, resolution: Vec2, round_time: f32) {
        let dt = round_time - self.last_spawn;
        if dt < self.spawn_interval {
            return;
        }

        let roll = gen_range(0f32, 1f32);
        if roll <= self.spawn_chance {
            self.last_spawn = round_time;
            self.base_chance += 0.01;
            self.base_chance = self.base_chance.min(0.9);
            self.spawn_chance = self.base_chance;

            let percent: i32 = gen_range(0, 100);
            self.obstacle_size += match percent {
                0..33 => vec2(0., 1.),
                33..66 => vec2(1., 0.),
                _ => vec2(1., 1.),
            };

            self.obstacle_size = self.obstacle_size.min(self.max_obstacle_size);
            self.obstacles
                .push(Obstacle::new(self.obstacle_size, resolution));
        } else {
            self.spawn_chance += 0.05;
        }
    }

    pub fn reset(&mut self) {
        self.spawn_chance = self.settings.base_spawn_chance;
        self.base_chance = self.spawn_chance;
        self.spawn_interval = self.settings.spawn_interval;
        self.obstacle_size = self.settings.base_size;
        self.last_spawn = 0.;
        self.obstacles.clear();
    }

    pub fn tick(&mut self) {
        for obstacle in &mut self.obstacles {
            obstacle.tick();
        }
    }

    pub fn step(&mut self, player_speed: f32) {
        for obstacle in &mut self.obstacles {
            obstacle.step(player_speed);
        }
        self.obstacles.drain_filter(|o| o.position.x < -o.size.x);
        self.spawn_interval -= TIMESTEP / 100.;
        self.spawn_interval = self.spawn_interval.max(0.5);
    }

    pub fn render(&self, texture: &Texture2D) {
        for obstacle in &self.obstacles {
            obstacle.render(texture);
        }
    }

    pub fn has_collision(&self, player: &Player) -> bool {
        let player_aabb = player.get_aabb();

        for obstacle in &self.obstacles {
            let obstacle_aabb = obstacle.get_aabb();
            if player_aabb.collides_with(&obstacle_aabb) {
                return true;
            }
        }
        false
    }
}
