//! Stereo redraws GlView::Redraw (Projection_Stereo): the left and right
//! eye scenes rendered offscreen (myMainSceneFbos) with the camera's ProjectionStereoLeftF and
//! ProjectionStereoRightF, then composed over the target by drawStereoPair.
use super::Graphic3dRenderView;
use crate::stereo_pair::StereoPairComposite;
use crate::surface_texture::SurfaceTextureImage;
use crate::{FrameCapture, RenderError, RenderFrame};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::Graphic3dCViewMembers as _;
use martensite_tkservice::graphic3d::{
    Graphic3dCView as _, Graphic3dCamera, Graphic3dCameraProjection, Graphic3dWorldViewProjState,
};
use std::sync::Arc;

/// Draws the camera with one eye's projection and restores its projection type when dropped.
struct EyeProjection {
    camera: Handle<Graphic3dCamera>,
    restored: Graphic3dCameraProjection,
}

impl EyeProjection {
    fn select(camera: &Handle<Graphic3dCamera>, eye: Graphic3dCameraProjection) -> Self {
        let restored = camera.projection_type();
        camera.set_projection_type(eye);
        Self {
            camera: camera.clone(),
            restored,
        }
    }
}

impl Drop for EyeProjection {
    fn drop(&mut self) {
        self.camera.set_projection_type(self.restored);
    }
}

/// Marks one eye of a stereo Redraw in the view's path-tracing accumulation until dropped.
struct StereoEye<'a> {
    view: &'a Graphic3dRenderView,
}

impl<'a> StereoEye<'a> {
    fn enter(view: &'a Graphic3dRenderView, eye: (usize, Graphic3dWorldViewProjState)) -> Self {
        view.raytrace_accumulation.write().set_stereo_eye(Some(eye));
        Self { view }
    }
}

impl Drop for StereoEye<'_> {
    fn drop(&mut self) {
        self.view.raytrace_accumulation.write().set_stereo_eye(None);
    }
}

impl Graphic3dRenderView {
    /// The frame the target draws and the scaled scene capture it blits, if any: the scene for
    /// a mono camera, the composed eye pair for a stereo one.
    pub(super) fn composed_scene(
        &self,
        extent: [u32; 2],
    ) -> Result<(RenderFrame, Option<FrameCapture>), RenderError> {
        let camera = crate::view_render_camera::owned_render_camera(self);
        if !camera.is_stereo() {
            let frame = self.fed_scene_frame(extent)?;
            return self.blit_scaled_scene(frame, extent);
        }
        // The eye projections change the camera's counters; native draws them without.
        let state = camera.world_view_proj_state();
        let (left, clear, flip) = self.eye_image(
            (&camera, state),
            Graphic3dCameraProjection::MonoLeftEye,
            extent,
        )?;
        let (right, _, _) = self.eye_image(
            (&camera, state),
            Graphic3dCameraProjection::MonoRightEye,
            extent,
        )?;
        self.raytrace_accumulation
            .write()
            .settle_camera(camera.world_view_proj_state());
        let params = self.rendering_params();
        let pair =
            StereoPairComposite::new(&params.stereo, [left, right], self.window().position())
                .into_target(extent, flip);
        let mut frame = RenderFrame::with_layers(extent[0], extent[1], clear, Vec::new())?;
        frame.stereo_pair = Some(Arc::new(pair));
        frame.flip_output = flip;
        Ok((frame, None))
    }

    /// One eye's scene rendered offscreen (at the render scale, as myMainSceneFbos), with the
    /// frame's clear colour and output flip.
    fn eye_image(
        &self,
        (camera, state): (&Handle<Graphic3dCamera>, Graphic3dWorldViewProjState),
        eye: Graphic3dCameraProjection,
        extent: [u32; 2],
    ) -> Result<(Arc<SurfaceTextureImage>, [f32; 4], bool), RenderError> {
        let _projection = EyeProjection::select(camera, eye);
        // Both eyes path trace in one Redraw: one frame count, one seed sequence.
        let _eye = StereoEye::enter(self, (crate::raytrace::eye_targets(eye), state));
        let mut frame = self.fed_scene_frame(extent)?;
        let flip = std::mem::take(&mut frame.flip_output);
        let capture = self.device.write().render(&frame)?;
        let image = crate::render_scale::capture_image(&capture);
        Ok((Arc::new(image), frame.clear_color, flip))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eye_projection_restores_the_stereo_camera() {
        let camera = Handle::new(Graphic3dCamera::default());
        camera.set_projection_type(Graphic3dCameraProjection::Stereo);
        {
            let _left = EyeProjection::select(&camera, Graphic3dCameraProjection::MonoLeftEye);
            assert_eq!(
                camera.projection_type(),
                Graphic3dCameraProjection::MonoLeftEye
            );
            assert!(!camera.is_stereo());
        }
        assert!(camera.is_stereo());
    }
}
