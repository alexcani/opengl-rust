mod buffer;
pub mod mesh;
pub mod shader;
pub mod texture;
pub mod material;

use std::cell::RefCell;
use std::ffi::CString;
use std::rc::Rc;
use std::time::Duration;

use glutin::display::GlDisplay;
use winit::keyboard::KeyCode;

use crate::input::InputManager;
use crate::scene::{Object, Scene};
use crate::ui::Ui;
use mesh::{Mesh, Vertex};
use shader::{Shader, ShaderProgram};
use texture::Texture2D;

use gl::types::*;

pub struct Renderer {
    wireframe: bool,
    shader: ShaderProgram,
    light_shader: ShaderProgram,
    texture: Texture2D,
    texture_2: Texture2D,
    size: (u32, u32),
    scene: Scene,
    cubes: Vec<Rc<RefCell<Object>>>,
    lights: Vec<Rc<RefCell<Object>>>,
    floor: Rc<RefCell<Object>>,
    flashlight: bool,
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
            shader: ShaderProgram::default(),
            light_shader: ShaderProgram::default(),
            texture: Texture2D::new(),
            texture_2: Texture2D::new(),
            size: (1, 1),
            scene: Scene::default(),
            cubes: Vec::new(),
            lights: Vec::new(),
            floor: Default::default(),
            flashlight: false,
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

