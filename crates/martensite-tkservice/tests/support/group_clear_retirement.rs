//! Real group destructors observe and reenter the actual structure during clear (#324).
#[path = "null_group_members.rs"]
mod null_group_members;
use martensite_tkernel::{ncollection::BaseAllocator, standard::*};
use martensite_tkservice::{graphic3d::*, null_driver::NullGroup};
use std::sync::Arc;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GroupDropAction {
    Observe,
    Append,
    AppendThenPanic,
}
#[derive(Debug, PartialEq, Eq)]
pub struct EntryGroupDropPanic {
    pub role: &'static str,
}
#[derive(Debug, PartialEq, Eq)]
pub struct GroupDropObservation {
    pub role: &'static str,
    pub state: Option<(usize, bool, bool, bool)>,
}
pub struct GroupClearTrace {
    pub presentation: HandleObserver<Graphic3dStructure>,
    pub structure: HandleObserver<dyn Graphic3dCStructure>,
    pub allocator: BaseAllocator,
    pub rows: SharedState<Vec<GroupDropObservation>>,
    pub created: SharedState<Vec<usize>>,
}
impl GroupClearTrace {
    /// Observe without retaining entry payloads; `GroupClearTrace::new(&presentation)`.
    pub fn new(presentation: &Handle<Graphic3dStructure>) -> Arc<Self> {
        let structure = presentation.cstructure();
        let allocator = structure
            .cstructure_base()
            .fields
            .read()
            .groups
            .allocator()
            .clone();
        Arc::new(Self {
            presentation: presentation.observe(),
            structure: structure.observe(),
            allocator,
            rows: SharedState::default(),
            created: SharedState::default(),
        })
    }
    fn observe_drop(&self, role: &'static str) -> bool {
        // Nonblocking access makes the pre-correction lock failure observable without hanging.
        let state = self
            .structure
            .cstructure_base()
            .fields
            .try_write()
            .map(|fields| {
                (
                    fields.groups.len(),
                    fields.is_2d_text,
                    fields.is_for_highlight,
                    fields.groups.allocator() == &self.allocator,
                )
            });
        let unlocked = state.is_some();
        self.rows.write().push(GroupDropObservation { role, state });
        unlocked
    }
    fn append_from_drop(&self, role: &str) {
        let group = self.presentation.new_group().unwrap_or_else(|error| {
            panic!("drop role={role} group creation failed={error}; expected a reachable new group")
        });
        self.created.write().push(group.object_address() as usize);
    }
}
pub struct ObservedGroupRetirement {
    transient: StandardTransient,
    inner: NullGroup,
    trace: Arc<GroupClearTrace>,
    role: &'static str,
    action: GroupDropAction,
}
martensite_tkernel::impl_transient!(ObservedGroupRetirement, "ObservedGroupRetirement", StandardTransient, transient, interfaces: [dyn Graphic3dGroup]);
impl Graphic3dGroup for ObservedGroupRetirement {
    null_group_members::forward_null_group_members!(inner);
}
impl Drop for ObservedGroupRetirement {
    fn drop(&mut self) {
        let unlocked = self.trace.observe_drop(self.role);
        if unlocked && self.action != GroupDropAction::Observe {
            self.trace.append_from_drop(self.role);
        }
        if self.action == GroupDropAction::AppendThenPanic {
            std::panic::panic_any(EntryGroupDropPanic { role: self.role });
        }
    }
}
/// Insert a real group with no external owner after return; `install_retiring_group(&trace, "A", action)`.
pub fn install_retiring_group(
    trace: &Arc<GroupClearTrace>,
    role: &'static str,
    action: GroupDropAction,
) {
    let group = Handle::new(ObservedGroupRetirement {
        transient: StandardTransient::default(),
        inner: NullGroup::new(&trace.presentation.cast()),
        trace: trace.clone(),
        role,
        action,
    });
    trace
        .structure
        .cstructure_base()
        .fields
        .write()
        .groups
        .append(group.cast());
}
