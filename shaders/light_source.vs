#version 450 core
layout (location = 0) in vec3 aPos;

layout (std140, binding = 0) uniform Camera {
    mat4 view;
    mat4 projection;
    vec4 position;
} camera;

uniform mat4 model;

void main()
{
    gl_Position = camera.projection * camera.view * model * vec4(aPos, 1.0);
}
