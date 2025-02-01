use glam::{Mat4, Vec3};
use winit::keyboard::KeyCode;

use crate::renderer::RenderInfo;

pub struct Camera {
    position: Vec3,
    direction: Vec3,
    up: Vec3,
    projection_matrix: Mat4,
    view_matrix: Mat4,
    pitch: f32, // In degrees
    yaw: f32,   // In degrees
    // Perspective parameters
    width: u32,
    height: u32,
    fov: f32,
    near: f32,
    far: f32,
}

impl Camera {
    pub fn new() -> Self {
        let mut m = Self {
            position: Vec3::Z,
            direction: Vec3::NEG_Z,
            up: Vec3::Y,
            projection_matrix: Mat4::IDENTITY,
            view_matrix: Mat4::IDENTITY,
            pitch: 0.0,
            yaw: -90.0,
            width: 800,
            height: 600,
            fov: 45.0,
            near: 0.1,
            far: 100.0,
        };
        m.view_matrix = Mat4::look_to_rh(m.position, m.direction, m.up);
        m
    }

    pub fn update(&mut self, args: &RenderInfo) {
        self.update_direction(args);
        self.update_position(args);
        self.view_matrix = Mat4::look_to_rh(self.position, self.direction, self.up);

        self.update_projection(args);
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn view_matrix(&self) -> &Mat4 {
        &self.view_matrix
    }

    pub fn projection_matrix(&self) -> &Mat4 {
        &self.projection_matrix
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    fn update_direction(&mut self, args: &RenderInfo) {
        let input = &args.input_manager;
        if !input.is_mouse_button_pressed(winit::event::MouseButton::Right) {
            return;
        }
        let mouse_delta = input.mouse_delta();
        let sensitivity = args.ui.camera_sensitivity;
        self.yaw += mouse_delta.0 as f32 * sensitivity;
        self.pitch -= mouse_delta.1 as f32 * sensitivity;
        self.pitch = self.pitch.clamp(-89.0, 89.0);

        let direction = Vec3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        );
        self.direction = direction.normalize();
    }

    fn update_position(&mut self, args: &RenderInfo) {
        let input = &args.input_manager;
        let speed = args.ui.camera_speed * args.dt.as_secs_f32();
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
        if input.is_key_pressed(KeyCode::KeyR) {
            self.position += self.up * speed;
        }
        if input.is_key_pressed(KeyCode::KeyF) {
            self.position -= self.up * speed;
        }
    }

    fn update_projection(&mut self, args: &RenderInfo) {
        let input = args.input_manager;
        let fov = self.fov - input.mouse_wheel_delta();
        self.fov = fov.clamp(1.0, 45.0);
        let aspect = self.width as f32 / self.height as f32;
        self.projection_matrix = Mat4::perspective_rh_gl(self.fov.to_radians(), aspect, self.near, self.far);
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}
