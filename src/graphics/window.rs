//! GeoWindow

use std::ops::{Deref, DerefMut};

use glutin::context::PossiblyCurrentContext;
use glutin::context::PossiblyCurrentContextGlSurfaceAccessor;
use glutin::prelude::GlSurface;
use glutin::surface::{Surface, WindowSurface};
use winit::window::Window;

pub struct GeoWindow {
    pub(crate) gl: glow::Context,
    pub(crate) gl_ctx: PossiblyCurrentContext,
    pub(crate) gl_surface: Surface<WindowSurface>,
    pub(crate) window: Window,
}

impl Deref for GeoWindow {
    type Target = Window;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

impl DerefMut for GeoWindow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.window
    }
}

impl GeoWindow {
    pub fn swap_buffers(&self) {
        self.gl_ctx.make_current(&self.gl_surface).unwrap();
        self.gl_surface.swap_buffers(&self.gl_ctx).unwrap();
    }
}
