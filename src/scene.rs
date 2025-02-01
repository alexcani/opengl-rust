pub mod camera;
pub mod object;

use std::{cell::RefCell, rc::Rc};

pub use camera::Camera;
pub use object::{Object, Transform};

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Rc<RefCell<Object>>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(),
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Rc<RefCell<Object>>) {
        self.objects.push(object);
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
