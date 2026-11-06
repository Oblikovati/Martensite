//! External composition retains separate derived split caches across base cleanup.
use martensite_tkbool::top_ope_brep_build::{
    Builder1SplitState, Builder1SplitStateMut, TopOpeBRepBuildBuilder, TopOpeBRepBuildBuilder1,
};
use martensite_tkbool::top_ope_brep_ds::TopOpeBRepDSBuildTool;
use martensite_tkbrep::topods::{Builder, Compound, Shape};
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::Failure;

struct SplitCacheExtension {
    core: TopOpeBRepBuildBuilder1,
}

impl SplitCacheExtension {
    fn new() -> Self {
        Self {
            core: TopOpeBRepBuildBuilder1::new(&TopOpeBRepDSBuildTool::new()),
        }
    }

    fn remember(&mut self, key: &Shape, part: &Shape) -> Result<(), Failure> {
        let mut parts = List::default();
        parts.append(part.clone());
        let mut state = self.core.split_extension_state_mut()?;
        assert!(state.face_splits_mut().bind(key.clone(), parts));
        assert!(state.edge_splits_mut().bind(part.clone(), List::default()));
        Ok(())
    }
}

fn distinct_shapes() -> [Shape; 2] {
    let mut first = Compound::default();
    let mut second = Compound::default();
    Builder.make_compound(&mut first);
    Builder.make_compound(&mut second);
    [first.into_shape(), second.into_shape()]
}

#[test]
fn external_extension_caches_remain_distinct_and_survive_base_cleanup() -> Result<(), Failure> {
    let [key, part] = distinct_shapes();
    let mut extension = SplitCacheExtension::new();
    extension.remember(&key, &part)?;
    extension.core.clear()?;
    let state = extension.core.split_extension_state()?;
    let cached = state.face_splits().find(&key)?;
    assert_eq!(cached.len(), 1);
    assert!(cached.first().expect("one cached part").is_same(&part));
    assert!(!state.face_splits().is_bound(&part));
    assert!(state.edge_splits().find(&part)?.is_empty());
    assert!(!state.edge_splits().is_bound(&key));
    Ok(())
}

#[test]
fn mutable_view_edits_the_same_cache_and_does_not_affect_another_builder() -> Result<(), Failure> {
    let [key, part] = distinct_shapes();
    let mut selected = SplitCacheExtension::new();
    let untouched = SplitCacheExtension::new();
    selected.remember(&key, &part)?;
    let mut mutation = selected.core.split_extension_state_mut()?;
    mutation.face_splits_mut().clear(true);
    let state = selected.core.split_extension_state()?;
    assert!(state.face_splits().is_empty());
    assert!(state.edge_splits().is_bound(&part));
    let unchanged = untouched.core.split_extension_state()?;
    assert!(unchanged.face_splits().is_empty());
    assert!(unchanged.edge_splits().is_empty());
    Ok(())
}

#[test]
fn replaced_base_reports_missing_derived_state_for_both_view_kinds() {
    let mut owner = TopOpeBRepBuildBuilder1::new(&TopOpeBRepDSBuildTool::new());
    *owner = TopOpeBRepBuildBuilder::new(&TopOpeBRepDSBuildTool::new());
    let shared_error = owner
        .split_extension_state()
        .err()
        .expect("missing derived state");
    let mutable_error = owner
        .split_extension_state_mut()
        .err()
        .expect("missing derived state");
    assert_eq!(shared_error.exception_type(), "Standard_NullObject");
    assert_eq!(mutable_error.exception_type(), "Standard_NullObject");
}

#[test]
fn split_views_preserve_thread_traits_without_claiming_builder_thread_independence() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Builder1SplitState<'static>>();
    assert_send_sync::<Builder1SplitStateMut<'static>>();
}
