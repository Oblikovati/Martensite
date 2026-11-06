use super::{
    GeometrySearch, TopOpeBRepFacesFiller, geometry::on_parameter, selection::*, vp_index,
};
use crate::top_ope_brep::{TopOpeBRepLineInter, TopOpeBRepVPointInter};
use crate::top_ope_brep_ds::process_interferences_tool::make_cpv_interference;
use crate::top_ope_brep_ds::{
    ListOfInterference, TopOpeBRepDSHDataStructure, TopOpeBRepDSKind as Kind, TopOpeBRepDSPoint,
    TopOpeBRepDSTransition as Transition,
};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkgeomalgo::int_patch::IntPatchPoint;
use martensite_tkmath::gp::Point3;

fn interference(
    index: i32,
    parameter: f64,
    orientation: Orientation,
    support: i32,
) -> Handle<crate::top_ope_brep_ds::TopOpeBRepDSInterference> {
    let mut transition = Transition::from_orientation(orientation);
    transition.set_index(7);
    make_cpv_interference(&transition, support, index, parameter, Kind::Point)
}

#[test]
fn parameter_lookup_is_strict_preserves_cursor_and_skips_nonparameters() {
    let mut list = ListOfInterference::default();
    list.append(interference(1, 1.0, Orientation::Forward, 4));
    list.append(interference(2, 1.5, Orientation::Reversed, 4));
    let mut position = 0;
    assert_eq!(
        on_parameter(&list, &mut position, 1.25, 0.25).unwrap(),
        None
    );
    assert_eq!(position, 2);
    position = 1;
    assert_eq!(
        on_parameter(&list, &mut position, 1.5, 0.01).unwrap(),
        Some((2, Kind::Point))
    );
    assert_eq!(position, 1);
    assert_eq!(on_parameter(&list, &mut position, 1.5, 0.0).unwrap(), None);
}

#[test]
fn interference_selectors_preserve_order_and_filter_external_transitions() {
    let mut list = ListOfInterference::default();
    for (index, orientation, support) in [
        (1, Orientation::Forward, 3),
        (2, Orientation::External, 4),
        (1, Orientation::Reversed, 3),
    ] {
        list.append(interference(index, f64::from(index), orientation, support));
    }
    let selected = on_transition_shape(&list, 7).unwrap();
    assert_eq!(selected.len(), 2);
    assert_eq!(on_transition_shape(&list, 8).unwrap().len(), 0);
    assert_eq!(on_geometry(&list, 1).len(), 2);
    assert_eq!(same_geometry_support(&list, 1, 3).unwrap().len(), 2);
    assert!(same_geometry_support(&list, 1, 4).unwrap().is_empty());
    assert_eq!(
        selected
            .iter()
            .next()
            .unwrap()
            .get()
            .unwrap()
            .transition()
            .before(),
        list.iter()
            .next()
            .unwrap()
            .get()
            .unwrap()
            .transition()
            .before()
    );
}

#[test]
fn scan_geometry_uses_each_points_own_tolerance_and_retains_all_matches() {
    let storage = TopOpeBRepDSHDataStructure::new();
    let a = storage
        .change_ds()
        .add_point(&TopOpeBRepDSPoint::new(Point3::new(0., 0., 0.), 0.1));
    let b = storage
        .change_ds()
        .add_point(&TopOpeBRepDSPoint::new(Point3::new(1., 0., 0.), 0.1));
    let mut list = ListOfInterference::default();
    for index in [a, b, a] {
        list.append(interference(index, 0., Orientation::Forward, 3));
    }
    let selected = at_point(
        &storage,
        &list,
        &TopOpeBRepDSPoint::new(Point3::new(0.05, 0., 0.), 0.),
    )
    .unwrap();
    assert_eq!(selected.len(), 2);
    assert_eq!(selected.iter().next().unwrap().get().unwrap().geometry(), a);
}

#[test]
fn make_geometry_mode_reuses_face_pair_point_without_claiming_interference() {
    let storage = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler.bind_storage(&storage).unwrap();
    let mut source = IntPatchPoint::new();
    source.set_value_tangent(Point3::new(2., 3., 4.), 1e-7, false);
    let mut point = TopOpeBRepVPointInter::new();
    point.set_owned_point(source);
    let line = TopOpeBRepLineInter::new();
    let mut first = GeometrySearch::default();
    vp_index(&mut filler, &line, &point, 1, 1, &mut first).unwrap();
    let mut second = GeometrySearch::default();
    vp_index(&mut filler, &line, &point, 1, 1, &mut second).unwrap();
    assert_eq!(first.kind, Kind::Point);
    assert_eq!(first.index, second.index);
    assert!(!second.edge_found && !second.curve_found);
    assert_eq!(storage.get().unwrap().nb_points(), 1);
}

#[test]
fn external_transition_still_validates_matching_shape_indices() {
    let mut transition = Transition::from_orientation(Orientation::External);
    transition.set_index_before(7);
    transition.set_index_after(8);
    let mut list = ListOfInterference::default();
    list.append(make_cpv_interference(&transition, 3, 1, 0., Kind::Point));
    let error = on_transition_shape(&list, 7)
        .err()
        .expect("unequal shape indices must fail before external filtering");
    assert_eq!(error.exception_type(), "Standard_Failure");
}
