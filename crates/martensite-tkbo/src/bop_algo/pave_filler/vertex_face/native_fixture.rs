//! Exact inputs/order of the frozen 4f10 Native producer, using existing Rust constructors.
use super::{
    BOPAlgoPaveFiller, Glue, configured_planar_vf_fixture, planar_vertex_face_fixture,
    vf_clear_source_surface, vf_owned_bounded_face, vf_source_face_index,
};
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeEdge;

/// Reconstructs the seven actual uncancelled Native profile/configuration cells.
/// Example: `let fixtures = uncancelled_vf_fixtures();`.
pub(super) fn uncancelled_vf_fixtures() -> [(&'static str, BOPAlgoPaveFiller); 7] {
    [
        ("one", planar_vertex_face_fixture(1, false)),
        ("three", planar_vertex_face_fixture(3, false)),
        ("glue-full", native_full_glue_fixture()),
        ("empty", planar_vertex_face_fixture(0, false)),
        ("ee-tail", native_crossing_ee_fixture()),
        ("task-failure", native_task_failure_fixture()),
        ("three", planar_vertex_face_fixture(3, true)),
    ]
}

fn native_full_glue_fixture() -> BOPAlgoPaveFiller {
    let glue = Glue::Full;
    configured_planar_vf_fixture(1, false, glue)
}

fn native_task_failure_fixture() -> BOPAlgoPaveFiller {
    let filler = planar_vertex_face_fixture(1, false);
    let index = vf_source_face_index(&filler);
    vf_clear_source_surface(&filler, index);
    filler
}

fn native_crossing_ee_fixture() -> BOPAlgoPaveFiller {
    let mut filler = BOPAlgoPaveFiller::new();
    native_crossing_edges(&mut filler);
    filler.add_argument(&vf_owned_bounded_face().into_shape());
    filler.options_mut().set_run_parallel(false);
    filler.initialize().unwrap();
    filler.perform_ee().unwrap();
    filler
}

fn native_crossing_edges(filler: &mut BOPAlgoPaveFiller) {
    for (first, last) in [
        (Point3::new(-0.5, 0., 0.), Point3::new(0.5, 0., 0.)),
        (Point3::new(0., -0.5, 0.), Point3::new(0., 0.5, 0.)),
    ] {
        let edge = BRepBuilderAPIMakeEdge::from_points(first, last)
            .unwrap()
            .edge()
            .unwrap();
        filler.add_argument(&edge.into_shape());
    }
}
