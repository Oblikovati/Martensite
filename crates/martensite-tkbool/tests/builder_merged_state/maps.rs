//! Whole-state map access is live in merge queries and normal builder cleanup.
use super::{MERGED_STATES, TopOpeBRepBuildBuilder, TopOpeBRepDSBuildTool, connected_edges};
use martensite_tkbool::top_ope_brep_ds::{MapOfShapeOn1State, TopOpeBRepDSListOfShapeOn1State};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::State;

fn merged_map_address(
    builder: &mut TopOpeBRepBuildBuilder,
    state: State,
) -> *const MapOfShapeOn1State {
    let shared = std::ptr::from_ref(
        builder
            .merged_state_results(state)
            .expect("supported state"),
    );
    let mutable = std::ptr::from_mut(
        builder
            .change_merged_state_results(state)
            .expect("supported state"),
    );
    assert_eq!(shared, mutable.cast_const());
    shared
}

fn append_part(builder: &mut TopOpeBRepBuildBuilder, source: &Shape, state: State, part: &Shape) {
    let mut payload = TopOpeBRepDSListOfShapeOn1State::default();
    payload.change_list_on_state().append(part.clone());
    builder
        .change_merged_state_results(state)
        .expect("supported state")
        .bind(source.clone(), payload);
}

#[test]
fn merged_state_reference_accessors_select_distinct_live_maps() {
    let mut builder = TopOpeBRepBuildBuilder::new(&TopOpeBRepDSBuildTool::new());
    let pointers = MERGED_STATES.map(|state| merged_map_address(&mut builder, state));
    assert!(pointers[0] != pointers[1] && pointers[0] != pointers[2] && pointers[1] != pointers[2]);
    assert!(builder.merged_state_results(State::Unknown).is_none());
    assert!(
        builder
            .change_merged_state_results(State::Unknown)
            .is_none()
    );
}

fn assert_flag_independence(
    builder: &mut TopOpeBRepBuildBuilder,
    source: &Shape,
    state: State,
) -> Result<(), Failure> {
    let map = builder
        .merged_state_results(state)
        .expect("supported state");
    assert!(!map.find(source)?.is_split());
    assert!(builder.is_merged(source, state));
    let map = builder
        .change_merged_state_results(state)
        .expect("supported state");
    let payload = map.change_find(&source.reversed())?;
    payload.split(true);
    payload.change_list_on_state().clear();
    assert!(!builder.is_merged(source, state));
    let map = builder
        .merged_state_results(state)
        .expect("supported state");
    assert!(map.find(source)?.is_split());
    Ok(())
}

#[test]
fn merge_queries_follow_direct_map_payload_changes_in_each_state() -> Result<(), Failure> {
    let [source, part] = connected_edges()?;
    let mut builder = TopOpeBRepBuildBuilder::new(&TopOpeBRepDSBuildTool::new());
    for state in MERGED_STATES {
        append_part(&mut builder, &source, state, &part);
        assert_flag_independence(&mut builder, &source, state)?;
    }
    Ok(())
}

#[test]
fn map_and_indexed_mutation_address_the_same_merged_list() -> Result<(), Failure> {
    let [source, part] = connected_edges()?;
    let mut builder = TopOpeBRepBuildBuilder::new(&TopOpeBRepDSBuildTool::new());
    append_part(&mut builder, &source, State::On, &part);
    builder
        .change_merged(&source, State::On)
        .append(source.clone());
    let map = builder
        .merged_state_results(State::On)
        .expect("supported state");
    let payload = map.find(&source)?;
    let direct = std::ptr::from_ref(payload.list_on_state());
    assert_eq!(
        direct,
        std::ptr::from_ref(builder.merged(&source, State::On))
    );
    assert_eq!(payload.list_on_state().len(), 2);
    Ok(())
}

#[test]
fn direct_whole_map_replacement_and_removal_feed_merged_queries() -> Result<(), Failure> {
    let [source, part] = connected_edges()?;
    let mut builder = TopOpeBRepBuildBuilder::new(&TopOpeBRepDSBuildTool::new());
    append_part(&mut builder, &source, State::In, &part);
    append_part(&mut builder, &source, State::Out, &part);
    let map = builder
        .change_merged_state_results(State::In)
        .expect("supported state");
    *map = MapOfShapeOn1State::default();
    assert!(!builder.is_merged(&source, State::In));
    assert!(builder.is_merged(&source, State::Out));
    let map = builder
        .change_merged_state_results(State::Out)
        .expect("supported state");
    assert!(map.unbind(&source.reversed()));
    assert!(!builder.is_merged(&source, State::Out));
    Ok(())
}

fn assert_cleanup_maps(
    builder: &mut TopOpeBRepBuildBuilder,
    before: &[*const MapOfShapeOn1State; 3],
) {
    for (state, pointer) in MERGED_STATES.into_iter().zip(before) {
        assert_eq!(*pointer, merged_map_address(builder, state));
        assert!(
            builder
                .merged_state_results(state)
                .expect("supported state")
                .is_empty()
        );
    }
}

#[test]
fn builder_cleanup_clears_the_same_maps_and_keeps_unrelated_split_results() -> Result<(), Failure> {
    let [source, part] = connected_edges()?;
    let mut builder = TopOpeBRepBuildBuilder::new(&TopOpeBRepDSBuildTool::new());
    let pointers = MERGED_STATES.map(|state| merged_map_address(&mut builder, state));
    for state in MERGED_STATES {
        append_part(&mut builder, &source, state, &part);
    }
    builder.change_split(&source, State::In).append(part);
    builder.mark_split(&source, State::In, true);
    builder.clear()?;
    assert_cleanup_maps(&mut builder, &pointers);
    assert!(builder.is_split(&source, State::In));
    assert_eq!(builder.splits(&source, State::In).len(), 1);
    Ok(())
}
