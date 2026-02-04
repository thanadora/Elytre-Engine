use crate::get_storages_mut;

pub struct GameEngine {
    pub window: GameWindow,
    pub gfx: GraphicsContext,
}

impl GameEngine {
    pub async fn new() -> Self {
        let window = GameWindow::new("moteur de jeu");
        let gfx = GraphicsContext::new(&window.window).await;

        Self { window, gfx }
    }

    pub fn run(self) {
        self.window.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => {}
                },
                Event::RedrawRequested(_) => {
                    // Ici tu mettras ton RenderSystem
                }
                _ => {}
            }
        });
    }
}
