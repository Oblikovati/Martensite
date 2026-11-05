//! The short-line guard preserves actual handles without claiming numerical splitting coverage.
use crate::blend::BlendPoint;
use crate::brep_blend::BRepBlendLine;
use crate::chfi_ds::{SurfData, lock, shared};
use crate::chfi3d::protected_guard_fixture::protected_guard_builder_face;
use martensite_tkmath::gp::Point3;
use std::sync::Arc;

fn protected_short_line(count: i32) -> BRepBlendLine {
    let mut line = BRepBlendLine::new();
    for index in 0..count {
        let x = f64::from(index);
        let point = Point3::new(x, 0., 0.);
        line.append(BlendPoint::on_surfaces(point, point, x, [x, 0., x, 0.]));
    }
    line
}

fn protected_split_short_line(count: i32) {
    let (mut builder, _) = protected_guard_builder_face();
    let line = shared(protected_short_line(count));
    let original = shared(SurfData::new());
    let mut sequence = vec![original.clone()];
    builder.split_surf(&mut sequence, &line).unwrap();
    let sequence_count = sequence.len();
    let handle_preserved = Arc::ptr_eq(&sequence[0], &original);
    let line_points = lock(&line).nb_points();
    assert_eq!(sequence_count, 1);
    assert!(
        handle_preserved,
        "SplitSurf handle_preserved={handle_preserved}; expected true"
    );
    assert_eq!(line_points, count);
    println!(
        "{{\"case\":\"split_surf_short_line\",\"points\":{count},\"sequence_count\":{sequence_count},\"handle_preserved\":{handle_preserved},\"line_points\":{line_points}}}"
    );
}

#[test]
fn protected_guard_split_short_line_preserves_sequence() {
    for count in [0, 2] {
        protected_split_short_line(count);
    }
}
