//! Live split maps let external operations prepare and consume state-local results.
use super::shape_neighbour_extension::connected_edges;
use martensite_tkbool::top_ope_brep_build::TopOpeBRepBuildBuilder;
use martensite_tkbool::top_ope_brep_ds::{
    MapOfShapeOn1State, TopOpeBRepDSBuildTool, TopOpeBRepDSListOfShapeOn1State,
};
use martensite_tkbrep::topods::{Builder, Compound, Shape, ShapeIterator};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::State;

const SPLIT_STATES: [State; 3] = [State::In, State::Out, State::On];

fn split_payload(parts: &[Shape]) -> TopOpeBRepDSListOfShapeOn1State {
    let mut result = TopOpeBRepDSListOfShapeOn1State::default();
    for part in parts {
        result.change_list_on_state().append(part.clone());
    }
    result.split(true);
    result
}

fn empty_builder() -> TopOpeBRepBuildBuilder {
    TopOpeBRepBuildBuilder::new(&TopOpeBRepDSBuildTool::new())
}

fn bind_parts(builder: &mut TopOpeBRepBuildBuilder, source: &Shape, state: State, parts: &[Shape]) {
    builder
        .change_msplit(state)
        .bind(source.clone(), split_payload(parts));
}

fn populate_states(builder: &mut TopOpeBRepBuildBuilder, source: &Shape, part: &Shape) {
    for state in SPLIT_STATES {
        bind_parts(builder, source, state, std::slice::from_ref(part));
    }
}

fn assert_live_map(
    builder: &mut TopOpeBRepBuildBuilder,
    state: State,
) -> *const MapOfShapeOn1State {
    let shared = std::ptr::from_ref(builder.msplit(state));
    let mutable = std::ptr::from_mut(builder.change_msplit(state));
    assert_eq!(shared, mutable.cast_const());
    shared
}

fn assert_single_part(
    builder: &TopOpeBRepBuildBuilder,
    source: &Shape,
    state: State,
    part: &Shape,
) {
    let result = builder.splits(source, state);
    assert_eq!(result.len(), 1);
    assert!(result.iter().next().expect("one part").is_same(part));
    assert!(builder.is_split(source, state));
}

#[test]
fn split_state_map_accessors_return_distinct_live_maps() -> Result<(), Failure> {
    let [source, part] = connected_edges()?;
    let mut builder = empty_builder();
    let pointers = SPLIT_STATES.map(|state| assert_live_map(&mut builder, state));
    assert!(pointers[0] != pointers[1] && pointers[0] != pointers[2] && pointers[1] != pointers[2]);
    assert_eq!(assert_live_map(&mut builder, State::Unknown), pointers[0]);
    populate_states(&mut builder, &source, &part);
    for state in SPLIT_STATES {
        assert_single_part(&builder, &source, state, &part);
    }
    assert!(!builder.is_split(&source, State::Unknown));
    Ok(())
}

fn mutate_out_payload(builder: &mut TopOpeBRepBuildBuilder, source: &Shape) -> Result<(), Failure> {
    let reversed = source.reversed();
    let payload = builder.change_msplit(State::Out).change_find(&reversed)?;
    payload.split(false);
    payload.change_list_on_state().append(source.clone());
    assert!(!builder.is_split(source, State::Out));
    assert_eq!(builder.splits(source, State::Out).len(), 2);
    builder.mark_split(source, State::Out, true);
    assert!(builder.msplit(State::Out).find(&reversed)?.is_split());
    Ok(())
}

#[test]
fn split_map_mutation_changes_flags_lists_and_only_the_selected_binding() -> Result<(), Failure> {
    let [source, part] = connected_edges()?;
    let mut builder = empty_builder();
    populate_states(&mut builder, &source, &part);
    mutate_out_payload(&mut builder, &source)?;
    assert!(builder.change_msplit(State::Out).unbind(&source));
    assert!(!builder.is_split(&source, State::Out));
    assert!(builder.splits(&source, State::Out).is_empty());
    for state in [State::In, State::On] {
        assert_single_part(&builder, &source, state, &part);
        assert_eq!(builder.msplit(state).len(), 1);
    }
    Ok(())
}

