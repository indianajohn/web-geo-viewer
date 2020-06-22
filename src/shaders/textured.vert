uniform mat4 modelMatrix;
uniform mat4 normalMatrix;

layout (std140) uniform Camera
{
    mat4 viewProjection;
    mat4 view;
    mat4 projection;
    vec3 position;
    float padding;
} camera;

in vec3 position;
in vec3 normal;
in vec3 uvw;

out vec3 pos;
out vec3 nor;
out vec3 uvw_passthrough;

void main()
{
    vec4 worldPosition = modelMatrix * vec4(position, 1.);
    nor = mat3(normalMatrix) * normal;
    pos = worldPosition.xyz;
    uvw_passthrough = uvw;
    gl_Position = camera.viewProjection * worldPosition;
}
