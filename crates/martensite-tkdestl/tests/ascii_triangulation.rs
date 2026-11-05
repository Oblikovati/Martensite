//! Independent text layout and synchronous stream lifecycle contracts.
use crate::writer_support::{CancelledProgress, HeaderCountMutation, RecordingStream, planar_mesh};
use martensite_tkdestl::rw_stl::RwStl;
use martensite_tkernel::{
    message::{ProgressIndicator, ProgressRange},
    standard::Handle,
};
use martensite_tkmath::{
    gp::Point3,
    poly::{Triangle, Triangulation},
};
const PLANAR_FACET: &str = concat!(
    " facet normal 0.000000000000e+00 0.000000000000e+00 1.000000000000e+00\n",
    "   outer loop\n",
    "     vertex 0.000000000000e+00 0.000000000000e+00 0.000000000000e+00\n",
    "     vertex 2.000000000000e+00 0.000000000000e+00 0.000000000000e+00\n",
    "     vertex 0.000000000000e+00 3.000000000000e+00 0.000000000000e+00\n",
    "   endloop\n endfacet\n",
);
#[test]
fn planar_ascii_bytes_preserve_whitespace_precision_and_exponent_sign() {
    let mut stream = RecordingStream::default();
    assert!(RwStl::write_ascii(&planar_mesh(1), &mut stream, &ProgressRange::default()).unwrap());
    assert_eq!(
        stream.bytes,
        format!("solid \n{PLANAR_FACET}endsolid\n").as_bytes()
    );
    assert_eq!(stream.lengths.len(), 33);
}
#[test]
fn every_ascii_insertion_failure_or_short_write_stops_without_retry_or_footer() {
    let mut baseline = RecordingStream::default();
    assert!(RwStl::write_ascii(&planar_mesh(1), &mut baseline, &ProgressRange::default()).unwrap());
    for call in 1..=33 {
        for short in [false, true] {
            let mut stream = RecordingStream {
                fail_call: (!short).then_some(call),
                short_call: short.then_some(call),
                ..Default::default()
            };
            assert!(
                !RwStl::write_ascii(&planar_mesh(1), &mut stream, &ProgressRange::default())
                    .unwrap()
            );
            assert_eq!(stream.lengths.len(), call);
            let completed: usize = baseline.lengths[..call - 1].iter().sum();
            let length = completed
                + if short {
                    baseline.lengths[call - 1] - 1
                } else {
                    0
                };
            assert_eq!(stream.bytes, &baseline.bytes[..length]);
        }
    }
}
#[test]
fn ascii_cancellation_follows_the_thousandth_published_facet() {
    for (count, success) in [(999, true), (1000, false)] {
        let indicator = ProgressIndicator::new(Box::new(CancelledProgress));
        let mut bytes = Vec::new();
        assert_eq!(
            RwStl::write_ascii(&planar_mesh(count), &mut bytes, &indicator.start()).unwrap(),
            success
        );
        let expected = format!(
            "solid \n{}{}",
            PLANAR_FACET.repeat(usize::try_from(count).unwrap()),
            if success { "endsolid\n" } else { "" }
        );
        assert_eq!(bytes, expected.as_bytes());
    }
}
#[test]
fn ascii_count_is_captured_after_header_output_and_then_retained() {
    let mesh = planar_mesh(1);
    let mut stream = HeaderCountMutation::new(mesh.clone(), 2);
    assert!(RwStl::write_ascii(&mesh, &mut stream, &ProgressRange::default()).unwrap());
    assert_eq!(
        stream.bytes,
        format!("solid \n{PLANAR_FACET}{PLANAR_FACET}endsolid\n").as_bytes()
    );
}
#[test]
fn null_empty_and_invalid_geometry_have_distinct_write_boundaries() {
    for mesh in [Handle::default(), Handle::new(Triangulation::new())] {
        let mut stream = RecordingStream::default();
        assert!(!RwStl::write_ascii(&mesh, &mut stream, &ProgressRange::default()).unwrap());
        assert!(stream.lengths.is_empty());
    }
    let mesh = planar_mesh(1);
    mesh.get()
        .unwrap()
        .set_triangle(1, Triangle::new(1, 2, 7))
        .unwrap();
    let mut stream = RecordingStream::default();
    assert!(RwStl::write_ascii(&mesh, &mut stream, &ProgressRange::default()).is_err());
    assert_eq!(stream.bytes, b"solid \n");
}
#[test]
fn reversed_and_degenerate_ascii_normals_follow_shared_geometry() {
    let mesh = planar_mesh(2);
    let object = mesh.get().unwrap();
    object.set_triangle(1, Triangle::new(1, 3, 2)).unwrap();
    object.set_triangle(2, Triangle::new(1, 1, 1)).unwrap();
    let mut bytes = Vec::new();
    assert!(RwStl::write_ascii(&mesh, &mut bytes, &ProgressRange::default()).unwrap());
    let text = String::from_utf8(bytes).unwrap();
    assert!(
        text.contains(" facet normal 0.000000000000e+00 0.000000000000e+00 -1.000000000000e+00\n")
    );
    assert!(
        text.contains(" facet normal 0.000000000000e+00 0.000000000000e+00 0.000000000000e+00\n")
    );
}
#[test]
fn signed_zero_and_fractional_large_small_coordinates_keep_scientific_layout() {
    let mesh = planar_mesh(1);
    let object = mesh.get().unwrap();
    object.set_triangle(1, Triangle::new(1, 1, 1)).unwrap();
    for (point, line) in [
        (
            Point3::new(-0., 1. / 3., 1e-100),
            "     vertex -0.000000000000e+00 3.333333333333e-01 1.000000000000e-100\n",
        ),
        (
            Point3::new(1e200, f64::INFINITY, f64::from_bits(0xfff8_0000_0000_0001)),
            "     vertex 1.000000000000e+200 inf -nan\n",
        ),
    ] {
        object.set_node(1, point).unwrap();
        let mut bytes = Vec::new();
        assert!(RwStl::write_ascii(&mesh, &mut bytes, &ProgressRange::default()).unwrap());
        assert_eq!(String::from_utf8(bytes).unwrap().matches(line).count(), 3);
    }
}
