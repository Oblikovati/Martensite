//! Private-method behavior does not establish public subclass accessibility.
use crate::chfi_ds::CommonPoint;
use crate::chfi3d::protected_guard_fixture::protected_guard_builder_face;

#[test]
fn protected_guard_search_face_no_arc_preserves_output() {
    let (builder, sentinel) = protected_guard_builder_face();
    let point = CommonPoint::new();
    let mut output = sentinel.clone();
    let found = builder
        .builder_base()
        .search_face(None, &point, &sentinel, &mut output)
        .unwrap();
    let preserved = output.as_ref().is_equal(sentinel.as_ref());
    assert_eq!((found, preserved), (false, true));
    let done = builder.builder_base().is_done();
    let has_result = builder.builder_base().has_result();
    assert_eq!((done, has_result), (false, false));
    println!(
        "{{\"case\":\"search_face_no_arc\",\"found\":{found},\"output_preserved\":{preserved},\"done\":{done},\"has_result\":{has_result}}}"
    );
}
