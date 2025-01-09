#version 450 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;

out vec4 ourColor;
out vec2 TexCoord;

uniform mat4 model;

void main()
{
    TexCoord = aTexCoord;
    gl_Position = model * vec4(aPos, 1.0);
}
