use std::rc::Rc;

use crate::renderer::mesh::Mesh;
use crate::renderer::shader::ShaderProgram;

pub struct Transform {
    pub position: glam::Vec3,
    pub scale: glam::Vec3,
    pub rotation: glam::Quat,
}

impl Transform {
    pub fn new(position: glam::Vec3, scale: glam::Vec3, rotation: glam::Quat) -> Self {
        Self {
            position,
            scale,
            rotation,
        }
    }

    pub fn model_matrix(&self) -> glam::Mat4 {
        glam::Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }

    pub fn normal_matrix(&self) -> glam::Mat3 {
        glam::Mat3::from_mat4(self.model_matrix()).inverse().transpose()
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new(glam::Vec3::ZERO, glam::Vec3::ONE, glam::Quat::IDENTITY)
    }
}

pub struct Object {
    pub transform: Transform,
    mesh: Rc<Mesh>,
}

impl Object {
    pub fn new(mesh: Rc<Mesh>) -> Self {
        Self {
            transform: Transform::default(),
            mesh,
        }
    }

    pub fn render(&self, shader: &mut ShaderProgram) {
        shader.set_uniform_mat4("model", &self.transform.model_matrix());
        let normal_matrix_uniform = "normal_matrix";
        if shader.contains_uniform(normal_matrix_uniform) {
            shader.set_uniform_mat3(normal_matrix_uniform, &self.transform.normal_matrix());
        }
        self.mesh.draw();
    }
}
