pub const BACKGROUND_FRAGMENT_SHADER: &'static str = r#"#version 100
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

pub const BACKGROUND_VERTEX_SHADER: &'static str = r#"#version 100
attribute vec3 position;
attribute vec2 texcoord;
uniform mat4 Model;
uniform mat4 Projection;
void main() {
    gl_Position = Projection * Model * vec4(position, 1);
}
"#;
