use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::Graphics::window::GeoWindow;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{WindowBuilder, WindowId};

static CORE_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct GeoCore {
    event_loop: EventLoop<()>,
    windows: HashMap<WindowId, GeoWindow>,
}

impl GeoCore {
    pub fn init() -> Self {
        if CORE_INITIALIZED.load(Ordering::Relaxed) {
            panic!("The GeoCore can only be initialized once.");
        }

        let event_loop = EventLoop::new();

        Self {
            event_loop,
            windows: HashMap::new(),
        }
    }

    pub fn create_window(&mut self, title: &str, width: u32, height: u32) -> &GeoWindow {
        // TODO: proper error handling
        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(width, height))
            .with_title(title)
            .build(&self.event_loop)
            .expect("Failed to create GeoWindow.");

        let id = window.id();
        self.windows.insert(id, GeoWindow { window });

        self.windows.get(&id).unwrap();
    }

    pub fn run(self) -> ! {
        let GeoCore {
            event_loop,
            mut windows,
        } = self;

        event_loop.run(move |event, _event_loop, control_flow| {
            control_flow.set_wait();

            if let Event::WindowEvent { event, window_id } = event {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        is_synthetic: false,
                        ..
                    } => {
                        eprintln!("Closing window {window_id:?}");

                        // This drops the window, causing it to close.
                        windows.remove(&window_id);

                        if windows.is_empty() {
                            control_flow.set_exit();
                        }
                    }
                    _ => (),
                }
            }
        })
    }
}
