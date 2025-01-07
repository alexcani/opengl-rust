use std::error::Error;
use std::time::Instant;

use glutin::context::{ContextAttributesBuilder, GlProfile, PossiblyCurrentContext};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, WindowSurface};
use glutin_winit::{DisplayBuilder, GlWindow};
use winit::application::ApplicationHandler;
use winit::event::{KeyEvent, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::raw_window_handle::HasWindowHandle;
use winit::window::Window;

use crate::renderer::{Renderer, RenderInfo};

struct GfxData {
    surface: Surface<WindowSurface>,
    context: PossiblyCurrentContext,
    // Must be dropped last
    window: Window,
}

pub struct App {
    gfx_data: Option<GfxData>,
    renderer: Option<Renderer>,
    start_time: Instant,
    last_frame_time: Instant,
    exit_state: Result<(), Box<dyn Error>>,
}

impl App {
    pub fn new() -> Self {
        App {
            gfx_data: None,
            renderer: None,
            start_time: Instant::now(),
            last_frame_time: Instant::now(),
            exit_state: Ok(()),
        }
    }

    /**
     * Consumes the App and returns the exit state.
     */
    pub fn get_exit_state(self) -> Result<(), Box<dyn Error>> {
        self.exit_state
    }

    fn render_and_swap(&mut self) {
        if let Some(GfxData {
            window: _,
            surface,
            context,
        }) = self.gfx_data.as_ref()
        {
            let now = Instant::now();
            let dt = now.duration_since(self.last_frame_time);
            let time = now.duration_since(self.start_time);
            self.last_frame_time = now;

            let renderer = self.renderer.as_mut().unwrap();
            renderer.render(RenderInfo { dt, time });
            surface.swap_buffers(context).unwrap();
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

        self.gfx_data = Some(GfxData {
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
            }
            WindowEvent::RedrawRequested => {
                self.render_and_swap();
            }
            WindowEvent::Resized(size) if size.height > 0 && size.width > 0 => {
                let renderer = self.renderer.as_ref().unwrap();
                renderer.resize(size.width, size.height);
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        repeat: false,
                        physical_key: PhysicalKey::Code(key_code),
                        state,
                        ..
                    },
                ..
            } => {
                match key_code {
                    KeyCode::Escape => {
                        event_loop.exit();
                    }
                    KeyCode::KeyL => {
                        if state == winit::event::ElementState::Pressed {
                            let renderer = self.renderer.as_mut().unwrap();
                            renderer.toggle_wireframe();
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        self.gfx_data = None;
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(GfxData { window, .. }) = self.gfx_data.as_ref() {
            window.request_redraw();
        }
    }
}
