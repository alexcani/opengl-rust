use gl::types::*;

pub enum ShaderType {
    Vertex,
    Fragment,
}

pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn new(shader_type: ShaderType, src: &str) -> Self {
        let t = match shader_type {
            ShaderType::Vertex => gl::VERTEX_SHADER,
            ShaderType::Fragment => gl::FRAGMENT_SHADER,
        };

        let id = unsafe {
            gl::CreateShader(t)
        };
        unsafe {
            gl::ShaderSource(id, 1, &(src.as_ptr().cast()), &(src.len().try_into().unwrap()));
        }
        Shader { id }
    }

    pub fn compile(&self) -> Result<(), String> {
        unsafe {
            gl::CompileShader(self.id);
        }

        let mut success = 0;
        unsafe {
            gl::GetShaderiv(self.id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len = 0;
            unsafe {
                gl::GetShaderiv(self.id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let mut buffer = vec![0; len as usize];
            unsafe {
                gl::GetShaderInfoLog(self.id, len, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut GLchar);
            }

            return Err(String::from_utf8(buffer).unwrap());
        }

        Ok(())
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

pub struct ShaderProgram {
    id: GLuint,
}

impl ShaderProgram {
    pub fn new() -> Self {
        let id = unsafe {
            gl::CreateProgram()
        };
        ShaderProgram { id }
    }

    pub fn attach_shader(&self, shader: &Shader) {
        unsafe {
            gl::AttachShader(self.id, shader.id());
        }
    }

    pub fn link(&self) -> Result<(), String> {
        unsafe {
            gl::LinkProgram(self.id);
        }

        let mut success = 0;
        unsafe {
            gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len = 0;
            unsafe {
                gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let mut buffer = vec![0; len as usize];
            unsafe {
                gl::GetProgramInfoLog(self.id, len, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut GLchar);
            }

            return Err(String::from_utf8(buffer).unwrap());
        }

        Ok(())
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
