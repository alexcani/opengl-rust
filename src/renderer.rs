use std::ffi::CString;
use glutin::display::GlDisplay;

use gl::types::*;

pub struct Renderer;
impl Renderer {
    pub fn new<D: GlDisplay>(display: &D) -> Self {
        gl::load_with(|s| {
            let s = CString::new(s).unwrap();
            display.get_proc_address(s.as_c_str())
        });

        Renderer
    }

    pub fn render(&self) {
        unsafe {
            gl::ClearColor(0.6, 0.4, 0.8, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn resize(&self, width: u32, height: u32) {
        unsafe {
            gl::Viewport(0, 0, width as GLsizei, height as GLsizei);
        }
    }
}
