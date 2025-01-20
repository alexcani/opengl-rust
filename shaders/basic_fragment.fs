#version 450 core

struct Material {
    sampler2D diffuse;
    sampler2D specular;
    int shininess;
};

struct Light {
    vec3 position;
    vec3 color;
    float ambient_strength;
    float specular_strength;
};

in vec2 TexCoord;
in vec3 Normal;
in vec3 FragPos;

out vec4 FragColor;

uniform vec3 viewPos;
uniform bool isFloor;
uniform vec3 floorColor;
uniform Material material;
uniform Light light;

void main()
{
    vec3 diffuse_color;
    if(!isFloor)
        diffuse_color = texture(material.diffuse, TexCoord).rgb;
    else
        diffuse_color = floorColor;

    // Ambient
    vec3 ambient_light = light.ambient_strength * light.color * diffuse_color;

    // Diffuse
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(light.position - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * light.color * diffuse_color;

    // Specular
    vec3 specular_color;
    if(!isFloor)
        specular_color = texture(material.specular, TexCoord).rgb;
    else
        specular_color = floorColor;
    vec3 viewDir = normalize(viewPos - FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specular = light.specular_strength * spec * light.color * specular_color;

    // Final light color
    vec3 light = ambient_light + diffuse + specular;

    FragColor = vec4(light, 1.0);
}
