use crate::{constants::*, geometry::Rectangle, player::Player};
use macroquad::prelude::*;

#[derive(Debug, Clone)]
struct Obstacle {
    size: Vec2,
    position: Vec2,
    color: Color,
}

impl Obstacle {
    pub fn new(size: Vec2, color: Color) -> Self {
        Self {
            size,
            color,
            position: Vec2::new(screen_width() + size.x, screen_height() - size.y),
        }
    }

    pub fn step(&mut self, player_speed: f32) {
        self.position.x -= player_speed * PLAYER_SPEED_TO_OBSTACLE_SPEED;
    }

    pub fn render(&self) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.size.x,
            self.size.y,
            self.color,
        )
    }

    fn get_aabb(&self) -> Rectangle {
        Rectangle::new(self.position, self.size)
    }
}

#[derive(Debug, Clone)]
pub struct ObstaclePool {
    obstacles: Vec<Obstacle>,
}

impl ObstaclePool {
    pub fn new(capacity: usize) -> ObstaclePool {
        ObstaclePool {
            obstacles: Vec::with_capacity(capacity),
        }
    }

    pub fn spawn(&mut self) {
        self.obstacles
            .push(Obstacle::new(Vec2::new(64., 32.), PALETTE[5]));
    }

    pub fn reset(&mut self) {
        self.obstacles.clear();
    }

    pub fn step(&mut self, player_speed: f32) {
        for obstacle in &mut self.obstacles {
            obstacle.step(player_speed);
        }
        self.obstacles.drain_filter(|o| o.position.x < -o.size.x);
    }

    pub fn render(&self) {
        for obstacle in &self.obstacles {
            obstacle.render();
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