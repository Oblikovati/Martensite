//! Defined-input branch checks for closed-face subdivision, with observed virtual callbacks.
use martensite_tkbrep::brep_tools::ReShape;
use martensite_tkbrep::{
    brep::BRepTool,
    topexp::Explorer,
    topods::{self, Face},
};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, SharedState},
};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::{
    gp::{Plane3, Transform3, Vector3},
    toploc::Location,
};
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakeCylinder, BRepPrimAPIMakeOneAxis};
use martensite_tkshhealing::{
    shape_extend::ShapeExtendStatus,
    shape_upgrade::{
        ShapeUpgradeClosedFaceDivide, ShapeUpgradeSplitSurface, SplitSurface, SplitSurfaceData,
    },
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

#[derive(Default)]
struct ObservedNoBuild {
    parent: ShapeUpgradeSplitSurface,
    calls: SharedState<String>,
}
impl_transient!(ObservedNoBuild, "ObservedNoBuild", ShapeUpgradeSplitSurface, parent, interfaces:[dyn SplitSurface]);

impl SplitSurface for ObservedNoBuild {
    fn split_surface_data(&self) -> &SplitSurfaceData {
        self.parent.split_surface_data()
    }
    fn compute(&self, segment: bool) -> Result<(), Failure> {
        self.calls
            .write()
            .push_str(if segment { "C1" } else { "C0" });
        self.parent.compute(segment)
    }
    fn build(&self, segment: bool) -> Result<(), Failure> {
        self.calls
            .write()
            .push_str(if segment { "B1" } else { "B0" });
        Ok(())
    }
}

fn early_row(name: &str, face: Face, segment: bool) -> String {
    let divider = ShapeUpgradeClosedFaceDivide::new(face.clone());
    let observer = Handle::new(ObservedNoBuild::default());
    divider.set_split_surface_tool(&observer.clone().cast());
    divider.set_surface_segment_mode(segment);
    let changed = divider.split_surface(73.).unwrap();
    describe_early(name, &face, &divider, observer.get().unwrap(), changed)
}

fn describe_early(
    name: &str,
    face: &Face,
    divider: &ShapeUpgradeClosedFaceDivide,
    observer: &ObservedNoBuild,
    changed: bool,
) -> String {
    let calls = observer.calls.read().clone();
    format!(
        "{name} changed={} same={} done={} fail={} calls={} u={} v={}",
        u8::from(changed),
        u8::from(divider.result().is_same(face)),
        u8::from(divider.status_flag(ShapeExtendStatus::Done)),
        u8::from(divider.status_flag(ShapeExtendStatus::Fail)),
        if calls.is_empty() { "none" } else { &calls },
        sequence_length(observer.u_split_values()),
        sequence_length(observer.v_split_values())
    )
}

fn sequence_length(values: martensite_tkshhealing::shape_upgrade::SurfaceSplitValues) -> usize {
    if values.is_null() {
        return 0;
    }
    values.sequence(|sequence| sequence.len())
}

fn assert_expected(actual: &str) {
    let expected = include_str!("fixtures/behavior/closed_face_branches.expected");
    assert!(
        expected.lines().any(|line| line == actual),
        "unexpected branch result: {actual}"
    );
}

#[test]
fn infinite_and_open_faces_return_before_splitter_callbacks() {
    let infinite = BRepBuilderAPIMakeFace::from_plane(Plane3::default())
        .unwrap()
        .face()
        .unwrap();
    let finite = BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-2., 3., -4., 5.))
        .unwrap()
        .face()
        .unwrap();
    assert_expected(&early_row("infinite_plane", infinite, true));
    assert_expected(&early_row("open_plane", finite, true));
}

#[test]
fn no_done_splitter_observes_compute_build_order_and_segment_policy() {
    for (name, segment) in [("no_done_segment", true), ("no_done_unsegmented", false)] {
        let face = BRepPrimAPIMakeCylinder::new(2., 5.).face().unwrap();
        assert_expected(&early_row(name, face, segment));
    }
}

fn placed_face(reversed: bool) -> Face {
    let face = BRepPrimAPIMakeCylinder::new(2., 5.).face().unwrap();
    let mut translation = Transform3::default();
    translation.set_translation(Vector3::new(7., -3., 11.));
    let shape = face
        .into_shape()
        .moved(&Location::new(translation), false)
        .unwrap();
    topods::face(&if reversed { shape.reversed() } else { shape }).unwrap()
}

fn count_faces(divider: &ShapeUpgradeClosedFaceDivide) -> (usize, usize, usize, usize) {
    let mut counts = (0, 0, 0, 0);
    for face in Explorer::new(&divider.result(), ShapeEnum::Face, ShapeEnum::Shape).unwrap() {
        let face = topods::face(&face).unwrap();
        counts.0 += 1;
        counts.3 += usize::from(face.orientation() == Orientation::Reversed);
        check_cylinder_points(&face);
        for edge in Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape).unwrap() {
            counts.1 += 1;
            counts.2 += usize::from(
                BRepTool::is_closed_on_face(&topods::edge(&edge).unwrap(), &face).unwrap(),
            );
        }
    }
    counts
}

fn check_cylinder_points(face: &Face) {
    let (surface, location) = BRepTool::surface_with_location(face).unwrap();
    let (u0, u1, v0, v1) =
        martensite_tkshhealing::shape_analysis::ShapeAnalysis::get_face_uv_bounds(face).unwrap();
    assert!(((u1 - u0) - std::f64::consts::TAU / 3.).abs() <= 1e-10);
    for (u, v) in [(u0, v0), (u1, v1), ((u0 + u1) / 2., (v0 + v1) / 2.)] {
        let mut point = surface.get().unwrap().value(u, v).unwrap();
        point.transform(location.transformation());
        assert!(((point.x() - 7.).hypot(point.y() + 3.) - 2.).abs() <= 1e-10);
        assert!((point.z() - (v + 11.)).abs() <= 1e-10);
    }
}

#[test]
fn translated_forward_and_reversed_faces_keep_geometry_and_recursive_status() {
    for reversed in [false, true] {
        assert_placed_case(reversed);
    }
}

fn assert_placed_case(reversed: bool) {
    let source = placed_face(reversed);
    let divider = ShapeUpgradeClosedFaceDivide::new(source.clone());
    divider.set_nb_split_points(2);
    let changed = divider.perform(0.).unwrap();
    let (faces, edges, seams, orientations) = count_faces(&divider);
    let applied = divider.context().apply(&source, ShapeEnum::Shape).unwrap();
    let context = Explorer::new(&applied, ShapeEnum::Face, ShapeEnum::Shape)
        .unwrap()
        .count();
    assert_expected(&format!(
        "placed_{} changed={} faces={faces} edges={edges} seams={seams} reversed={orientations} done2={} fail={} context={context}",
        u8::from(reversed),
        u8::from(changed),
        u8::from(divider.status_flag(ShapeExtendStatus::Done2)),
        u8::from(divider.status_flag(ShapeExtendStatus::Fail))
    ));
}
