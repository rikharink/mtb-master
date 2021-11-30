use crate::{constants::*, util::color_to_vector};
use macroquad::prelude::*;

pub fn get_background_material() -> Material {
    let background_fragment_shader = BACKGROUND_FRAGMENT_SHADER.to_string();
    let background_vertex_shader = DEFAULT_VERTEX_SHADER.to_string();

    let pipeline_params = PipelineParams {
        ..Default::default()
    };

    load_material(
        &background_vertex_shader,
        &background_fragment_shader,
        MaterialParams {
            pipeline_params,
            uniforms: vec![
                ("iTime".to_string(), UniformType::Float1),
                ("iResolution".to_string(), UniformType::Float2),
                ("color_mountain_1".to_string(), UniformType::Float3),
                ("color_mountain_2".to_string(), UniformType::Float3),
                ("sky_gradient_start".to_string(), UniformType::Float3),
                ("sky_gradient_end".to_string(), UniformType::Float3),
                ("color_sun".to_string(), UniformType::Float3),
                ("position_sun".to_string(), UniformType::Float2),
                ("radius_sun".to_string(), UniformType::Float1),
                ("world_time".to_string(), UniformType::Float1),
                ("player_speed".to_string(), UniformType::Float1),
            ],
            textures: vec!["rgba_noise_texture".to_string()],
        },
    )
    .unwrap()
}

pub fn get_post_processing_material() -> Material {
    let fragment_shader = POST_PROCESSSING_FRAGMENT_SHADER.to_string();
    let vertex_shader = DEFAULT_VERTEX_SHADER.to_string();

    let pipeline_params = PipelineParams {
        ..Default::default()
    };

    let material = load_material(
        &vertex_shader,
        &fragment_shader,
        MaterialParams {
            pipeline_params,
            uniforms: vec![
                ("iTime".to_string(), UniformType::Float1),
                ("iResolution".to_string(), UniformType::Float2),
                ("radius".to_string(), UniformType::Float1),
                ("smoothness".to_string(), UniformType::Float1),
                ("headlight".to_string(), UniformType::Float2),
                ("headlight_color".to_string(), UniformType::Float3),
                ("taillight".to_string(), UniformType::Float2),
                ("taillight_color".to_string(), UniformType::Float3),
                ("darkness".to_string(), UniformType::Float1),
            ],
            ..Default::default()
        },
    )
    .unwrap();
    material.set_uniform("headlight_color", color_to_vector(PALETTE[14]));
    material.set_uniform("taillight_color", color_to_vector(PALETTE[4]));

    material.set_uniform("radius", VIGNETTE_RADIUS);
    material.set_uniform("smoothness", VIGNETTE_SMOOTHNESS);
    material
}
