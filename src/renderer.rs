mod buffer;
pub mod material;
pub mod mesh;
pub mod shader;
pub mod texture;

use std::ffi::CString;
use std::mem::MaybeUninit;
use std::time::Duration;

use glutin::display::GlDisplay;
use winit::keyboard::KeyCode;

use crate::input::InputManager;
use crate::scene::Scene;
use crate::ui::Ui;
use buffer::UniformBuffer;

use gl::types::*;

pub struct Renderer {
    wireframe: bool,
    flashlight: bool,
    camera_ubo: UniformBuffer,
    light_ubo: UniformBuffer,
}

pub struct RenderInfo<'a> {
    pub dt: Duration,   // Time since the last frame
    pub time: Duration, // Time since the start of the application
    pub input_manager: &'a InputManager,
    pub ui: &'a Ui,
}

impl Renderer {
    pub fn new<D: GlDisplay>(display: &D) -> Self {
        gl::load_with(|s| {
            let s = CString::new(s).unwrap();
            display.get_proc_address(s.as_c_str())
        });

        Renderer {
            wireframe: false,
            flashlight: false,
            camera_ubo: UniformBuffer::new(0, std::mem::size_of::<CameraUniforms>()),
            light_ubo: UniformBuffer::new(1, std::mem::size_of::<LightUniforms>()),
        }
    }

    pub fn render(&mut self, scene: &Scene, args: &RenderInfo) {
        let input = args.input_manager;
        if input.is_key_just_pressed(KeyCode::KeyL) {
            self.toggle_wireframe();
        }
        if input.is_key_just_pressed(KeyCode::KeyG) {
            self.flashlight = !self.flashlight;
        }

        let color = args.ui.clear_color;
        unsafe {
            gl::ClearColor(color[0], color[1], color[2], 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);
        }

        self.update_camera_buffer(scene);
        self.update_light_parameters(scene);

        // Render objects
        for object in &scene.objects {
            object.borrow().render();
        }
    }

    fn update_camera_buffer(&self, scene: &Scene) {
        self.camera_ubo
            .map_data(0, 1, |camera: &mut [CameraUniforms]| {
                camera[0].view = *scene.camera.view_matrix();
                camera[0].projection = *scene.camera.projection_matrix();
                camera[0].view_pos = scene.camera.position().extend(1.0);
            })
            .expect("Couldn't update camera UBO");
    }

    fn update_light_parameters(&self, scene: &Scene) {
        let mut light_uniforms = unsafe { MaybeUninit::<LightUniforms>::zeroed().assume_init() };
        for light in &scene.lights {
            let light = light.borrow();
            let color = light.color;
            let position = light.position;
            if light.is_spot_light() {
                let index = light_uniforms.nr_spot_lights as usize;
                if index >= MAX_SPOT_LIGHTS {
                    panic!("Exceeded maximum number of spot lights");
                }

                light_uniforms.spot[index].color = [color[0], color[1], color[2], 1.0];
                light_uniforms.spot[index].position = [position[0], position[1], position[2], 1.0];
                light_uniforms.spot[index].intensity = light.intensity;
                let light = light.as_spot_light().unwrap();
                let direction = light.direction;
                let attenuation = light.attenuation;
                light_uniforms.spot[index].direction =
                    [direction[0], direction[1], direction[2], 1.0];
                light_uniforms.spot[index].inner_cutoff_cos = light.inner_cutoff_rad.cos();
                light_uniforms.spot[index].outer_cutoff_cos = light.outer_cutoff_rad.cos();
                light_uniforms.spot[index].attenuation =
                    [attenuation[0], attenuation[1], attenuation[2]];
                light_uniforms.nr_spot_lights += 1;
            } else if light.is_point_light() {
                let index = light_uniforms.nr_point_lights as usize;
                if index >= MAX_POINT_LIGHTS {
                    panic!("Exceeded maximum number of point lights");
                }

                light_uniforms.point[index].color = [color[0], color[1], color[2], 1.0];
                light_uniforms.point[index].position = [position[0], position[1], position[2], 1.0];
                light_uniforms.point[index].intensity = light.intensity;
                let light = light.as_point_light().unwrap();
                let attenuation = light.attenuation;
                light_uniforms.point[index].attenuation =
                    [attenuation[0], attenuation[1], attenuation[2]];
                light_uniforms.nr_point_lights += 1;
            } else if light.is_directional_light() {
                let index = light_uniforms.nr_directional_lights as usize;
                if index >= MAX_DIRECTIONAL_LIGHTS {
                    panic!("Exceeded maximum number of directional lights");
                }

                light_uniforms.directional[index].color =
                    [light.color[0], light.color[1], light.color[2], 1.0];
                light_uniforms.directional[index].intensity = light.intensity;
                let light = light.as_directional_light().unwrap();
                let direction = light.direction;
                light_uniforms.directional[index].direction =
                    [direction[0], direction[1], direction[2], 1.0];
                light_uniforms.nr_directional_lights += 1;
            }
        }

        light_uniforms.ambient.color = {
            let ambient = &scene.ambient_light;
            [ambient.color[0], ambient.color[1], ambient.color[2], 1.0]
        };
        light_uniforms.ambient.intensity = scene.ambient_light.intensity;

        self.light_ubo
            .map_data(0, 1, |data: &mut [LightUniforms]| {
                data[0] = light_uniforms;
            })
            .expect("Couldn't update light UBO");
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        unsafe {
            gl::Viewport(0, 0, width as GLsizei, height as GLsizei);
        }
    }

    pub fn toggle_wireframe(&mut self) {
        self.wireframe = !self.wireframe;
        unsafe {
            if self.wireframe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            } else {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            }
        }
    }
}

#[repr(C)]
struct CameraUniforms {
    view: glam::Mat4,
    projection: glam::Mat4,
    view_pos: glam::Vec4,
}

#[repr(C)]
struct DirectionalLightUniforms {
    color: [f32; 4],
    direction: [f32; 4],
    intensity: f32,
    _padding: [f32; 3],
}

#[repr(C)]
struct PointLightUniforms {
    color: [f32; 4],
    position: [f32; 4],
    attenuation: [f32; 3], // constant, linear, quadratic
    intensity: f32,
}

#[repr(C)]
struct SpotLightUniforms {
    color: [f32; 4],
    position: [f32; 4],
    direction: [f32; 4],
    inner_cutoff_cos: f32,
    outer_cutoff_cos: f32,
    attenuation: [f32; 3], // constant, linear, quadratic
    intensity: f32,
    _padding: [f32; 2],
}

#[repr(C)]
struct AmbientLightUniforms {
    color: [f32; 4],
    intensity: f32,
    _padding: [f32; 3],
}

const MAX_POINT_LIGHTS: usize = 10;
const MAX_SPOT_LIGHTS: usize = 5;
const MAX_DIRECTIONAL_LIGHTS: usize = 5;

#[repr(C)]
struct LightUniforms {
    ambient: AmbientLightUniforms,
    directional: [DirectionalLightUniforms; MAX_DIRECTIONAL_LIGHTS],
    point: [PointLightUniforms; MAX_POINT_LIGHTS],
    spot: [SpotLightUniforms; MAX_SPOT_LIGHTS],
    nr_point_lights: i32,
    nr_spot_lights: i32,
    nr_directional_lights: i32
}
