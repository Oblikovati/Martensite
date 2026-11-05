//! The original mandatory file artifact is produced by the actual view, then consumed.
use super::support::document_viewer::ViewerLifecycle;
use flate2::read::GzDecoder;
use std::io::Read;
#[path = "../support/document_view_capture.rs"]
mod document_view_capture;

pub fn assert_case_saved_view(lifecycle: &ViewerLifecycle, case: &str) {
    assert_exact_native_file(&document_view_capture::read_case_saved_view(
        &lifecycle.current_view(),
        case,
    ));
}

fn assert_exact_native_file(bytes: &[u8]) {
    let mut expected = Vec::new();
    GzDecoder::new(
        &include_bytes!("../fixtures/empty_compound_views/native-default-view.ppm.gz")[..],
    )
    .read_to_end(&mut expected)
    .unwrap();
    assert_eq!(bytes.len(), 501858);
    assert_eq!(&bytes[..15], b"P6\n409 409\n255\n");
    assert!(bytes[15..].iter().all(|byte| *byte == 0));
    let first_difference = bytes
        .iter()
        .zip(&expected)
        .position(|(actual, reference)| actual != reference);
    assert_eq!(bytes.len(), expected.len());
    assert_eq!(first_difference, None);
}
