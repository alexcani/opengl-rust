use gl::types::*;

#[derive(Copy, Clone)]
pub enum BufferType {
    Vertex,
    Index,
}

impl BufferType {
    fn as_gl_enum(&self) -> GLenum {
        match self {
            BufferType::Vertex => gl::ARRAY_BUFFER,
            BufferType::Index => gl::ELEMENT_ARRAY_BUFFER,
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
