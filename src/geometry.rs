use macroquad::prelude::*;

#[derive(Debug)]
pub struct Rectangle {
    origin: Vec2,
    size: Vec2,
}

impl Rectangle {
    pub fn new(origin: Vec2, size: Vec2) -> Self {
        Self { origin, size }
    }

    pub fn from_min_max(min: Vec2, max: Vec2) -> Self {
        Self::new(min, max - min)
    }

    pub fn min(&self) -> Vec2 {
        let p1 = self.origin;
        let p2 = self.origin + self.size;
        Vec2::new(p1.x.min(p2.x), p1.y.min(p2.y))
    }

    pub fn max(&self) -> Vec2 {
        let p1 = self.origin;
        let p2 = self.origin + self.size;
        Vec2::new(p1.x.max(p2.x), p1.y.max(p2.y))
    }

    pub fn collides_with(&self, other: &Rectangle) -> bool {
        let min_self = self.min();
        let max_self = self.max();
        let min_other = other.min();
        let max_other = other.max();
        let overlap_x = (min_other.x <= max_self.x) && (min_self.x <= max_other.x);
        let overlap_y = (min_other.y <= max_self.y) && (min_self.y <= max_other.y);
        overlap_x && overlap_y
    }
}
