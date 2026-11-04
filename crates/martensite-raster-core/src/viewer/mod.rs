//! `RasterViewer`: up to `MAXVIEW` views and the list of displayed drawables (`dout`).
//!
//! In batch mode (no window system) every operation does nothing and the queries return
//! the C++ batch values, exactly as each C++ method starts with `if (RasterBatch) return`.
//! A view index outside `0..MAXVIEW` is treated as an absent view (the C++ code indexes
//! its array without a check).
mod drawing;
mod picking;
mod views;

pub use picking::RasterSelection;

use super::display::DisplayState;
use super::view::RasterView;
use super::window::RasterWindowSystem;
use crate::drawable::{DrawableAdapter, NativeDrawableAdapter, RasterDrawable};
use std::marker::PhantomData;
use std::rc::Rc;

/// `MAXVIEW`: the number of view slots.
pub const MAXVIEW: usize = 30;

/// The command-host viewer.
pub struct RasterViewer<
    D: ?Sized = dyn RasterDrawable,
    A: DrawableAdapter<D> = NativeDrawableAdapter,
> {
    system: Option<Rc<dyn RasterWindowSystem>>,
    views: Vec<Option<RasterView>>,
    drawables: Vec<Rc<D>>,
    state: DisplayState,
    adapter: PhantomData<A>,
}

impl<D: ?Sized, A: DrawableAdapter<D>> RasterViewer<D, A> {
    /// Creates empty view slots using the selected stateless drawable adapter.
    /// Pass `None` for inert batch mode; `RasterViewer::with_adapter(Some(system))`.
    pub fn with_adapter(system: Option<Rc<dyn RasterWindowSystem>>) -> Self {
        let views = if system.is_some() {
            (0..MAXVIEW).map(|_| None).collect()
        } else {
            Vec::new()
        };
        Self {
            system,
            views,
            drawables: Vec::new(),
            state: DisplayState::default(),
            adapter: PhantomData,
        }
    }

    /// `RasterBatch`: true without a window system.
    pub fn is_batch(&self) -> bool {
        self.system.is_none()
    }

    /// `DefineColor`: allocates palette entry `index` (true in batch mode).
    pub fn define_color(&self, index: usize, name: &str) -> bool {
        self.system
            .as_ref()
            .is_none_or(|system| system.define_color(index, name))
    }

    /// `Flush`: sends pending drawing to the screen.
    pub fn flush(&self) {
        if let Some(system) = &self.system {
            system.flush();
        }
    }

    /// `GetDrawables`: the displayed drawables, in display order.
    pub fn drawables(&self) -> &[Rc<D>] {
        &self.drawables
    }

    /// The shared drawing state (read access for tests and PostScript output).
    pub fn display_state(&self) -> &DisplayState {
        &self.state
    }

    /// The view in slot `id`, if any.
    pub fn view(&self, id: i32) -> Option<&RasterView> {
        let index = usize::try_from(id).ok()?;
        self.views.get(index)?.as_ref()
    }

    /// The view in slot `id`, for modification.
    pub fn view_mut(&mut self, id: i32) -> Option<&mut RasterView> {
        let index = usize::try_from(id).ok()?;
        self.views.get_mut(index)?.as_mut()
    }

    /// The ids of the existing views.
    fn view_ids(&self) -> Vec<i32> {
        (0..self.views.len() as i32)
            .filter(|id| self.view(*id).is_some())
            .collect()
    }
}
