use gl::types::*;

#[derive(Copy, Clone)]
pub enum BufferType {
    Vertex,
    Index,
    Uniform,
}

impl BufferType {
    fn as_gl_enum(&self) -> GLenum {
        match self {
            BufferType::Vertex => gl::ARRAY_BUFFER,
            BufferType::Index => gl::ELEMENT_ARRAY_BUFFER,
            BufferType::Uniform => gl::UNIFORM_BUFFER,
        }
    }
}

pub struct Buffer {
    id: GLuint,
    ty: GLenum,
}

impl Buffer {
    pub fn new(type_: BufferType) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Buffer { id, ty: type_.as_gl_enum() }
    }

    pub fn upload_data<T>(&self, data: &[T]) {
        self.bind();
        unsafe {
            gl::BufferData(
                self.ty,
                size_of_val(data) as GLsizeiptr,
                data.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
        }
    }

    #[allow(dead_code)]
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.ty, self.id);
        }
    }

    #[allow(dead_code)]
    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.ty, 0);
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

pub struct UniformBuffer {
    binding_point: GLuint,
    buffer: Buffer,
}

impl UniformBuffer {
    pub fn new(binding_point: GLuint, size: usize) -> Self {
        let buffer = Buffer::new(BufferType::Uniform);
        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, buffer.id);
            gl::BufferData(
                gl::UNIFORM_BUFFER,
                size as isize,
                std::ptr::null(),
                gl::DYNAMIC_DRAW,
            );
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
            gl::BindBufferBase(gl::UNIFORM_BUFFER, binding_point, buffer.id);
        }
        UniformBuffer { binding_point, buffer }
    }

    pub fn update_data<T>(&self, offset: usize, data: &[T]) {
        unsafe {
            gl::BufferSubData(
                gl::UNIFORM_BUFFER,
                offset as isize,
                size_of_val(data) as isize,
                data.as_ptr() as *const _,
            );
        }
    }

    pub fn bind(&self) {
        self.buffer.bind();
    }

    pub fn unbind(&self) {
        self.buffer.unbind();
    }
}
