use std::collections::HashMap;

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

    pub fn from_file(shader_type: ShaderType, path: &str) -> Result<Self, String> {
        let src = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        Ok(Shader::new(shader_type, &src))
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
    uniforms: HashMap<Box<str>, GLint>,
}

impl ShaderProgram {
    pub fn new() -> Self {
        let id = unsafe {
            gl::CreateProgram()
        };
        ShaderProgram { id, uniforms: HashMap::new() }
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

    #[allow(dead_code)]
    pub fn set_uniform_4f(&mut self, name: &str, x: f32, y: f32, z: f32, w: f32) {
        unsafe {
            gl::Uniform4f(self.get_uniform_location(name), x, y, z, w);
        }
    }

    #[allow(dead_code)]
    pub fn set_uniform_1f(&mut self, name: &str, x: f32) {
        unsafe {
            gl::Uniform1f(self.get_uniform_location(name), x);
        }
    }

    fn get_uniform_location(&mut self, name: &str) -> i32 {
        if let Some(location) = self.uniforms.get(name) {
            return *location;
        }

        let c_name = std::ffi::CString::new(name).unwrap();
        let location = unsafe {
            gl::GetUniformLocation(self.id, c_name.as_ptr())
        };
        if location == -1 {
            panic!("Uniform '{}' not found", name);
        }

        self.uniforms.insert(name.into(), location);
        location
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
