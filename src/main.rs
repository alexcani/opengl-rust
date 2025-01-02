use std::error::Error;
use std::ffi::CString;
use std::time::Instant;

use glutin::context::{ContextAttributesBuilder, GlProfile, PossiblyCurrentContext};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, WindowSurface};
use glutin_winit::{DisplayBuilder, GlWindow};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::raw_window_handle::HasWindowHandle;
use winit::window::Window;

use gl::types::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();
    let event_loop = EventLoop::new()?;
    event_loop.run_app(&mut app)?;
    app.exit_state
}

struct GxfData {
    window: Window,
    surface: Surface<WindowSurface>,
    context: PossiblyCurrentContext,
}

struct App {
    gfx_data: Option<GxfData>,
    renderer: Option<Renderer>,
    exit_state: Result<(), Box<dyn Error>>,
    frame_count: u64,
    frame_time: Instant,
}

impl App {
    fn new() -> Self {
        App {
            gfx_data: None,
            renderer: None,
            exit_state: Ok(()),
            frame_count: 0,
            frame_time: Instant::now(),
        }
    }

    fn render_and_swap(&mut self) {
        if let Some(GxfData {
            window: _,
            surface,
            context,
        }) = self.gfx_data.as_ref()
        {
            let renderer = self.renderer.as_ref().unwrap();
            renderer.render();
            surface.swap_buffers(context).unwrap();
            self.frame_count = self.frame_count.checked_add(1).unwrap();
            if self.frame_time.elapsed().as_millis() >= 1000 {
                println!("FPS: {}", self.frame_count);
                self.frame_count = 0;
                self.frame_time = Instant::now();
            }
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.gfx_data.is_some() {
            panic!("Resumed called twice");
        }

        let attributes = Window::default_attributes().with_title("Triangle");
        let template_builder = glutin::config::ConfigTemplateBuilder::new();
        let (window, config) = DisplayBuilder::new()
            .with_window_attributes(Some(attributes))
            .build(event_loop, template_builder, |configs| {
                configs
                    .reduce(|accum, config| {
                        if config.num_samples() > accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            })
            .unwrap();

        let window = window.unwrap();
        let raw_window_handle = window.window_handle().ok().map(|wh| wh.as_raw());
        let context_attributes = ContextAttributesBuilder::new()
            .with_profile(GlProfile::Core)
            .build(raw_window_handle);
        let context = unsafe {
            config
                .display()
                .create_context(&config, &context_attributes)
                .expect("Unable to create context")
        };

        let surface_attributes = window.build_surface_attributes(Default::default()).unwrap();
        let surface = unsafe {
            config
                .display()
                .create_window_surface(&config, &surface_attributes)
                .unwrap()
        };
        let context = context.make_current(&surface).unwrap();

        self.gfx_data = Some(GxfData {
            window,
            surface,
            context,
        });
        self.renderer = Some(Renderer::new(&config.display()));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.render_and_swap();
            },
            WindowEvent::Resized(size) if size.height > 0 && size.width > 0 => {
                let renderer = self.renderer.as_ref().unwrap();
                renderer.resize(size.width, size.height);
            },
            _ => {},
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        println!("Exiting");
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(GxfData { window, .. }) = self.gfx_data.as_ref() {
            window.request_redraw();
        }
    }
}

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
