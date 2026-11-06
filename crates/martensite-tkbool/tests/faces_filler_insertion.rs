//! Full insertion exposes geometric incidences without detaching shared line slots.
use martensite_tkbool::{
    top_ope_brep::{TopOpeBRepFacesFiller as Filler, TopOpeBRepFacesIntersector},
    top_ope_brep_ds::TopOpeBRepDSHDataStructure,
};
use martensite_tkbrep::topods::Face;
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::{Direction3, Plane3, Point3};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;
fn face(plane: Plane3) -> Face {
    BRepBuilderAPIMakeFace::from_plane_bounds(plane, (-1., 1., -1., 1.))
        .unwrap()
        .face()
        .unwrap()
}
#[test]
fn coplanar_faces_register_reciprocal_same_domain_without_curves() {
    let first = face(Plane3::default());
    let second = face(Plane3::default());
    let mut pair = TopOpeBRepFacesIntersector::new();
    pair.perform(first.as_ref(), second.as_ref()).unwrap();
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = Filler::new();
    filler
        .insert(first.as_ref(), second.as_ref(), &mut pair, &hds)
        .unwrap();
    let storage = hds.get().unwrap();
    let ds = storage.ds();
    assert_eq!(ds.nb_curves(), 0);
    assert_eq!(ds.nb_points(), 0);
    assert!(
        ds.shape_same_domain(first.as_ref())
            .iter()
            .any(|shape| shape.is_same(second.as_ref()))
    );
    assert!(
        ds.shape_same_domain(second.as_ref())
            .iter()
            .any(|shape| shape.is_same(first.as_ref()))
    );
}
#[test]
fn perpendicular_planar_faces_create_one_curve_with_two_boundary_points() {
    let first = face(Plane3::default());
    let second = face(Plane3::new(Point3::default(), Direction3::X).unwrap());
    let mut pair = TopOpeBRepFacesIntersector::new();
    pair.perform(first.as_ref(), second.as_ref()).unwrap();
    let published = pair.lines();
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = Filler::new();
    filler
        .insert(first.as_ref(), second.as_ref(), &mut pair, &hds)
        .unwrap();
    filler
        .change_line_inter_dummy()
        .unwrap()
        .unwrap()
        .set_index(57);
    assert_eq!(
        published.get().unwrap().array1().value(1).unwrap().index(),
        57
    );
    {
        let mut slots =
            martensite_tkbool::top_ope_brep::TopOpeBRepFacesFillerArrayMut::new(&published)
                .unwrap();
        assert!(filler.check_line_in_array(&mut slots, 1).unwrap());
    }

    let storage = hds.get().unwrap();
    let ds = storage.ds();
    assert_eq!(ds.nb_curves(), 1);
    assert_eq!(ds.nb_points(), 2);
    let curve = ds.curve(1);
    let geometry = curve.curve().get().unwrap();
    let (first_parameter, last_parameter) = curve.range().unwrap();
    let mut endpoints = [
        geometry.value(first_parameter).unwrap(),
        geometry.value(last_parameter).unwrap(),
    ];
    endpoints.sort_by(|a, b| a.y().total_cmp(&b.y()));
    for (point, y) in endpoints.iter().zip([-1., 1.]) {
        assert!(point.x().abs() < 1e-12);
        assert!((point.y() - y).abs() < 1e-12);
        assert!(point.z().abs() < 1e-12);
    }
    assert_eq!(
        published
            .get()
            .unwrap()
            .array1()
            .value(1)
            .unwrap()
            .nb_vpoint(),
        2
    );
    assert_eq!(ds.shape_interferences(first.as_ref(), true).len(), 1);
    assert_eq!(ds.shape_interferences(second.as_ref(), true).len(), 1);
}
#[test]
fn explicit_restriction_processing_registers_both_face_edge_supports() {
    let first = face(Plane3::default());
    let second = face(Plane3::new(Point3::new(-1., 0., 0.), Direction3::X).unwrap());
    let mut pair = TopOpeBRepFacesIntersector::new();
    pair.perform(first.as_ref(), second.as_ref()).unwrap();
    assert!(!pair.restrictions().is_empty());
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = Filler::new();
    filler
        .insert(first.as_ref(), second.as_ref(), &mut pair, &hds)
        .unwrap();
    let lines = filler.p_faces_intersector_dummy().unwrap().lines();
    let mut array = lines.get().unwrap().change_array1();
    let restriction = array
        .as_mut_slice()
        .iter_mut()
        .find(|line| {
            line.type_line_curve()
                == martensite_tkbool::top_ope_brep::TopOpeBRepTypeLineCurve::Restriction
        })
        .unwrap();
    filler.load_line(restriction).unwrap();
    filler.process_rline(restriction).unwrap();
    let storage = hds.get().unwrap();
    let ds = storage.ds();
    assert_eq!(ds.nb_curves(), 0);
    assert!(
        ds.shape_interferences(first.as_ref(), true)
            .iter()
            .any(|i| i.get().unwrap().geometry_type()
                == martensite_tkbool::top_ope_brep_ds::TopOpeBRepDSKind::Edge)
    );
    assert!(
        ds.shape_interferences(second.as_ref(), true)
            .iter()
            .any(|i| i.get().unwrap().geometry_type()
                == martensite_tkbool::top_ope_brep_ds::TopOpeBRepDSKind::Edge)
    );
}

