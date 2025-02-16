#version 450 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoord;

out vec2 TexCoord;
out vec3 Normal;
out vec3 FragPos;  // position of the fragment in world space, for lighting calculations

layout (std140, binding = 0) uniform Camera {
    mat4 view;
    mat4 projection;
    vec4 position;
} camera;

uniform mat4 model;

void main()
{
    TexCoord = aTexCoord;
    Normal = transpose(inverse(mat3(model))) * aNormal;
    FragPos = vec3(model * vec4(aPos, 1.0));
    gl_Position = camera.projection * camera.view * model * vec4(aPos, 1.0);
}
