use super::*;

macro_rules! split_cache {
    ($test:ident, $read:ident, $write:ident) => {
        #[test]
        fn $test() {
            let mut selected = builder();
            let untouched = builder();
            let key = distinct_shape();
            let value = distinct_shape();
            let mut edit = selected.extension_state_mut();
            edit.$write().bind(key.clone(), shape_list(&value));
            let read = selected.extension_state();
            let list = read.$read().find(&key).unwrap();
            assert!(list.first().unwrap().is_same(&value));
            assert!(!untouched.extension_state().$read().is_bound(&key));
            selected.extension_state_mut().$write().unbind(&key);
            assert!(!selected.extension_state().$read().is_bound(&key));
        }
    };
}
split_cache!(face_split_cache_is_live, face_splits, face_splits_mut);
split_cache!(edge_split_cache_is_live, edge_splits, edge_splits_mut);
split_cache!(paired_face_cache_is_live, paired_faces, paired_faces_mut);

#[test]
fn consumed_face_binding_preserves_false_values_and_presence() {
    let mut selected = builder();
    let untouched = builder();
    let key = distinct_shape();
    let mut edit = selected.extension_state_mut();
    edit.consumed_faces_mut().bind(key.clone(), false);
    let read = selected.extension_state();
    assert!(read.consumed_faces().is_bound(&key));
    assert!(!read.consumed_faces().find(&key).unwrap());
    assert!(!untouched.extension_state().consumed_faces().is_bound(&key));
    selected
        .extension_state_mut()
        .consumed_faces_mut()
        .unbind(&key);
    assert!(!selected.extension_state().consumed_faces().is_bound(&key));
}

fn assert_retained_structure(
    builder: &TopOpeBRepBuildBuilder,
    expected: &Handle<TopOpeBRepDSHDataStructure>,
) {
    let query = builder.data_structure();
    let read = builder.extension_state();
    let retained = read.intersection_structure();
    assert!(std::ptr::eq(query.get().unwrap(), expected.get().unwrap()));
    assert!(std::ptr::eq(
        retained.get().unwrap(),
        expected.get().unwrap()
    ));
}

#[test]
fn data_structure_rebinding_retains_the_supplied_shared_handle() {
    let mut selected = builder();
    let untouched = builder();
    let structure = Handle::new(TopOpeBRepDSHDataStructure::new());
    *selected.extension_state_mut().intersection_structure_mut() = structure.clone();
    assert_retained_structure(&selected, &structure);
    assert!(untouched.data_structure().is_null());
    *selected.extension_state_mut().intersection_structure_mut() = Handle::default();
    assert!(selected.data_structure().is_null());
    assert!(!structure.is_null());
}

fn pair_array<T: Default + Send + Sync + 'static>(first: T, second: T) -> BuilderResultArray<T> {
    let array = BuilderResultArray::with_bounds(0, 1).unwrap();
    let mut values = array.write_array().unwrap();
    values.set_value(0, first).unwrap();
    values.set_value(1, second).unwrap();
    drop(values);
    array
}

struct ComposedBuilder {
    base: TopOpeBRepBuildBuilder,
}
impl ComposedBuilder {
    fn seed_results(&mut self, first: &Shape, second: &Shape) {
        let mut extension = self.base.extension_state_mut();
        *extension.created_vertices_mut() = pair_array(first.clone(), second.clone());
        *extension.created_faces_mut() = pair_array(shape_list(first), shape_list(second));
    }
}

#[test]
fn external_composition_sets_zero_based_results_seen_by_existing_queries() {
    let mut composed = ComposedBuilder { base: builder() };
    let untouched = builder();
    let first = distinct_shape();
    let second = distinct_shape();
    composed.seed_results(&first, &second);
    assert!(composed.base.new_vertex(0).unwrap().is_same(&first));
    assert!(composed.base.new_vertex(1).unwrap().is_same(&second));
    let faces = composed.base.new_faces(1).unwrap();
    assert!(faces.first().unwrap().is_same(&second));
    let state = composed.base.extension_state();
    assert_eq!(state.created_vertices().read_array().unwrap().len(), 2);
    assert_eq!(state.created_faces().read_array().unwrap().len(), 2);
    assert!(untouched.new_vertex(0).is_err());
    assert!(untouched.new_faces(0).is_err());
}

fn seed_on_maps(builder: &mut TopOpeBRepBuildBuilder, key: &Shape, value: &Shape) {
    let mut edit = builder.extension_state_mut();
    edit.on_elements_mut().add(key.clone());
    edit.on_faces_mut().add(key.clone(), value.clone());
    edit.split_memo_mut().add(value.clone());
}

fn assert_on_maps(builder: &TopOpeBRepBuildBuilder, key: &Shape, value: &Shape) {
    let read = builder.extension_state();
    assert!(read.on_elements().find_key(1).unwrap().is_same(key));
    assert!(read.on_faces().find_from_index(1).unwrap().is_same(value));
    assert!(read.split_memo().contains(value));
    assert!(!read.split_memo().contains(key));
}

#[test]
fn indexed_on_maps_and_split_memo_preserve_independent_membership() {
    let mut selected = builder();
    let untouched = builder();
    let key = distinct_shape();
    let value = distinct_shape();
    seed_on_maps(&mut selected, &key, &value);
    assert_on_maps(&selected, &key, &value);
    assert!(untouched.extension_state().on_elements().is_empty());
    assert!(untouched.extension_state().on_faces().is_empty());
    assert!(untouched.extension_state().split_memo().is_empty());
}
