#version 450 core

in vec2 TexCoord;
in vec3 Normal;
in vec3 FragPos;

out vec4 FragColor;

uniform sampler2D texture1;
uniform sampler2D texture2;
uniform vec3 lightColor;
uniform vec3 lightPos;
uniform vec3 viewPos;
uniform bool isFloor;
uniform vec3 floorColor;

uniform int shininess;

void main()
{
    // Ambient
    float ambientStrength = 0.1;
    vec3 ambient_light = ambientStrength * lightColor;

    // Diffuse
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(lightPos - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColor;

    // Specular
    float specularStrength = 0.5;
    vec3 viewDir = normalize(viewPos - FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), shininess);
    vec3 specular = specularStrength * spec * lightColor;

    // Final light color
    vec3 light = ambient_light + diffuse + specular;

    if(!isFloor)
        FragColor = mix(texture(texture1, TexCoord), texture(texture2, TexCoord), 0.2) * vec4(light, 1.0);
    else
        FragColor = vec4(floorColor * light, 1.0);
}
