//! Ray-traced rendering of the active view, its environment texture, its gradient background
//! and the direction of a viewer light.
use super::*;
use martensite_tkernel::quantity::Color;
use martensite_tkmath::gp::Direction3;
use martensite_tkservice::aspect::AspectGradientFillMethod;
use martensite_tkservice::graphic3d::{
    Graphic3dNameOfTextureEnv, Graphic3dRenderingMode, Graphic3dTextureEnv,
};

impl ViewerSession {
    /// Ray-trace the active view, with
    /// specular reflections and adaptive anti-aliasing as asked, then redraw.
    /// Example: `session.set_ray_tracing(true, true)?`.
    pub fn set_ray_tracing(&self, reflections: bool, antialiasing: bool) -> Result<(), Failure> {
        let view = self.view().view_base();
        view.change_rendering_params(&mut |params| {
            params.method = Graphic3dRenderingMode::Raytracing;
            params.ray_tracing.is_reflection_enabled = reflections;
            params.ray_tracing.is_antialiasing_enabled = antialiasing;
        });
        self.view().redraw();
        Ok(())
    }

    /// The predefined environment texture `name` on the active view,
    /// then redraw; `session.set_texture_env(Graphic3dNameOfTextureEnv::Sky2)?`.
    pub fn set_texture_env(&self, name: Graphic3dNameOfTextureEnv) -> Result<(), Failure> {
        let texture = Graphic3dTextureEnv::from_name(name)?;
        self.view()
            .view_base()
            .set_texture_env(&Handle::new(texture));
        self.view().redraw();
        Ok(())
    }

    /// A gradient background on the active view, then redraw.
    /// Example: `session.set_gradient_background(top, bottom, AspectGradientFillMethod::Vertical)?`.
    pub fn set_gradient_background(
        &self,
        first: Color,
        second: Color,
        method: AspectGradientFillMethod,
    ) -> Result<(), Failure> {
        let view = self.view().view_base();
        view.set_bg_gradient_colors(first, second, method, false);
        self.view().redraw();
        Ok(())
    }

    /// The direction of active light `index`, then redraw.
    /// Example: `session.set_light_direction(0, [0.577, -0.577, -0.577])?`.
    pub fn set_light_direction(&self, index: usize, [x, y, z]: [f64; 3]) -> Result<(), Failure> {
        let light = self.active_light(index)?;
        let light = light.get().ok_or_else(|| {
            martensite_tkernel::standard::exceptions::program_error(&format!(
                "active light {index} is null; expected a live light"
            ))
        })?;
        light.set_direction(&Direction3::new(x, y, z)?)?;
        self.view().redraw();
        Ok(())
    }
}
