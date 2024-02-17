use std::sync::Arc;

use winit_input_helper::WinitInputHelper;

pub trait Application {
    fn update(&mut self);
    fn draw(&mut self);
    fn handle_input(&mut self, input: &WinitInputHelper);
}

pub struct WindowManager {
    input: WinitInputHelper,
    event_loop: winit::event_loop::EventLoop<()>,
    pub window: Arc<winit::window::Window>,
}

impl WindowManager {
    pub fn new(title: &str, size: (u32, u32)) -> Self {
        let input = WinitInputHelper::new();
        let event_loop = winit::event_loop::EventLoop::new().unwrap();
        let window = Arc::new(
            winit::window::WindowBuilder::new()
                .with_title(title)
                .with_inner_size(winit::dpi::PhysicalSize::new(size.0, size.1))
                .build(&event_loop)
                .unwrap(),
        );

        Self {
            input,
            event_loop,
            window,
        }
    }

    pub fn run(mut self, mut app: Box<dyn Application>) {
        self.event_loop
            .run(move |event, elwt| {
                if let winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::RedrawRequested,
                    ..
                } = event
                {
                    app.draw();
                }

                if self.input.update(&event) {
                    if self.input.close_requested() || self.input.destroyed() {
                        elwt.exit();
                        return;
                    }

                    app.handle_input(&self.input);
                    app.update();
                    self.window.request_redraw();
                }
            })
            .expect("Event loop failed to run.");
    }
}
