mod buffer;
mod shader;
mod texture;

use std::ffi::CString;
use std::time::Duration;

use glutin::display::GlDisplay;

use buffer::Buffer;
use shader::{Shader, ShaderProgram};
use texture::Texture2D;

use gl::types::*;

#[repr(C)]
struct Vertex(
    [f32; 3], // Position
    [f32; 3], // Color
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
}

pub struct RenderInfo {
    pub dt: Duration,   // Time since the last frame
    pub time: Duration, // Time since the start of the application
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
        };

        renderer.init().unwrap_or_else(|e| {
            println!("Failed to initialize renderer: {}", e);
            std::process::exit(1);
        });

        renderer
    }

    fn init(&mut self) -> Result<(), String> {
        unsafe {
            gl::ClearColor(0.6, 0.4, 0.8, 1.0);
        }

        let vertices: [Vertex; 4] = [
            Vertex([0.5, 0.5, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0]),
            Vertex([0.5, -0.5, 0.0], [0.0, 1.0, 0.0], [1.0, 0.0]),
            Vertex([-0.5, -0.5, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0]),
            Vertex([-0.5, 0.5, 0.0], [1.0, 1.0, 1.0], [0.0, 1.0]),
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
                3,
                gl::FLOAT,
                gl::FALSE,
                size_of::<Vertex>() as GLsizei,
                std::mem::offset_of!(Vertex, 1) as *const _,
            );
            gl::EnableVertexAttribArray(1);

            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                size_of::<Vertex>() as GLsizei,
                std::mem::offset_of!(Vertex, 2) as *const _,
            );
            gl::EnableVertexAttribArray(2);
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

    pub fn render(&mut self, args: RenderInfo) {
        self.shader.use_program();
        self.shader.set_uniform_1i("texture2", 1);
        self.texture.bind_slot(0);
        self.texture_2.bind_slot(1);
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
    }

    pub fn resize(&self, width: u32, height: u32) {
        unsafe {
            gl::Viewport(0, 0, width as GLsizei, height as GLsizei);
        }
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
