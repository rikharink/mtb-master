use macroquad::prelude::*;

const BACKGROUND_FRAGMENT_SHADER: &'static str = r#"#version 100
precision mediump float;

uniform float iTime;
uniform vec2 iResolution;
uniform sampler2D iChannel0;
uniform vec3 iMountain1;
uniform vec3 iMountain2;
uniform vec3 iGradientStart;
uniform vec3 iGradientEnd;

float grad(float p) {
	const float texture_width = 256.0;
	float v = texture2D(iChannel0, vec2(p / texture_width, p)).r;
    return v > 0.5 ? 1.0 : -1.0;
}

/* S-shaped curve for 0 <= t <= 1 */
float fade(float t) {
  return t*t*t*(t*(t*6.0 - 15.0) + 10.0);
}


/* 1D noise */
float noise(float p) {
  float p0 = floor(p);
  float p1 = p0 + 1.0;
    
  float t = p - p0;
  float fade_t = fade(t);

  float g0 = grad(p0);
  float g1 = grad(p1);

  return (1.0-fade_t)*g0*(p - p0) + fade_t*g1*(p - p1);
}

float mountain2(float x) {
	float position = iTime * 12.5 + x;
    return 
        0.5*(noise(position * (1.0/300.0)) * 1.0 +
        	 noise(position * (1.0/150.0)) * 0.5 +
        	 noise(position * (1.0/75.0))  * 0.25 +
        	 noise(position * (1.0/37.5))  * 0.125);
}

float mountain1(float x) {
	float position = iTime * 50.0 + x;
    return 
        noise(position * (1.0/300.0)) * 1.0 +
        noise(position * (1.0/150.0)) * 0.5 +
        noise(position * (1.0/75.0))  * 0.25 +
        noise(position * (1.0/37.5))  * 0.125;
}

float mountain3(float x) {
	float position = iTime * 124.3 + x;
    return 
        noise(position * (1.0/300.0)) * 1.0 +
        noise(position * (1.0/150.0)) * 0.5 +
        noise(position * (1.0/75.0))  * 0.25 +
        noise(position * (1.0/37.5))  * 0.125;
}

void main(){
    float n1 = mountain1(gl_FragCoord.x);
    float n2 = mountain2(gl_FragCoord.x);
    float ypos = gl_FragCoord .y/iResolution.y;
    float y = 2.0 * (ypos) - 1.0; /* map gl_FragCoord .y into [-1; 1] range */
    vec3 color = mix(iGradientEnd, iGradientStart, ypos / 1.2);
    if(n2 > y) color = iMountain2;
    if(n1 >  y) color = iMountain1;

	gl_FragColor = vec4(color, 1.0);
}
"#;

const DEFAULT_VERTEX_SHADER: &'static str = r#"#version 100
precision mediump float;

attribute vec3 position;
attribute vec2 texcoord;

varying vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    uv = texcoord;
}
"#;

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
                ("iMountain1".to_string(), UniformType::Float3),
                ("iMountain2".to_string(), UniformType::Float3),
                ("iGradientStart".to_string(), UniformType::Float3),
                ("iGradientEnd".to_string(), UniformType::Float3),
            ],
            textures: vec!["iChannel0".to_string()],
        },
    )
    .unwrap()
}

const POST_PROCESSSING_FRAGMENT_SHADER: &'static str = r#"#version 100
precision mediump float;

varying vec2 uv;

uniform float iTime;
uniform vec2 iResolution;
uniform float radius;
uniform float smoothness;
uniform sampler2D Texture;

float vignette(vec2 uv, float radius, float smoothness) {
	float diff = radius - distance(uv, vec2(0.5, 0.5));
	return smoothstep(-smoothness, smoothness, diff);
}

void main() {
    float vignetteValue = vignette(uv, 0.5, 0.5);
    vec4 color = texture2D(Texture, uv);
    color *= vignetteValue;
    gl_FragColor = color;
}
"#;

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
            ],
            ..Default::default()
        },
    )
    .unwrap();
    material.set_uniform("radius", 0.95f32);
    material.set_uniform("smoothness", 0.9f32);
    material
}