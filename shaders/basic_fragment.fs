#version 450 core

struct Material {
    sampler2D diffuse;
    sampler2D specular;
    int shininess;
};

struct DirectionalLight {
    vec3 direction;
    vec3 color;
    float ambient_strength;
    float specular_strength;
};

struct PointLight {
    vec3 position;

    float constant;
    float linear;
    float quadratic;

    vec3 color;
    float ambient_strength;
    float specular_strength;
};

struct SpotLight {
    vec3 position;
    vec3 direction;

    float cutOff;
    float outerCutOff;

    float constant;
    float linear;
    float quadratic;

    vec3 color;
};

in vec2 TexCoord;
in vec3 Normal;
in vec3 FragPos;

out vec4 FragColor;

uniform vec3 viewPos;
uniform bool isFloor;
uniform vec3 floorColor;
uniform Material material;
uniform DirectionalLight directionalLight;
#define NR_POINT_LIGHTS 4
uniform PointLight pointLights[NR_POINT_LIGHTS];
uniform SpotLight flashlight;

vec3 CalculateDirectionalLight(DirectionalLight light, vec3 normal, vec3 viewDir, vec3 diffuse_color, vec3 specular_color) {
    vec3 ambient = light.ambient_strength * light.color * diffuse_color;

    vec3 lightDir = normalize(-light.direction);
    float diff = max(dot(normal, lightDir), 0.0);
    vec3 diffuse = diff * light.color * diffuse_color;

    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specular = light.specular_strength * spec * light.color * specular_color;

    return light.ambient_strength * (ambient + diffuse + specular);
}

vec3 CalculatePointLight(PointLight light, vec3 normal, vec3 viewDir, vec3 diffuse_color, vec3 specular_color) {
    vec3 lightDir = normalize(light.position - FragPos);
    float diff = max(dot(normal, lightDir), 0.0);

    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);

    float distance = length(light.position - FragPos);
    float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));

    vec3 ambient = light.color * diffuse_color * light.ambient_strength;
    vec3 diffuse = diff * light.color * diffuse_color;
    vec3 specular = light.specular_strength * light.color * specular_color * spec;
    ambient *= attenuation;
    diffuse *= attenuation;
    specular *= attenuation;

    return (ambient + diffuse + specular);
}

vec3 CalculateSpotlight(SpotLight light, vec3 normal, vec3 viewDir, vec3 diffuse_color, vec3 specular_color) {
    vec3 lightDir = normalize(light.position - FragPos);
    float diff = max(dot(normal, lightDir), 0.0);

    float theta = dot(lightDir, normalize(-light.direction));
    float epsilon = light.cutOff - light.outerCutOff;
    float intensity = clamp((theta - light.outerCutOff) / epsilon, 0.0, 1.0);

    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);

    float distance = length(light.position - FragPos);
    float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));

    vec3 diffuse =  light.color * diffuse_color * diff;
    vec3 specular = light.color * specular_color * spec;
    diffuse *= intensity * attenuation;
    specular *= intensity * attenuation;

    return (diffuse + specular);
}

void main()
{
    // Base fragment color -> is it a floor or a model?
    vec3 diffuse_color;
    if(!isFloor)
        diffuse_color = texture(material.diffuse, TexCoord).rgb;
    else
        diffuse_color = floorColor;

    vec3 specular_color;
    if(!isFloor)
        specular_color = texture(material.specular, TexCoord).rgb;
    else
        specular_color = floorColor;

    vec3 normal = normalize(Normal);
    vec3 viewDir = normalize(viewPos - FragPos);

    vec3 light;
    // Directional light
    light += CalculateDirectionalLight(directionalLight, normal, viewDir, diffuse_color, specular_color);

    // Point lights
    for(int i = 0; i < NR_POINT_LIGHTS; i++)
        light += CalculatePointLight(pointLights[i], normal, viewDir, diffuse_color, specular_color);

    // Spotlight
    light += CalculateSpotlight(flashlight, normal, viewDir, diffuse_color, specular_color);

    FragColor = vec4(light, 1.0);
}
