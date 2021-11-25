use crate::{constants::*, shaders::*, util::*};
use lerp::Lerp;
use macroquad::prelude::*;
use std::f32::consts::TAU;

#[derive(Debug, Clone, Lerp)]
struct Sky {
    #[lerp(f32)]
    start: Vec3,
    #[lerp(f32)]
    end: Vec3,
}

impl Default for Sky {
    fn default() -> Self {
        Sky::sun_set_rise()
    }
}

impl Sky {
    pub fn day() -> Sky {
        Sky {
            start: color_to_vector(PALETTE[13]),
            end: color_to_vector(PALETTE[15]),
        }
    }

    pub fn sun_set_rise() -> Sky {
        Sky {
            start: color_to_vector(PALETTE[8]),
            end: color_to_vector(PALETTE[4]),
        }
    }

    pub fn night() -> Sky {
        Sky {
            start: color_to_vector(PALETTE[0]),
            end: color_to_vector(PALETTE[0]),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Background {
    sky: Sky,
    material: Material,
    rgba_texture: Texture2D,
}

impl Default for Background {
    fn default() -> Self {
        let rgba_texture = rgba_texture(256, 256);
        Self {
            sky: Sky::default(),
            material: get_background_material(),
            rgba_texture,
        }
    }
}

struct CelestialBody {
    position: Vec2,
    color: Vec3,
    sky: Sky,
    is_night: bool,
}

impl Background {
    fn get_celestial_body_position(
        &self,
        time: f32,
        resolution: Vec2,
        radius: f32,
        cycle_time: f32,
    ) -> Vec2 {
        let h = resolution.y - radius * 1.5;
        let w = resolution.x;
        let r = (h * 0.5) + ((w * w) / (8. * h));

        let t = (time % cycle_time) / cycle_time;
        let theta = 0f32.lerp(TAU, t);
        let origin = vec2(w * 0.5, h - r);

        let x = w - (origin.x + r * theta.cos());
        let y = origin.y + r * theta.sin();
        vec2(x, y)
    }

    fn get_celestial_body(
        &self,
        time: f32,
        resolution: Vec2,
        radius: f32,
        cycle_time: f32,
    ) -> CelestialBody {
        let position = self.get_celestial_body_position(time, resolution, radius, cycle_time);
        let is_night = position.y < 0.;
        if is_night {
            let t = time - cycle_time * 0.5;
            let position = self.get_celestial_body_position(t, resolution, radius, cycle_time);
            return CelestialBody {
                position,
                color: color_to_vector(PALETTE[12]),
                sky: Sky::night(),
                is_night: true,
            };
        }

        let day_sun = color_to_vector(PALETTE[14]);
        let set_sun = color_to_vector(PALETTE[4]);

        let t = position.y / (resolution.y - radius * 1.5);
        let color = set_sun.lerp(day_sun, t);
        let mut sky = Sky::sun_set_rise().lerp(Sky::day(), t);
        sky = Sky::night().lerp(sky, t);

        CelestialBody {
            position: vec2(position.x, position.y),
            color,
            sky,
            is_night: false,
        }
    }

    pub fn render(
        &self,
        time: f32,
        world_time: f32,
        resolution: Vec2,
        cycle_time: f32,
        player_speed: f32,
    ) {
        self.material.set_uniform("iTime", time);
        self.material.set_uniform("world_time", world_time);
        self.material
            .set_uniform("iResolution", vec2(resolution.x, resolution.y));
        let mountain1 = PALETTE[3];
        let mountain2 = PALETTE[6];

        self.material.set_uniform(
            "color_mountain_1",
            Vec3::new(mountain1.r, mountain1.g, mountain1.b),
        );
        self.material.set_uniform(
            "color_mountain_2",
            Vec3::new(mountain2.r, mountain2.g, mountain2.b),
        );

        let radius_celestial_body = 50f32;
        let celestial_body =
            self.get_celestial_body(world_time, resolution, radius_celestial_body, cycle_time);
        self.material
            .set_uniform("sky_gradient_start", celestial_body.sky.start);
        self.material
            .set_uniform("sky_gradient_end", celestial_body.sky.end);
        self.material.set_uniform("color_sun", celestial_body.color);
        self.material
            .set_uniform("radius_sun", radius_celestial_body);
        self.material
            .set_uniform("position_sun", celestial_body.position);
        self.material
            .set_uniform("is_night", celestial_body.is_night as u32);
        self.material.set_uniform("player_speed", 1. + player_speed);
        self.material
            .set_texture("rgba_noise_texture", self.rgba_texture);
        gl_use_material(self.material);
        draw_rectangle(0., 0., resolution.x, resolution.y, WHITE);
        gl_use_default_material();
    }
}
