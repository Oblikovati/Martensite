//! The view-space shader grid: the capability of GlShaderGrid. The CPU part keeps the
//! displayed parameters, the plane frame, Z-fit bounds, echo/snap and the shader block; the
//! fragment shader (shaders/grid.frag) intersects each pixel's ray with the grid plane.
mod bounds;
mod echo;
mod uniforms;
pub use echo::{RenderGridEcho, RenderGridPixel};
use martensite_tkernel::ncollection::Mat4;
use martensite_tkmath::gp::{Axis3, Point3, Xyz};
use martensite_tkservice::aspect::{AspectGridDrawMode, AspectGridParams};
use martensite_tkservice::graphic3d::Graphic3dCamera;
pub use uniforms::RenderShaderGridBlock;

const TWO_PI: f64 = 2.0 * std::f64::consts::PI;

/// The shader grid of a view (GlShaderGrid): displayed parameters and plane, Z-fit
/// bounds, echo/snap and the shader block; `render_view.shader_grid.is_drawn`.
#[derive(Clone, Debug)]
pub struct RenderShaderGrid {
    params: AspectGridParams,
    plane: Axis3,
    reference_view: Mat4<f32>,
    /// The world-view of the last frame: native `WorldViewState.Current()` between frames.
    last_world_view: Mat4<f32>,
    is_shown: bool,
}

impl Default for RenderShaderGrid {
    fn default() -> Self {
        Self {
            params: AspectGridParams::default(),
            plane: Axis3::default(),
            reference_view: Mat4::identity(),
            last_world_view: Mat4::identity(),
            is_shown: false,
        }
    }
}

/// The rotated grid frame: origin, in-plane axes and normal (`frame` of the native).
#[derive(Clone, Copy, Debug)]
pub(crate) struct GridFrame {
    pub origin: Xyz,
    pub x: Xyz,
    pub y: Xyz,
    pub normal: Xyz,
}

impl RenderShaderGrid {
    /// `IsShown`; `grid.is_shown()`.
    pub fn is_shown(&self) -> bool {
        self.is_shown
    }
    /// `Params`; `grid.params().draw_mode()`.
    pub fn params(&self) -> &AspectGridParams {
        &self.params
    }
    /// `IsBackground`; `grid.is_background()`.
    pub fn is_background(&self) -> bool {
        self.params.is_background()
    }
    /// Whether a frame draws the grid (`renderGrid` early exits).
    pub fn is_drawn(&self) -> bool {
        self.is_shown && self.params.draw_mode() != AspectGridDrawMode::None
    }
    /// Keep the world-view a frame applied; `grid.observe_world_view(camera.orientation_matrix_f())`.
    pub fn observe_world_view(&mut self, world_view: Mat4<f32>) {
        self.last_world_view = world_view;
    }
    /// `Display`: keep the parameters; view-adaptive steps keep the current camera scale and
    /// background grids capture the last applied world-view state.
    pub fn display(&mut self, params: &AspectGridParams, plane: &Axis3, camera: &Graphic3dCamera) {
        if params.draw_mode() == AspectGridDrawMode::None {
            self.erase();
            return;
        }
        let was_background = self.is_shown && self.params.is_background();
        let same_anchor = self.is_shown && self.has_same_anchor_frame(params, plane);
        let capture = params.is_background() && (!self.is_shown || !was_background || !same_anchor);
        self.params = *params;
        self.plane = *plane;
        self.is_shown = true;
        if params.is_view_adaptive() {
            self.adapt_scale(params, camera);
        }
        if capture {
            self.reference_view = self.last_world_view;
        }
    }
    fn adapt_scale(&mut self, params: &AspectGridParams, camera: &Graphic3dCamera) {
        let reference = if camera.scale() > martensite_tkernel::precision::confusion() {
            camera.scale()
        } else {
            1.0
        };
        // The setters only refuse non-positive values, which the products keep positive.
        let _ = self.params.set_scale(params.scale() * reference);
        if params.scale_y() > 0.0 {
            let _ = self.params.set_scale_y(params.scale_y() * reference);
        }
    }
    /// `Erase`; `grid.erase()`.
    pub fn erase(&mut self) {
        self.is_shown = false;
    }
    /// The plane frame rotated by the grid angle, offset by origin and Z offset.
    pub(crate) fn frame(&self) -> GridFrame {
        let (sin, cos) = martensite_cmath::sincos(self.params.rotation_angle());
        let raw_x = self.plane.x_direction().xyz();
        let raw_y = self.plane.y_direction().xyz();
        let normal = self.plane.direction().xyz();
        let local = self.params.origin();
        let origin = self.plane.location().xyz()
            + raw_x * local.x()
            + raw_y * local.y()
            + normal * (local.z() + self.params.z_offset());
        GridFrame {
            origin,
            x: raw_x * cos - raw_y * sin,
            y: raw_x * sin + raw_y * cos,
            normal,
        }
    }
    /// Step frequencies; view-adaptive grids divide by the current camera scale.
    pub fn effective_scale(&self, camera: &Graphic3dCamera) -> (f64, f64) {
        let (x, y) = (self.params.scale(), self.params.effective_scale_y());
        if !self.params.is_view_adaptive() {
            return (x, y);
        }
        let current = camera
            .scale()
            .max(martensite_tkernel::precision::confusion());
        (x / current, y / current)
    }
    /// `DrawWorldView`: background grids stay where they were displayed on screen.
    pub fn draw_world_view(&self, current: &Mat4<f32>) -> Mat4<f32> {
        if !self.params.is_background() {
            return *current;
        }
        let mut inverse = Mat4::<f32>::identity();
        if !self.reference_view.inverted_to(&mut inverse) {
            inverse = Mat4::identity();
        }
        current.multiplied(&inverse)
    }
    fn has_same_anchor_frame(&self, params: &AspectGridParams, plane: &Axis3) -> bool {
        use martensite_tkernel::precision::{angular, confusion, square_confusion};
        let same_direction = |a: martensite_tkmath::gp::Direction3, b| a.angle(&b) <= angular();
        self.plane.location().square_distance(&plane.location()) <= square_confusion()
            && same_direction(self.plane.direction(), plane.direction())
            && same_direction(self.plane.x_direction(), plane.x_direction())
            && same_direction(self.plane.y_direction(), plane.y_direction())
            && self.params.origin().square_distance(&params.origin()) <= square_confusion()
            && same_angle(self.params.rotation_angle(), params.rotation_angle())
            && (self.params.z_offset() - params.z_offset()).abs() <= confusion()
    }
}

