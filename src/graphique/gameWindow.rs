//fenetre
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

pub struct GameWindow {
    pub event_loop: EventLoop<()>,
    pub window: Window,
}

impl GameWindow {
    pub fn new(title: &str) -> Self {
        let event_loop = EventLoop::new();
        let window = Window::new(&event_loop).unwrap();
        window.set_title(title);

        Self { event_loop, window }
    }
}
 