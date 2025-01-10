use glam::{Mat4, Vec3};
use winit::keyboard::KeyCode;

use crate::renderer::RenderInfo;

pub struct Camera {
    position: Vec3,
    direction: Vec3,
    up: Vec3,
    projection_matrix: Mat4,
    view_matrix: Mat4,
}

impl Camera {
    pub fn new() -> Self {
        let mut m = Self {
            position: Vec3::Z,
            direction: Vec3::NEG_Z,
            up: Vec3::Y,
            projection_matrix: Mat4::IDENTITY,
            view_matrix: Mat4::IDENTITY,
        };
        m.view_matrix = Mat4::look_to_rh(m.position, m.direction, m.up);
        m
    }

    pub fn update(&mut self, args: &RenderInfo) {
        let input = &args.input_manager;
        let speed = 2.5 * args.dt.as_secs_f32();
        if input.is_key_pressed(KeyCode::KeyW) {
            self.position += self.direction * speed;
        }
        if input.is_key_pressed(KeyCode::KeyS) {
            self.position -= self.direction * speed;
        }
        if input.is_key_pressed(KeyCode::KeyA) {
            self.position -= self.direction.cross(self.up).normalize() * speed;
        }
        if input.is_key_pressed(KeyCode::KeyD) {
            self.position += self.direction.cross(self.up).normalize() * speed;
        }
        self.view_matrix = Mat4::look_to_rh(self.position, self.direction, self.up);
    }

    pub fn view_matrix(&self) -> &Mat4 {
        &self.view_matrix
    }

    pub fn projection_matrix(&self) -> &Mat4 {
        &self.projection_matrix
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.projection_matrix = Mat4::perspective_rh_gl(
            45.0f32.to_radians(),
            width as f32 / height as f32,
            0.1,
            100.0,
        );
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}
