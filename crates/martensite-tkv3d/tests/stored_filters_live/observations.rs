//! Native records distinguish retained collection aliases from copied element ownership.
use super::probe::{LiveProbeFilter, LiveTrace};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkv3d::select_mgr::*;
use std::{
    fmt::Write,
    sync::{Arc, atomic::Ordering},
};

type FilterPair = (
    Arc<LiveTrace>,
    Handle<dyn SelectMgrFilter>,
    Handle<dyn SelectMgrFilter>,
);
type Composition = Handle<dyn SelectMgrCompositionFilter>;

fn actual_pair(parent: &Composition, first: bool) -> FilterPair {
    let trace = Arc::new(LiveTrace::default());
    *trace.parent.write() = parent.downgrade();
    let one = LiveProbeFilter::new(&trace, 1, first).cast();
    let two = LiveProbeFilter::new(&trace, 2, !first).cast();
    (trace, one, two)
}

fn actual_trace(name: &str, accepted: bool, trace: &LiveTrace) -> String {
    let mut record = format!("{name} {} calls", i32::from(accepted));
    for (id, _) in trace.calls.read().iter() {
        write!(record, " {id}").unwrap();
    }
    record.push_str(" refs");
    for (_, count) in trace.calls.read().iter() {
        write!(record, " {count}").unwrap();
    }
    record.push('\n');
    record
}

fn actual_evaluation(parent: &Composition, acts_on: bool) -> bool {
    if acts_on {
        return parent.acts_on(ShapeEnum::Face);
    }
    let owner = SelectMgrEntityOwner::new(0);
    parent.is_ok(&owner.cast()).unwrap()
}

fn append_summary(
    length: usize,
    one: &Handle<dyn SelectMgrFilter>,
    two: &Handle<dyn SelectMgrFilter>,
) -> String {
    let first = one.ref_count();
    let second = two.ref_count();
    format!("appended {length} {first} {second}\n")
}

fn append_case(name: &str, parent: Composition, first: bool, acts_on: bool) -> String {
    let (trace, one, two) = actual_pair(&parent, first);
    *trace.append.write() = two.downgrade();
    parent.add(&one);
    let alias = parent.stored_filters();
    let accepted = actual_evaluation(&parent, acts_on);
    let record = actual_trace(name, accepted, &trace);
    let summary = append_summary(alias.len(), &one, &two);
    parent.clear();
    record + &summary
}

fn alias_case() -> String {
    let parent = Handle::new(SelectMgrAndFilter::default());
    let trace = Arc::new(LiveTrace::default());
    let child: Handle<dyn SelectMgrFilter> = LiveProbeFilter::new(&trace, 1, true).cast();
    let alias = parent.stored_filters();
    let mut record = format!("alias {}", alias.len());
    parent.add(&child);
    parent.add(&child);
    write!(record, " {} {}", alias.len(), child.ref_count()).unwrap();
    parent.remove(&child);
    write!(record, " {} {}", alias.len(), child.ref_count()).unwrap();
    parent.clear();
    writeln!(record, " {} {}", alias.len(), child.ref_count()).unwrap();
    record
}

fn lifetime_case() -> String {
    let trace = Arc::new(LiveTrace::default());
    let parent = Handle::new(SelectMgrAndFilter::default());
    let child = LiveProbeFilter::new(&trace, 1, true).cast();
    parent.add(&child);
    drop(child);
    let before = trace.destructions.load(Ordering::Acquire);
    parent.clear();
    format!(
        "lifetime {before} {}\n",
        trace.destructions.load(Ordering::Acquire)
    )
}

fn removed_summary(parent: &Composition, two: &Handle<dyn SelectMgrFilter>) -> String {
    let length = parent.stored_filters().len();
    let references = two.ref_count();
    format!("removed {length} {references}\n")
}

fn remove_case() -> String {
    let parent: Composition = Handle::new(SelectMgrAndFilter::default()).cast();
    let (trace, one, two) = actual_pair(&parent, true);
    *trace.remove.write() = two.downgrade();
    parent.add(&one);
    parent.add(&two);
    let accepted = actual_evaluation(&parent, false);
    let record = actual_trace("remove_later", accepted, &trace);
    let summary = removed_summary(&parent, &two);
    parent.clear();
    record + &summary
}

/// Compare actual live-list content/refcount records; `native_records()`.
pub(super) fn native_records() -> String {
    let and = || Handle::new(SelectMgrAndFilter::default()).cast();
    let or = || Handle::new(SelectMgrOrFilter::default()).cast();
    let and_or = Handle::new(SelectMgrAndOrFilter::new(SelectMgrFilterType::Or)).cast();
    let mut records = alias_case() + &lifetime_case();
    records += &append_case("append_and", and(), true, false);
    records += &append_case("append_or", or(), false, false);
    records += &append_case("append_andor", and_or, false, false);
    records += &append_case("append_actson", and(), false, true);
    records += &append_case("short_and", and(), false, false);
    records += &append_case("short_or", or(), true, false);
    records + &remove_case()
}
