//! Source image dumping and command updates use the same initialized view, with or without geometry.
use super::*;
use martensite_tkernel::standard::Failure;
use martensite_tkservice::image::ImagePixMap;

/// Borrow invocation-local UI resources until source completion; `capture_case(driver, view, name)?`.
/// Implementors stay on their owning worker thread; this is not a published modelling result.
pub trait SourceViewOutput {
    /// Borrow the actual retained view; `output.source_view_handle()`.
    fn source_view_handle(&self) -> &Handle<dyn V3dViewSource>;
    /// Read the actual window extent; `output.source_window_size()`.
    fn source_window_size(&self) -> (i32, i32);

    /// Use the original RGB/mono default layer range; `output.source_dump(&image)?`.
    fn source_dump(&self, image: &ImagePixMap) -> Result<bool, Failure> {
        dump_view(
            self.source_view_handle(),
            self.source_window_size(),
            V3dStereoDumpOptions::Mono,
            image,
        )
    }

    /// Borrow all explicit original capture options; `output.source_dump_options(&image, &options)?`.
    fn source_dump_options(
        &self,
        image: &ImagePixMap,
        options: &martensite_tkv3d::v3d::V3dImageDumpOptions,
    ) -> Result<bool, Failure> {
        let target = dump_target::BorrowedDumpView {
            view: self.source_view_handle(),
        };
        dump_target::perform_source_dump_options(&target, options, image)
    }

    /// Retire only after source END; `output.close()`.
    fn close(&self) {
        // Completion belongs to the caller; dumping must not hide earlier view retirement.
        self.source_view_handle().remove();
    }
}

#[path = "source_dump_target.rs"]
mod dump_target;

/// Dump the actual borrowed view with explicit stereo projection; `dump_view(view, size, Mono, image)?`.
pub fn dump_view(
    view: &Handle<dyn V3dViewSource>,
    size: (i32, i32),
    stereo: V3dStereoDumpOptions,
    image: &ImagePixMap,
) -> Result<bool, Failure> {
    let target = dump_target::BorrowedDumpView { view };
    dump_target::perform_source_dump(&target, size, stereo, image)
}

impl SourceViewOutput for HatchBoxScene {
    fn source_view_handle(&self) -> &Handle<dyn V3dViewSource> {
        // Borrowing keeps the original view owner and dynamic type intact.
        &self.view
    }

    fn source_window_size(&self) -> (i32, i32) {
        // The actual window remains authoritative after any source resize.
        self._window.size()
    }
}

/// Restore the source update state before its actual updater, including a returned Failure.
/// Example: `with_source_view_update(view, || Ok(()), || view.redraw())?`.
pub(super) fn with_source_view_update(
    view: &dyn V3dViewSource,
    operation: impl FnOnce() -> Result<(), Failure>,
    update: impl FnOnce(),
) -> Result<(), Failure> {
    let previous = view.set_immediate_update(false);
    let result = operation();
    view.set_immediate_update(previous);
    if previous {
        update();
    }
    result
}
