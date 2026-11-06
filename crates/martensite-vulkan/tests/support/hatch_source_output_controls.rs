//! Owned empty-view evidence exercises the interface without claiming an reference case.
use super::{
    device_options,
    scene::{HatchSourceView, SourceViewOutput},
};
use martensite_tkernel::standard::{Failure, exceptions::program_error};
use martensite_tkservice::image::{ImageFormat, ImagePixMap};
use martensite_vulkan::Graphic3dRenderDriver;
use std::{cell::Cell, error::Error};

#[test]
fn initialized_view_without_geometry_dumps_black_and_restores_updates() -> Result<(), Box<dyn Error>>
{
    let driver = Graphic3dRenderDriver::new(device_options::validation_options())?;
    let initialized = HatchSourceView::new(&driver)?;
    require_update_restoration(&initialized, false);
    require_update_restoration(&initialized, true);
    let image = ImagePixMap::default();
    assert!(
        initialized.source_dump(&image)?,
        "actual dump=false; expected initialized empty-view output"
    );
    require_black_view_image(&image);
    initialized.close();
    let diagnostics = driver.validation_errors().unwrap();
    assert!(
        diagnostics.is_empty(),
        "diagnostics={diagnostics:?}; expected no owned control validation errors"
    );
    Ok(())
}

fn require_update_restoration(view: &HatchSourceView, reject: bool) {
    let (result, observed) = observe_source_update(view, reject);
    assert_eq!(
        result.is_err(),
        reject,
        "reject={reject}; expected matching update result"
    );
    assert_eq!(
        observed,
        Some(false),
        "actual={:?}; expected disabled immediate update in command",
        observed
    );
    let restored = view.source_view_handle().set_immediate_update(true);
    assert!(
        restored,
        "restored={restored}; expected original true immediate update after command"
    );
}

fn require_black_view_image(image: &ImagePixMap) {
    assert_eq!(image.format(), ImageFormat::Rgb);
    assert_eq!([image.width(), image.height()], [409, 409]);
    let row_bytes = image.width() * image.size_pixel_bytes();
    let storage = image.data();
    for row in 0..image.height() {
        let actual = &storage.data.row(row).unwrap()[..row_bytes];
        assert!(
            actual.iter().all(|value| *value == 0),
            "row={row}, pixels={actual:?}; expected exact owned black background"
        );
    }
}

fn observe_source_update(
    view: &HatchSourceView,
    reject: bool,
) -> (Result<(), Failure>, Option<bool>) {
    let observed: Cell<Option<bool>> = Cell::new(None);
    let result = view.source_update(|actual| {
        observed.set(Some(actual.set_immediate_update(false)));
        if reject {
            return Err(program_error(
                "reject=true; expected controlled owned update refusal",
            ));
        }
        Ok(())
    });
    (result, observed.get())
}
