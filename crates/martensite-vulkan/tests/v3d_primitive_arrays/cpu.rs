use super::*;
use crate::case_view::{SourceViewOutput, viewer_session::ViewerSession};
#[test]
fn all_twenty_four_array_topologies_keep_both_layouts() {
    assert_eq!(specification::specifications().count(), 24);
    for non_interleaved in [false, true] {
        for spec in specification::specifications() {
            let array = arrays::build(spec, non_interleaved).unwrap();
            assert_eq!(array.vertex_number() as usize, spec.vertices.len());
            assert_eq!(array.bound_number(), native_buffer_count(spec.bounds.len()));
            assert_eq!(array.edge_number(), native_buffer_count(spec.edges.len()));
            assert_eq!(array.vertex_color_rgb(1).unwrap(), spec.color);
            assert_eq!(array.vertex_normal_xyz(1).unwrap(), [0., 0., -1.]);
        }
    }
}
#[test]
fn every_buffer_layout_stage_displays_all_real_array_objects() {
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let mut session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    for (enabled, non_interleaved) in [(false, false), (false, true), (true, false), (true, true)] {
        scene::stage(&mut session, enabled, non_interleaved).unwrap();
        for spec in specification::specifications() {
            assert!(session.object(spec.name).is_some());
        }
        assert!(session.object("point1").is_some());
        assert!(session.object("point2").is_some());
    }
    session.close();
}

fn native_buffer_count(length: usize) -> i32 {
    if length == 0 {
        return -1;
    }
    length as i32
}
