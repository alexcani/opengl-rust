pub mod camera;
pub mod light;
pub mod object;

pub use camera::Camera;
pub use light::Light;
pub use object::{Object, Transform};

use std::{cell::RefCell, rc::Rc};

use crate::renderer::RenderInfo;
use crate::renderer::material::{Material, MaterialProperty};
use crate::renderer::mesh::{Mesh, Vertex};
use crate::renderer::shader::{Shader, ShaderProgram, ShaderType};
use crate::renderer::texture::Texture2D;

pub struct AmbientLight {
    pub color: glam::Vec3,
    pub intensity: f32,
}

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Rc<RefCell<Object>>>,
    pub lights: Vec<Rc<RefCell<Light>>>,
    pub ambient_light: AmbientLight,
    light_materials: Vec<Rc<RefCell<Material>>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(),
            objects: Vec::new(),
            lights: Vec::new(),
            ambient_light: AmbientLight {
                color: glam::Vec3::new(1.0, 1.0, 1.0),
                intensity: 0.0,
            },
            light_materials: Vec::new(),
        }
    }

    pub fn init(&mut self) -> Result<(), String> {
        let cube_vertices: [Vertex; 36] = [
            Vertex([-0.5, -0.5, -0.5], [0.0, 0.0, -1.0], [0.0, 0.0]),
            Vertex([0.5, -0.5, -0.5], [0.0, 0.0, -1.0], [1.0, 0.0]),
            Vertex([0.5, 0.5, -0.5], [0.0, 0.0, -1.0], [1.0, 1.0]),
            Vertex([0.5, 0.5, -0.5], [0.0, 0.0, -1.0], [1.0, 1.0]),
            Vertex([-0.5, 0.5, -0.5], [0.0, 0.0, -1.0], [0.0, 1.0]),
            Vertex([-0.5, -0.5, -0.5], [0.0, 0.0, -1.0], [0.0, 0.0]),
            Vertex([-0.5, -0.5, 0.5], [0.0, 0.0, 1.0], [0.0, 0.0]),
            Vertex([0.5, -0.5, 0.5], [0.0, 0.0, 1.0], [1.0, 0.0]),
            Vertex([0.5, 0.5, 0.5], [0.0, 0.0, 1.0], [1.0, 1.0]),
            Vertex([0.5, 0.5, 0.5], [0.0, 0.0, 1.0], [1.0, 1.0]),
            Vertex([-0.5, 0.5, 0.5], [0.0, 0.0, 1.0], [0.0, 1.0]),
            Vertex([-0.5, -0.5, 0.5], [0.0, 0.0, 1.0], [0.0, 0.0]),
            Vertex([-0.5, 0.5, 0.5], [-1.0, 0.0, 0.0], [1.0, 0.0]),
            Vertex([-0.5, 0.5, -0.5], [-1.0, 0.0, 0.0], [1.0, 1.0]),
            Vertex([-0.5, -0.5, -0.5], [-1.0, 0.0, 0.0], [0.0, 1.0]),
            Vertex([-0.5, -0.5, -0.5], [-1.0, 0.0, 0.0], [0.0, 1.0]),
            Vertex([-0.5, -0.5, 0.5], [-1.0, 0.0, 0.0], [0.0, 0.0]),
            Vertex([-0.5, 0.5, 0.5], [-1.0, 0.0, 0.0], [1.0, 0.0]),
            Vertex([0.5, 0.5, 0.5], [1.0, 0.0, 0.0], [1.0, 0.0]),
            Vertex([0.5, 0.5, -0.5], [1.0, 0.0, 0.0], [1.0, 1.0]),
            Vertex([0.5, -0.5, -0.5], [1.0, 0.0, 0.0], [0.0, 1.0]),
            Vertex([0.5, -0.5, -0.5], [1.0, 0.0, 0.0], [0.0, 1.0]),
            Vertex([0.5, -0.5, 0.5], [1.0, 0.0, 0.0], [0.0, 0.0]),
            Vertex([0.5, 0.5, 0.5], [1.0, 0.0, 0.0], [1.0, 0.0]),
            Vertex([-0.5, -0.5, -0.5], [0.0, -1.0, 0.0], [0.0, 1.0]),
            Vertex([0.5, -0.5, -0.5], [0.0, -1.0, 0.0], [1.0, 1.0]),
            Vertex([0.5, -0.5, 0.5], [0.0, -1.0, 0.0], [1.0, 0.0]),
            Vertex([0.5, -0.5, 0.5], [0.0, -1.0, 0.0], [1.0, 0.0]),
            Vertex([-0.5, -0.5, 0.5], [0.0, -1.0, 0.0], [0.0, 0.0]),
            Vertex([-0.5, -0.5, -0.5], [0.0, -1.0, 0.0], [0.0, 1.0]),
            Vertex([-0.5, 0.5, -0.5], [0.0, 1.0, 0.0], [0.0, 1.0]),
            Vertex([0.5, 0.5, -0.5], [0.0, 1.0, 0.0], [1.0, 1.0]),
            Vertex([0.5, 0.5, 0.5], [0.0, 1.0, 0.0], [1.0, 0.0]),
            Vertex([0.5, 0.5, 0.5], [0.0, 1.0, 0.0], [1.0, 0.0]),
            Vertex([-0.5, 0.5, 0.5], [0.0, 1.0, 0.0], [0.0, 0.0]),
            Vertex([-0.5, 0.5, -0.5], [0.0, 1.0, 0.0], [0.0, 1.0]),
        ];

        // ==== Shaders ====
        // Object rendering shader
        let vertex_shader = Shader::from_file(ShaderType::Vertex, "./shaders/basic_vertex.vs")?;
        vertex_shader.compile()?;

        let fragment_shader =
            Shader::from_file(ShaderType::Fragment, "./shaders/basic_fragment.fs")?;
        fragment_shader.compile()?;

        let mut shader = ShaderProgram::new();
        shader.attach_shader(&vertex_shader);
        shader.attach_shader(&fragment_shader);
        shader.link()?;

        let objects_shader = Rc::new(shader);

        // Light source rendering shader
        let vertex_shader = Shader::from_file(ShaderType::Vertex, "./shaders/light_source.vs")?;
        vertex_shader.compile()?;
        let fragment_shader = Shader::from_file(ShaderType::Fragment, "./shaders/light_source.fs")?;
        fragment_shader.compile()?;
        let mut light_shader = ShaderProgram::new();
        light_shader.attach_shader(&vertex_shader);
        light_shader.attach_shader(&fragment_shader);
        light_shader.link()?;

        let light_shader = Rc::new(light_shader);

        // ==== Textures ====
        let container_texture_diffuse =
            Rc::new(Texture2D::new_from_file("./textures/container2.png")?);
        let container_texture_specular = Rc::new(Texture2D::new_from_file(
            "./textures/container2_specular.png",
        )?);

        // ==== Meshes ====
        let mut cube_mesh = Mesh::new();
        cube_mesh.init(&cube_vertices, None);
        let cube_mesh = Rc::new(cube_mesh);

        // ==== Materials ====
        let phong_material = Rc::new(RefCell::new(Material::new_with_properties(
            "phong_textured",
            Rc::clone(&objects_shader),
            [
                (
                    "material.diffuse".to_string(),
                    MaterialProperty::Texture(Rc::clone(&container_texture_diffuse)),
                ),
                (
                    "material.specular".to_string(),
                    MaterialProperty::Texture(Rc::clone(&container_texture_specular)),
                ),
                (
                    "material.shininess".to_string(),
                    MaterialProperty::Integer(32),
                ),
                ("isFloor".to_string(), MaterialProperty::Boolean(false)),
                (
                    "floorColor".to_string(),
                    MaterialProperty::Color(0.5, 0.5, 0.5),
                ),
            ]
            .into(),
        )));

        let light_material = Rc::new(RefCell::new(Material::new(
            "light_source",
            Rc::clone(&light_shader),
        )));

        self.light_materials.push(Rc::clone(&light_material));

        // ==== Scene ====
        let cube_positions = [
            glam::Vec3::new(0.0, 0.0, 0.0),
            glam::Vec3::new(2.0, 5.0, -15.0),
            glam::Vec3::new(-1.5, -2.2, -2.5),
            glam::Vec3::new(-3.8, -2.0, -12.3),
            glam::Vec3::new(2.4, -0.4, -3.5),
            glam::Vec3::new(-1.7, 3.0, -7.5),
            glam::Vec3::new(1.3, -2.0, -2.5),
            glam::Vec3::new(1.5, 2.0, -2.5),
            glam::Vec3::new(1.5, 0.2, -1.5),
            glam::Vec3::new(-1.3, 1.0, -1.5),
        ];

        for position in cube_positions {
            let cube = Rc::new(RefCell::new(Object::new(
                Rc::clone(&cube_mesh),
                Rc::clone(&phong_material),
            )));
            cube.borrow_mut().transform.position = position;
            cube.borrow_mut().rotate = true;
            self.add_object(Rc::clone(&cube));
        }

        // Floor
        let floor = Rc::new(RefCell::new(Object::new(
            Rc::clone(&cube_mesh),
            Rc::clone(&phong_material),
        )));
        {
            let mut floor = floor.borrow_mut();
            floor.transform.position = glam::vec3(0.0, -3.0, 0.0);
            floor.transform.scale = glam::Vec3::new(50.0, 0.1, 50.0);
            floor.material_overrides.set_boolean("isFloor", true);
        }
        self.add_object(Rc::clone(&floor));

        // Light sources
        let light_positions = [
            glam::Vec3::new(0.7, 0.2, 2.0),
            glam::Vec3::new(2.3, 10.3, -4.0),
            glam::Vec3::new(-4.0, 2.0, -12.0),
            glam::Vec3::new(0.0, 0.0, -3.0),
        ];

        for position in light_positions {
            // Light source object
            let light = Rc::new(RefCell::new(Object::new(
                Rc::clone(&cube_mesh),
                Rc::clone(&light_material),
            )));
            {
                let mut light = light.borrow_mut();
                light.transform.position = position;
                light.transform.scale = glam::Vec3::splat(0.2);
            }
            self.add_object(Rc::clone(&light));

            // Actual Light
            let light = Rc::new(RefCell::new(Light::new_point_light()));
            {
                let mut light = light.borrow_mut();
                light.position = position;
            }
            self.add_light(light);
        }

        // Directional light
        let light = Rc::new(RefCell::new(Light::new_directional_light()));
        light.borrow_mut().intensity = 0.4;
        light
            .borrow_mut()
            .as_directional_light_mut()
            .unwrap()
            .direction = glam::Vec3::new(-0.2, -1.0, -0.3);
        self.add_light(light);

        // Flashlight
        let light = Rc::new(RefCell::new(Light::new_spot_light()));
        self.add_light(light);

        Ok(())
    }

    pub fn add_object(&mut self, object: Rc<RefCell<Object>>) {
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: Rc<RefCell<Light>>) {
        self.lights.push(light);
    }

    pub fn update(&mut self, render_info: &RenderInfo) {
        self.camera.update(render_info);

        // Rotate cubes, a bit hacky
        let mut i = 0;
        for cube in self.objects.iter() {
            let mut cube = cube.borrow_mut();
            if !cube.rotate {
                continue;
            }

            let angle = (20.0 * i as f32).to_radians();
            let axis = glam::Vec3::new(1.0, 0.3, 0.5).normalize();
            let quat = glam::Quat::from_axis_angle(axis, render_info.time.as_secs_f32() * angle);
            cube.transform.rotation = quat;
            i += 1;
        }

        for light in &self.lights {
            let mut light = light.borrow_mut();
            light.color = render_info.ui.light_color;
            if light.is_spot_light() {
                light.as_spot_light_mut().unwrap().direction = self.camera.direction();
                light.position = self.camera.position();
            }
        }

        // Color of the light emitter, still one color for all lights
        if let Some(material) = self.light_materials.first() {
            let shader = material.borrow_mut().shader();
            shader.use_program();
            let (r, g, b) = render_info.ui.light_color.into();
            material
                .borrow_mut()
                .properties_mut()
                .set_color("lightColor", r, g, b);
        }
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
