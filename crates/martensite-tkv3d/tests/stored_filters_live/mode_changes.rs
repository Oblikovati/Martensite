//! Actual callbacks may change the parent's mode without invalidating its live list.
use martensite_tkernel::standard::{Failure, Handle, SharedState, StandardTransient, WeakHandle};
use martensite_tkv3d::select_mgr::*;
use std::{fmt::Write, sync::Arc};

struct ModeTrace {
    parent: WeakHandle<SelectMgrAndOrFilter>,
    replacement: SelectMgrFilterType,
    calls: SharedState<Vec<(u8, i32)>>,
}

struct ModeChangingFilter {
    transient: StandardTransient,
    trace: Arc<ModeTrace>,
    identity: u8,
    accepted: bool,
}
martensite_tkernel::impl_transient!(ModeChangingFilter, "ModeChangingFilter", dyn SelectMgrFilter, transient,
    interfaces: [dyn SelectMgrFilter]);

impl ModeChangingFilter {
    fn new(trace: &Arc<ModeTrace>, identity: u8, accepted: bool) -> Handle<Self> {
        Handle::new(Self {
            transient: StandardTransient::default(),
            trace: trace.clone(),
            identity,
            accepted,
        })
    }
}

impl SelectMgrFilter for ModeChangingFilter {
    fn is_ok(&self, _owner: &Handle<dyn SelectMgrEntityOwnerSource>) -> Result<bool, Failure> {
        self.trace
            .calls
            .write()
            .push((self.identity, self.transient.ref_count()));
        if self.identity == 1 {
            self.trace
                .parent
                .observe()
                .set_filter_type(self.trace.replacement);
        }
        Ok(self.accepted)
    }
}

fn actual_parent(
    initial: SelectMgrFilterType,
    replacement: SelectMgrFilterType,
) -> (Handle<SelectMgrAndOrFilter>, Arc<ModeTrace>) {
    let parent = Handle::new(SelectMgrAndOrFilter::new(initial));
    let trace = Arc::new(ModeTrace {
        parent: parent.downgrade(),
        replacement,
        calls: SharedState::new(Vec::new()),
    });
    (parent, trace)
}

fn observed_record(
    name: &str,
    accepted: bool,
    mode: SelectMgrFilterType,
    trace: &ModeTrace,
) -> String {
    let mut record = format!(
        "{name} accepted {} mode {} calls",
        i32::from(accepted),
        mode as i32
    );
    for (identity, _) in trace.calls.read().iter() {
        write!(record, " {identity}").unwrap();
    }
    record.push_str(" refs");
    for (_, count) in trace.calls.read().iter() {
        write!(record, " {count}").unwrap();
    }
    record + "\n"
}

fn mode_case(
    name: &str,
    initial: SelectMgrFilterType,
    replacement: SelectMgrFilterType,
    first: bool,
) -> String {
    let (parent, trace) = actual_parent(initial, replacement);
    let one = ModeChangingFilter::new(&trace, 1, first).cast();
    let two = ModeChangingFilter::new(&trace, 2, !first).cast();
    parent.add(&one);
    parent.add(&two);
    let owner = SelectMgrEntityOwner::new(0).cast();
    let accepted = parent.is_ok(&owner).unwrap();
    observed_record(name, accepted, parent.filter_type(), &trace)
}

#[test]
fn callback_mode_changes_match_both_native_directions() {
    let and = SelectMgrFilterType::And;
    let or = SelectMgrFilterType::Or;
    let mut actual = mode_case("or_to_and_stop", or, and, false);
    actual += &mode_case("and_to_or_stop", and, or, true);
    actual += &mode_case("or_to_and_continue", or, and, true);
    actual += &mode_case("and_to_or_continue", and, or, false);
    println!("{actual}");
    assert_eq!(
        actual,
        include_str!("../fixtures/composition-filter-live/mode-native.txt")
    );
}