#[test]
fn same_domain_early_return_preserves_the_previous_pair_point_search_range() {
    use martensite_tkbool::top_ope_brep_ds::TopOpeBRepDSPoint;
    let first = face(Plane3::default());
    let vertical = face(Plane3::new(Point3::default(), Direction3::X).unwrap());
    let second = face(Plane3::default());
    let mut crossing = TopOpeBRepFacesIntersector::new();
    crossing.perform(first.as_ref(), vertical.as_ref()).unwrap();
    let mut coplanar = TopOpeBRepFacesIntersector::new();
    coplanar.perform(first.as_ref(), second.as_ref()).unwrap();
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = Filler::new();
    filler
        .insert(first.as_ref(), vertical.as_ref(), &mut crossing, &hds)
        .unwrap();
    let point = TopOpeBRepDSPoint::new(Point3::new(10., 20., 30.), 1e-7);
    let index = hds.get().unwrap().change_ds().add_point(&point);
    filler
        .insert(first.as_ref(), second.as_ref(), &mut coplanar, &hds)
        .unwrap();
    assert_eq!(
        filler.get_ff_geometry_point(&point).unwrap(),
        Some((
            martensite_tkbool::top_ope_brep_ds::TopOpeBRepDSKind::Point,
            index
        ))
    );
}

fn validation_circle(parameters: [f64; 2]) -> martensite_tkbool::top_ope_brep::TopOpeBRepLineInter {
    use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
    use martensite_tkgeomalgo::int_patch::{IntPatchGLine, IntPatchLineTransitions, IntPatchPoint};
    let source = IntPatchGLine::from_circle(
        &martensite_tkmath::gp::Circle3::default(),
        false,
        IntPatchLineTransitions::Undecided,
    );
    for parameter in parameters {
        let mut point = IntPatchPoint::new();
        point.set_parameter(parameter);
        source.add_vertex(&point).unwrap();
    }
    let mut line = martensite_tkbool::top_ope_brep::TopOpeBRepLineInter::new();
    line.set_line(
        &Handle::new(source).cast(),
        &BRepAdaptorSurface::default(),
        &BRepAdaptorSurface::default(),
    )
    .unwrap();
    for index in 1..=2 {
        line.change_vpoint(index).unwrap().change_keep(true);
    }
    line.set_vp_bounds();
    line
}

#[test]
fn public_shared_validation_retains_owner_and_slots_across_array_replacement() {
    use martensite_tkbool::top_ope_brep::{TopOpeBRepFacesFillerArrayMut, TopOpeBRepLineInter};
    use martensite_tkernel::ncollection::{Array1, HArray1};
    let first = face(Plane3::default());
    let second = face(Plane3::new(Point3::default(), Direction3::X).unwrap());
    let mut pair = TopOpeBRepFacesIntersector::new();
    pair.force_tolerances(1e-4, 1e-8);
    pair.perform(first.as_ref(), second.as_ref()).unwrap();
    let published = pair.lines();
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = Filler::new();
    filler
        .insert(first.as_ref(), second.as_ref(), &mut pair, &hds)
        .unwrap();
    let mut slots = TopOpeBRepFacesFillerArrayMut::new(&published).unwrap();
    let mut replacement = Array1::with_bounds(1, 2).unwrap();
    replacement
        .set_value(1, validation_circle([-1., 1.]))
        .unwrap();
    replacement
        .set_value(2, validation_circle([0., 1e-5]))
        .unwrap();
    *slots = replacement;
    assert!(filler.check_line_in_array(&mut slots, 1).unwrap());
    assert!(filler.check_line_in_array(&mut slots, 2).unwrap());
    let mut external = validation_circle([0., 1e-5]);
    assert!(
        filler
            .check_external_line_with_array(&mut slots, &mut external)
            .unwrap()
    );
    assert_eq!(slots.len(), 2);

    assert_eq!(
        filler
            .check_line(slots.change_value(1).unwrap())
            .unwrap_err()
            .exception_type(),
        "Standard_ProgramError"
    );
    assert_eq!(
        filler
            .check_line(slots.change_value(2).unwrap())
            .unwrap_err()
            .exception_type(),
        "Standard_ProgramError"
    );
    slots.set_value(1, validation_circle([0., 1e-5])).unwrap();
    slots.set_value(2, validation_circle([-1., 1.])).unwrap();
    assert!(!filler.check_line_in_array(&mut slots, 2).unwrap());
    external = validation_circle([-1., 1.]);
    assert!(
        !filler
            .check_external_line_with_array(&mut slots, &mut external)
            .unwrap()
    );
    assert!(
        filler
            .check_external_line_with_array(&mut slots, &mut TopOpeBRepLineInter::new())
            .unwrap()
    );

    let foreign = Handle::new(HArray1::<TopOpeBRepLineInter>::with_bounds(1, 2).unwrap());
    let mut foreign_slots = TopOpeBRepFacesFillerArrayMut::new(&foreign).unwrap();
    assert_eq!(
        filler
            .check_line_in_array(&mut foreign_slots, 1)
            .unwrap_err()
            .exception_type(),
        "Standard_DomainError"
    );

    assert_eq!(
        filler
            .check_external_line_with_array(&mut foreign_slots, &mut external)
            .unwrap_err()
            .exception_type(),
        "Standard_DomainError"
    );
    *slots = Array1::with_bounds(2, 3).unwrap();
    assert_eq!(
        filler
            .check_line_in_array(&mut slots, 2)
            .unwrap_err()
            .exception_type(),
        "Standard_OutOfRange"
    );
    assert_eq!(
        filler
            .check_external_line_with_array(&mut slots, &mut external)
            .unwrap_err()
            .exception_type(),
        "Standard_OutOfRange"
    );
}
