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
        // draw_rectangle(
        //     self.position.x,
        //     self.position.y,
        //     self.size.x,
        //     self.size.y,
        //     self.color,
        // )
    }

    fn get_aabb(&self) -> Rectangle {
        Rectangle::new(self.position, self.size)
    }
}

#[derive(Debug, Clone)]
pub struct ObstaclePool {
    obstacles: Vec<Obstacle>,
    base_chance: f32,
    spawn_chance: f32,
    spawn_interval: f32,
    last_spawn: f32,
    obstacle_size: Vec2
}

impl ObstaclePool {
    pub fn new(capacity: usize) -> ObstaclePool {
        ObstaclePool {
            obstacles: Vec::with_capacity(capacity),
            base_chance: 0.1,
            spawn_chance: 0.1,
            spawn_interval: 2.,
            last_spawn: 0.,
            obstacle_size: vec2(64., 44.),
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
            self.obstacle_size += match percent 
            { 
                0..33 => vec2(0., 1.),
                33..66 => vec2(1., 0.),
                _ => vec2(1., 1.),
            };

            self.obstacle_size = self.obstacle_size.min(vec2(128., 64.));
            self.obstacles.push(Obstacle::new(self.obstacle_size, resolution));
        } else {
            self.spawn_chance += 0.05;
        }
    }

    pub fn reset(&mut self) {
        self.spawn_chance = 0.1;
        self.base_chance = 0.1;
        self.spawn_interval = 2.;
        self.last_spawn = 0.;
        self.obstacle_size = vec2(64., 44.);
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

    pub fn render(&self, rock: &Texture2D) {
        for obstacle in &self.obstacles {
            obstacle.render(rock);
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
