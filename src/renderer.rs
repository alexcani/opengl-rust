mod buffer;
pub mod material;
pub mod mesh;
pub mod shader;
pub mod texture;

use std::cell::RefCell;
use std::ffi::CString;
use std::rc::Rc;
use std::time::Duration;

use glutin::display::GlDisplay;
use winit::keyboard::KeyCode;

use crate::input::InputManager;
use crate::scene::{Light, Object, Scene};
use crate::ui::Ui;
use buffer::UniformBuffer;
use material::{Material, PropertyValue};
use mesh::{Mesh, Vertex};
use shader::{Shader, ShaderProgram};
use texture::Texture2D;

use gl::types::*;

pub struct Renderer {
    wireframe: bool,
    size: (u32, u32),
    scene: Scene,
    phong_materials: Vec<Rc<RefCell<Material>>>,
    light_materials: Vec<Rc<RefCell<Material>>>,
    flashlight: bool,
    camera_ubo: UniformBuffer,
}

#[repr(C)]
struct CameraUniforms {
    view: glam::Mat4,
    projection: glam::Mat4,
    view_pos: glam::Vec4,
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
            phong_materials: Vec::new(),
            light_materials: Vec::new(),
            flashlight: false,
            camera_ubo: UniformBuffer::new(0, std::mem::size_of::<CameraUniforms>()),
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
        let phong_textured = Rc::new(RefCell::new(Material::new_with_properties(
            "phong_textured",
            Rc::clone(&objects_shader),
            [
                (
                    "material.diffuse".to_string(),
                    PropertyValue::Texture(Rc::clone(&container_texture_diffuse)),
                ),
                (
                    "material.specular".to_string(),
                    PropertyValue::Texture(Rc::clone(&container_texture_specular)),
                ),
                ("material.shininess".to_string(), PropertyValue::Integer(32)),
                ("isFloor".to_string(), PropertyValue::Boolean(false)),
            ]
            .into(),
        )));

        let phong_floor = Rc::new(RefCell::new(
            phong_textured.borrow().clone_with_overrides(
                "phong_floor",
                [
                    ("isFloor".to_string(), PropertyValue::Boolean(true)),
                    (
                        "floorColor".to_string(),
                        PropertyValue::Color(0.5, 0.5, 0.5),
                    ),
                ]
                .into(),
            ),
        ));
        phong_floor.borrow_mut().delete_property("material.diffuse");
        phong_floor
            .borrow_mut()
            .delete_property("material.specular");

        let light_source = Rc::new(RefCell::new(Material::new(
            "light_source",
            Rc::clone(&light_shader),
        )));

        self.phong_materials.push(Rc::clone(&phong_textured));
        self.phong_materials.push(Rc::clone(&phong_floor));
        self.light_materials.push(Rc::clone(&light_source));

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
                Rc::clone(&phong_textured),
            )));
            cube.borrow_mut().transform.position = position;
            cube.borrow_mut().rotate = true;
            self.scene.add_object(Rc::clone(&cube));
        }

        // Floor
        let floor = Rc::new(RefCell::new(Object::new(
            Rc::clone(&cube_mesh),
            Rc::clone(&phong_floor),
        )));
        {
            let mut floor = floor.borrow_mut();
            floor.transform.position = glam::vec3(0.0, -3.0, 0.0);
            floor.transform.scale = glam::Vec3::new(50.0, 0.1, 50.0);
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
                Rc::clone(&light_source),
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
        light.borrow_mut().as_directional_light_mut().unwrap().direction = glam::Vec3::new(-0.2, -1.0, -0.3);
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

        // Camera parameters
        self.camera_ubo.map_data(0, 1, |camera: &mut [CameraUniforms]| {
            camera[0].view = *self.scene.camera.view_matrix();
            camera[0].projection = *self.scene.camera.projection_matrix();
            camera[0].view_pos = self.scene.camera.position().extend(1.0);
        }).expect("Couldn't update camera UBO");

        // Temporary hack, set camera and light properties for all materials
        // Will be replaced with UBOs
        if let Some(material) = self.phong_materials.first() {
            let shader = material.borrow_mut().shader();
            shader.use_program();
            // Point lights
            for (i, light) in self
                .scene
                .lights
                .iter()
                .filter(|l| l.borrow().is_point_light())
                .enumerate()
            {
                let light = light.borrow();
                shader.set_uniform_3fv(
                    &format!("pointLights[{}].position", i),
                    &light.position.into(),
                );
                shader.set_uniform_3fv(&format!("pointLights[{}].color", i), &light.color);
                shader.set_uniform_1f(&format!("pointLights[{}].intensity", i), light.intensity);

                let light = light.as_point_light().unwrap();
                shader.set_uniform_1f(
                    &format!("pointLights[{}].constant", i),
                    light.attenuation[0],
                );
                shader.set_uniform_1f(&format!("pointLights[{}].linear", i), light.attenuation[1]);
                shader.set_uniform_1f(
                    &format!("pointLights[{}].quadratic", i),
                    light.attenuation[2],
                );
            }

            // Directional light
            if let Some(light) = self
                .scene
                .lights
                .iter()
                .find(|l| l.borrow().is_directional_light())
            {
                let light = light.borrow();
                shader.set_uniform_3fv("directionalLight.color", &light.color);
                shader.set_uniform_1f("directionalLight.intensity", light.intensity);

                let light = light.as_directional_light().unwrap();
                shader.set_uniform_3fv("directionalLight.direction", &light.direction.into());
            }

            // Flashlight
            if let Some(light) = self
                .scene
                .lights
                .iter()
                .find(|l| l.borrow().is_spot_light())
            {
                let light = light.borrow();
                shader.set_uniform_3fv("flashlight.position", &light.position.into());
                if self.flashlight {
                    shader.set_uniform_3fv("flashlight.color", &light.color);
                } else {
                    shader.set_uniform_3fv("flashlight.color", &[0.0, 0.0, 0.0]);
                }
                shader.set_uniform_1f("flashlight.intensity", light.intensity);

                let light = light.as_spot_light().unwrap();
                shader.set_uniform_3fv("flashlight.direction", &light.direction.into());
                shader.set_uniform_1f("flashlight.cutOff", light.inner_cutoff_rad.cos());
                shader.set_uniform_1f("flashlight.outerCutOff", light.outer_cutoff_rad.cos());
                shader.set_uniform_1f("flashlight.constant", light.attenuation[0]);
                shader.set_uniform_1f("flashlight.linear", light.attenuation[1]);
                shader.set_uniform_1f("flashlight.quadratic", light.attenuation[2]);
            }

            // Camera
        }

        if let Some(material) = self.light_materials.first() {
            let shader = material.borrow_mut().shader();
            shader.use_program();
            let (r, g, b) = args.ui.light_color.into();
            material.borrow_mut().set_color("lightColor", r, g, b);
        }

        // Render objects
        for object in &self.scene.objects {
            object.borrow().render();
        }
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
