pub const BACKGROUND_FRAGMENT_SHADER: &'static str = r#"
float grad(float p) {
	const float texture_width = 256.0;
	float v = texture(iChannel0, vec2(p / texture_width, p)).r;
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

void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
    float n1 = mountain1(fragCoord.x);
    float n2 = mountain2(fragCoord.x);
    float n3 = mountain3(fragCoord.x);
    float ypos = fragCoord.y/iResolution.y;
    float y = 2.0 * (ypos) - 1.0; /* map fragCoord.y into [-1; 1] range */
    vec3 color = mix(mix(vec3(1.0, 1.0, 0.0), vec3(1.0, 0.0, 0.0), ypos/1.1), vec3(0.0, 0.1, 0.2), ypos / 1.2);
    if(n2 > y) color = vec3(0.2, 0.2, 0.);
    if(n1 >  y) color = vec3(0.);

	fragColor = vec4(color, 1.0);
}

void main(){

}
"#;

pub const BACKGROUND_VERTEX_SHADER: &'static str = r#"#version 100
attribute vec3 position;
attribute vec2 texcoord;
varying lowp vec2 uv;
uniform mat4 Model;
uniform mat4 Projection;
void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    uv = texcoord;
}
"#;
