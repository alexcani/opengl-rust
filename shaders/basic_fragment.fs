#version 450 core

in vec4 ourColor;
out vec4 FragColor;

uniform float time;

void main()
{
    float multiplier = 0.5 + 0.5 * sin(time);
    FragColor = vec4(ourColor.xyz * multiplier, 1.0);
}
