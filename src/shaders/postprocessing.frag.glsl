#version 100
precision lowp float;

varying vec2 uv;

uniform float iTime;
uniform vec2 iResolution;
uniform float radius;
uniform float smoothness;
uniform sampler2D Texture;

uniform float darkness;
uniform vec2 headlight;
uniform vec3 headlight_color;
uniform vec2 taillight;
uniform vec3 taillight_color;

vec3 pointlight(vec3 color, float range, vec2 position) {
    vec2 directionToLight = normalize(position);
    float lightLambert = (dot(directionToLight, vec2(0., 0.)) + 1.0) * 0.5;
    float attenuation = pow(1.0 - length(position), range);
    vec3 light = vec3(lightLambert * attenuation);
    return clamp(color * light, 0., 1.);
}

vec3 spotlight(vec3 color, float range, vec2 position, vec2 direction) {
    return color * pow(max(dot(normalize(direction), normalize(position)), 0.), range);
}

float vignette(vec2 uv, float radius, float smoothness) {
    float diff = radius - distance(uv, vec2(0.5, 0.5));
    return smoothstep(-smoothness, smoothness, diff);
}

void main() {
    vec3 color = texture2D(Texture, uv).rgb;
    color += spotlight(headlight_color, 90., uv - headlight, vec2(.2, -.025)) * darkness;
    color += pointlight(taillight_color, 100., taillight - uv) * darkness;
    color *= vignette(uv, radius, smoothness);
    gl_FragColor = vec4(color, 1.);
}