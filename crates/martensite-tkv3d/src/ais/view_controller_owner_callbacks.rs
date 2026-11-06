//! Both public ownership interfaces use the same observations and actual callback inputs.
use super::view_controller_owner_fixture::*;
use super::*;
use crate::v3d::V3dViewSource;
use martensite_tkernel::standard::{Failure, Handle};
use std::sync::Arc;

pub(super) struct SelectionLegacyCallbacks(pub(super) Arc<SelectionOwnerRecorder>);
pub(super) struct SelectionDefaultArc;
impl AisViewControllerCameraHooks for SelectionDefaultArc {}

fn selected_call(
    slot: SelectionOwnerSlot,
    context: &Handle<AisInteractiveContext>,
    view: &Handle<dyn V3dViewSource>,
    new_view: &Handle<dyn V3dViewSource>,
    action: AisDragAction,
) -> SelectionOwnerCall {
    SelectionOwnerCall {
        slot,
        context: context.clone(),
        view: view.clone(),
        new_view: new_view.clone(),
        action,
    }
}
impl AisViewControllerCameraHooks for SelectionOwnerRecorder {
    fn on_selection_changed(
        &self,
        c: &mut AisViewController,
        context: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
    ) -> Result<(), Failure> {
        let call = selected_call(
            SelectionOwnerSlot::Selection,
            context,
            view,
            &self.expected.new_view,
            self.case.action,
        );
        self.invoke(c, call)
    }
    fn on_object_dragged(
        &self,
        c: &mut AisViewController,
        context: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
        action: AisDragAction,
    ) -> Result<(), Failure> {
        let call = selected_call(
            SelectionOwnerSlot::Drag,
            context,
            view,
            &self.expected.new_view,
            action,
        );
        self.invoke(c, call)
    }
    fn on_subview_changed(
        &self,
        c: &mut AisViewController,
        context: &Handle<AisInteractiveContext>,
        old: &Handle<dyn V3dViewSource>,
        new: &Handle<dyn V3dViewSource>,
    ) -> Result<(), Failure> {
        let call = selected_call(
            SelectionOwnerSlot::Subview,
            context,
            old,
            new,
            self.case.action,
        );
        self.invoke(c, call)
    }
}
impl AisViewControllerHooks for SelectionLegacyCallbacks {
    fn on_selection_changed(
        &mut self,
        c: &mut AisViewController,
        context: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
    ) -> Result<(), Failure> {
        AisViewControllerCameraHooks::on_selection_changed(self.0.as_ref(), c, context, view)
    }
    fn on_object_dragged(
        &mut self,
        c: &mut AisViewController,
        context: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
        action: AisDragAction,
    ) -> Result<(), Failure> {
        AisViewControllerCameraHooks::on_object_dragged(self.0.as_ref(), c, context, view, action)
    }
    fn on_subview_changed(
        &mut self,
        c: &mut AisViewController,
        context: &Handle<AisInteractiveContext>,
        old: &Handle<dyn V3dViewSource>,
        new: &Handle<dyn V3dViewSource>,
    ) -> Result<(), Failure> {
        AisViewControllerCameraHooks::on_subview_changed(self.0.as_ref(), c, context, old, new)
    }
}
