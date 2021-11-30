#version 100
precision lowp float;

varying vec2 uv;

uniform float iTime;
uniform vec2 iResolution;
uniform sampler2D rgba_noise_texture;
uniform vec3 color_sun;
uniform vec2 position_sun;
uniform float radius_sun;
uniform vec3 color_mountain_1;
uniform vec3 color_mountain_2;
uniform vec3 sky_gradient_start;
uniform vec3 sky_gradient_end;
uniform float world_time;
uniform float player_speed;

float grad(float p) {
    const float texture_width = 256.0;
    float v = texture2D(rgba_noise_texture, vec2(p / texture_width, p)).r;
    return v > 0.5 ? 1.0 : -1.0;
}

/* S-shaped curve for 0 <= t <= 1 */
float fade(float t) {
    return t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
}

/* 1D noise */
float noise(float p) {
    float p0 = floor(p);
    float p1 = p0 + 1.0;

    float t = p - p0;
    float fade_t = fade(t);

    float g0 = grad(p0);
    float g1 = grad(p1);

    return (1.0 - fade_t) * g0 * (p - p0) + fade_t * g1 * (p - p1);
}

float mountain2(float x) {
    float position = iTime * 12.5 * player_speed + x;
    return 0.5 * (noise(position * (1.0 / 300.0)) * 1.0 +
        noise(position * (1.0 / 150.0)) * 0.5 +
        noise(position * (1.0 / 75.0)) * 0.25 +
        noise(position * (1.0 / 37.5)) * 0.125);
}

float mountain1(float x) {
    float position = iTime * 50.0 * player_speed + x;
    return noise(position * (1.0 / 300.0)) * 1.0 +
        noise(position * (1.0 / 150.0)) * 0.5 +
        noise(position * (1.0 / 75.0)) * 0.25 +
        noise(position * (1.0 / 37.5)) * 0.125;
}

vec3 rgb(float r, float g, float b) {
    return vec3(r / 255.0, g / 255.0, b / 255.0);
}

vec4 circle(vec2 pos, float rad, vec3 color) {
    float d = length(pos - gl_FragCoord.xy) - rad;
    float t = clamp(d, 0., 1.);
    return vec4(color, 1. - t);
}

void main() {
    float n1 = mountain1(gl_FragCoord.x);
    float n2 = mountain2(gl_FragCoord.x);
    float ypos = gl_FragCoord.y / iResolution.y;
    float y = 2.0 * (ypos) - 1.0;
    vec3 color = mix(sky_gradient_end, sky_gradient_start, ypos / 1.2);
    vec4 sun = circle(position_sun, radius_sun, color_sun);
    color = mix(color, sun.rgb, sun.a);
    if(n2 > y)
        color = color_mountain_2;
    if(n1 > y)
        color = color_mountain_1;
    gl_FragColor = vec4(color, 1.);
}