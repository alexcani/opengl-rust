use std::error::Error;
use std::sync::Arc;
use std::time::Instant;

use glutin::context::{ContextAttributesBuilder, GlProfile, PossiblyCurrentContext};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, WindowSurface};
use glutin_winit::{DisplayBuilder, GlWindow};
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;
use winit::raw_window_handle::HasWindowHandle;
use winit::window::{CursorGrabMode, Window};

use opengl_rust::input::InputManager;
use opengl_rust::renderer::{RenderInfo, Renderer};
use opengl_rust::ui::Ui;

struct GfxData {
    surface: Surface<WindowSurface>,
    context: PossiblyCurrentContext,
    cursor_grabbed: bool,
    egui_glow: egui_glow::EguiGlow,
    // Must be dropped last
    window: Window,
}

pub struct App {
    gfx_data: Option<GfxData>,
    renderer: Option<Renderer>,
    gui: Ui,
    input_manager: InputManager,
    start_time: Instant,
    last_frame_time: Instant,
    exit_state: Result<(), Box<dyn Error>>,
}

impl App {
    pub fn new() -> Self {
        App {
            gfx_data: None,
            renderer: None,
            gui: Ui::default(),
            input_manager: InputManager::default(),
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
            surface,
            context,
            egui_glow,
            window,
            ..
        }) = self.gfx_data.as_mut()
        {
            let now = Instant::now();
            let dt = now.duration_since(self.last_frame_time);
            let time = now.duration_since(self.start_time);
            self.last_frame_time = now;

            // Update the UI
            egui_glow.run(window, |ctx| {
                self.gui.run(ctx);
            });

            let renderer = self.renderer.as_mut().unwrap();
            renderer.render(&RenderInfo {
                dt,
                time,
                input_manager: &self.input_manager,
                ui: &self.gui,
            });

            // Render UI on top of everything
            egui_glow.paint(window);

            surface.swap_buffers(context).unwrap();
            self.input_manager.update();
        }
    }

    fn toggle_cursor_grab(&mut self) {
        if let Some(GfxData {
            cursor_grabbed,
            ..
        }) = self.gfx_data.as_mut()
        {
            *cursor_grabbed = !*cursor_grabbed;
        }
        self.apply_cursor_grab();
    }

    fn apply_cursor_grab(&self) {
        if let Some(GfxData {
            window,
            cursor_grabbed,
            ..
        }) = self.gfx_data.as_ref()
        {
            if *cursor_grabbed {
                let _ = window
                    .set_cursor_grab(CursorGrabMode::Confined)
                    .or_else(|_| window.set_cursor_grab(CursorGrabMode::Locked))
                    .map(|_| {
                        window.set_cursor_visible(false);
                    }); // it's okay if this fails, state is the same as before
            } else {
                window.set_cursor_visible(true);
                window.set_cursor_grab(CursorGrabMode::None).unwrap();
            }
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.gfx_data.is_some() {
            panic!("Resumed called twice");
        }

        let attributes = Window::default_attributes().with_title("OpenGL");
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

        // Initialize glow for egui
        let glow_ctx = unsafe {
            egui_glow::glow::Context::from_loader_function(|s| {
                let s = std::ffi::CString::new(s).unwrap();
                config.display().get_proc_address(s.as_c_str())
            })
        };
        let egui_glow = egui_glow::EguiGlow::new(event_loop, Arc::new(glow_ctx), None, None, true);
        egui_glow.egui_ctx.set_theme(egui::Theme::Dark);

        self.gfx_data = Some(GfxData {
            surface,
            context,
            cursor_grabbed: false,
            egui_glow,
            window,
        });
        self.renderer = Some(Renderer::new(&config.display()));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let gfx_data = self.gfx_data.as_mut().unwrap();
        let event_result = gfx_data.egui_glow.on_window_event(&gfx_data.window, &event);

        if event_result.consumed {
            return;
        }
        if event_result.repaint {
            gfx_data.window.request_redraw();
        }

        if self.gui.quit {
            event_loop.exit();
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.render_and_swap();
            }
            WindowEvent::Resized(size) if size.height > 0 && size.width > 0 => {
                let renderer = self.renderer.as_mut().unwrap();
                renderer.resize(size.width, size.height);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                self.input_manager.process_key_event(&event);
                if self.input_manager.is_key_just_pressed(KeyCode::Escape) {
                    event_loop.exit();
                }
                if self.input_manager.is_key_just_pressed(KeyCode::AltLeft) {
                    self.toggle_cursor_grab();
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.input_manager
                    .process_mouse_position(position.x, position.y);
            }
            WindowEvent::MouseWheel {
                delta: MouseScrollDelta::LineDelta(_, y),
                ..
            } => {
                self.input_manager.process_mouse_wheel_scroll(y);
            },
            WindowEvent::MouseInput { state, button, .. } => {
                self.input_manager.process_mouse_button(button, state);
                if self.input_manager.is_mouse_button_just_pressed(MouseButton::Right) {
                    gfx_data.cursor_grabbed = true;
                    self.apply_cursor_grab();
                } else if self.input_manager.is_mouse_button_just_released(MouseButton::Right) {
                    gfx_data.cursor_grabbed = false;
                    self.apply_cursor_grab();
                }
            }
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        if let DeviceEvent::MouseMotion { delta } = event {
            self.input_manager.process_mouse_delta(delta.0, delta.1);
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
