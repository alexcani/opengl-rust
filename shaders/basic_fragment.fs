#version 450 core

in vec2 TexCoord;

out vec4 FragColor;

uniform sampler2D texture1;
uniform sampler2D texture2;
uniform vec3 lightColor;
uniform bool isFloor;
uniform vec3 floorColor;

void main()
{
    if(!isFloor)
        FragColor = mix(texture(texture1, TexCoord), texture(texture2, TexCoord), 0.2) * vec4(lightColor, 1.0);
    else
        FragColor = vec4(floorColor * lightColor, 1.0);
}
