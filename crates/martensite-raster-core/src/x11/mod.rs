//! Unmapped windows retain server-backed raster storage for repeatable capture.
mod surface;
use crate::{COLOR_NAMES, MAXCOLOR};
use crate::{PointerEvent, RasterWindow, RasterWindowSystem};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkservice::aspect::{
    AspectDisplayConnection, AspectDisplayConnectionLease, AspectDisplayLeasePurpose,
};
use martensite_tkservice::xw::{XwRasterCanvas, xw_raster_flush, xw_raster_named_pixel};
use std::{cell::RefCell, rc::Rc};
use surface::VirtualWindow;

/// Real X11 virtual-window backend; `VirtualWindowSystem::connect_virtual()?`.
/// A live `DISPLAY` is required even though windows remain unmapped; this backend is thread-confined.
pub struct VirtualWindowSystem<C: WindowFailureContext = RasterFailureContext> {
    context: std::marker::PhantomData<C>,
    connection: Handle<AspectDisplayConnection>,
    lease: AspectDisplayConnectionLease,
    palette: Rc<RefCell<[u64; MAXCOLOR]>>,
}
impl<C: WindowFailureContext> VirtualWindowSystem<C> {
    /// Opens the native display and resolves the standard palette; `Self::connect_virtual()?`.
    pub fn connect_virtual() -> Result<Self, Failure> {
        let connection = Handle::new(AspectDisplayConnection::new()?);
        let lease =
            AspectDisplayConnection::acquire_lease(&connection, AspectDisplayLeasePurpose::Window)
                .map_err(|failure| C::failure(&format!("display lease refused: {failure:?}")))?;
        let system = Self {
            context: std::marker::PhantomData,
            connection,
            lease,
            palette: Rc::new(RefCell::new([0; MAXCOLOR])),
        };
        for (index, name) in COLOR_NAMES.iter().enumerate() {
            if !system.define_color(index, name) {
                return Err(C::failure(&format!("palette color={name:?}")));
            }
        }
        Ok(system)
    }
}
/// Supply contextual diagnostics without coupling raster operations to an application.
pub trait WindowFailureContext {
    /// Describe a failed allocation; `RasterFailureContext::failure("existing window=7")`.
    fn failure(value: &str) -> Failure;
}
/// Default diagnostic context for standalone raster captures.
pub struct RasterFailureContext;
impl WindowFailureContext for RasterFailureContext {
    fn failure(value: &str) -> Failure {
        Failure::with_type(
            "Aspect_WindowDefinitionError",
            &format!(
                "Raster X11 {value}; expected a new virtual window and a resolvable palette color"
            ),
            "VirtualWindowSystem",
        )
    }
}
impl<C: WindowFailureContext> RasterWindowSystem for VirtualWindowSystem<C> {
    fn create_window(
        &self,
        title: &str,
        position: (i32, i32),
        size: (i32, i32),
        existing: u64,
    ) -> Box<dyn RasterWindow> {
        self.try_create_window(title, position, size, existing).expect("native window allocation failed; use try_create_window to preserve the typed failure")
    }
    fn try_create_window(
        &self,
        title: &str,
        position: (i32, i32),
        size: (i32, i32),
        existing: u64,
    ) -> Result<Box<dyn RasterWindow>, Failure> {
        if existing != 0 {
            return Err(C::failure(&format!("existing window={existing}")));
        }
        let canvas = XwRasterCanvas::create(
            &self.connection,
            title,
            [position.0, position.1],
            [size.0, size.1],
        )?;
        Ok(Box::new(VirtualWindow::new(
            canvas,
            Rc::clone(&self.palette),
        )))
    }
    fn define_color(&self, index: usize, name: &str) -> bool {
        if index >= MAXCOLOR {
            return false;
        }
        let Some(pixel) = xw_raster_named_pixel(&self.lease, name) else {
            return false;
        };
        self.palette.borrow_mut()[index] = pixel;
        true
    }
    fn flush(&self) {
        xw_raster_flush(&self.lease);
    }
    fn next_event(&self) -> PointerEvent {
        // Unmapped capture windows do not consume interactive pointer events.
        PointerEvent::default()
    }
}