fn normalized_angle(angle: f64) -> f64 {
    let wrapped = angle % TWO_PI;
    if wrapped < 0.0 {
        wrapped + TWO_PI
    } else {
        wrapped
    }
}

fn same_angle(first: f64, second: f64) -> bool {
    let delta = (normalized_angle(first) - normalized_angle(second)).abs();
    delta.min(TWO_PI - delta) <= martensite_tkernel::precision::angular()
}

/// The arc span from start to end in [0, 2π]; a full turn stays 2π.
pub(crate) fn positive_angle_span(start: f64, end: f64) -> f64 {
    use martensite_tkernel::precision::angular;
    let span = normalized_angle(end - start);
    if span.abs() <= angular() && (end - start).abs() >= TWO_PI - angular() {
        return TWO_PI;
    }
    span
}

/// `angleInArc`: whether `angle` lies in the counter-clockwise arc from start to end.
pub(crate) fn angle_in_arc(start: f64, end: f64, angle: f64) -> bool {
    use martensite_tkernel::precision::angular;
    let span = positive_angle_span(start, end);
    if span >= TWO_PI - angular() {
        return true;
    }
    let delta = normalized_angle(normalized_angle(angle) - normalized_angle(start));
    delta <= span + angular()
}

/// The point of `frame` at local coordinates; `local_point(&frame, 1.0, 0.0)`.
pub(crate) fn local_point(frame: &GridFrame, x: f64, y: f64) -> Xyz {
    frame.origin + frame.x * x + frame.y * y
}

fn is_finite_coord(value: f64) -> bool {
    value.is_finite() && !martensite_tkernel::precision::is_infinite(value)
}

pub(crate) fn is_finite_point(point: Xyz) -> bool {
    is_finite_coord(point.x()) && is_finite_coord(point.y()) && is_finite_coord(point.z())
}

pub(crate) fn to_point(xyz: Xyz) -> Point3 {
    Point3::new(xyz.x(), xyz.y(), xyz.z())
}

#[cfg(test)]
#[path = "../shader_grid_tests.rs"]
mod tests;
