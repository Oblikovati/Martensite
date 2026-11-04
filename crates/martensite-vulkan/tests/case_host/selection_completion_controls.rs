//! Selection images do not publish a scene capture; completion still retires and validates.
use super::{SessionRun, Stage};
use crate::case_view::viewer_session::ViewerSession;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkservice::graphic3d::{Graphic3dCView, Graphic3dCViewMembers};
use martensite_tkservice::image::{ImageAlienPixMap, ImageFormat};
use martensite_tkv3d::std_select::StdSelectTypeOfSelectionImage as SelectionImage;
use martensite_vulkan::Graphic3dRenderDriver;
#[path = "../support/recording_device.rs"]
mod recording_device;

const OUTPUTS: [(&str, SelectionImage); 4] = [
    (
        "selection_completion_ortho_depth.png",
        SelectionImage::NORMALIZED_DEPTH,
    ),
    (
        "selection_completion_ortho_entity.png",
        SelectionImage::COLORED_ENTITY,
    ),
    (
        "selection_completion_persp_depth.png",
        SelectionImage::NORMALIZED_DEPTH,
    ),
    (
        "selection_completion_persp_entity.png",
        SelectionImage::COLORED_ENTITY,
    ),
];

fn recorded_selection_run() -> (SessionRun, Vec<Handle<dyn Graphic3dCView>>) {
    let device = recording_device::RecordingDevice::default();
    let driver = Graphic3dRenderDriver::with_backend(device, 4096).unwrap();
    let mut session = ViewerSession::with_view_size(&driver, 16, 16).unwrap();
    let first = session.view().get().unwrap().view().clone();
    session.open_view(16, 16).unwrap();
    let second = session.view().get().unwrap().view().clone();
    let run = SessionRun {
        #[cfg(feature = "vertex-output-diagnostics")]
        vertex_output: None,
        stage: Some(Stage::Open(driver, session)),
        pending_witness_error: None,
        observed_dump_ordinal: 0,
    };
    (run, vec![first, second])
}

fn write_selection_images(run: &mut SessionRun) {
    for (name, kind) in OUTPUTS {
        run.selection_dump(name, kind).unwrap();
        let path = crate::source_output::capture_path(name).unwrap();
        assert!(path.metadata().unwrap().len() > 0);
    }
    assert!(matches!(run.stage, Some(Stage::Open(..))));
}

fn assert_validation_unavailable(failure: Box<dyn std::any::Any + Send>) {
    let message = failure
        .downcast_ref::<String>()
        .expect("validation panic text");
    assert!(
        message.contains("RenderValidationObservationFailure"),
        "{message}"
    );
    assert!(message.contains("Unavailable"), "{message}");
}

#[test]
fn selection_only_completion_retires_views_before_refusing_unavailable_validation() {
    let (mut run, views) = recorded_selection_run();
    write_selection_images(&mut run);
    assert!(views.iter().all(|view| !view.get().unwrap().is_removed()));
    let result =
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run.finish_without_image()));
    assert_validation_unavailable(result.expect_err("injected backend has no GPU validation"));
    assert!(views.iter().all(|view| view.get().unwrap().is_removed()));
    for (name, _) in OUTPUTS {
        std::fs::remove_file(crate::source_output::capture_path(name).unwrap()).unwrap();
    }
}

/// The pixel format of the PNG that `dump` writes, read back through the image file loader.
fn dumped_format(name: &str, dump: impl FnOnce(&mut SessionRun)) -> ImageFormat {
    let (mut run, _) = recorded_selection_run();
    dump(&mut run);
    let path = crate::source_output::capture_path(name).unwrap();
    let loaded = ImageAlienPixMap::default();
    let path_text = AsciiString::from(path.to_string_lossy().as_ref());
    assert!(loaded.load(&path_text), "{}", path.display());
    std::fs::remove_file(path).unwrap();
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run.finish_without_image()));
    loaded.format()
}

/// An untyped selection dump stores depth through BGR; a typed depth dump through float gray.
#[test]
fn untyped_depth_dump_keeps_the_command_default_bgr_image() {
    let untyped = "selection_format_untyped.png";
    let format = dumped_format(untyped, |run| run.untyped_selection_dump(untyped).unwrap());
    assert_eq!(format, ImageFormat::Bgr);
    let typed = "selection_format_typed.png";
    let format = dumped_format(typed, |run| {
        run.selection_dump(typed, SelectionImage::NORMALIZED_DEPTH)
            .unwrap();
    });
    assert_eq!(format, ImageFormat::Gray);
}
