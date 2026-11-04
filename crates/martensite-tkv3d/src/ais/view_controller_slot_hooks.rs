//! Legitimate installed overrides preserve their shared recorder during nested calls.
use super::view_controller_slot_fixture::*;
use super::view_controller_slot_recording::ControllerSlotRecorder;
use super::*;
use crate::v3d::{V3dView, V3dViewSource};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::Point3;
use std::sync::atomic::Ordering;

impl AisViewControllerCameraHooks for ControllerSlotRecorder {
    fn gravity_point(
        &self,
        c: &mut AisViewController,
        ctx: &AisInteractiveContext,
        view: &V3dView,
    ) -> Result<Point3, Failure> {
        if self.case.slot != ControllerSlot::Gravity {
            return c.gravity_point_base(ctx, view);
        }
        self.enter("enter", c, None);
        self.apply_slot_effect(|| self.reenter_gravity(c, ctx, view))?;
        let result = self.return_gravity(c, ctx, view)?;
        self.point("return", &result);
        Ok(result)
    }
    fn handle_navigation_keys(
        &self,
        c: &mut AisViewController,
        ctx: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
    ) -> Result<AisWalkDelta, Failure> {
        if self.case.slot != ControllerSlot::Navigation {
            return c.handle_navigation_keys_base(ctx, view);
        }
        self.enter("enter", c, None);
        self.apply_slot_effect(|| self.reenter_navigation(c, ctx, view))?;
        let result = self.return_navigation(c, ctx, view)?;
        self.walk("return", &result);
        Ok(result)
    }
    fn handle_view_events(
        &self,
        c: &mut AisViewController,
        ctx: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
    ) -> Result<(), Failure> {
        self.dispatch_void_member(c, ctx, view, VoidSlotInvocation::Frame)
    }
    fn flush_view_events(
        &self,
        c: &mut AisViewController,
        ctx: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
        to_handle: bool,
    ) -> Result<(), Failure> {
        self.dispatch_void_member(c, ctx, view, VoidSlotInvocation::Flush(to_handle))
    }
}
impl ControllerSlotRecorder {
    fn apply_slot_effect(
        &self,
        reenter: impl FnOnce() -> Result<(), Failure>,
    ) -> Result<(), Failure> {
        if !self.arm_slot_effect() {
            return Ok(());
        }
        match self.case.mode {
            ControllerSlotMode::TypedFailure => Err(slot_callback_failure()),
            ControllerSlotMode::UnrelatedFailure => {
                panic!("controller callback kind=unrelated; expected success")
            }
            _ => self.run_slot_reentry(reenter),
        }
    }
    fn arm_slot_effect(&self) -> bool {
        if matches!(
            self.case.mode,
            ControllerSlotMode::Intercept | ControllerSlotMode::Delegate
        ) {
            return false;
        }
        self.armed.swap(false, Ordering::SeqCst)
    }
    fn run_slot_reentry(
        &self,
        reenter: impl FnOnce() -> Result<(), Failure>,
    ) -> Result<(), Failure> {
        self.depth.fetch_add(1, Ordering::SeqCst);
        let result = reenter();
        self.depth.fetch_sub(1, Ordering::SeqCst);
        result
    }
    fn reenter_gravity(
        &self,
        c: &mut AisViewController,
        ctx: &AisInteractiveContext,
        view: &V3dView,
    ) -> Result<(), Failure> {
        let result = c.gravity_point(ctx, view)?;
        self.point("direct-result", &result);
        Ok(())
    }
    fn reenter_navigation(
        &self,
        c: &mut AisViewController,
        ctx: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
    ) -> Result<(), Failure> {
        let result = c.handle_navigation_keys(ctx, view)?;
        self.walk("direct-result", &result);
        Ok(())
    }
    fn return_gravity(
        &self,
        c: &mut AisViewController,
        ctx: &AisInteractiveContext,
        view: &V3dView,
    ) -> Result<Point3, Failure> {
        if self.case.mode == ControllerSlotMode::Delegate {
            return c.gravity_point_base(ctx, view);
        }
        Ok(Point3::new(7.0, 8.0, 9.0))
    }
    fn return_navigation(
        &self,
        c: &mut AisViewController,
        ctx: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
    ) -> Result<AisWalkDelta, Failure> {
        if self.case.mode == ControllerSlotMode::Delegate {
            return c.handle_navigation_keys_base(ctx, view);
        }
        Ok(literal_slot_walk())
    }
    fn dispatch_void_member(
        &self,
        c: &mut AisViewController,
        ctx: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
        invocation: VoidSlotInvocation,
    ) -> Result<(), Failure> {
        if self.case.slot != invocation.slot() {
            return self.call_void_base(c, ctx, view, invocation);
        }
        self.enter("enter", c, invocation.flag());
        self.apply_slot_effect(|| self.reenter_void(c, ctx, view, invocation))?;
        if self.case.mode == ControllerSlotMode::Delegate {
            self.call_void_base(c, ctx, view, invocation)?;
        }
        self.enter("leave", c, invocation.flag());
        Ok(())
    }
    fn call_void_base(
        &self,
        c: &mut AisViewController,
        ctx: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
        invocation: VoidSlotInvocation,
    ) -> Result<(), Failure> {
        match invocation {
            VoidSlotInvocation::Frame => c.handle_view_events_base(ctx, view),
            VoidSlotInvocation::Flush(flag) => c.flush_view_events_base(ctx, view, flag),
        }
    }
    fn reenter_void(
        &self,
        c: &mut AisViewController,
        ctx: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
        invocation: VoidSlotInvocation,
    ) -> Result<(), Failure> {
        match invocation {
            VoidSlotInvocation::Frame => c.handle_view_events(ctx, view),
            VoidSlotInvocation::Flush(flag) => c.flush_view_events(ctx, view, flag),
        }
    }
}

fn slot_callback_failure() -> Failure {
    Failure::with_type(
        "Standard_ProgramError",
        "controller callback kind=typed; expected success",
        "",
    )
}
#[derive(Clone, Copy)]
enum VoidSlotInvocation {
    Frame,
    Flush(bool),
}
impl VoidSlotInvocation {
    fn slot(self) -> ControllerSlot {
        match self {
            Self::Frame => ControllerSlot::Frame,
            Self::Flush(_) => ControllerSlot::Flush,
        }
    }
    fn flag(self) -> Option<bool> {
        match self {
            Self::Frame => None,
            Self::Flush(flag) => Some(flag),
        }
    }
}
