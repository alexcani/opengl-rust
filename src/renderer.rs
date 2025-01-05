mod buffer;
mod shader;

use std::ffi::CString;
use std::time::Duration;

use glutin::display::GlDisplay;

use buffer::Buffer;
use shader::{Shader, ShaderProgram};

use gl::types::*;

const VERTEX_SHADER: &str = r#"
#version 330 core
layout (location = 0) in vec3 aPos;

void main()
{
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
"#;

const FRAGMENT_SHADER: &str = r#"#version 330 core
out vec4 FragColor;

void main()
{
    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}
"#;

type Vertex = [f32; 3];

pub struct Renderer {
    wireframe: bool,
    shader: ShaderProgram,
    vbo: Buffer,
    ebo: Buffer,
    vao: GLuint,
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
            [0.5, 0.5, 0.0],
            [0.5, -0.5, 0.0],
            [-0.5, -0.5, 0.0],
            [-0.5, 0.5, 0.0],
        ];

        let indices: [u32; 6] = [0, 1, 3, 1, 2, 3];

        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
        }

        let vbo = Buffer::new(buffer::BufferType::Vertex);
        vbo.set_data(&vertices);

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
        }

        let ebo = Buffer::new(buffer::BufferType::Index);
        ebo.set_data(&indices);

        let vertex_shader = Shader::new(shader::ShaderType::Vertex, VERTEX_SHADER);
        if let Err(e) = vertex_shader.compile() {
            println!("Failed to compile vertex shader: {}", e);
        }

        let fragment_shader = Shader::new(shader::ShaderType::Fragment, FRAGMENT_SHADER);
        if let Err(e) = fragment_shader.compile() {
            println!("Failed to compile fragment shader: {}", e);
        }

        let shader_program = ShaderProgram::new();
        shader_program.attach_shader(&vertex_shader);
        shader_program.attach_shader(&fragment_shader);
        if let Err(e) = shader_program.link() {
            println!("Failed to link shader program: {}", e);
        }

        self.shader = shader_program;
        self.vbo = vbo;
        self.ebo = ebo;
        self.vao = vao;
    }

    pub fn render(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            self.shader.use_program();
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
