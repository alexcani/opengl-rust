mod app;

use std::error::Error;
use winit::event_loop::EventLoop;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = app::App::new();
    let event_loop = EventLoop::new()?;
    event_loop.run_app(&mut app)?;

    app.get_exit_state()
}
