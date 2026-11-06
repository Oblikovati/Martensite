//! Explicit caller-owned interference outputs retain their state for null inputs.
use martensite_tkbool::top_ope_brep_ds::{
    TopOpeBRepDSInterference, TopOpeBRepDSKind as Kind, TopOpeBRepDSTransition,
    export::{
        fds_data, fds_data_into, fds_data_iter, fds_idata, fds_idata_into, fds_tdata,
        fds_tdata_into,
    },
};
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::top_abs::{ShapeEnum, State};

fn loaded_interference() -> Handle<TopOpeBRepDSInterference> {
    let mut transition =
        TopOpeBRepDSTransition::new(State::Out, State::In, ShapeEnum::Edge, ShapeEnum::Face);
    transition.set_index_before(7);
    transition.set_index_after(11);
    Handle::new(TopOpeBRepDSInterference::new(
        &transition,
        Kind::Surface,
        19,
        Kind::Curve,
        23,
    ))
}

#[test]
fn geometry_outputs_are_untouched_for_null_and_replaced_for_live_input() {
    let mut slots = (Kind::Solid, -17, Kind::Vertex, 31);
    fds_data_into(&Handle::default(), &mut slots);
    assert_eq!(slots, (Kind::Solid, -17, Kind::Vertex, 31));
    fds_data_into(&loaded_interference(), &mut slots);
    assert_eq!(slots, (Kind::Curve, 23, Kind::Surface, 19));
}

#[test]
fn transition_outputs_are_untouched_for_null_and_replaced_for_live_input() {
    let mut slots = (ShapeEnum::Shell, -37, ShapeEnum::Wire, 41);
    fds_tdata_into(&Handle::default(), &mut slots);
    assert_eq!(slots, (ShapeEnum::Shell, -37, ShapeEnum::Wire, 41));
    fds_tdata_into(&loaded_interference(), &mut slots);
    assert_eq!(slots, (ShapeEnum::Edge, 7, ShapeEnum::Face, 11));
}

#[test]
fn combined_outputs_preserve_both_groups_before_a_live_read() {
    let mut transition = (ShapeEnum::Shell, -37, ShapeEnum::Wire, 41);
    let mut geometry = (Kind::Solid, -17, Kind::Vertex, 31);
    fds_idata_into(&Handle::default(), &mut transition, &mut geometry);
    assert_eq!(transition, (ShapeEnum::Shell, -37, ShapeEnum::Wire, 41));
    assert_eq!(geometry, (Kind::Solid, -17, Kind::Vertex, 31));
    fds_idata_into(&loaded_interference(), &mut transition, &mut geometry);
    assert_eq!(transition, (ShapeEnum::Edge, 7, ShapeEnum::Face, 11));
    assert_eq!(geometry, (Kind::Curve, 23, Kind::Surface, 19));
}

#[test]
fn returned_conveniences_keep_their_default_initialization_contracts() {
    let null = Handle::default();
    let transition = (ShapeEnum::Shape, 0, ShapeEnum::Shape, 0);
    let geometry = (Kind::Unknown, 0, Kind::Unknown, 0);
    assert_eq!(fds_tdata(&null), transition);
    assert_eq!(fds_data(&null), geometry);
    assert_eq!(fds_idata(&null), (transition, geometry));
    let live = loaded_interference();
    assert_eq!(fds_idata(&live), (fds_tdata(&live), fds_data(&live)));
}

#[test]
fn iterator_data_keeps_outputs_at_end_and_copies_current_interference() {
    let sentinel = (Kind::Solid, -17, Kind::Vertex, 31);
    let mut current = Handle::default();
    let mut data = sentinel;
    assert!(!fds_data_iter(None, &mut current, &mut data).unwrap());
    assert!(current.is_null());
    assert_eq!(data, sentinel);

    let live = loaded_interference();
    let entries: List<_> = [live.clone()].into_iter().collect();
    assert!(!fds_data_iter(entries.iter().next(), &mut current, &mut data).unwrap());
    assert_eq!(data, (Kind::Curve, 23, Kind::Surface, 19));
    assert_eq!(current.get().unwrap().geometry(), 23);
}
