mod buffer;
pub mod material;
pub mod mesh;
pub mod shader;
pub mod texture;

use std::cell::RefCell;
use std::ffi::CString;
use std::mem::MaybeUninit;
use std::rc::Rc;
use std::time::Duration;

use glutin::display::GlDisplay;
use winit::keyboard::KeyCode;

use crate::input::InputManager;
use crate::scene::{Light, Object, Scene};
use crate::ui::Ui;
use buffer::UniformBuffer;
use material::{Material, MaterialProperty};
use mesh::{Mesh, Vertex};
use shader::{Shader, ShaderProgram};
use texture::Texture2D;

use gl::types::*;

pub struct Renderer {
    wireframe: bool,
    size: (u32, u32),
    scene: Scene,
    light_materials: Vec<Rc<RefCell<Material>>>,
    flashlight: bool,
    camera_ubo: UniformBuffer,
    light_ubo: UniformBuffer,
}

pub struct RenderInfo<'a> {
    pub dt: Duration,   // Time since the last frame
    pub time: Duration, // Time since the start of the application
    pub input_manager: &'a InputManager,
    pub ui: &'a Ui,
}

impl Renderer {
    pub fn new<D: GlDisplay>(display: &D) -> Self {
        gl::load_with(|s| {
            let s = CString::new(s).unwrap();
            display.get_proc_address(s.as_c_str())
        });

        let mut renderer = Renderer {
            wireframe: false,
            size: (1, 1),
            scene: Scene::default(),
            light_materials: Vec::new(),
            flashlight: false,
            camera_ubo: UniformBuffer::new(0, std::mem::size_of::<CameraUniforms>()),
            light_ubo: UniformBuffer::new(1, std::mem::size_of::<LightUniforms>()),
        };

        renderer.init().unwrap_or_else(|e| {
            println!("Failed to initialize renderer: {}", e);
            std::process::exit(1);
        });

        renderer
    }

    fn init(&mut self) -> Result<(), String> {
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
        let vertex_shader =
            Shader::from_file(shader::ShaderType::Vertex, "./shaders/basic_vertex.vs")?;
        vertex_shader.compile()?;

        let fragment_shader =
            Shader::from_file(shader::ShaderType::Fragment, "./shaders/basic_fragment.fs")?;
        fragment_shader.compile()?;

        let mut shader = ShaderProgram::new();
        shader.attach_shader(&vertex_shader);
        shader.attach_shader(&fragment_shader);
        shader.link()?;

        let objects_shader = Rc::new(shader);

        // Light source rendering shader
        let vertex_shader =
            Shader::from_file(shader::ShaderType::Vertex, "./shaders/light_source.vs")?;
        vertex_shader.compile()?;
        let fragment_shader =
            Shader::from_file(shader::ShaderType::Fragment, "./shaders/light_source.fs")?;
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
                ("material.shininess".to_string(), MaterialProperty::Integer(32)),
                ("isFloor".to_string(), MaterialProperty::Boolean(false)),
                ("floorColor".to_string(), MaterialProperty::Color(0.5, 0.5, 0.5)),
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
            self.scene.add_object(Rc::clone(&cube));
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
        self.scene.add_object(Rc::clone(&floor));

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
            self.scene.add_object(Rc::clone(&light));

            // Actual Light
            let light = Rc::new(RefCell::new(Light::new_point_light()));
            {
                let mut light = light.borrow_mut();
                light.position = position;
            }
            self.scene.add_light(light);
        }

        // Directional light
        let light = Rc::new(RefCell::new(Light::new_directional_light()));
        light.borrow_mut().intensity = 0.4;
        light
            .borrow_mut()
            .as_directional_light_mut()
            .unwrap()
            .direction = glam::Vec3::new(-0.2, -1.0, -0.3);
        self.scene.add_light(light);

        // Flashlight
        let light = Rc::new(RefCell::new(Light::new_spot_light()));
        self.scene.add_light(light);

