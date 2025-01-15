pub mod mesh;
mod buffer;
mod shader;
mod texture;

use std::ffi::CString;
use std::time::Duration;

use glutin::display::GlDisplay;
use winit::keyboard::KeyCode;

use crate::entity::Camera;
use crate::input::InputManager;
use crate::ui::Ui;
use buffer::Buffer;
use shader::{Shader, ShaderProgram};
use texture::Texture2D;

use gl::types::*;

#[repr(C)]
struct Vertex(
    [f32; 3], // Position
    [f32; 2], // TexCoords
);

pub struct Renderer {
    wireframe: bool,
    shader: ShaderProgram,
    vbo: Buffer,
    ebo: Buffer,
    vao: GLuint,
    texture: Texture2D,
    texture_2: Texture2D,
    size: (u32, u32),
    camera: Camera,
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
            shader: ShaderProgram::new(),
            vbo: Buffer::new(buffer::BufferType::Vertex),
            ebo: Buffer::new(buffer::BufferType::Index),
            vao: 0,
            texture: Texture2D::new(),
            texture_2: Texture2D::new(),
            size: (1, 1),
            camera: Camera::default(),
        };

        renderer.init().unwrap_or_else(|e| {
            println!("Failed to initialize renderer: {}", e);
            std::process::exit(1);
        });

        renderer
    }

    fn init(&mut self) -> Result<(), String> {
        let vertices: [Vertex; 36] = [
            Vertex([-0.5, -0.5, -0.5], [0.0, 0.0]),
            Vertex([0.5, -0.5, -0.5], [1.0, 0.0]),
            Vertex([0.5, 0.5, -0.5], [1.0, 1.0]),
            Vertex([0.5, 0.5, -0.5], [1.0, 1.0]),
            Vertex([-0.5, 0.5, -0.5], [0.0, 1.0]),
            Vertex([-0.5, -0.5, -0.5], [0.0, 0.0]),
            Vertex([-0.5, -0.5, 0.5], [0.0, 0.0]),
            Vertex([0.5, -0.5, 0.5], [1.0, 0.0]),
            Vertex([0.5, 0.5, 0.5], [1.0, 1.0]),
            Vertex([0.5, 0.5, 0.5], [1.0, 1.0]),
            Vertex([-0.5, 0.5, 0.5], [0.0, 1.0]),
            Vertex([-0.5, -0.5, 0.5], [0.0, 0.0]),
            Vertex([-0.5, 0.5, 0.5], [1.0, 0.0]),
            Vertex([-0.5, 0.5, -0.5], [1.0, 1.0]),
            Vertex([-0.5, -0.5, -0.5], [0.0, 1.0]),
            Vertex([-0.5, -0.5, -0.5], [0.0, 1.0]),
            Vertex([-0.5, -0.5, 0.5], [0.0, 0.0]),
            Vertex([-0.5, 0.5, 0.5], [1.0, 0.0]),
            Vertex([0.5, 0.5, 0.5], [1.0, 0.0]),
            Vertex([0.5, 0.5, -0.5], [1.0, 1.0]),
            Vertex([0.5, -0.5, -0.5], [0.0, 1.0]),
            Vertex([0.5, -0.5, -0.5], [0.0, 1.0]),
            Vertex([0.5, -0.5, 0.5], [0.0, 0.0]),
            Vertex([0.5, 0.5, 0.5], [1.0, 0.0]),
            Vertex([-0.5, -0.5, -0.5], [0.0, 1.0]),
            Vertex([0.5, -0.5, -0.5], [1.0, 1.0]),
            Vertex([0.5, -0.5, 0.5], [1.0, 0.0]),
            Vertex([0.5, -0.5, 0.5], [1.0, 0.0]),
            Vertex([-0.5, -0.5, 0.5], [0.0, 0.0]),
            Vertex([-0.5, -0.5, -0.5], [0.0, 1.0]),
            Vertex([-0.5, 0.5, -0.5], [0.0, 1.0]),
            Vertex([0.5, 0.5, -0.5], [1.0, 1.0]),
            Vertex([0.5, 0.5, 0.5], [1.0, 0.0]),
            Vertex([0.5, 0.5, 0.5], [1.0, 0.0]),
            Vertex([-0.5, 0.5, 0.5], [0.0, 0.0]),
            Vertex([-0.5, 0.5, -0.5], [0.0, 1.0]),
        ];

        let indices: [u32; 6] = [0, 1, 3, 1, 2, 3];

        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
        }

        let vbo = Buffer::new(buffer::BufferType::Vertex);
        vbo.upload_data(&vertices);

        unsafe {
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                size_of::<Vertex>() as GLsizei,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                size_of::<Vertex>() as GLsizei,
                std::mem::offset_of!(Vertex, 1) as *const _,
            );
            gl::EnableVertexAttribArray(1);
        }

        let ebo = Buffer::new(buffer::BufferType::Index);
        ebo.upload_data(&indices);

        let vertex_shader =
            Shader::from_file(shader::ShaderType::Vertex, "./shaders/basic_vertex.vs")?;
        vertex_shader.compile()?;

        let fragment_shader =
            Shader::from_file(shader::ShaderType::Fragment, "./shaders/basic_fragment.fs")?;
        fragment_shader.compile()?;

        let shader_program = ShaderProgram::new();
        shader_program.attach_shader(&vertex_shader);
        shader_program.attach_shader(&fragment_shader);
        shader_program.link()?;

        let texture = Texture2D::new_from_file("./textures/container.jpg")?;
        let texture_2 = Texture2D::new_from_file("./textures/awesomeface.png")?;

        self.shader = shader_program;
        self.vbo = vbo;
        self.ebo = ebo;
        self.vao = vao;
        self.texture = texture;
        self.texture_2 = texture_2;

        Ok(())
    }

    pub fn render(&mut self, args: &RenderInfo) {
        let input = args.input_manager;
        if input.is_key_just_pressed(KeyCode::KeyL) {
            self.toggle_wireframe();
        }
        self.shader.use_program();

        // Textures
        self.shader.set_uniform_1i("texture1", 0);
        self.shader.set_uniform_1i("texture2", 1);
        self.texture.bind_slot(0);
        self.texture_2.bind_slot(1);

        // Camera
        self.camera.update(args);
        self.shader
            .set_uniform_mat4("projection", self.camera.projection_matrix());
        self.shader
            .set_uniform_mat4("view", self.camera.view_matrix());

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

        unsafe {
            let color = args.ui.clear_color;
            gl::ClearColor(color[0], color[1], color[2], 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);
            gl::BindVertexArray(self.vao);
        }

        for (i, coords) in cube_positions.iter().enumerate() {
            let angle = (20.0 * (i + 1) as f32).to_radians();
            let axis = glam::Vec3::new(1.0, 0.3, 0.5).normalize();
            let quat = glam::Quat::from_axis_angle(axis, args.time.as_secs_f32() * angle);
            let rotation = glam::Mat4::from_quat(quat);
            let translation = glam::Mat4::from_translation(*coords);
            let model = translation * rotation;
            self.shader.set_uniform_mat4("model", &model);
            unsafe {
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        unsafe {
            gl::Viewport(0, 0, width as GLsizei, height as GLsizei);
        }
        self.size = (width, height);
        self.camera.resize(width, height);
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
