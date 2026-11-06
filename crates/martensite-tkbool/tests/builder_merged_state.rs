//! Merged payload queries and builder cleanup preserve distinct split state.
use super::shape_neighbour_extension::connected_edges;
use martensite_tkbool::top_ope_brep_build::TopOpeBRepBuildBuilder;
use martensite_tkbool::top_ope_brep_ds::TopOpeBRepDSBuildTool;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::State;

const MERGED_STATES: [State; 3] = [State::In, State::Out, State::On];

fn pin_merged_payload(
    builder: &mut TopOpeBRepBuildBuilder,
    source: &Shape,
    part: &Shape,
    state: State,
) {
    assert!(!builder.is_merged(source, state));
    assert!(builder.change_merged(source, state).is_empty());
    assert!(!builder.is_merged(source, state));
    builder.change_merged(source, state).append(part.clone());
    let shared = std::ptr::from_ref(builder.merged(source, state));
    let mutable = std::ptr::from_mut(builder.change_merged(source, state));
    assert_eq!(shared, mutable.cast_const());
    assert!(builder.is_merged(source, state));
    builder.change_merged(source, state).clear();
    assert!(!builder.is_merged(source, state));
}

#[test]
fn merged_queries_follow_the_live_payload_not_merely_the_binding() -> Result<(), Failure> {
    let [source, part] = connected_edges()?;
    let mut builder = TopOpeBRepBuildBuilder::new(&TopOpeBRepDSBuildTool::new());
    for state in MERGED_STATES {
        pin_merged_payload(&mut builder, &source, &part, state);
    }
    Ok(())
}

#[test]
fn cleanup_removes_merged_payloads_and_retains_split_state_without_a_structure()
-> Result<(), Failure> {
    let [source, part] = connected_edges()?;
    let mut builder = TopOpeBRepBuildBuilder::new(&TopOpeBRepDSBuildTool::new());
    for state in MERGED_STATES {
        builder.change_merged(&source, state).append(part.clone());
    }
    builder
        .change_split(&source, State::In)
        .append(part.clone());
    builder.mark_split(&source, State::In, true);
    builder.clear()?;
    for state in MERGED_STATES {
        assert!(builder.merged(&source, state).is_empty());
        assert!(!builder.is_merged(&source, state));
    }
    assert!(builder.is_split(&source, State::In));
    assert_eq!(builder.splits(&source, State::In).len(), 1);
    Ok(())
}

#[path = "builder_merged_state/maps.rs"]
mod maps;
