//! Analytic line filling stores endpoint transitions, shared points, and face history.
use martensite_tkbool::{
    top_ope_brep::{TopOpeBRepFacesFiller, TopOpeBRepFacesIntersector, TopOpeBRepLineInter},
    top_ope_brep_ds::{TopOpeBRepDSHDataStructure, TopOpeBRepDSInterferenceTool, TopOpeBRepDSKind},
};
use martensite_tkbrep::{brep_adaptor::BRepAdaptorSurface, topods::Face};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::top_abs::State;
use martensite_tkgeomalgo::{
    int_patch::{IntPatchGLine, IntPatchLineTransitions, IntPatchPoint},
    int_surf::IntSurfTypeTrans,
};
use martensite_tkmath::gp::{Line3, Plane3, Point3};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

fn face() -> Face {
    BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-2., 2., -2., 2.))
        .unwrap()
        .face()
        .unwrap()
}

fn line(parameters: &[f64], first: &Face, second: &Face) -> TopOpeBRepLineInter {
    let source = IntPatchGLine::from_line(
        &Line3::default(),
        false,
        IntPatchLineTransitions::Types(IntSurfTypeTrans::In, IntSurfTypeTrans::Out),
    )
    .unwrap();
    for parameter in parameters {
        let mut point = IntPatchPoint::new();
        point.set_value_tangent(Point3::new(*parameter, 0., 0.), 1e-7, false);
        point.set_parameter(*parameter);
        source.add_vertex(&point).unwrap();
    }
    let mut line = TopOpeBRepLineInter::new();
    line.set_line(
        &Handle::new(source).cast(),
        &BRepAdaptorSurface::default(),
        &BRepAdaptorSurface::default(),
    )
    .unwrap();
    for index in 1..=line.nb_vpoint() {
        line.change_vpoint(index).unwrap().change_keep(true);
    }
    line.set_faces(first, second);
    line.compute_face_face_transition().unwrap();
    line.set_vp_bounds();
    line.set_inl().unwrap();
    line
}

#[test]
fn ordinary_line_fill_publishes_endpoints_and_both_face_interferences() {
    let (first, second) = (face(), face());
    let mut line = line(&[-1., 1.], &first, &second);
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut pair = TopOpeBRepFacesIntersector::new();
    let mut filler = TopOpeBRepFacesFiller::new();
    filler
        .bind_context(first.as_ref(), second.as_ref(), &mut pair, &hds)
        .unwrap();
    filler.fill_line(&mut line).unwrap();
    filler.add_shapes_line(&mut line).unwrap();
    filler.add_shapes_line(&mut line).unwrap();
    let storage = hds.get().unwrap().ds();
    assert_eq!(storage.nb_points(), 2);
    assert_eq!(storage.nb_curves(), 1);
    assert_eq!(storage.shape_interferences(first.as_ref(), true).len(), 1);
    assert_eq!(storage.shape_interferences(second.as_ref(), true).len(), 1);
    let curve = storage.curve(1);
    assert_eq!(curve.range(), Some((-1., 1.)));
    assert_eq!(curve.tolerance(), 1e-8);
    assert!(curve.shape1().is_same(first.as_ref()));
    assert!(curve.shape2().is_same(second.as_ref()));
    assert!(!curve.get_sci1().is_null() && !curve.get_sci2().is_null());
    let endpoints: Vec<_> = storage
        .curve_interferences(1)
        .iter()
        .filter_map(|entry| entry.get())
        .filter(|entry| entry.geometry_type() == TopOpeBRepDSKind::Point)
        .collect();
    assert_eq!(endpoints.len(), 2);
    assert_eq!(
        TopOpeBRepDSInterferenceTool::parameter(
            storage.curve_interferences(1).iter().next().unwrap()
        )
        .unwrap(),
        -1.
    );
    assert_eq!(
        (
            endpoints[0].transition().before(),
            endpoints[0].transition().after()
        ),
        (State::Out, State::In)
    );
    assert_eq!(
        (
            endpoints[1].transition().before(),
            endpoints[1].transition().after()
        ),
        (State::In, State::Out)
    );
}

