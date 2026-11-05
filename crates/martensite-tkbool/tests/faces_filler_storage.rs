//! Face-pair storage boundaries and current-curve allocation are operation-local.
use martensite_tkbool::{
    top_ope_brep::TopOpeBRepFacesFiller,
    top_ope_brep_ds::{
        TopOpeBRepDSHDataStructure, TopOpeBRepDSInterference, TopOpeBRepDSKind as Kind,
        TopOpeBRepDSPoint, TopOpeBRepDSTransition,
    },
    top_ope_brep_tool::TopOpeBRepToolShapeClassifier,
};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Point3;

fn point(x: f64) -> TopOpeBRepDSPoint {
    TopOpeBRepDSPoint::new(Point3::new(x, 0.0, 0.0), 1.0e-7)
}

fn interference() -> Handle<TopOpeBRepDSInterference> {
    Handle::new(TopOpeBRepDSInterference::new(
        &TopOpeBRepDSTransition::default(),
        Kind::Curve,
        0,
        Kind::Point,
        1,
    ))
}

#[test]
fn constructor_trace_and_classifier_binding_preserve_caller_state() {
    let mut classifier = TopOpeBRepToolShapeClassifier::default();
    let expected = &classifier as *const _;
    let mut filler = TopOpeBRepFacesFiller::new();
    assert_eq!(filler.get_trace_index(), [0, 0]);
    assert!(filler.hdata_structure().is_null());
    assert!(filler.p_shape_classifier().is_none());
    filler.set_trace_index(-4, 29);
    assert_eq!(filler.get_trace_index(), [-4, 29]);
    filler.set_p_shape_classifier(Some(&mut classifier));
    assert_eq!(filler.p_shape_classifier().unwrap() as *const _, expected);
    filler.set_p_shape_classifier(None);
    assert!(filler.p_shape_classifier().is_none());
    filler.change_point_classifier().init();
}

#[test]
fn current_pair_search_excludes_earlier_points_and_selects_first_equal_point() {
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    hds.get().unwrap().change_ds().add_point(&point(1.0));
    let mut filler = TopOpeBRepFacesFiller::new();
    filler.bind_storage(&hds).unwrap();
    assert_eq!(filler.get_ff_geometry_point(&point(1.0)).unwrap(), None);
    filler
        .change_data_structure()
        .unwrap()
        .add_point(&point(2.0));
    filler
        .change_data_structure()
        .unwrap()
        .add_point(&point(2.0));
    assert_eq!(
        filler.get_ff_geometry_point(&point(2.0)).unwrap(),
        Some((Kind::Point, 2))
    );
    assert_eq!(filler.get_ff_geometry_point(&point(8.0)).unwrap(), None);
}

#[test]
fn storing_interferences_allocates_one_curve_per_reset_and_retains_old_curve() {
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler.bind_storage(&hds).unwrap();
    let first = interference();
    let second = interference();
    filler.store_curve_interference(&first).unwrap();
    filler.store_curve_interference(&second).unwrap();
    assert_eq!(first.get().unwrap().support(), 1);
    assert_eq!(second.get().unwrap().support(), 1);
    assert_eq!(hds.get().unwrap().ds().nb_curves(), 1);
    filler.reset_dsc();
    let third = interference();
    filler.store_curve_interference(&third).unwrap();
    assert_eq!(third.get().unwrap().support(), 2);
    assert_eq!(hds.get().unwrap().ds().nb_curves(), 2);
    assert_eq!(hds.get().unwrap().ds().curve_interferences(1).len(), 0);
}

#[test]
fn missing_storage_and_null_interference_are_checked_before_allocation() {
    let mut filler = TopOpeBRepFacesFiller::new();
    assert!(filler.change_data_structure().is_err());
    assert!(filler.get_ff_geometry_point(&point(0.0)).is_err());
    assert!(filler.bind_storage(&Handle::default()).is_err());
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    filler.bind_storage(&hds).unwrap();
    assert!(filler.store_curve_interference(&Handle::default()).is_err());
    assert_eq!(hds.get().unwrap().ds().nb_curves(), 0);
}

#[test]
fn geometric_creation_and_interference_search_preserve_data_structure_indices() {
    use martensite_tkbool::top_ope_brep::TopOpeBRepVPointInter;
    use martensite_tkbool::top_ope_brep_ds::ListOfInterference;
    use martensite_tkgeomalgo::int_patch::IntPatchPoint;
    let mut source = IntPatchPoint::new();
    source.set_value_tangent(Point3::new(1.0, 2.0, 3.0), 0.125, false);
    let mut point = TopOpeBRepVPointInter::new();
    point.set_point(&source);
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler.bind_storage(&hds).unwrap();
    assert_eq!(filler.make_geometry(&point, 1).unwrap(), (Kind::Point, 1));
    assert_eq!(
        filler.get_ff_geometry_vpoint(&point).unwrap(),
        Some((Kind::Point, 1))
    );
    let entries: ListOfInterference = [interference()].into_iter().collect();
    let mut position = 0;
    assert_eq!(
        filler
            .get_geometry(&entries, &mut position, &point)
            .unwrap(),
        Some((1, Kind::Point))
    );
    assert_eq!(position, 0);
    assert_eq!(filler.make_geometry(&point, 0).unwrap(), (Kind::Point, 2));
}

#[test]
fn domain_vertices_are_registered_with_operand_rank_without_new_points() {
    use martensite_tkbool::top_ope_brep::TopOpeBRepVPointInter;
    use martensite_tkgeomalgo::{int_patch::IntPatchPoint, int_start::TheVertex};
    use martensite_tktopalgo::{
        brep_builder_api::BRepBuilderAPIMakeVertex, brep_top_adaptor::BRepTopAdaptorHVertex,
    };
    let vertex = BRepBuilderAPIMakeVertex::new(Point3::new(7.0, 8.0, 9.0))
        .unwrap()
        .vertex()
        .unwrap();
    let adapted: TheVertex =
        Handle::new(BRepTopAdaptorHVertex::new(&vertex, &Handle::default())).cast();
    let mut source = IntPatchPoint::new();
    source.set_vertex(false, &adapted);
    let mut point = TopOpeBRepVPointInter::new();
    point.set_point(&source);
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler.bind_storage(&hds).unwrap();
    assert_eq!(filler.make_geometry(&point, 2).unwrap(), (Kind::Vertex, 1));
    assert_eq!(hds.get().unwrap().ds().ancestor_rank_index(1), 2);
    assert_eq!(hds.get().unwrap().ds().nb_points(), 0);
    assert_eq!(filler.make_geometry(&point, 2).unwrap(), (Kind::Vertex, 1));
}
