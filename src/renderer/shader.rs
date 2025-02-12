use std::cell::RefCell;
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

        let id = unsafe { gl::CreateShader(t) };
        unsafe {
            gl::ShaderSource(
                id,
                1,
                &(src.as_ptr().cast()),
                &(src.len().try_into().unwrap()),
            );
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
                gl::GetShaderInfoLog(
                    self.id,
                    len,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut GLchar,
                );
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
    uniform_cache: RefCell<HashMap<Box<str>, UniformValue>>,
}

#[allow(dead_code)]
impl ShaderProgram {
    pub fn new() -> Self {
        let id = unsafe { gl::CreateProgram() };
        ShaderProgram {
            id,
            uniforms: HashMap::new(),
            uniform_cache: RefCell::new(HashMap::new()),
        }
    }

    pub fn attach_shader(&self, shader: &Shader) {
        unsafe {
            gl::AttachShader(self.id, shader.id());
        }
    }

    pub fn link(&mut self) -> Result<(), String> {
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
                gl::GetProgramInfoLog(
                    self.id,
                    len,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut GLchar,
                );
            }

            return Err(String::from_utf8(buffer).unwrap());
        }

        self.populate_uniform_indices();

        Ok(())
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    fn set_uniform<T: Into<UniformValue>>(&self, name: &str, value: T, setter: impl FnOnce()) {
        let new_value = UniformValue::from_value(value);
        {
            let cache = self.uniform_cache.borrow();
            if let Some(cached_value) = cache.get(name) {
                if cached_value == &new_value {
                    return;
                }
            }
        }

        self.uniform_cache
            .borrow_mut()
            .insert(name.into(), new_value);
        setter();
    }

    pub fn set_uniform_4f(&self, name: &str, x: f32, y: f32, z: f32, w: f32) {
        self.set_uniform(name, [x, y, z, w], || unsafe {
            gl::Uniform4f(self.get_uniform_location(name), x, y, z, w);
        });
    }

    pub fn set_uniform_1f(&self, name: &str, x: f32) {
        self.set_uniform(name, x, || unsafe {
            gl::Uniform1f(self.get_uniform_location(name), x);
        });
    }

    pub fn set_uniform_1i(&self, name: &str, x: i32) {
        self.set_uniform(name, x, || unsafe {
            gl::Uniform1i(self.get_uniform_location(name), x);
        });
    }

    pub fn set_uniform_1ui(&self, name: &str, x: u32) {
        self.set_uniform(name, x, || unsafe {
            gl::Uniform1ui(self.get_uniform_location(name), x);
        });
    }

    pub fn set_uniform_mat4(&self, name: &str, mat: &glam::Mat4) {
        self.set_uniform(name, *mat, || unsafe {
            gl::UniformMatrix4fv(
                self.get_uniform_location(name),
                1,
                gl::FALSE,
                mat.to_cols_array().as_ptr(),
            );
        });
    }

    pub fn set_uniform_mat3(&self, name: &str, mat: &glam::Mat3) {
        self.set_uniform(name, *mat, || unsafe {
            gl::UniformMatrix3fv(
                self.get_uniform_location(name),
                1,
                gl::FALSE,
                mat.to_cols_array().as_ptr(),
            );
        });
    }

    pub fn set_uniform_3fv(&self, name: &str, x: &[f32; 3]) {
        self.set_uniform(name, *x, || unsafe {
            gl::Uniform3fv(self.get_uniform_location(name), 1, x.as_ptr());
        });
    }

    pub fn set_uniform_3f(&self, name: &str, x: f32, y: f32, z: f32) {
        self.set_uniform(name, [x, y, z], || unsafe {
            gl::Uniform3f(self.get_uniform_location(name), x, y, z);
        });
    }

    fn get_uniform_location(&self, name: &str) -> i32 {
        if let Some(location) = self.uniforms.get(name) {
            return *location;
        }

        panic!("Uniform '{}' not found", name);
    }

    fn populate_uniform_indices(&mut self) {
        let mut max_length = 0;
        let mut num_active_uniforms = 0;
        unsafe {
            gl::GetProgramiv(self.id, gl::ACTIVE_UNIFORM_MAX_LENGTH, &mut max_length);
            gl::GetProgramiv(self.id, gl::ACTIVE_UNIFORMS, &mut num_active_uniforms);
        }

        for i in 0..num_active_uniforms {
            let mut buffer = vec![0; max_length as usize];
            let mut written_length = 0;
            let mut size = 0;
            let mut type_ = 0;
            unsafe {
                gl::GetActiveUniform(
                    self.id,
                    i as u32,
                    max_length,
                    &mut written_length,
                    &mut size,
                    &mut type_,
                    buffer.as_mut_ptr() as *mut GLchar,
                );
            }
            let uniform_name =
                String::from_utf8(buffer[0..written_length as usize].to_vec()).unwrap();
            let location =
                unsafe { gl::GetUniformLocation(self.id, buffer.as_ptr() as *const GLchar) };
            self.uniforms
                .insert(uniform_name.into_boxed_str(), location);
        }
    }

    pub fn contains_uniform(&self, name: &str) -> bool {
        self.uniforms.contains_key(name)
    }
}

impl Default for ShaderProgram {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

#[derive(PartialEq)]
enum UniformValue {
    Int(i32),
    UInt(u32),
    Float(f32),
    VecF3([f32; 3]),
    VecF4([f32; 4]),
    Mat3(glam::Mat3),
    Mat4(glam::Mat4),
}

impl UniformValue {
    fn from_value<T: Into<Self>>(value: T) -> Self {
        value.into()
    }
}

impl From<u32> for UniformValue {
    fn from(value: u32) -> Self {
        UniformValue::UInt(value)
    }
}

impl From<i32> for UniformValue {
    fn from(value: i32) -> Self {
        UniformValue::Int(value)
    }
}

impl From<f32> for UniformValue {
    fn from(value: f32) -> Self {
        UniformValue::Float(value)
    }
}

impl From<[f32; 3]> for UniformValue {
    fn from(value: [f32; 3]) -> Self {
        UniformValue::VecF3(value)
    }
}

impl From<[f32; 4]> for UniformValue {
    fn from(value: [f32; 4]) -> Self {
        UniformValue::VecF4(value)
    }
}

impl From<glam::Mat3> for UniformValue {
    fn from(value: glam::Mat3) -> Self {
        UniformValue::Mat3(value)
    }
}

impl From<glam::Mat4> for UniformValue {
    fn from(value: glam::Mat4) -> Self {
        UniformValue::Mat4(value)
    }
}
