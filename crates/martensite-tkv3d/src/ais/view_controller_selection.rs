//! Selection accumulation retains pending-tool flags across gesture edits.
//! The four public entrypoints are overridable virtuals: they dispatch through the installed
//! shared hooks, whose defaults run the `_base` bodies (witness:
//! the recorded controller_selection_input capture).
use super::{AisSelectionScheme, AisViewController, AisViewSelectionTool};
use martensite_tkernel::ncollection::{Sequence, Vec2};
use martensite_tkernel::standard::Failure;
impl AisViewController {
    /// Virtual `SelectInViewer(point)`: installed hook or base; `controller.select_in_viewer(Vec2::new(2, 3), AisSelectionScheme::Replace)?`.
    pub fn select_in_viewer(
        &mut self,
        point: Vec2<i32>,
        scheme: AisSelectionScheme,
    ) -> Result<(), Failure> {
        if let Some(hooks) = self.dispatch.camera_hooks.clone() {
            return hooks.select_in_viewer(self, point, scheme);
        }
        self.select_in_viewer_base(point, scheme);
        Ok(())
    }
    /// Virtual `SelectInViewer(points)`: installed hook or base; `controller.select_points_in_viewer(&points, AisSelectionScheme::Add)?`.
    pub fn select_points_in_viewer(
        &mut self,
        points: &Sequence<Vec2<i32>>,
        scheme: AisSelectionScheme,
    ) -> Result<(), Failure> {
        if let Some(hooks) = self.dispatch.camera_hooks.clone() {
            return hooks.select_points_in_viewer(self, points, scheme);
        }
        self.select_points_in_viewer_base(points, scheme);
        Ok(())
    }
    /// Virtual `UpdateRubberBand`: installed hook or base; `controller.update_rubber_band(Vec2::new(1, 2), Vec2::new(3, 4))?`.
    pub fn update_rubber_band(
        &mut self,
        first: Vec2<i32>,
        second: Vec2<i32>,
    ) -> Result<(), Failure> {
        if let Some(hooks) = self.dispatch.camera_hooks.clone() {
            return hooks.update_rubber_band(self, first, second);
        }
        self.update_rubber_band_base(first, second);
        Ok(())
    }
    /// Virtual `UpdatePolySelection`: installed hook or base; `controller.update_poly_selection(Vec2::new(2, 3), true)?`.
    pub fn update_poly_selection(&mut self, point: Vec2<i32>, append: bool) -> Result<(), Failure> {
        if let Some(hooks) = self.dispatch.camera_hooks.clone() {
            return hooks.update_poly_selection(self, point, append);
        }
        self.update_poly_selection_base(point, append);
        Ok(())
    }
    /// Queue one pick, clearing points only on a tool transition; `controller.select_in_viewer_base(Vec2::new(2, 3), AisSelectionScheme::Replace)`.
    pub fn select_in_viewer_base(&mut self, point: Vec2<i32>, scheme: AisSelectionScheme) {
        let selection = &mut self.ui_input.selection;
        if selection.tool != AisViewSelectionTool::Picking {
            selection.tool = AisViewSelectionTool::Picking;
            selection.points.clear(None);
        }
        selection.scheme = scheme;
        selection.points.append(point);
    }
    /// Queue an owned snapshot, choosing the tool by length; `controller.select_points_in_viewer_base(&points, AisSelectionScheme::Add)`.
    pub fn select_points_in_viewer_base(
        &mut self,
        points: &Sequence<Vec2<i32>>,
        scheme: AisSelectionScheme,
    ) {
        let selection = &mut self.ui_input.selection;
        selection.scheme = scheme;
        selection.points = points.clone();
        selection.to_apply_tool = true;
        selection.tool = match points.len() {
            1 => AisViewSelectionTool::Picking,
            2 => AisViewSelectionTool::RubberBand,
            _ => AisViewSelectionTool::Polygon,
        };
    }
    /// Replace rectangle corners without applying selection; `controller.update_rubber_band_base(Vec2::new(1, 2), Vec2::new(3, 4))`.
    pub fn update_rubber_band_base(&mut self, first: Vec2<i32>, second: Vec2<i32>) {
        let selection = &mut self.ui_input.selection;
        selection.tool = AisViewSelectionTool::RubberBand;
        selection.points.clear(None);
        selection.points.append(first);
        selection.points.append(second);
    }
    /// Append a distinct vertex or replace the last one; `controller.update_poly_selection_base(Vec2::new(2, 3), true)`.
    pub fn update_poly_selection_base(&mut self, point: Vec2<i32>, append: bool) {
        let selection = &mut self.ui_input.selection;
        if selection.tool != AisViewSelectionTool::Polygon {
            selection.tool = AisViewSelectionTool::Polygon;
            selection.points.clear(None);
        }
        if selection.points.is_empty() || (append && selection.points.last() != Some(&point)) {
            selection.points.append(point);
            return;
        }
        *selection
            .points
            .last_mut()
            .expect("nonempty native polygon") = point;
    }
}
