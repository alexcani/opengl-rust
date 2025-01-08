#version 450 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec2 aTexCoord;

out vec4 ourColor;
out vec2 TexCoord;

uniform mat4 model;

void main()
{
    ourColor = vec4(aColor, 1.0);
    TexCoord = aTexCoord;
    gl_Position = model * vec4(aPos, 1.0);
}
