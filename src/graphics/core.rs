use std::collections::HashMap;
use std::env;
use std::ffi::CString;
use std::num::NonZeroU32;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::graphics::window::GeoWindow;
use glow::HasContext;
use glutin::context::{ContextApi, ContextAttributesBuilder, Version};
use glutin::display::GetGlDisplay;
use glutin::prelude::{
    GlConfig, GlDisplay, NotCurrentGlContextSurfaceAccessor,
    PossiblyCurrentContextGlSurfaceAccessor,
};
use glutin::surface::{GlSurface, SurfaceAttributesBuilder};
use glutin_winit::ApiPrefence;
use raw_window_handle::HasRawWindowHandle;
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

        if cfg!(target_os = "linux") {
            // disables vsync sometimes on x11
            if env::var("vblank_mode").is_err() {
                env::set_var("vblank_mode", "0");
            }
        }

        Self {
            event_loop,
            windows: HashMap::new(),
        }
    }

    pub fn create_window(&mut self, title: &str, width: u32, height: u32) -> &GeoWindow {
        // TODO: proper error handling

        let window_builder = WindowBuilder::new()
            .with_resizable(true)
            .with_inner_size(PhysicalSize::new(width, height))
            .with_title(title);

        let (window, gl_config) = glutin_winit::DisplayBuilder::new()
            .with_preference(ApiPrefence::FallbackEgl)
            .with_window_builder(Some(window_builder))
            .build(&self.event_loop, <_>::default(), |configs| {
                configs
                    .filter(|c| c.srgb_capable())
                    .max_by_key(|c| c.num_samples())
                    .unwrap()
            })
            .expect("Couldn't build window display.");

        let window = window.unwrap(); // set in display builder
        let raw_window_handle = window.raw_window_handle();
        let gl_display = gl_config.display();

        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 1))))
            .with_profile(glutin::context::GlProfile::Core)
            .build(Some(raw_window_handle));

        let dimensions = window.inner_size();

        let (gl_surface, gl_ctx) = {
            let attrs = SurfaceAttributesBuilder::<glutin::surface::WindowSurface>::new().build(
                raw_window_handle,
                NonZeroU32::new(dimensions.width).unwrap(),
                NonZeroU32::new(dimensions.height).unwrap(),
            );

            // Lots of unwraps there... Will be way better once there's proper error handling
            let surface = unsafe { gl_display.create_window_surface(&gl_config, &attrs) }.unwrap();

            let context = unsafe { gl_display.create_context(&gl_config, &context_attributes) }
                .unwrap()
                .make_current(&surface)
                .unwrap();
            (surface, context)
        };

        // Load the OpenGL function pointers
        let gl = unsafe {
            glow::Context::from_loader_function(|symbol| {
                gl_display.get_proc_address(&CString::new(symbol).unwrap()) as *const _
            })
        };

        let id = window.id();
        let geo_window = GeoWindow {
            gl,
            gl_ctx,
            gl_surface,
            window,
        };

        Self::redraw_window(&geo_window);

        self.windows.insert(id, geo_window);
        self.windows.get(&id).unwrap()
    }

    fn redraw_window(window: &GeoWindow) {
        let gl = &window.gl;
        unsafe {
            gl.clear_color(0.1, 0.2, 0.3, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
        }

        window.gl_ctx.make_current(&window.gl_surface).unwrap();
        window.gl_surface.swap_buffers(&window.gl_ctx).unwrap();
    }

    pub fn run(self) -> ! {
        let GeoCore {
            event_loop,
            mut windows,
        } = self;

        event_loop.run(move |event, _event_loop, control_flow| {
            control_flow.set_wait();

            match event {
                Event::RedrawRequested(ref window_id) => {
                    let window = windows.get(window_id).unwrap();
                    Self::redraw_window(window);
                }

                Event::WindowEvent {
                    event,
                    ref window_id,
                } => {
                    let Some(window) = windows.get(window_id) else {
                        return;
                    };

                    match event {
                        WindowEvent::Resized(physical_size) => {
                            window.gl_surface.resize(
                                &window.gl_ctx,
                                NonZeroU32::new(physical_size.width).unwrap(),
                                NonZeroU32::new(physical_size.height).unwrap(),
                            );

                            window.request_redraw();
                        }
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
                            // This drops the window, causing it to close.
                            windows.remove(window_id);

                            if windows.is_empty() {
                                control_flow.set_exit();
                            }
                        }
                        _ => (),
                    }
                }

                _ => (),
            }
        })
    }
}
