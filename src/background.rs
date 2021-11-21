use crate::{constants::*, shaders::*, util::*};
use macroquad::prelude::*;

#[derive(Debug, Clone)]
struct Sky {
    start: Color,
    end: Color,
}

impl Default for Sky {
    fn default() -> Self {
        Sky::night()
    }
}

impl Sky {
    pub fn day() -> Sky {
        Sky {
            start: PALETTE[13],
            end: PALETTE[15],
        }
    }

    pub fn night() -> Sky {
        Sky {
            start: PALETTE[8],
            end: PALETTE[4],
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
        let background_fragment_shader = BACKGROUND_FRAGMENT_SHADER.to_string();
        let background_vertex_shader = BACKGROUND_VERTEX_SHADER.to_string();

        let pipeline_params = PipelineParams {
            depth_write: true,
            depth_test: Comparison::LessOrEqual,
            ..Default::default()
        };

        let material = load_material(
            &background_vertex_shader,
            &background_fragment_shader,
            MaterialParams {
                pipeline_params,
                uniforms: vec![
                    ("iTime".to_string(), UniformType::Float1),
                    ("iResolution".to_string(), UniformType::Float2),
                    ("iMountain1".to_string(), UniformType::Float3),
                    ("iMountain2".to_string(), UniformType::Float3),
                    ("iGradientStart".to_string(), UniformType::Float3),
                    ("iGradientEnd".to_string(), UniformType::Float3),
                ],
                textures: vec!["iChannel0".to_string()],
            },
        )
        .unwrap();

        Self {
            sky: Sky::default(),
            material,
            rgba_texture,
        }
    }
}

impl Background {
    pub fn render(&self, time: f32) {
        self.material.set_uniform("iTime", time);
        self.material
            .set_uniform("iResolution", Vec2::new(screen_width(), screen_height()));
        let mountain1 = PALETTE[3];
        let mountain2 = PALETTE[6];

        self.material.set_uniform(
            "iGradientStart",
            Vec3::new(self.sky.start.r, self.sky.start.g, self.sky.start.b),
        );
        self.material.set_uniform(
            "iGradientEnd",
            Vec3::new(self.sky.end.r, self.sky.end.g, self.sky.end.b),
        );
        self.material.set_uniform(
            "iMountain1",
            Vec3::new(mountain1.r, mountain1.g, mountain1.b),
        );
        self.material.set_uniform(
            "iMountain2",
            Vec3::new(mountain2.r, mountain2.g, mountain2.b),
        );

        self.material.set_texture("iChannel0", self.rgba_texture);
        gl_use_material(self.material);
        draw_rectangle(0., 0., screen_width(), screen_height(), WHITE);
        gl_use_default_material();
    }
}
