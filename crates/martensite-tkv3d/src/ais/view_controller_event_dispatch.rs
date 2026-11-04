//! Frame/flush staging preserves native callback, subview and failure order.
use super::view_controller_lazy_move::camera_action_view;
use super::{AisInteractiveContext, AisViewController, AisViewInputBuffer, AisViewSelectionTool};
use crate::v3d::{V3dView, V3dViewSource};
use martensite_tkernel::{
    ncollection::Vec2,
    standard::{Failure, Handle},
};
use martensite_tkservice::graphic3d::{Graphic3dCViewSubviews, Graphic3dCViewXr};

impl AisViewController {
    /// `HandleViewEvents`: process the rendering-thread queue on a view (immediate update
    /// suspended): subview picking, orientation keys, navigation, XR input, highlighting and
    /// selection, camera actions, XR presentations and redraw, then the queue is reset.
    /// Example: `controller.handle_view_events(&context, &view)?`.
    pub fn handle_view_events(
        &mut self,
        context: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
    ) -> Result<(), Failure> {
        if let Some(hooks) = self.dispatch.camera_hooks.clone() {
            return hooks.handle_view_events(self, context, view);
        }
        self.handle_view_events_base(context, view)
    }
    /// Retain the unoverridden handle_view_events behavior; `controller.handle_view_events_base(context, view)?`.
    pub fn handle_view_events_base(
        &mut self,
        context: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
    ) -> Result<(), Failure> {
        let actual = camera_action_view(view)?;
        let was_immediate_update = actual.set_immediate_update(false);
        let picked_view = self.pick_controller_frame_subview(view, actual)?;
        self.run_controller_frame_actions(context, view, actual)?;
        actual.set_immediate_update(was_immediate_update);
        self.finish_controller_frame_input(context, view, &picked_view)
    }
    fn pick_controller_frame_subview(
        &self,
        view: &Handle<dyn V3dViewSource>,
        actual: &V3dView,
    ) -> Result<Handle<dyn V3dViewSource>, Failure> {
        let mut picked_view = Handle::default();
        if actual.is_subview() || !actual.subviews().is_empty() {
            picked_view = self.pick_controller_selection_subview(view, actual)?;
        }
        Ok(picked_view)
    }
    fn pick_controller_selection_subview(
        &self,
        view: &Handle<dyn V3dViewSource>,
        actual: &V3dView,
    ) -> Result<Handle<dyn V3dViewSource>, Failure> {
        let selection = &self.render_input.selection;
        if selection.tool != AisViewSelectionTool::Picking || selection.points.is_empty() {
            return Ok(Handle::default());
        }
        let mut click = *selection.points.last().expect("nonempty points");
        adjust_controller_frame_click(actual, &mut click);
        let parent = controller_frame_selection_parent(actual, view);
        Ok(camera_action_view(&parent)?.pick_subview(click))
    }
    fn run_controller_frame_actions(
        &mut self,
        context: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
        actual: &V3dView,
    ) -> Result<(), Failure> {
        self.handle_view_orientation_keys(context, view)?;
        let walk = self.handle_navigation_keys(context, view)?;
        self.handle_xr_input(context, view, &walk)?;
        if actual.view().is_active_xr() {
            actual.view().setup_xr_posed_camera();
        }
        self.handle_move_to(context, view)?;
        self.handle_camera_actions(context, view, &walk)?;
        self.finish_controller_frame_redraw(context, view, actual)
    }
    fn finish_controller_frame_redraw(
        &mut self,
        context: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
        actual: &V3dView,
    ) -> Result<(), Failure> {
        // Camera actions may move the posed camera: carry it to the base camera.
        actual.view().synchronize_xr_posed_to_base_camera()?;
        self.handle_xr_presentations(context, view)?;
        self.handle_view_redraw(context, view)?;
        actual.view().unset_xr_posed_camera()?;
        Ok(())
    }
    fn finish_controller_frame_input(
        &mut self,
        context: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
        picked_view: &Handle<dyn V3dViewSource>,
    ) -> Result<(), Failure> {
        if !picked_view.is_null() && picked_view != view {
            self.on_subview_changed(context, view, picked_view)?;
        }
        self.render_input.reset();
        self.navigation_state.to_ask_next_frame = false;
        Ok(())
    }
    /// `FlushViewEvents`: move the UI queue to the rendering queue (with the touch gestures),
    /// shift its pixels into a subview and optionally handle it.
    /// Example: `controller.flush_view_events(&context, &view, true)?`.
    pub fn flush_view_events(
        &mut self,
        context: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
        to_handle: bool,
    ) -> Result<(), Failure> {
        if let Some(hooks) = self.dispatch.camera_hooks.clone() {
            return hooks.flush_view_events(self, context, view, to_handle);
        }
        self.flush_view_events_base(context, view, to_handle)
    }
    /// Retain the unoverridden flush_view_events behavior; `controller.flush_view_events_base(context, view, to_handle)?`.
    pub fn flush_view_events_base(
        &mut self,
        context: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
        to_handle: bool,
    ) -> Result<(), Failure> {
        self.flush_buffers(context, view)?;
        self.flush_gestures(context, view)?;
        let actual = camera_action_view(view)?;
        if actual.is_subview() {
            let delta = actual.view().subview_top_left();
            self.shift_controller_subview_input(delta);
        }
        if to_handle {
            self.handle_view_events(context, view)?;
        }
        Ok(())
    }
    fn shift_controller_subview_input(&mut self, delta: Vec2<i32>) {
        let input = &mut self.render_input;
        shift_controller_pointer_requests(input, delta);
        shift_controller_rotation_requests(input, delta);
        shift_controller_selection_requests(input, delta);
    }
}
fn adjust_controller_frame_click(actual: &V3dView, click: &mut Vec2<i32>) {
    if actual.is_subview() {
        let corner = actual.view().subview_top_left();
        *click = Vec2::new(click.x + corner.x, click.y + corner.y);
    }
}
fn controller_frame_selection_parent(
    actual: &V3dView,
    view: &Handle<dyn V3dViewSource>,
) -> Handle<dyn V3dViewSource> {
    if actual.is_subview() {
        actual.parent_view()
    } else {
        view.clone()
    }
}
fn shift_controller_pixel(point: &mut Vec2<i32>, delta: Vec2<i32>) {
    let shifted = Vec2::new(point.x - delta.x, point.y - delta.y);
    *point = shifted;
}
fn shift_controller_rotation_pixel(point: &mut Vec2<f64>, delta: Vec2<i32>) {
    let shifted = Vec2::new(point.x - f64::from(delta.x), point.y - f64::from(delta.y));
    *point = shifted;
}
fn shift_controller_pointer_requests(input: &mut AisViewInputBuffer, delta: Vec2<i32>) {
    if input.move_to.to_hilight || input.dragging.to_start {
        shift_controller_pixel(&mut input.move_to.point, delta);
    }
    if input.panning.to_start {
        shift_controller_pixel(&mut input.panning.point_start, delta);
    }
    if input.dragging.to_start {
        shift_controller_pixel(&mut input.dragging.point_start, delta);
    }
    if input.dragging.to_move {
        shift_controller_pixel(&mut input.dragging.point_to, delta);
    }
}
fn shift_controller_rotation_requests(input: &mut AisViewInputBuffer, delta: Vec2<i32>) {
    if input.orbit_rotation.to_start {
        shift_controller_rotation_pixel(&mut input.orbit_rotation.point_start, delta);
    }
    if input.orbit_rotation.to_rotate {
        shift_controller_rotation_pixel(&mut input.orbit_rotation.point_to, delta);
    }
    if input.view_rotation.to_start {
        shift_controller_rotation_pixel(&mut input.view_rotation.point_start, delta);
    }
    if input.view_rotation.to_rotate {
        shift_controller_rotation_pixel(&mut input.view_rotation.point_to, delta);
    }
}
fn shift_controller_selection_requests(input: &mut AisViewInputBuffer, delta: Vec2<i32>) {
    for point in input.selection.points.iter_mut() {
        shift_controller_pixel(point, delta);
    }
    for zoom in input.zoom_actions.iter_mut() {
        shift_controller_pixel(&mut zoom.point, delta);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkservice::aspect::{ASPECT_VKEY_FLAGS_CTRL, AspectScrollDelta};

    fn subview_translation_controller() -> AisViewController {
        let mut controller = AisViewController::new().unwrap();
        let input = &mut controller.render_input;
        seed_pointer_translation(input);
        seed_rotation_translation(input);
        input.selection.points.append(Vec2::new(30, 40));
        input.zoom_actions.append(AspectScrollDelta::new(
            Vec2::new(30, 40),
            11.0,
            ASPECT_VKEY_FLAGS_CTRL,
        ));
        input
            .zoom_actions
            .append(AspectScrollDelta::with_delta(22.0, 0));
        controller
    }
    fn seed_pointer_translation(input: &mut AisViewInputBuffer) {
        input.move_to.to_hilight = true;
        input.panning.to_start = true;
        input.dragging.to_start = true;
        input.dragging.to_move = true;
        input.move_to.point = Vec2::new(30, 40);
        input.panning.point_start = Vec2::new(30, 40);
        input.dragging.point_start = Vec2::new(30, 40);
        input.dragging.point_to = Vec2::new(30, 40);
    }
    fn seed_rotation_translation(input: &mut AisViewInputBuffer) {
        input.orbit_rotation.to_start = true;
        input.orbit_rotation.to_rotate = true;
        input.view_rotation.to_start = true;
        input.view_rotation.to_rotate = true;
        input.orbit_rotation.point_start = Vec2::new(30.5, 40.25);
        input.orbit_rotation.point_to = Vec2::new(30.5, 40.25);
        input.view_rotation.point_start = Vec2::new(30.5, 40.25);
        input.view_rotation.point_to = Vec2::new(30.5, 40.25);
    }
    fn assert_pointer_translation(input: &AisViewInputBuffer, point: Vec2<i32>) {
        let actual = [
            input.move_to.point,
            input.panning.point_start,
            input.dragging.point_start,
            input.dragging.point_to,
        ];
        assert_eq!(actual, [point; 4]);
        assert_eq!(input.zoom_actions.value(1).unwrap().delta, 11.0);
        assert_eq!(
            input.zoom_actions.value(1).unwrap().flags,
            ASPECT_VKEY_FLAGS_CTRL
        );
    }
    fn assert_rotation_translation(input: &AisViewInputBuffer, point: Vec2<f64>) {
        let actual = [
            input.orbit_rotation.point_start,
            input.orbit_rotation.point_to,
            input.view_rotation.point_start,
            input.view_rotation.point_to,
        ];
        assert_eq!(actual, [point; 4]);
        assert_eq!(input.selection.points.length(), 1);
    }
    #[test]
    fn subview_translation_preserves_fractional_requests_and_zoom_payload() {
        let mut c = subview_translation_controller();
        c.shift_controller_subview_input(Vec2::new(7, 9));
        assert_pointer_translation(&c.render_input, Vec2::new(23, 31));
        assert_rotation_translation(&c.render_input, Vec2::new(23.5, 31.25));
        assert_eq!(
            *c.render_input.selection.points.value(1).unwrap(),
            Vec2::new(23, 31)
        );
        assert_eq!(
            c.render_input.zoom_actions.value(2).unwrap().point,
            Vec2::new(-8, -10)
        );
    }
    fn disable_translation_requests(input: &mut AisViewInputBuffer) {
        input.move_to.to_hilight = false;
        input.panning.to_start = false;
        input.dragging.to_start = false;
        input.dragging.to_move = false;
        input.orbit_rotation.to_start = false;
        input.orbit_rotation.to_rotate = false;
        input.view_rotation.to_start = false;
        input.view_rotation.to_rotate = false;
    }
    #[test]
    fn inactive_requests_stay_local_while_selection_and_scroll_always_shift() {
        let mut c = subview_translation_controller();
        disable_translation_requests(&mut c.render_input);
        c.shift_controller_subview_input(Vec2::new(7, 9));
        assert_pointer_translation(&c.render_input, Vec2::new(30, 40));
        assert_rotation_translation(&c.render_input, Vec2::new(30.5, 40.25));
        assert_eq!(
            *c.render_input.selection.points.value(1).unwrap(),
            Vec2::new(23, 31)
        );
        assert_eq!(
            c.render_input.zoom_actions.value(1).unwrap().point,
            Vec2::new(23, 31)
        );
    }
    #[test]
    fn inverse_subview_translation_restores_owned_request_coordinates() {
        let mut c = subview_translation_controller();
        c.shift_controller_subview_input(Vec2::new(7, 9));
        c.shift_controller_subview_input(Vec2::new(-7, -9));
        assert_pointer_translation(&c.render_input, Vec2::new(30, 40));
        assert_rotation_translation(&c.render_input, Vec2::new(30.5, 40.25));
        assert_eq!(
            c.render_input.zoom_actions.value(2).unwrap().point,
            Vec2::new(-1, -1)
        );
    }
}
