//! Caller-owned image dump options read at each native `ToPixMap` read point.
//!
//! Native `ToPixMap` holds a const reference to the caller's options, but a virtual callback
//! (`Redraw`, a view or driver override) may legally edit the same aggregate through another
//! alias. The later reads (BufferType and LightName after Redraw) then see the edit. A source
//! re-reads the options at every such point; plain options never change during the call.
use super::V3dImageDumpOptions;

/// Options that `V3dView::to_pix_map` reads at the native read points.
///
/// Example: `view.to_pix_map(&image, &options)?` for plain options; an application whose
/// callbacks edit the options implements this trait over its shared storage.
pub trait V3dImageDumpOptionsSource<'anchor> {
    /// The options as they are now; `let buffer = source.current_options().buffer_type;`.
    fn current_options(&self) -> V3dImageDumpOptions<'anchor>;
}

impl<'anchor> V3dImageDumpOptionsSource<'anchor> for V3dImageDumpOptions<'anchor> {
    /// Plain options are borrowed immutably, so every read sees the same values.
    fn current_options(&self) -> V3dImageDumpOptions<'anchor> {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkservice::graphic3d::Graphic3dBufferType;

    #[test]
    fn plain_options_are_their_own_source() {
        let options = V3dImageDumpOptions {
            width: 3,
            buffer_type: Graphic3dBufferType::Depth,
            ..Default::default()
        };
        assert_eq!(options.current_options(), options);
    }
}