        let mut cube_mesh = Mesh::new();
        cube_mesh.init(&cube_vertices, None);
        let cube_mesh = Rc::new(cube_mesh);

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
            let cube = Rc::new(RefCell::new(Object::new(Rc::clone(&cube_mesh))));
            cube.borrow_mut().transform.position = position;
            self.scene.add_object(Rc::clone(&cube));
            self.cubes.push(cube);
        }

        // Floor
        let floor = Rc::new(RefCell::new(Object::new(Rc::clone(&cube_mesh))));
        {
            let mut floor = floor.borrow_mut();
            floor.transform.position = glam::vec3(0.0, -3.0, 0.0);
            floor.transform.scale = glam::Vec3::new(50.0, 0.1, 50.0);
        }
        self.scene.add_object(Rc::clone(&floor));
        self.floor = floor;

        // Light sources
        let light_positions = [
            glam::Vec3::new(0.7, 0.2, 2.0),
            glam::Vec3::new(2.3, 10.3, -4.0),
            glam::Vec3::new(-4.0, 2.0, -12.0),
            glam::Vec3::new(0.0, 0.0, -3.0),
        ];

        for position in light_positions {
            let light = Rc::new(RefCell::new(Object::new(Rc::clone(&cube_mesh))));
            {
                let mut light = light.borrow_mut();
                light.transform.position = position;
                light.transform.scale = glam::Vec3::splat(0.2);
            }
            self.lights.push(Rc::clone(&light));
            self.scene.add_object(Rc::clone(&light));
        }

        // Shader for rendering objects
        let vertex_shader =
            Shader::from_file(shader::ShaderType::Vertex, "./shaders/basic_vertex.vs")?;
        vertex_shader.compile()?;

        let fragment_shader =
            Shader::from_file(shader::ShaderType::Fragment, "./shaders/basic_fragment.fs")?;
        fragment_shader.compile()?;

        self.shader.attach_shader(&vertex_shader);
        self.shader.attach_shader(&fragment_shader);
        self.shader.link()?;

        // Shader for rendering light sources
        let vertex_shader =
            Shader::from_file(shader::ShaderType::Vertex, "./shaders/light_source.vs")?;
        vertex_shader.compile()?;
        let fragment_shader =
            Shader::from_file(shader::ShaderType::Fragment, "./shaders/light_source.fs")?;
        fragment_shader.compile()?;
        self.light_shader.attach_shader(&vertex_shader);
        self.light_shader.attach_shader(&fragment_shader);
        self.light_shader.link()?;

        self.texture.load_file("./textures/container2.png")?;
        self.texture_2
            .load_file("./textures/container2_specular.png")?;

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

        self.scene.camera.update(args);

        // Render objects
        self.shader.use_program();

        // Set camera position
        self.shader
            .set_uniform_3fv("viewPos", &self.scene.camera.position().into());

        // Set light properties
        // Point lights
        for (i, light) in self.lights.iter().enumerate() {
            let x = light.borrow_mut().transform.position;
            self.shader
                .set_uniform_3fv(&format!("pointLights[{}].position", i), &x.into());
            self.shader
                .set_uniform_3fv(&format!("pointLights[{}].color", i), &args.ui.light_color);
            self.shader
                .set_uniform_1f(&format!("pointLights[{}].constant", i), 1.0);
            self.shader
                .set_uniform_1f(&format!("pointLights[{}].linear", i), 0.09);
            self.shader
                .set_uniform_1f(&format!("pointLights[{}].quadratic", i), 0.032);
            self.shader.set_uniform_1f(
                &format!("pointLights[{}].ambient_strength", i),
                args.ui.ambient_strength,
            );
            self.shader.set_uniform_1f(
                &format!("pointLights[{}].specular_strength", i),
                args.ui.specular_strength,
            );
        }

        // Directional light
        self.shader
            .set_uniform_3fv("directionalLight.direction", &[-0.2, -1.0, -0.3]);
        self.shader
            .set_uniform_3fv("directionalLight.color", &args.ui.light_color);
        self.shader.set_uniform_1f(
            "directionalLight.ambient_strength",
            args.ui.ambient_strength,
        );
        self.shader.set_uniform_1f(
            "directionalLight.specular_strength",
            args.ui.specular_strength,
        );

        // Flashlight
        self.shader
            .set_uniform_3fv("flashlight.position", &self.scene.camera.position().into());
        self.shader.set_uniform_3fv(
            "flashlight.direction",
            &self.scene.camera.direction().into(),
        );
        self.shader
            .set_uniform_1f("flashlight.cutOff", 12.5_f32.to_radians().cos());
        self.shader
            .set_uniform_1f("flashlight.outerCutOff", 17.5_f32.to_radians().cos());
        self.shader.set_uniform_1f("flashlight.constant", 1.0);
        self.shader.set_uniform_1f("flashlight.linear", 0.09);
        self.shader.set_uniform_1f("flashlight.quadratic", 0.032);
        if self.flashlight {
            self.shader
                .set_uniform_3fv("flashlight.color", &[1.0, 1.0, 1.0]);
        } else {
            self.shader
                .set_uniform_3fv("flashlight.color", &[0.0, 0.0, 0.0]);
        }

        // Is floor
        self.shader.set_uniform_1i("isFloor", gl::FALSE.into());

        // Material properties
        self.shader
            .set_uniform_1i("material.shininess", args.ui.shininess);
        self.shader.set_uniform_1i("material.diffuse", 0);
        self.shader.set_uniform_1i("material.specular", 1);
        self.texture.bind_slot(0);
        self.texture_2.bind_slot(1);

        self.shader
            .set_uniform_mat4("projection", self.scene.camera.projection_matrix());
        self.shader
            .set_uniform_mat4("view", self.scene.camera.view_matrix());

        for (i, cube) in self.cubes.iter_mut().enumerate() {
            let angle = (20.0 * i as f32).to_radians();
            let axis = glam::Vec3::new(1.0, 0.3, 0.5).normalize();
            let quat = glam::Quat::from_axis_angle(axis, args.time.as_secs_f32() * angle);
            cube.borrow_mut().transform.rotation = quat;
            cube.borrow_mut().render(&mut self.shader);
        }

        // Floor
        self.shader.set_uniform_1i("isFloor", gl::TRUE.into());
        self.shader.set_uniform_3f("floorColor", 0.5, 0.5, 0.5);
        self.floor.borrow_mut().render(&mut self.shader);

        // Render light sources
        self.light_shader.use_program();
        self.light_shader
            .set_uniform_3fv("lightColor", &args.ui.light_color);
        self.light_shader
            .set_uniform_mat4("projection", self.scene.camera.projection_matrix());
        self.light_shader
            .set_uniform_mat4("view", self.scene.camera.view_matrix());

        for light in &self.lights {
            light.borrow_mut().render(&mut self.light_shader);
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
