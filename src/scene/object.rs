use std::rc::Rc;
use std::cell::RefCell;

use crate::renderer::mesh::Mesh;
use crate::renderer::material::{Material, PropertiesMap};

#[derive(Debug)]
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
}

impl Default for Transform {
    fn default() -> Self {
        Self::new(glam::Vec3::ZERO, glam::Vec3::ONE, glam::Quat::IDENTITY)
    }
}

pub struct Object {
    pub transform: Transform,
    pub rotate: bool,
    pub material_overrides: PropertiesMap,
    material: Rc<RefCell<Material>>,
    mesh: Rc<Mesh>,
}

impl Object {
    pub fn new(mesh: Rc<Mesh>, material: Rc<RefCell<Material>>) -> Self {
        Self {
            transform: Transform::default(),
            rotate: false,
            material_overrides: PropertiesMap::new(),
            material,
            mesh,
        }
    }

    pub fn render(&self) {
        let material = self.material.borrow();
        material.use_material(&self.material_overrides);
        material.shader().set_uniform_mat4("model", &self.transform.model_matrix());
        self.mesh.draw();
    }
}
