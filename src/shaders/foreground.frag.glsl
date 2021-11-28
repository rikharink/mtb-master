#version 100
precision lowp float;

varying vec2 uv;

uniform float iTime;
uniform vec2 iResolution;
uniform sampler2D obstacles;
uniform float darkness;
uniform vec2 headlight;
uniform vec3 headlight_color;
uniform vec2 taillight;
uniform vec3 taillight_color;

vec3 pointlight(vec3 color, float range, vec2 position) {
    vec2 directionToLight = normalize(position);
    float lightLambert = (dot(directionToLight, vec2(0., 0.)) + 1.0) * 0.5;
    float attenuation = pow(1.0 - length(position), range);
    return color * vec3(lightLambert * attenuation);
}

vec3 spotlight(vec3 color, float range, vec2 position, vec2 direction) {
    return color * pow(max(dot(normalize(direction), normalize(position)), 0.), range);
}

void main() {
    vec4 color = vec4(0., 0., 0., 0.);
    color += vec4(spotlight(headlight_color, 90., uv - headlight, vec2(.2, -.1)), darkness);
    color += vec4(pointlight(taillight_color, 100., taillight - uv), darkness);
    gl_FragColor = color;
}
