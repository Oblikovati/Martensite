//! Mirror GlText::setupMatrix, including floor-aligned billboard anchors.
use crate::{RenderError, error::rejected};
use martensite_tkernel::ncollection::{Mat4, Vec3};
use martensite_tkservice::graphic3d::{transform_utils as tr, *};
pub(crate) struct TextTransform {
    pub projection: Mat4<f64>,
    pub model_view: Mat4<f64>,
    pub model_world: Mat4<f64>,
    pub camera_rotation: Mat4<f64>,
    pub viewport: [i32; 4],
    pub pixel_coordinates: bool,
    /// The model scale of zoomable text: `RenderScaleInv` under a render scale, otherwise 1.
    pub zoomable_scale: f64,
}
impl TextTransform {
    pub fn clip(
        &self,
        text: &Graphic3dText,
        aspect: &dyn Graphic3dAspectsSource,
        size: u32,
    ) -> Result<[f32; 16], RenderError> {
        self.clip_offset(text, aspect, size, [0., 0.])
    }
    pub fn clip_offset(
        &self,
        text: &Graphic3dText,
        aspect: &dyn Graphic3dAspectsSource,
        size: u32,
        offset: [f64; 2],
    ) -> Result<[f32; 16], RenderError> {
        if self.pixel_coordinates {
            return Ok(self.pixel_clip(text, aspect, offset));
        }
        let win = self.projected_anchor(text)?;
        let projection = self.glyph_projection(text);
        let scale = if aspect.is_text_zoomable() {
            self.zoomable_scale
        } else {
            self.pixel_scale(win, size)?
        };
        let shifted = [win[0] + offset[0], win[1] + offset[1], win[2]];
        let mut model = self.text_model(text, aspect, shifted, &projection)?;
        tr::scale(&mut model, [scale; 3]);
        if text.has_plane() && !text.has_own_anchor_point() {
            model = self.model_view * model;
        }
        Ok(gpu_clip(&projection, &model))
    }
    fn projected_anchor(&self, text: &Graphic3dText) -> Result<[f64; 3], RenderError> {
        let p = text.position();
        tr::project(
            [p.x(), p.y(), p.z()],
            &self.model_view,
            &self.projection,
            self.viewport,
        )
        .ok_or_else(|| rejected(p, "a projectable text anchor"))
    }
    fn glyph_projection(&self, text: &Graphic3dText) -> Mat4<f64> {
        if text.has_plane() && text.has_own_anchor_point() {
            return self.projection * self.camera_rotation;
        }
        self.projection
    }
    fn pixel_scale(&self, win: [f64; 3], size: u32) -> Result<f64, RenderError> {
        let first = self.unproject(win, &self.projection)?;
        let second =
            self.unproject([win[0], win[1] + f64::from(size), win[2]], &self.projection)?;
        Ok((second[1] - first[1]) / f64::from(size))
    }
    fn unproject(&self, win: [f64; 3], projection: &Mat4<f64>) -> Result<[f64; 3], RenderError> {
        tr::un_project(win, &Mat4::identity(), projection, self.viewport)
            .ok_or_else(|| rejected(win, "an invertible text projection"))
    }
    fn text_model(
        &self,
        text: &Graphic3dText,
        aspect: &dyn Graphic3dAspectsSource,
        mut win: [f64; 3],
        projection: &Mat4<f64>,
    ) -> Result<Mat4<f64>, RenderError> {
        if !text.has_plane() && !aspect.is_text_zoomable() {
            win[0] = win[0].floor();
            win[1] = win[1].floor();
        }
        let anchor = self.unproject(win, projection)?;
        if text.has_plane() {
            return Ok(plane_model(text, anchor));
        }
        let mut model = Mat4::identity();
        tr::translate(&mut model, anchor);
        tr::rotate(&mut model, f64::from(aspect.text_angle()), [0., 0., 1.]);
        Ok(model)
    }
}
/// The clip transform the native font program applies: `setupMatrix` stores the double
/// projection and world-view in the float ProjectionState and WorldViewState,
/// `ApplyWorldViewMatrix` resets the model-world to identity, and the vertex shader multiplies
/// the float matrices (`occProjectionMatrix * occWorldViewMatrix * occModelWorldMatrix`).
/// Rounding the double product instead moved glyph depths across a line end at the same depth
/// (bug22368, glyph under its axis line). Example: `gpu_clip(&projection, &world_view)`.
pub(crate) fn gpu_clip(projection: &Mat4<f64>, world_view: &Mat4<f64>) -> [f32; 16] {
    (Mat4::<f32>::from_other(projection) * Mat4::<f32>::from_other(world_view)).data()
}
fn plane_model(text: &Graphic3dText, anchor: [f64; 3]) -> Mat4<f64> {
    let orientation = text.orientation();
    let mut model = Mat4::identity();
    for (i, dir) in [
        orientation.x_direction(),
        orientation.y_direction(),
        orientation.direction(),
    ]
    .into_iter()
    .enumerate()
    {
        model.set_column3(i, Vec3::new(dir.x(), dir.y(), dir.z()));
    }
    let p = text.position();
    let anchor = if text.has_own_anchor_point() {
        anchor
    } else {
        [p.x(), p.y(), p.z()]
    };
    model.set_column3(3, Vec3::new(anchor[0], anchor[1], anchor[2]));
    model
}

#[cfg(test)]
#[path = "text_transform_tests.rs"]
mod tests;

#[path = "text_pass_transform.rs"]
mod pass_transform;

#[path = "text_pixel_transform.rs"]
mod pixel_transform;
