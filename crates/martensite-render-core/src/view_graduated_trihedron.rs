//! The GlView graduated trihedron members: `myGraduatedTrihedron`
//! (`ChangeGraduatedTrihedron`), `myToShowGradTrihedron` and `GraduatedTrihedronDisplay`,
//! `GraduatedTrihedronErase`, `GraduatedTrihedronMinMaxValues`.
use crate::{Graphic3dRenderView, RenderGraduatedTrihedron};
use martensite_tkernel::ncollection::Vec3;
use martensite_tkservice::graphic3d::{Graphic3dCView, Graphic3dGraduatedTrihedron};

impl Graphic3dRenderView {
    /// `ChangeGraduatedTrihedron`: the view's trihedron element.
    /// Example: `render_view.graduated_trihedron().set_min_max(min, max)`.
    pub fn graduated_trihedron(&self) -> &RenderGraduatedTrihedron {
        &self.retained.graduated_trihedron
    }
    /// `myToShowGradTrihedron`: whether the next frames draw the trihedron.
    /// Example: `render_view.is_graduated_trihedron_shown()`.
    pub fn is_graduated_trihedron_shown(&self) -> bool {
        *self.retained.graduated_trihedron_shown.read()
    }
    /// `GraduatedTrihedronDisplay`: stores the parameters with the scene-bounds callback as
    /// `myGTrihedronData`, hands them to the element and shows it.
    pub(crate) fn display_graduated_trihedron(&self, data: &Graphic3dGraduatedTrihedron) {
        let mut stored = data.clone();
        stored.cubic_axes_callback = Some(crate::graduated_trihedron::min_max_values_callback);
        self.retained.graduated_trihedron.set_values(&stored);
        self.cview_base().fields.write().g_trihedron_data = stored;
        *self.retained.graduated_trihedron_shown.write() = true;
    }
    /// `GraduatedTrihedronErase`: releases the element and hides it.
    pub(crate) fn erase_graduated_trihedron(&self) {
        self.retained.graduated_trihedron.release();
        *self.retained.graduated_trihedron_shown.write() = false;
    }
    /// `GraduatedTrihedronMinMaxValues`: the element's next bounds.
    pub(crate) fn graduated_trihedron_bounds(&self, min: Vec3<f32>, max: Vec3<f32>) {
        self.retained.graduated_trihedron.set_min_max(min, max);
    }
    /// The trihedron the frame draws: the element while shown (`renderTrihedron`).
    pub(crate) fn shown_graduated_trihedron(&self) -> Option<&RenderGraduatedTrihedron> {
        self.is_graduated_trihedron_shown()
            .then_some(&self.retained.graduated_trihedron)
    }
}
