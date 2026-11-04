//! Composition filters retain their native runtime parents, not only callback methods.
#[path = "composition_filter_ancestry/concurrent_registration.rs"]
mod concurrent_registration;
#[path = "composition_filter_ancestry/decision_filter.rs"]
mod decision_filter;
#[path = "composition_filter_ancestry/declared_parents.rs"]
mod declared_parents;
#[path = "composition_filter_ancestry/observations.rs"]
mod observations;
use martensite_tkernel::standard::{Handle, RuntimeType, StandardTransient};
use martensite_tkv3d::select_mgr::{
    SelectMgrAndFilter, SelectMgrAndOrFilter, SelectMgrFilterType, SelectMgrOrFilter,
};

fn native_classes() -> [Handle<StandardTransient>; 3] {
    [
        Handle::new(SelectMgrAndFilter::default()).cast(),
        Handle::new(SelectMgrOrFilter::default()).cast(),
        Handle::new(SelectMgrAndOrFilter::new(SelectMgrFilterType::Or)).cast(),
    ]
}

fn ancestry(mut descriptor: Option<&RuntimeType>) -> String {
    let mut names = Vec::new();
    while let Some(current) = descriptor {
        names.push(current.name());
        descriptor = current.parent();
    }
    names.join(">")
}

#[test]
fn native_parent_chain_retains_both_abstract_filter_bases() {
    let actual = native_classes().map(|filter| ancestry(filter.dynamic_type()));
    let expected: Vec<&str> = include_str!("fixtures/composition-filter-ancestry/native.txt")
        .lines()
        .filter(|line| line.contains('>'))
        .collect();
    assert_eq!(actual.as_slice(), expected);
}

#[test]
fn real_composition_views_match_native_owner_collection_and_dispatch() {
    let actual = observations::native_observations();
    println!("{actual}");
    assert_eq!(
        actual,
        include_str!("fixtures/composition-filter-ancestry/native.txt")
    );
}
