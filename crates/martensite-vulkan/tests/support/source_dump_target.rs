//! One borrowed view boundary makes dump-state restoration observable without a GPU.
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkservice::image::ImagePixMap;
use martensite_tkv3d::v3d::{V3dImageDumpOptions, V3dStereoDumpOptions, V3dViewSource};

pub(super) trait SourceDumpTarget {
    fn set_immediate_update(&self, enabled: bool) -> bool;
    fn dump(&self, image: &ImagePixMap, options: &V3dImageDumpOptions) -> Result<bool, Failure>;
}

pub(super) struct BorrowedDumpView<'a> {
    pub view: &'a Handle<dyn V3dViewSource>,
}

impl SourceDumpTarget for BorrowedDumpView<'_> {
    fn set_immediate_update(&self, enabled: bool) -> bool {
        self.view.set_immediate_update(enabled)
    }

    fn dump(&self, image: &ImagePixMap, options: &V3dImageDumpOptions) -> Result<bool, Failure> {
        self.view.to_pix_map(image, options)
    }
}

/// Execute one explicit dump request; e.g. `perform_source_dump(&target, (409, 409), Mono, &image)`.
pub(super) fn perform_source_dump(
    target: &impl SourceDumpTarget,
    (width, height): (i32, i32),
    stereo: V3dStereoDumpOptions,
    image: &ImagePixMap,
) -> Result<bool, Failure> {
    let options = V3dImageDumpOptions {
        width,
        height,
        stereo_options: stereo,
        ..Default::default()
    };
    perform_source_dump_options(target, &options, image)
}

/// Pass every explicit option to the actual target; `perform_source_dump_options(&target, &options, &image)?`.
pub(super) fn perform_source_dump_options(
    target: &impl SourceDumpTarget,
    options: &V3dImageDumpOptions,
    image: &ImagePixMap,
) -> Result<bool, Failure> {
    let previous = target.set_immediate_update(false);
    let result = target.dump(image, options);
    target.set_immediate_update(previous);
    result
}

#[cfg(test)]
#[path = "source_dump_contract.rs"]
mod source_dump_contract;

#[cfg(test)]
#[path = "source_dump_options_contract.rs"]
mod source_dump_options_contract;
