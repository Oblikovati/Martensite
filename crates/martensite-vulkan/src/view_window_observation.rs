//! A view reads its native window's size and mapped state once and keeps them until the
//! window is set again or the view is resized, as OCCT's GL view keeps its window size
//! between `MustBeResized` calls (#594). Each live read is an X server round trip, and the
//! WSI threads share that connection, so a per-frame read cost more than a whole frame budget.
use crate::Graphic3dRenderView;
use martensite_tkernel::standard::SharedState;
use martensite_tkservice::aspect::{AspectWindow, AspectWindowMembers};
use martensite_tkservice::graphic3d::Graphic3dCView;

/// The size of the bound window, kept only once the window was seen mapped.
#[derive(Default)]
pub(crate) struct WindowObservation {
    mapped_size: SharedState<Option<(i32, i32)>>,
}

impl WindowObservation {
    /// Forget the kept size; the next frame reads the window again; `observation.forget()`.
    pub fn forget(&self) {
        *self.mapped_size.write() = None;
    }
    fn kept(&self) -> Option<(i32, i32)> {
        *self.mapped_size.read()
    }
    fn keep(&self, size: (i32, i32)) {
        *self.mapped_size.write() = Some(size);
    }
}

impl Graphic3dRenderView {
    /// The bound window's size, read live until the window was seen mapped;
    /// `view.observed_window_size()`.
    pub(crate) fn observed_window_size(&self) -> Option<(i32, i32)> {
        if let Some(size) = self.window_observation.kept() {
            return Some(size);
        }
        self.window().get().map(|window| window.size())
    }
    /// Whether the bound window is mapped. A mapped window's size is kept, so later frames
    /// need no X request; an unmapped window is read again next time;
    /// `view.observed_window_mapped(&*window)`.
    pub(crate) fn observed_window_mapped(&self, window: &dyn AspectWindow) -> bool {
        if self.window_observation.kept().is_some() {
            return true;
        }
        if !window.is_mapped() {
            return false;
        }
        // A virtual window answers from its stored rectangle, so it needs no kept copy.
        if !window.is_virtual() {
            self.window_observation.keep(window.size());
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::WindowObservation;

    #[test]
    fn a_kept_size_lasts_until_it_is_forgotten() {
        let observation = WindowObservation::default();
        assert_eq!(observation.kept(), None);
        observation.keep((640, 480));
        assert_eq!(observation.kept(), Some((640, 480)));
        observation.forget();
        assert_eq!(observation.kept(), None);
    }
}
