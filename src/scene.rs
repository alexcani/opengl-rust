pub mod camera;
pub mod light;
pub mod object;

use std::{cell::RefCell, rc::Rc};

use crate::renderer::RenderInfo;

pub use camera::Camera;
pub use light::Light;
pub use object::{Object, Transform};

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Rc<RefCell<Object>>>,
    pub lights: Vec<Rc<RefCell<Light>>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(),
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Rc<RefCell<Object>>) {
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: Rc<RefCell<Light>>) {
        self.lights.push(light);
    }

    pub fn update(&mut self, render_info: &RenderInfo) {
        self.camera.update(render_info);

        // Rotate cubes, a bit hacky
        let mut i = 0;
        for cube in self.objects.iter() {
            let mut cube = cube.borrow_mut();
            if !cube.rotate {
                continue;
            }

            let angle = (20.0 * i as f32).to_radians();
            let axis = glam::Vec3::new(1.0, 0.3, 0.5).normalize();
            let quat = glam::Quat::from_axis_angle(axis, render_info.time.as_secs_f32() * angle);
            cube.transform.rotation = quat;
            i += 1;
        }

        for light in &self.lights {
            let mut light = light.borrow_mut();
            light.color = render_info.ui.light_color;
            if light.is_spot_light() {
                light.as_spot_light_mut().unwrap().direction = self.camera.direction();
                light.position = self.camera.position();
            }
        }
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
