use crate::renderer::buffer::{Buffer, BufferType};

use gl::types::*;

#[repr(C)]
pub struct Vertex(
    pub [f32; 3], // position
    pub [f32; 3], // normal vectors
    pub [f32; 2], // texture coordinates
);

pub struct Mesh {
    vbo: Buffer,
    ebo: Option<Buffer>,
    vao: GLuint,
    number_of_drawables: GLsizei,
}

impl Mesh {
    pub fn new() -> Self {
        let mut mesh = Self {
            vbo: Buffer::new(BufferType::Vertex),
            ebo: None,
            vao: 0,
            number_of_drawables: 0,
        };
        unsafe {
            gl::GenVertexArrays(1, &mut mesh.vao);
        }
        mesh
    }

    pub fn init(&mut self, vertices: &[Vertex], indices: Option<&[u32]>) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }

        self.vbo.upload_data(vertices);

        // If indices are not provided, then the number of drawables is the number of vertices
        self.number_of_drawables = vertices.len() as GLsizei;

        if let Some(indices) = indices {
            self.ebo = Some(Buffer::new(BufferType::Index));
            self.ebo.as_ref().unwrap().upload_data(indices);
            self.number_of_drawables = indices.len() as GLsizei;
        }

        unsafe {
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as GLsizei,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as GLsizei,
                std::mem::offset_of!(Vertex, 1) as *const _,
            );
            gl::EnableVertexAttribArray(1);

            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as GLsizei,
                std::mem::offset_of!(Vertex, 2) as *const _,
            );
            gl::EnableVertexAttribArray(2);
        }

        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            if self.ebo.is_some() {
                gl::DrawElements(
                    gl::TRIANGLES,
                    self.number_of_drawables,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                );
            } else {
                gl::DrawArrays(gl::TRIANGLES, 0, self.number_of_drawables);
            }
        }
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}