        Ok(())
    }

    pub fn render(&mut self, args: &RenderInfo) {
        let input = args.input_manager;
        if input.is_key_just_pressed(KeyCode::KeyL) {
            self.toggle_wireframe();
        }
        if input.is_key_just_pressed(KeyCode::KeyG) {
            self.flashlight = !self.flashlight;
        }

        let color = args.ui.clear_color;
        unsafe {
            gl::ClearColor(color[0], color[1], color[2], 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);
        }

        self.scene.update(args);

        self.update_camera_buffer();
        self.update_light_parameters();

        // Color of the light emitter, still one color for all lights
        if let Some(material) = self.light_materials.first() {
            let shader = material.borrow_mut().shader();
            shader.use_program();
            let (r, g, b) = args.ui.light_color.into();
            material.borrow_mut().properties_mut().set_color("lightColor", r, g, b);
        }

        // Render objects
        for object in &self.scene.objects {
            object.borrow().render();
        }
    }

    fn update_camera_buffer(&self) {
        self.camera_ubo
            .map_data(0, 1, |camera: &mut [CameraUniforms]| {
                camera[0].view = *self.scene.camera.view_matrix();
                camera[0].projection = *self.scene.camera.projection_matrix();
                camera[0].view_pos = self.scene.camera.position().extend(1.0);
            })
            .expect("Couldn't update camera UBO");
    }

    fn update_light_parameters(&self) {
        let mut light_uniforms = unsafe { MaybeUninit::<LightUniforms>::zeroed().assume_init() };
        for light in &self.scene.lights {
            let light = light.borrow();
            let color = light.color;
            let position = light.position;
            if light.is_spot_light() {
                let index = light_uniforms.nr_spot_lights as usize;
                if index >= MAX_SPOT_LIGHTS {
                    panic!("Exceeded maximum number of spot lights");
                }

                light_uniforms.spot[index].color = [color[0], color[1], color[2], 1.0];
                light_uniforms.spot[index].position = [position[0], position[1], position[2], 1.0];
                light_uniforms.spot[index].intensity = light.intensity;
                let light = light.as_spot_light().unwrap();
                let direction = light.direction;
                let attenuation = light.attenuation;
                light_uniforms.spot[index].direction =
                    [direction[0], direction[1], direction[2], 1.0];
                light_uniforms.spot[index].inner_cutoff_cos = light.inner_cutoff_rad.cos();
                light_uniforms.spot[index].outer_cutoff_cos = light.outer_cutoff_rad.cos();
                light_uniforms.spot[index].attenuation =
                    [attenuation[0], attenuation[1], attenuation[2]];
                light_uniforms.nr_spot_lights += 1;
            } else if light.is_point_light() {
                let index = light_uniforms.nr_point_lights as usize;
                if index >= MAX_POINT_LIGHTS {
                    panic!("Exceeded maximum number of point lights");
                }

                light_uniforms.point[index].color = [color[0], color[1], color[2], 1.0];
                light_uniforms.point[index].position = [position[0], position[1], position[2], 1.0];
                light_uniforms.point[index].intensity = light.intensity;
                let light = light.as_point_light().unwrap();
                let attenuation = light.attenuation;
                light_uniforms.point[index].attenuation =
                    [attenuation[0], attenuation[1], attenuation[2]];
                light_uniforms.nr_point_lights += 1;
            } else if light.is_directional_light() {
                let index = light_uniforms.nr_directional_lights as usize;
                if index >= MAX_DIRECTIONAL_LIGHTS {
                    panic!("Exceeded maximum number of directional lights");
                }

                light_uniforms.directional[index].color =
                    [light.color[0], light.color[1], light.color[2], 1.0];
                light_uniforms.directional[index].intensity = light.intensity;
                let light = light.as_directional_light().unwrap();
                let direction = light.direction;
                light_uniforms.directional[index].direction =
                    [direction[0], direction[1], direction[2], 1.0];
                light_uniforms.nr_directional_lights += 1;
            }
        }

        light_uniforms.ambient.color = {
            let ambient = &self.scene.ambient_light;
            [ambient.color[0], ambient.color[1], ambient.color[2], 1.0]
        };
        light_uniforms.ambient.intensity = self.scene.ambient_light.intensity;

        self.light_ubo
            .map_data(0, 1, |data: &mut [LightUniforms]| {
                data[0] = light_uniforms;
            })
            .expect("Couldn't update light UBO");
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        unsafe {
            gl::Viewport(0, 0, width as GLsizei, height as GLsizei);
        }
        self.size = (width, height);
        self.scene.camera.resize(width, height);
    }

    pub fn toggle_wireframe(&mut self) {
        self.wireframe = !self.wireframe;
        unsafe {
            if self.wireframe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            } else {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            }
        }
    }
}

#[repr(C)]
struct CameraUniforms {
    view: glam::Mat4,
    projection: glam::Mat4,
    view_pos: glam::Vec4,
}

#[repr(C)]
struct DirectionalLightUniforms {
    color: [f32; 4],
    direction: [f32; 4],
    intensity: f32,
    _padding: [f32; 3],
}

#[repr(C)]
struct PointLightUniforms {
    color: [f32; 4],
    position: [f32; 4],
    attenuation: [f32; 3], // constant, linear, quadratic
    intensity: f32,
}

#[repr(C)]
struct SpotLightUniforms {
    color: [f32; 4],
    position: [f32; 4],
    direction: [f32; 4],
    inner_cutoff_cos: f32,
    outer_cutoff_cos: f32,
    attenuation: [f32; 3], // constant, linear, quadratic
    intensity: f32,
    _padding: [f32; 2],
}

#[repr(C)]
struct AmbientLightUniforms {
    color: [f32; 4],
    intensity: f32,
    _padding: [f32; 3],
}

const MAX_POINT_LIGHTS: usize = 10;
const MAX_SPOT_LIGHTS: usize = 5;
const MAX_DIRECTIONAL_LIGHTS: usize = 5;

#[repr(C)]
struct LightUniforms {
    ambient: AmbientLightUniforms,
    directional: [DirectionalLightUniforms; MAX_DIRECTIONAL_LIGHTS],
    point: [PointLightUniforms; MAX_POINT_LIGHTS],
    spot: [SpotLightUniforms; MAX_SPOT_LIGHTS],
    nr_point_lights: i32,
    nr_spot_lights: i32,
    nr_directional_lights: i32
}
