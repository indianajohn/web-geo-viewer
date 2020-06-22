uniform float diffuse_intensity;
uniform float specular_intensity;
uniform float specular_power;
uniform sampler2D texture0;

in vec3 nor;
in vec3 pos;
in vec3 uvw_passthrough;

layout (location = 0) out vec4 out_color;
layout (location = 1) out vec4 normal;

vec3 blendNormal(vec3 normal){
	vec3 blending = abs(normal);
	blending = normalize(max(blending, 0.00001));
	blending /= vec3(blending.x + blending.y + blending.z);
	return blending;
}

void main()
{
	vec3 n = normalize(gl_FrontFacing ? nor : -nor);
  out_color = vec4(texture(texture0, vec2(uvw_passthrough.x, 1.0 - uvw_passthrough.y)).rgb, diffuse_intensity);
	int intensity = int(floor(specular_intensity * 15.0));
	int power = int(floor(specular_power*0.5));
  normal = vec4(0.5 * n + 0.5, float(power << 4 | intensity)/255.0);
}
