//! Mirror GlText::setupMatrix, including floor-aligned billboard anchors.
use crate::{RenderError, error::rejected};
use martensite_tkernel::ncollection::{Mat4, Vec3};
use martensite_tkservice::graphic3d::{transform_utils as tr, *};
/// Where a text sits, read once: a label read its text's lock for these up to six times
/// on every frame, 7% of `frame_text_1k` (#594). A label record keeps it with the text
/// revision. `LabelPlacement::of(&text).has_plane()`
#[derive(Clone, Copy, Debug)]
pub(crate) struct LabelPlacement {
    orientation: martensite_tkmath::gp::Axis2,
    has_plane: bool,
    has_own_anchor: bool,
}
impl From<&Graphic3dText> for LabelPlacement {
    fn from(text: &Graphic3dText) -> Self {
        Self::of(text)
    }
}
impl From<&LabelPlacement> for LabelPlacement {
    fn from(placement: &LabelPlacement) -> Self {
        *placement
    }
}
impl LabelPlacement {
    /// The placement of `text` now.
    pub(crate) fn of(text: &Graphic3dText) -> Self {
        Self {
            orientation: text.orientation(),
            has_plane: text.has_plane(),
            has_own_anchor: text.has_own_anchor_point(),
        }
    }
    pub(crate) fn orientation(&self) -> martensite_tkmath::gp::Axis2 {
        self.orientation
    }
    pub(crate) fn position(&self) -> martensite_tkmath::gp::Point3 {
        self.orientation.location()
    }
    pub(crate) fn has_plane(&self) -> bool {
        self.has_plane
    }
    pub(crate) fn has_own_anchor_point(&self) -> bool {
        self.has_own_anchor
    }
}
#[derive(Clone)]
pub(crate) struct TextTransform {
    pub projection: Mat4<f64>,
    pub model_view: Mat4<f64>,
    pub model_world: Mat4<f64>,
    pub camera_rotation: Mat4<f64>,
    pub viewport: [i32; 4],
    pub pixel_coordinates: bool,
    /// The model scale of zoomable text: `RenderScaleInv` under a render scale, otherwise 1.
    pub zoomable_scale: f64,
    /// `projection` inverted on first use; each label unprojects up to three points through
    /// it, and inverting it for each point took 18% of `frame_text_1k` (#594).
    pub unview: std::sync::OnceLock<Option<Mat4<f64>>>,
}
impl TextTransform {
    pub fn clip(
        &self,
        text: impl Into<LabelPlacement>,
        aspect: &dyn Graphic3dAspectsSource,
        size: u32,
    ) -> Result<[f32; 16], RenderError> {
        self.clip_offset(text, aspect, size, [0., 0.])
    }
    pub fn clip_offset(
        &self,
        text: impl Into<LabelPlacement>,
        aspect: &dyn Graphic3dAspectsSource,
        size: u32,
        offset: [f64; 2],
    ) -> Result<[f32; 16], RenderError> {
        let placement = text.into();
        let text = &placement;
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
    fn projected_anchor(&self, text: &LabelPlacement) -> Result<[f64; 3], RenderError> {
        let p = text.position();
        tr::project(
            [p.x(), p.y(), p.z()],
            &self.model_view,
            &self.projection,
            self.viewport,
        )
        .ok_or_else(|| rejected(p, "a projectable text anchor"))
    }
    fn glyph_projection(&self, text: &LabelPlacement) -> Mat4<f64> {
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
        let point = if *projection == self.projection {
            self.projection_unview()
                .and_then(|unview| tr::un_project_inverted(win, &unview, self.viewport))
        } else {
            tr::un_project(win, &Mat4::identity(), projection, self.viewport)
        };
        point.ok_or_else(|| rejected(win, "an invertible text projection"))
    }
    /// The inverse `tr::un_project` computes for `projection`, computed once.
    fn projection_unview(&self) -> Option<Mat4<f64>> {
        *self
            .unview
            .get_or_init(|| invert_projection(&self.projection))
    }
    fn text_model(
        &self,
        text: &LabelPlacement,
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
/// The inverse `tr::un_project` computes for `projection`.
fn invert_projection(projection: &Mat4<f64>) -> Option<Mat4<f64>> {
    let mut unview = Mat4::identity();
    (*projection * Mat4::identity())
        .inverted_to(&mut unview)
        .then_some(unview)
}

/// The text transform of one group lowering with the model and view overrides it was built
/// for. Each label of a group asked for it, and building it took 19% of a text frame (#594).
pub(crate) type GroupTextTransform =
    std::cell::RefCell<Option<((Option<[u32; 16]>, Option<[u32; 16]>), TextTransform)>>;

/// The inverse of the last text projection of a pass. The labels of a pass share the camera
/// projection, and inverting it once per label took 10% of `frame_text_1k` (#594).
#[derive(Debug, Default)]
pub(crate) struct SharedTextUnview(std::sync::Mutex<Option<KeptUnview>>);

/// A projection's bits with its inverse, `None` when it has none.
type KeptUnview = ([u64; 16], Option<Mat4<f64>>);

impl SharedTextUnview {
    /// The inverse of `projection`, computed again only for another projection;
    /// `shared.inverse(&projection)`.
    pub(crate) fn inverse(&self, projection: &Mat4<f64>) -> Option<Mat4<f64>> {
        let bits = projection.data().map(f64::to_bits);
        if let Ok(slot) = self.0.lock()
            && let Some((kept, unview)) = slot.as_ref()
            && *kept == bits
        {
            return *unview;
        }
        let unview = invert_projection(projection);
        if let Ok(mut slot) = self.0.lock() {
            *slot = Some((bits, unview));
        }
        unview
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
fn plane_model(text: &LabelPlacement, anchor: [f64; 3]) -> Mat4<f64> {
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
