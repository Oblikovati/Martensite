//! View output modes the shared retained view does not keep: immediate draw-to-front and flip.
//!
//! The shared TKService retained view answers `SetImmediateModeDrawToFront` with a constant and
//! has no flip flag. The renderer view wraps it; method calls the shared forwarding makes on
//! the `retained` field resolve to these inherent methods first and to the shared view otherwise.
use martensite_tkservice::graphic3d::{
    Graphic3dRenderTransparentMethod, Graphic3dRenderingParams, Graphic3dRetainedView,
};
use std::sync::atomic::{AtomicBool, Ordering};

/// Shared retained view plus the renderer's output-mode flags; `view.retained.to_flip_output()`.
pub(crate) struct OutputModeRetainedView {
    shared: Graphic3dRetainedView,
    /// Native GlView starts with transient (immediate) drawing to the front buffer enabled.
    draw_to_front: AtomicBool,
    /// Native GlView starts without flipped output.
    flip_output: AtomicBool,
}

impl OutputModeRetainedView {
    /// Wrap a freshly created shared view with native default modes; `OutputModeRetainedView::new(shared)`.
    pub(crate) fn new(shared: Graphic3dRetainedView) -> Self {
        Self {
            shared,
            draw_to_front: AtomicBool::new(true),
            flip_output: AtomicBool::new(false),
        }
    }

    /// Store the immediate-mode target and return the previous one, as natively;
    /// `let previous = view.retained.set_immediate_mode_draw_to_front(false);`.
    pub(crate) fn set_immediate_mode_draw_to_front(&self, draw_to_front_buffer: bool) -> bool {
        self.draw_to_front
            .swap(draw_to_front_buffer, Ordering::Relaxed)
    }

    /// Read the flip request; `view.retained.to_flip_output()`.
    pub(crate) fn to_flip_output(&self) -> bool {
        self.flip_output.load(Ordering::Relaxed)
    }

    /// Store the flip request; `view.retained.set_to_flip_output(true)`.
    pub(crate) fn set_to_flip_output(&self, flip: bool) {
        self.flip_output.store(flip, Ordering::Relaxed);
    }

    /// Whether a frame drawn with `params` reaches its target upside down.
    ///
    /// Natively the flip happens only in the textured blit from an intermediate scene buffer.
    /// That buffer exists when immediate drawing goes to the front buffer or OIT is requested,
    /// and a multisampled buffer is resolved by a framebuffer blit that ignores the flip
    /// (the recorded view_flip_output capture, identical on llvmpipe and AMD).
    /// `view.retained.flips_output(&view.rendering_params)`.
    pub(crate) fn flips_output(&self, params: &Graphic3dRenderingParams) -> bool {
        let draw_to_front = self.draw_to_front.load(Ordering::Relaxed);
        self.to_flip_output() && flip_reaches_target(draw_to_front, params)
    }
}

/// The native intermediate-buffer condition of a requested flip; `flip_reaches_target(true, &params)`.
fn flip_reaches_target(draw_to_front: bool, params: &Graphic3dRenderingParams) -> bool {
    let oit = params.transparency_method != Graphic3dRenderTransparentMethod::BlendUnordered;
    params.nb_msaa_samples == 0 && (oit || draw_to_front)
}

impl std::ops::Deref for OutputModeRetainedView {
    type Target = Graphic3dRetainedView;
    fn deref(&self) -> &Graphic3dRetainedView {
        &self.shared
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params(msaa: i32, oit: bool) -> Graphic3dRenderingParams {
        let mut params = Graphic3dRenderingParams::new().unwrap();
        params.nb_msaa_samples = msaa;
        if oit {
            params.transparency_method = Graphic3dRenderTransparentMethod::BlendOit;
        }
        params
    }

    #[test]
    fn flip_rule_matches_the_native_window_table() {
        // Native window rows (msaa, oit, draw_to_front) -> rows reversed.
        let native = [
            ((0, false, true), true),
            ((0, false, false), false),
            ((0, true, true), true),
            ((0, true, false), true),
            ((4, false, true), false),
            ((4, false, false), false),
            ((4, true, true), false),
            ((4, true, false), false),
        ];
        for ((msaa, oit, front), expected) in native {
            assert_eq!(
                flip_reaches_target(front, &params(msaa, oit)),
                expected,
                "{msaa} {oit} {front}"
            );
        }
    }
}
