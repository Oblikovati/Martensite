#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_basic::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

step_entity! {
    /// Representation of STEP entity Change
    StepAP203Change, StepAP203ChangeState, "StepAP203Change", StepBasicActionAssignment, {
        the_items: Handle<HArray1<StepAP203WorkItem>>,
    }
}
impl StepAP203Change {
    step_methods! {
        get items = the_items: Handle<HArray1<StepAP203WorkItem>>;
        set set_items = the_items: Handle<HArray1<StepAP203WorkItem>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_action_assignment_assigned_action: &Handle<StepBasicAction>,
        a_items: &Handle<HArray1<StepAP203WorkItem>>,
    ) {
        self.parent.init(a_action_assignment_assigned_action);
        self.state.write().the_items = a_items.clone();
    }
}

step_entity! {
    /// Representation of STEP entity ChangeRequest
    StepAP203ChangeRequest, StepAP203ChangeRequestState, "StepAP203ChangeRequest", StepBasicActionRequestAssignment, {
        the_items: Handle<HArray1<StepAP203ChangeRequestItem>>,
    }
}
impl StepAP203ChangeRequest {
    step_methods! {
        get items = the_items: Handle<HArray1<StepAP203ChangeRequestItem>>;
        set set_items = the_items: Handle<HArray1<StepAP203ChangeRequestItem>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_action_request_assignment_assigned_action_request: &Handle<
            StepBasicVersionedActionRequest,
        >,
        a_items: &Handle<HArray1<StepAP203ChangeRequestItem>>,
    ) {
        self.parent
            .init(a_action_request_assignment_assigned_action_request);
        self.state.write().the_items = a_items.clone();
    }
}

step_entity! {
    /// Representation of STEP entity StartRequest
    StepAP203StartRequest, StepAP203StartRequestState, "StepAP203StartRequest", StepBasicActionRequestAssignment, {
        the_items: Handle<HArray1<StepAP203StartRequestItem>>,
    }
}
impl StepAP203StartRequest {
    step_methods! {
        get items = the_items: Handle<HArray1<StepAP203StartRequestItem>>;
        set set_items = the_items: Handle<HArray1<StepAP203StartRequestItem>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_action_request_assignment_assigned_action_request: &Handle<
            StepBasicVersionedActionRequest,
        >,
        a_items: &Handle<HArray1<StepAP203StartRequestItem>>,
    ) {
        self.parent
            .init(a_action_request_assignment_assigned_action_request);
        self.state.write().the_items = a_items.clone();
    }
}

step_entity! {
    /// Representation of STEP entity StartWork
    StepAP203StartWork, StepAP203StartWorkState, "StepAP203StartWork", StepBasicActionAssignment, {
        the_items: Handle<HArray1<StepAP203WorkItem>>,
    }
}
impl StepAP203StartWork {
    step_methods! {
        get items = the_items: Handle<HArray1<StepAP203WorkItem>>;
        set set_items = the_items: Handle<HArray1<StepAP203WorkItem>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_action_assignment_assigned_action: &Handle<StepBasicAction>,
        a_items: &Handle<HArray1<StepAP203WorkItem>>,
    ) {
        self.parent.init(a_action_assignment_assigned_action);
        self.state.write().the_items = a_items.clone();
    }
}
