//! Native-owned observations expose real descriptor, owner and callback behavior.
use super::{ancestry, native_classes};
use crate::decision_filter::ReadinessDecisionFilter;
use martensite_tkernel::standard::{Handle, RuntimeTyped, SharedState, StandardTransient};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkv3d::select_mgr::{
    SelectMgrCompositionFilter, SelectMgrEntityOwner, SelectMgrFilter,
};
use std::sync::Arc;

fn projected_owner(root: &Handle<StandardTransient>) -> String {
    let before = root.ref_count();
    let projected: Handle<dyn SelectMgrCompositionFilter> = root.cast();
    assert!(!projected.is_null());
    let same = projected.object_address() == root.object_address();
    let retained = root.ref_count() - before;
    drop(projected);
    format!(
        "ownership {} {retained} {}\n",
        u8::from(same),
        root.ref_count() - before
    )
}

fn membership(composition: &Handle<dyn SelectMgrCompositionFilter>) -> String {
    let calls = Arc::new(SharedState::new(Vec::new()));
    let child = ReadinessDecisionFilter::new(true, 7, &calls).cast::<dyn SelectMgrFilter>();
    composition.add(&child);
    composition.add(&child);
    let count = composition.stored_filters().len();
    let present = composition.is_in(&child);
    composition.remove(&child);
    let remaining = composition.stored_filters().len();
    let acts = composition.acts_on(ShapeEnum::Face);
    composition.clear();
    format!(
        "duplicates {count} {} {remaining} {} {} {}\n",
        u8::from(present),
        u8::from(acts),
        u8::from(composition.is_empty()),
        u8::from(composition.is_in(&child))
    )
}

fn evaluation(filter: &Handle<dyn SelectMgrCompositionFilter>, first: bool) -> String {
    let calls = Arc::new(SharedState::new(Vec::new()));
    filter.add(&ReadinessDecisionFilter::new(first, 1, &calls).cast());
    filter.add(&ReadinessDecisionFilter::new(!first, 2, &calls).cast());
    let owner = SelectMgrEntityOwner::new(0).cast();
    let result = filter.is_ok(&owner).unwrap();
    let visited = calls
        .read()
        .iter()
        .map(|id| format!(" {id}"))
        .collect::<String>();
    filter.clear();
    format!("evaluation {}{visited}\n", u8::from(result))
}

/// Exercise real ancestry, projected ownership and live mutations; `native_observations()`.
pub(super) fn native_observations() -> String {
    let roots = native_classes();
    let mut output = String::new();
    for root in &roots {
        output.push_str(&format!("{}\n", ancestry(root.dynamic_type())));
        output.push_str(&format!(
            "kind {} {}\n",
            u8::from(root.is_kind(<dyn SelectMgrCompositionFilter>::static_type())),
            u8::from(root.is_kind(<dyn SelectMgrFilter>::static_type()))
        ));
        output.push_str(&projected_owner(root));
        output.push_str(&membership(&root.cast()));
    }
    for (root, first) in roots.iter().zip([false, true, true]) {
        output.push_str(&evaluation(&root.cast(), first));
    }
    output
}
