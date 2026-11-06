use super::*;
use std::cell::{Cell, RefCell};

struct FakeDumpTarget {
    immediate: Cell<bool>,
    outcome: Option<bool>,
    requests: RefCell<Vec<(i32, i32, V3dStereoDumpOptions)>>,
}

impl FakeDumpTarget {
    fn new(immediate: bool, outcome: Option<bool>) -> Self {
        Self {
            immediate: Cell::new(immediate),
            outcome,
            requests: RefCell::new(Vec::new()),
        }
    }
}

impl SourceDumpTarget for FakeDumpTarget {
    fn set_immediate_update(&self, enabled: bool) -> bool {
        self.immediate.replace(enabled)
    }

    fn dump(&self, _: &ImagePixMap, options: &V3dImageDumpOptions) -> Result<bool, Failure> {
        assert!(
            !self.immediate.get(),
            "dump must run with immediate update disabled"
        );
        self.requests
            .borrow_mut()
            .push((options.width, options.height, options.stereo_options));
        self.outcome
            .ok_or_else(|| Failure::new("owned dump target failure"))
    }
}

#[test]
fn source_dump_completed_image_restores_its_initial_update_state() {
    for initial in [false, true] {
        let target = FakeDumpTarget::new(initial, Some(true));
        let result = perform_source_dump(
            &target,
            (37, 19),
            V3dStereoDumpOptions::Mono,
            &ImagePixMap::default(),
        );
        assert!(result.unwrap());
        assert_eq!(target.immediate.get(), initial);
    }
}

#[test]
fn source_dump_false_result_restores_its_initial_update_state() {
    for initial in [false, true] {
        let target = FakeDumpTarget::new(initial, Some(false));
        let result = perform_source_dump(
            &target,
            (37, 19),
            V3dStereoDumpOptions::Mono,
            &ImagePixMap::default(),
        );
        assert!(!result.unwrap());
        assert_eq!(target.immediate.get(), initial);
    }
}

#[test]
fn source_dump_returned_error_restores_its_initial_update_state() {
    for initial in [false, true] {
        let target = FakeDumpTarget::new(initial, None);
        let error = perform_source_dump(
            &target,
            (37, 19),
            V3dStereoDumpOptions::Mono,
            &ImagePixMap::default(),
        )
        .err()
        .unwrap();
        assert_eq!(
            error.to_string(),
            "Standard_Failure: owned dump target failure"
        );
        assert_eq!(target.immediate.get(), initial);
    }
}

#[test]
fn source_dump_request_preserves_extent_and_explicit_stereo_projection() {
    for stereo in [
        V3dStereoDumpOptions::Mono,
        V3dStereoDumpOptions::Blended,
        V3dStereoDumpOptions::LeftEye,
        V3dStereoDumpOptions::RightEye,
    ] {
        let target = FakeDumpTarget::new(true, Some(true));
        perform_source_dump(&target, (37, 19), stereo, &ImagePixMap::default()).unwrap();
        assert_eq!(*target.requests.borrow(), [(37, 19, stereo)]);
    }
}
