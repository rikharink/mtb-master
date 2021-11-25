#version 100
precision lowp float;

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
    float vignetteValue = vignette(uv, radius, smoothness);
    vec4 color = texture2D(Texture, uv);
    color *= vignetteValue;
    gl_FragColor = color;
}