#[test]
fn empty_line_bounds_do_not_allocate_geometry_or_curves() {
    let (first, second) = (face(), face());
    let mut line = line(&[], &first, &second);
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler.bind_storage(&hds).unwrap();
    filler.fill_line(&mut line).unwrap();
    filler.add_shapes_line(&mut line).unwrap();
    assert_eq!(hds.get().unwrap().ds().nb_points(), 0);
    assert_eq!(hds.get().unwrap().ds().nb_curves(), 0);
}

#[test]
fn coincident_points_reuse_geometry_and_complement_the_previous_transition() {
    let (first, second) = (face(), face());
    let mut line = line(&[-1., 1.], &first, &second);
    let mut replacement = IntPatchPoint::new();
    replacement.set_value_tangent(Point3::new(-1., 0., 0.), 1e-7, false);
    replacement.set_parameter(1.);
    line.change_vpoint(2).unwrap().set_owned_point(replacement);
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut pair = TopOpeBRepFacesIntersector::new();
    let mut filler = TopOpeBRepFacesFiller::new();
    filler
        .bind_context(first.as_ref(), second.as_ref(), &mut pair, &hds)
        .unwrap();
    filler.fill_line(&mut line).unwrap();
    filler.add_shapes_line(&mut line).unwrap();
    let storage = hds.get().unwrap().ds();
    assert_eq!(storage.nb_points(), 1);
    let entries: Vec<_> = storage
        .curve_interferences(1)
        .iter()
        .filter_map(|e| e.get())
        .filter(|e| e.geometry_type() == TopOpeBRepDSKind::Point)
        .collect();
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].geometry(), entries[1].geometry());
    assert_eq!(
        entries[1].transition(),
        entries[0].transition().complement()
    );
}

#[test]
fn source_tiny_curve_guard_marks_the_curve_unkept_without_face_interferences() {
    let (first, second) = (face(), face());
    let mut line = line(&[0., 5.0e-8], &first, &second);
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut pair = TopOpeBRepFacesIntersector::new();
    let mut filler = TopOpeBRepFacesFiller::new();
    filler
        .bind_context(first.as_ref(), second.as_ref(), &mut pair, &hds)
        .unwrap();
    filler.fill_line(&mut line).unwrap();
    filler.add_shapes_line(&mut line).unwrap();
    let storage = hds.get().unwrap().ds();
    assert!(!storage.curve(1).keep());
    assert!(!storage.curve(1).curve().is_null());
    assert_eq!(storage.curve(1).range(), Some((0.0, 5e-8)));
    assert_eq!(storage.curve(1).tolerance(), 1e-7);
    assert!(storage.curve(1).shape1().is_null() && storage.curve(1).shape2().is_null());
    assert!(storage.curve_interferences(1).is_empty());
    assert!(storage.curve(1).get_sci1().is_null());
    assert!(storage.curve(1).get_sci2().is_null());
}

#[test]
fn face_tolerance_failure_retains_previously_built_curve_geometry() {
    let (first, second) = (face(), face());
    let mut line = line(&[-1.0, 1.0], &first, &second);
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler.bind_storage(&hds).unwrap();
    filler.fill_line(&mut line).unwrap();
    // Storage-only binding intentionally leaves operand faces null: geometry is built first.
    assert!(filler.add_shapes_line(&mut line).is_err());
    let storage = hds.get().unwrap().ds();
    let curve = storage.curve(1);
    assert!(!curve.curve().is_null());
    assert_eq!(curve.range(), Some((-1.0, 1.0)));
    assert_eq!(curve.tolerance(), 1e-7);
    assert!(curve.keep());
    assert!(curve.get_sci1().is_null() && curve.get_sci2().is_null());
    assert!(curve.shape1().is_null() && curve.shape2().is_null());
    assert!(storage.curve_interferences(1).is_empty());
}