#[test]
fn split_list_accessors_mutate_the_existing_map_value() -> Result<(), Failure> {
    let [source, part] = connected_edges()?;
    let mut builder = empty_builder();
    bind_parts(&mut builder, &source, State::In, &[]);
    builder
        .change_split(&source, State::In)
        .append(part.clone());
    let payload = builder.msplit(State::In).find(&source)?;
    let direct = std::ptr::from_ref(payload.list_on_state());
    let accessor = std::ptr::from_ref(builder.splits(&source, State::In));
    assert_eq!(direct, accessor);
    assert_single_part(&builder, &source, State::In, &part);
    Ok(())
}

fn assert_fallback_aliases(builder: &mut TopOpeBRepBuildBuilder, source: &Shape) {
    let fallback = std::ptr::from_ref(builder.splits(source, State::In));
    let merged = std::ptr::from_ref(builder.merged(source, State::On));
    let mutable_split = std::ptr::from_mut(builder.change_split(source, State::Unknown));
    let mutable_merge = std::ptr::from_mut(builder.change_merged(source, State::Unknown));
    assert_eq!(fallback, merged);
    assert_eq!(fallback, mutable_split.cast_const());
    assert_eq!(fallback, mutable_merge.cast_const());
}

fn assert_fallback_payload(builder: &TopOpeBRepBuildBuilder, source: &Shape) {
    for state in [State::In, State::Out, State::On, State::Unknown] {
        assert_eq!(builder.splits(source, state).len(), 1);
        assert_eq!(builder.merged(source, state).len(), 1);
        assert!(!builder.is_split(source, state));
        assert!(!builder.is_merged(source, state));
    }
}

#[test]
fn empty_result_fallback_is_shared_and_its_payload_remains_mutable() -> Result<(), Failure> {
    let [source, part] = connected_edges()?;
    let mut builder = empty_builder();
    assert_fallback_aliases(&mut builder, &source);
    assert!(builder.splits(&source, State::In).is_empty());
    builder.change_split(&source, State::Unknown).append(part);
    assert_fallback_payload(&builder, &source);
    builder.change_merged(&source, State::Unknown).clear();
    assert!(builder.splits(&source, State::In).is_empty());
    assert!(builder.msplit(State::In).is_empty());
    Ok(())
}

fn compound_of(parts: &[Shape]) -> Result<Shape, Failure> {
    let mut compound = Compound::default();
    Builder.make_compound(&mut compound);
    let mut result = compound.into_shape();
    for part in parts {
        Builder.add(&mut result, part)?;
    }
    Ok(result)
}

struct SplitAssembly {
    builder: TopOpeBRepBuildBuilder,
}

impl SplitAssembly {
    fn assemble(&self, source: &Shape, state: State) -> Result<Shape, Failure> {
        let payload = self.builder.msplit(state).find(source)?;
        if !payload.is_split() {
            return compound_of(&[]);
        }
        let parts: Vec<Shape> = payload.list_on_state().iter().cloned().collect();
        compound_of(&parts)
    }
}

fn assert_assembly_parts(result: &Shape, parts: &[Shape; 2]) {
    let children: Vec<Shape> = ShapeIterator::new(result, true, true).collect();
    assert_eq!(children.len(), 2);
    assert!(children[0].is_same(&parts[0]) && children[1].is_same(&parts[1]));
}

#[test]
fn external_assembly_consumes_live_split_payload_and_flag() -> Result<(), Failure> {
    let parts = connected_edges()?;
    let source = compound_of(&parts)?;
    let mut assembly = SplitAssembly {
        builder: empty_builder(),
    };
    bind_parts(&mut assembly.builder, &source, State::In, &parts);
    assert_assembly_parts(&assembly.assemble(&source, State::In)?, &parts);
    let map = assembly.builder.change_msplit(State::In);
    map.change_find(&source)?.split(false);
    let empty = assembly.assemble(&source, State::In)?;
    assert_eq!(ShapeIterator::new(&empty, true, true).count(), 0);
    assert_eq!(ShapeIterator::new(&source, true, true).count(), 2);
    Ok(())
}
