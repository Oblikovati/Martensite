//! Context actions on the current selection: highlighting it on detection, fitting the view
//! to it and erasing it.
use super::*;

impl ViewerSession {
    /// Clear selected owners before a reversible topology control.
    /// Example: `session.clear_selected_for_control()?`.
    pub(crate) fn clear_selected_for_control(&self) -> Result<(), Failure> {
        self.with_update(|context| context.clear_selected(false))
    }

    /// Detection also highlights selected objects; the detected
    /// owner is cleared on a change, without a redraw. Example: `session.set_highlight_selected(true)`.
    pub fn set_highlight_selected(&self, enabled: bool) {
        if enabled != self.context.to_hilight_selected() {
            self.context.clear_detected(false);
            self.context.set_to_hilight_selected(enabled);
        }
    }

    /// Fit the view to the selection with a 1% margin, then redraw.
    /// Example: `session.fit_selected()?`.
    pub fn fit_selected(&self) -> Result<(), Failure> {
        let view = self.view();
        with_source_view_update(
            &**view,
            || self.context.fit_selected(view, 0.01, false),
            || view.redraw(),
        )
    }

    /// Erase the selected objects, then update the viewer.
    /// Example: `session.erase_selected()?`.
    pub fn erase_selected(&self) -> Result<(), Failure> {
        self.with_update(|context| context.erase_selected(false))
    }
}

impl ViewerSession {
    /// The display mode of the selection highlight; -1 highlights in the object's own mode.
    /// The change applies to the selection highlight style.
    /// Example: `session.set_selection_display_mode(-1)?`.
    pub fn set_selection_display_mode(&self, mode: i32) -> Result<(), Failure> {
        use martensite_tkservice::graphic3d::Graphic3dPresentationAttributesMembers as _;
        let selected = martensite_tkv3d::prs3d::Prs3dTypeOfHighlight::Selected;
        let style = (*self.context.highlight_style(selected)?).clone();
        let style = style.get().ok_or_else(|| {
            martensite_tkernel::standard::exceptions::program_error(
                "selection style; expected a drawer",
            )
        })?;
        style.set_display_mode(mode);
        Ok(())
    }
}
