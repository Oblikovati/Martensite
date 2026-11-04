//! View-wide rendering state: background, shading model, transparency method, frustum
//! culling and flipped output.
use super::*;
use martensite_tkernel::quantity::Color;
use martensite_tkservice::graphic3d::{
    Graphic3dRenderTransparentMethod, Graphic3dStereoMode, Graphic3dTypeOfShadingModel,
};

impl ViewerSession {
    /// The background color of the active view; `session.set_background_color(grey)?`.
    pub fn set_background_color(&self, color: Color) -> Result<(), Failure> {
        let view = self.view().get().ok_or_else(null_view)?;
        view.set_background_color(color);
        self.view().redraw();
        Ok(())
    }

    /// The shading model of the active view.
    /// Example: `session.set_shading_model(Graphic3dTypeOfShadingModel::Phong)?`.
    pub fn set_shading_model(&self, model: Graphic3dTypeOfShadingModel) -> Result<(), Failure> {
        let view = self.view().get().ok_or_else(null_view)?;
        view.change_rendering_params(&mut |params| params.shading_model = model);
        self.view().redraw();
        Ok(())
    }

    /// The transparency method of the active view, with the
    /// weighted method's depth factor; `session.set_transparency_method(BlendOit, Some(0.))?`.
    pub fn set_transparency_method(
        &self,
        method: Graphic3dRenderTransparentMethod,
        depth_factor: Option<f32>,
    ) -> Result<(), Failure> {
        let view = self.view().get().ok_or_else(null_view)?;
        view.change_rendering_params(&mut |params| {
            params.transparency_method = method;
            if let Some(factor) = depth_factor {
                params.oit_depth_factor = factor;
            }
        });
        self.view().redraw();
        Ok(())
    }

    /// The active view renders off-screen at `scale` times
    /// its size and blits the result back, then redraws; `session.set_render_scale(2.)?`.
    pub fn set_render_scale(&self, scale: f32) -> Result<(), Failure> {
        let view = self.view().get().ok_or_else(null_view)?;
        view.change_rendering_params(&mut |params| params.render_resolution_scale = scale);
        self.view().redraw();
        Ok(())
    }

    /// Frustum culling on the active view, then redraw.
    /// Example: `session.set_frustum_culling(false)?`.
    pub fn set_frustum_culling(&self, enabled: bool) -> Result<(), Failure> {
        self.view().view_base().set_frustum_culling(enabled);
        self.view().redraw();
        Ok(())
    }

    /// The active view's stereo mode, with a stereo camera projection, then
    /// redraw; `session.set_stereo_mode(Graphic3dStereoMode::Anaglyph)?`.
    pub fn set_stereo_mode(&self, mode: Graphic3dStereoMode) -> Result<(), Failure> {
        let view = self.view().get().ok_or_else(null_view)?;
        view.change_rendering_params(&mut |params| params.stereo.stereo_mode = mode);
        self.view().camera().set_projection_type(
            martensite_tkservice::graphic3d::Graphic3dCameraProjection::Stereo,
        );
        self.view().redraw();
        Ok(())
    }

    /// The stereo projection the next image dumps render, until set
    /// again; `session.set_stereo_dump(V3dStereoDumpOptions::Blended)`.
    pub fn set_stereo_dump(&self, option: V3dStereoDumpOptions) {
        self.stereo_dump.set(option);
    }

    /// Read the current dump selection without changing the view; `session.stereo_dump_option()`.
    pub fn stereo_dump_option(&self) -> V3dStereoDumpOptions {
        self.stereo_dump.get()
    }

    /// Flip the active view's output vertically, then redraw.
    /// Example: `session.set_flip_output(true)?`.
    pub fn set_flip_output(&self, flip: bool) -> Result<(), Failure> {
        let cview = self.view().view_base().view();
        cview.get().ok_or_else(null_view)?.set_to_flip_output(flip);
        self.view().redraw();
        Ok(())
    }
}

fn null_view() -> Failure {
    martensite_tkernel::standard::exceptions::program_error(
        "a null active view; expected a live V3d view",
    )
}
