#version 450 core

struct Material {
    sampler2D diffuse;
    sampler2D specular;
    int shininess;
};

struct AmbientLight {
    vec4 color;
    float intensity;
};

struct DirectionalLight {
    vec4 color;
    vec4 direction;
    float intensity;
};

struct PointLight {
    vec4 color;
    vec4 position;

    float constant;
    float linear;
    float quadratic;
    float intensity;
};

struct SpotLight {
    vec4 color;
    vec4 position;
    vec4 direction;

    float cutOff_cos;
    float outerCutOff_cos;

    float constant;
    float linear;
    float quadratic;

    float intensity;
};

in vec2 TexCoord;
in vec3 Normal;
in vec3 FragPos;

out vec4 FragColor;

layout (std140, binding = 0) uniform Camera {
    mat4 view;
    mat4 projection;
    vec4 position;
} camera;

#define MAX_POINT_LIGHTS 10
#define MAX_SPOT_LIGHTS 5
#define MAX_DIRECTIONAL_LIGHTS 5
layout (std140, binding = 1) uniform LightData {
    AmbientLight ambient;
    DirectionalLight directionalLights[MAX_DIRECTIONAL_LIGHTS];
    PointLight pointLights[MAX_POINT_LIGHTS];
    SpotLight spotLights[MAX_SPOT_LIGHTS];
    int nrPointLights;
    int nrSpotLights;
    int nrDirectionalLights;
} lights;

uniform bool isFloor;
uniform vec3 floorColor;
uniform Material material;

vec3 CalculateDirectionalLight(DirectionalLight light, vec3 normal, vec3 viewDir, vec3 diffuse_color, vec3 specular_color) {
    vec3 lightDir = normalize(-light.direction.xyz);
    float diff = max(dot(normal, lightDir), 0.0);
    vec3 diffuse = diff * light.color.rgb * diffuse_color;

    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specular = spec * light.color.rgb * specular_color;

    return light.intensity * (diffuse + specular);
}

vec3 CalculatePointLight(PointLight light, vec3 normal, vec3 viewDir, vec3 diffuse_color, vec3 specular_color) {
    vec3 lightDir = normalize(light.position.xyz - FragPos);
    float diff = max(dot(normal, lightDir), 0.0);

    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);

    float distance = length(light.position.xyz - FragPos);
    float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));

    vec3 diffuse = diff * light.color.rgb * diffuse_color;
    vec3 specular = light.color.rgb * specular_color * spec;

    return light.intensity * attenuation * (diffuse + specular);
}

vec3 CalculateSpotlight(SpotLight light, vec3 normal, vec3 viewDir, vec3 diffuse_color, vec3 specular_color) {
    vec3 lightDir = normalize(light.position.xyz - FragPos);
    float diff = max(dot(normal, lightDir), 0.0);

    float theta = dot(lightDir, normalize(-light.direction.xyz));
    float epsilon = light.cutOff_cos - light.outerCutOff_cos;
    float intensity = clamp((theta - light.outerCutOff_cos) / epsilon, 0.0, 1.0);

    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);

    float distance = length(light.position.xyz - FragPos);
    float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));

    vec3 diffuse =  light.color.rgb * diffuse_color * diff;
    vec3 specular = light.color.rgb * specular_color * spec;

    return (diffuse + specular) * intensity * attenuation * light.intensity;
}

void main()
{
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

    vec3 viewPos = camera.position.xyz;
    vec3 normal = normalize(Normal);
    vec3 viewDir = normalize(viewPos - FragPos);

    vec3 light;
    // Directional lights
    for(int i = 0; i < lights.nrDirectionalLights; i++)
        light += CalculateDirectionalLight(lights.directionalLights[i], normal, viewDir, diffuse_color, specular_color);

    // Point lights
    for(int i = 0; i < lights.nrPointLights; i++)
        light += CalculatePointLight(lights.pointLights[i], normal, viewDir, diffuse_color, specular_color);

    // Spot lights
    for(int i = 0; i < lights.nrSpotLights; i++)
        light += CalculateSpotlight(lights.spotLights[i], normal, viewDir, diffuse_color, specular_color);

    // Ambient light
    light += lights.ambient.color.rgb * lights.ambient.intensity * diffuse_color;

    FragColor = vec4(light, 1.0);
}
