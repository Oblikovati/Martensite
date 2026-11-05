//! A derived split cache is distinct from the inherited regularization cache.
use super::{Builder1Data, TopOpeBRepBuildBuilder1};
use crate::top_ope_brep_build::builder::DataMapOfShapeListOfShape;
use crate::top_ope_brep_ds::TopOpeBRepDSBuildTool;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::List;

fn assert_distinct_cache(
    base: &mut DataMapOfShapeListOfShape,
    derived: &mut DataMapOfShapeListOfShape,
) {
    let key = Shape::default();
    assert!(base.bind(key.clone(), List::default()));
    assert!(base.is_bound(&key));
    assert!(!derived.is_bound(&key));
    assert!(derived.bind(key.clone(), List::default()));
    base.clear(true);
    assert!(!base.is_bound(&key));
    assert!(derived.is_bound(&key));
}

#[test]
fn derived_face_split_storage_does_not_alias_the_base_cache() {
    let mut builder = TopOpeBRepBuildBuilder1::new(&TopOpeBRepDSBuildTool::new());
    let base = &mut builder.0;
    let derived: &mut Builder1Data = base.b1.as_mut().unwrap();
    assert_distinct_cache(&mut base.my_f_splits, &mut derived.my_f_splits);
}

#[test]
fn derived_edge_split_storage_does_not_alias_the_base_cache() {
    let mut builder = TopOpeBRepBuildBuilder1::new(&TopOpeBRepDSBuildTool::new());
    let base = &mut builder.0;
    let derived: &mut Builder1Data = base.b1.as_mut().unwrap();
    assert_distinct_cache(&mut base.my_e_splits, &mut derived.my_e_splits);
}
