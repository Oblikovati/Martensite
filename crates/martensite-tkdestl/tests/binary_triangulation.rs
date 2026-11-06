//! Independent binary triangulation serialization and stream lifecycle contracts.
use crate::writer_support::{CancelledProgress, RecordingStream, expected_facet, planar_mesh};
use martensite_tkdestl::rw_stl::RwStl;
use martensite_tkernel::{
    message::{ProgressIndicator, ProgressRange},
    standard::Handle,
};
use martensite_tkmath::{
    gp::Point3,
    poly::{Triangle, Triangulation},
};

#[test]
fn right_triangle_binary_layout_has_independently_known_normal_and_vertices() {
    let mut stream = RecordingStream::default();
    assert!(RwStl::write_binary(&planar_mesh(1), &mut stream, &ProgressRange::default()).unwrap());
    assert_eq!(stream.lengths, [80, 4, 50]);
    assert_eq!(&stream.bytes[..26], b"STL Exported by Martensite");
    assert_eq!(&stream.bytes[26..80], &[0; 54]);
    assert_eq!(&stream.bytes[80..84], &1i32.to_le_bytes());
    assert_eq!(
        stream.bytes[84..],
        expected_facet([0., 0., 1.], [0., 0., 0., 2., 0., 0., 0., 3., 0.])
    );
}
#[test]
fn winding_and_degenerate_facets_have_geometric_normals() {
    let mesh = planar_mesh(2);
    let object = mesh.get().unwrap();
    object.set_triangle(1, Triangle::new(1, 3, 2)).unwrap();
    object.set_triangle(2, Triangle::new(1, 1, 1)).unwrap();
    let mut bytes = Vec::new();
    assert!(RwStl::write_binary(&mesh, &mut bytes, &ProgressRange::default()).unwrap());
    assert_eq!(
        &bytes[84..96],
        &[0f32, 0f32, -1f32]
            .into_iter()
            .flat_map(f32::to_le_bytes)
            .collect::<Vec<_>>()
    );
    assert_eq!(&bytes[134..184], expected_facet([0.; 3], [0.; 9]));
}
#[test]
fn null_and_empty_meshes_leave_stream_untouched() {
    for mesh in [Handle::default(), Handle::new(Triangulation::new())] {
        let mut stream = RecordingStream::default();
        assert!(!RwStl::write_binary(&mesh, &mut stream, &ProgressRange::default()).unwrap());
        assert!(stream.lengths.is_empty());
    }
}
#[test]
fn all_stream_failure_and_short_write_boundaries_stop_without_retry() {
    for call in 1..=3 {
        for short in [false, true] {
            let mut stream = RecordingStream {
                fail_call: (!short).then_some(call),
                short_call: short.then_some(call),
                ..Default::default()
            };
            assert!(
                !RwStl::write_binary(&planar_mesh(1), &mut stream, &ProgressRange::default())
                    .unwrap()
            );
            assert_eq!(stream.lengths.len(), call);
        }
    }
}
#[test]
fn chunk_boundaries_preserve_zero_length_final_write() {
    for (count, lengths) in [
        (4096, vec![80, 4, 204800, 0]),
        (4097, vec![80, 4, 204800, 50]),
    ] {
        let mut stream = RecordingStream::default();
        assert!(
            RwStl::write_binary(&planar_mesh(count), &mut stream, &ProgressRange::default())
                .unwrap()
        );
        assert_eq!(stream.lengths, lengths);
        assert_eq!(
            stream.bytes.len(),
            84 + usize::try_from(count).unwrap() * 50
        );
    }
}
#[test]
fn cancellation_is_polled_only_at_thousand_triangle_boundaries() {
    for (count, success) in [(999, true), (1000, false)] {
        let indicator = ProgressIndicator::new(Box::new(CancelledProgress));
        let mut stream = RecordingStream::default();
        assert_eq!(
            RwStl::write_binary(&planar_mesh(count), &mut stream, &indicator.start()).unwrap(),
            success
        );
        assert_eq!(stream.bytes.len(), if success { 84 + 999 * 50 } else { 84 });
    }
}
#[test]
fn invalid_mesh_index_retains_geometry_failure_after_header() {
    let mesh = planar_mesh(1);
    mesh.get()
        .unwrap()
        .set_triangle(1, Triangle::new(1, 2, 7))
        .unwrap();
    let mut stream = RecordingStream::default();
    assert!(RwStl::write_binary(&mesh, &mut stream, &ProgressRange::default()).is_err());
    assert_eq!(stream.lengths, [80, 4]);
}
#[test]
fn float32_rounding_and_squared_resolution_threshold_are_preserved() {
    let mesh = planar_mesh(1);
    let object = mesh.get().unwrap();
    object
        .set_node(
            2,
            Point3::new(1. + martensite_cmath::powi(2f64, -24), 0., 0.),
        )
        .unwrap();
    object.set_node(3, Point3::new(0., 1., 0.)).unwrap();
    let mut bytes = Vec::new();
    assert!(RwStl::write_binary(&mesh, &mut bytes, &ProgressRange::default()).unwrap());
    assert_eq!(&bytes[108..112], &1f32.to_le_bytes());
    object.set_node(2, Point3::new(1e-80, 0., 0.)).unwrap();
    object.set_node(3, Point3::new(0., 1e-80, 0.)).unwrap();
    bytes.clear();
    assert!(RwStl::write_binary(&mesh, &mut bytes, &ProgressRange::default()).unwrap());
    assert_eq!(&bytes[84..96], &[0; 12]);
}
#[test]
fn chunk_and_empty_final_stream_errors_remain_failures() {
    for call in [3, 4] {
        let mut stream = RecordingStream {
            fail_call: Some(call),
            ..Default::default()
        };
        assert!(
            !RwStl::write_binary(&planar_mesh(4096), &mut stream, &ProgressRange::default())
                .unwrap()
        );
        assert_eq!(stream.lengths.len(), call);
    }
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<RwStl>();
}
#[test]
fn binary_triangle_count_is_captured_after_the_header_stream_call() {
    let mesh = planar_mesh(1);
    let mut stream = crate::writer_support::HeaderCountMutation::new(mesh.clone(), 2);
    assert!(RwStl::write_binary(&mesh, &mut stream, &ProgressRange::default()).unwrap());
    assert_eq!(&stream.bytes[80..84], &2i32.to_le_bytes());
    assert_eq!(stream.bytes.len(), 184);
    assert_eq!(&stream.bytes[84..134], &stream.bytes[134..184]);
}
