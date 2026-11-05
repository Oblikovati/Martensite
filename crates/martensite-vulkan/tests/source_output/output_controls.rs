//! Pure output-obligation controls do not pretend a placeholder path is a rendered image.
use super::*;

#[test]
fn first_missing_dump_is_not_lost_when_the_second_succeeded() {
    let additional = vec![(Some(PathBuf::from("retained-second.png")), "second.png")];
    let error = require_source_captures(&None, "first.png", &additional).unwrap_err();
    assert!(error.to_string().contains("first.png"));
}

#[test]
fn second_missing_dump_is_not_lost_when_the_first_succeeded() {
    let first = Some(PathBuf::from("retained-first.png"));
    let error = require_source_captures(&first, "first.png", &[(None, "second.png")]).unwrap_err();
    assert!(error.to_string().contains("second.png"));
}

#[test]
fn unchanged_single_capture_and_complete_two_capture_obligations_pass() {
    let first = Some(PathBuf::from("retained-first.png"));
    let additional = vec![(Some(PathBuf::from("retained-second.png")), "second.png")];
    assert!(require_source_captures(&first, "first.png", &[]).is_ok());
    assert!(require_source_captures(&first, "first.png", &additional).is_ok());
}

/// A deliberate dump refusal is invocation-local and performs no external I/O.
struct RefusingSourceOutput {
    view: Handle<dyn martensite_tkv3d::v3d::V3dViewSource>,
}
impl SourceViewOutput for RefusingSourceOutput {
    fn source_view_handle(&self) -> &Handle<dyn martensite_tkv3d::v3d::V3dViewSource> {
        // The refusal override never dereferences this absent view.
        &self.view
    }
    fn source_window_size(&self) -> (i32, i32) {
        // No live window is queried when this named fake refuses the dump.
        (409, 409)
    }
    fn source_dump(
        &self,
        _: &martensite_tkservice::image::ImagePixMap,
    ) -> Result<bool, martensite_tkernel::standard::Failure> {
        let refused = false;
        Ok(refused)
    }
}

#[test]
fn additional_dump_refusal_is_retained_until_completion_qualification() {
    let scene = RefusingSourceOutput {
        view: Handle::default(),
    };
    let mut completion = SourceCompletion {
        driver: Handle::default(),
        scene,
        screenshot: Some(PathBuf::from("first.png")),
        basename: "first.png",
        additional: Vec::new(),
    };
    completion.capture_additional("second.png").unwrap();
    assert_eq!(completion.additional, vec![(None, "second.png")]);
    require_missing_additional(&completion);
}

fn require_missing_additional(completion: &SourceCompletion<RefusingSourceOutput>) {
    let result = require_source_captures(
        &completion.screenshot,
        completion.basename,
        &completion.additional,
    );
    assert!(result.is_err());
}
