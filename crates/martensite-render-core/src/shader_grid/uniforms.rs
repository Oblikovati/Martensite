//! GlShaderGrid::SetUniforms as one std430 block (shaders/grid.frag `GridParams`).
use super::RenderShaderGrid;
use martensite_tkernel::ncollection::{Mat4, Vec4};
use martensite_tkmath::gp::Xyz;
use martensite_tkservice::graphic3d::Graphic3dCamera;

/// The grid shader block: two matrices, ten vec4 rows and two ivec4 flag rows.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderShaderGridBlock {
    /// `occProjectionMatrix`, column-major float.
    pub projection: [f32; 16],
    /// `occProjectionMatrixInverse`, inverted in float like GlProjectionState.
    pub projection_inverse: [f32; 16],
    /// Colors, plane frame, scales, bounds, phase shifts and arc (see shaders/grid.frag).
    pub rows: [[f32; 4]; 10],
    /// Axis/type/mode/perspective, then depth/background/fade/arc flags.
    pub flags: [[i32; 4]; 2],
}

impl RenderShaderGridBlock {
    /// Byte image in std430 order; `uniforms.bytes().len() == 336`.
    pub fn bytes(&self) -> Vec<u8> {
        let floats = self
            .projection
            .iter()
            .chain(&self.projection_inverse)
            .chain(self.rows.iter().flatten());
        let mut bytes: Vec<u8> = floats.flat_map(|value| value.to_ne_bytes()).collect();
        bytes.extend(
            self.flags
                .iter()
                .flatten()
                .flat_map(|value| value.to_ne_bytes()),
        );
        bytes
    }
}

/// The plane origin and reference shifted to the visible grid phase.
struct GridPhase {
    local: [f32; 2],
    accent_local: [f32; 2],
    radial: f32,
    accent_radial: f32,
}

impl RenderShaderGrid {
    /// The block for this frame; `world_view` is the camera orientation of the frame.
    pub fn uniforms(
        &self,
        camera: &Graphic3dCamera,
        world_view: &Mat4<f32>,
    ) -> RenderShaderGridBlock {
        let world_view = self.draw_world_view(world_view);
        let projection = camera.projection_matrix_f();
        let mut inverse = Mat4::<f32>::identity();
        // GlProjectionState inverts the float matrix; a singular one keeps identity.
        let _ = projection.inverted_to(&mut inverse);
        let scale = self.effective_scale(camera);
        let phase = self.grid_phase(camera, scale);
        RenderShaderGridBlock {
            projection: projection.data(),
            projection_inverse: inverse.data(),
            rows: self.uniform_rows(&world_view, scale, &phase),
            flags: self.uniform_flags(camera),
        }
    }
    fn uniform_rows(
        &self,
        world_view: &Mat4<f32>,
        (scale_x, scale_y): (f64, f64),
        phase: &GridPhase,
    ) -> [[f32; 4]; 10] {
        let params = &self.params;
        let frame = self.frame();
        let reference = frame.origin
            + frame.x * f64::from(phase.local[0])
            + frame.y * f64::from(phase.local[1]);
        let rgb = |color: martensite_tkernel::quantity::Color| {
            [
                color.red() as f32,
                color.green() as f32,
                color.blue() as f32,
            ]
        };
        let [r, g, b] = rgb(params.color());
        let [ar, ag, ab] = rgb(params.accent_color());
        let point = |xyz: Xyz, w: f64| with_w(view_vector(world_view, xyz, 1.0), w);
        let direction = |xyz: Xyz, w: f64| with_w(view_vector(world_view, xyz, 0.0), w);
        let half = |size: f64| if size > 0.0 { (size * 0.5) as f32 } else { 0.0 };
        let radius = if params.radius() > 0.0 {
            params.radius() as f32
        } else {
            0.0
        };
        [
            [r, g, b, params.line_thickness() as f32],
            [ar, ag, ab, params.accent_angular_scale() as f32],
            point(frame.origin, scale_x),
            point(reference, scale_y),
            direction(frame.x, self.angular_scale()),
            direction(frame.y, params.accent_scale_x()),
            direction(frame.normal, params.accent_scale_y()),
            [
                half(params.size_x()),
                half(params.size_y()),
                radius,
                angular_tolerance(),
            ],
            [
                phase.local[0],
                phase.local[1],
                phase.accent_local[0],
                phase.accent_local[1],
            ],
            [
                phase.radial,
                phase.accent_radial,
                params.angle_start() as f32,
                params.angle_end() as f32,
            ],
        ]
    }
    fn angular_scale(&self) -> f64 {
        if self.params.is_circular() {
            f64::from(self.params.angular_divisions()) / std::f64::consts::PI
        } else {
            0.0
        }
    }
    fn uniform_flags(&self, camera: &Graphic3dCamera) -> [[i32; 4]; 2] {
        let params = &self.params;
        let flag = |value: bool| i32::from(value);
        let points = params.draw_mode() == martensite_tkservice::aspect::AspectGridDrawMode::Points;
        [
            [
                flag(params.is_draw_axis()),
                flag(params.is_circular()),
                flag(points),
                flag(!camera.is_orthographic()),
            ],
            [
                flag(camera.is_zero_to_one_depth()),
                flag(params.is_background()),
                flag(params.is_bounded() && !params.is_view_adaptive()),
                flag(params.is_arc()),
            ],
        ]
    }
    /// Snap the visible reference point to the grid phase (keeps float line phases stable).
    fn grid_phase(&self, camera: &Graphic3dCamera, (scale_x, scale_y): (f64, f64)) -> GridPhase {
        let mut phase = GridPhase {
            local: [0.0; 2],
            accent_local: [0.0; 2],
            radial: 0.0,
            accent_radial: 0.0,
        };
        let Some([x, y]) = self.reference_local(camera) else {
            return phase;
        };
        let accent = |scale: f64, value: f64, fallback: f32| {
            if scale > martensite_tkernel::precision::confusion() {
                snapped_shift(value, scale) as f32
            } else {
                fallback
            }
        };
        let params = &self.params;
        if params.is_circular() {
            let radius = (x * x + y * y).sqrt();
            phase.radial = snapped_shift(radius, scale_x) as f32;
            phase.accent_radial = accent(params.accent_scale_x(), radius, phase.radial);
            return phase;
        }
        phase.local = [
            snapped_shift(x, scale_x) as f32,
            snapped_shift(y, scale_y) as f32,
        ];
        phase.accent_local = [
            accent(params.accent_scale_x(), x, phase.local[0]),
            accent(params.accent_scale_y(), y, phase.local[1]),
        ];
        phase
    }
    /// `referenceLocal`: the first NDC sample (center, corners, edges) that hits the plane.
    fn reference_local(&self, camera: &Graphic3dCamera) -> Option<[f64; 2]> {
        const SAMPLES: [[f64; 2]; 9] = [
            [0.0, 0.0],
            [-1.0, -1.0],
            [1.0, -1.0],
            [1.0, 1.0],
            [-1.0, 1.0],
            [0.0, -1.0],
            [1.0, 0.0],
            [0.0, 1.0],
            [-1.0, 0.0],
        ];
        SAMPLES
            .iter()
            .find_map(|sample| self.plane_local_hit(camera, *sample, true))
            .map(|hit| hit.local)
    }
}

/// `snappedLocalShift`: the nearest multiple of the step `1/scale`.
fn snapped_shift(value: f64, scale: f64) -> f64 {
    if scale <= martensite_tkernel::precision::confusion() {
        return 0.0;
    }
    let step = 1.0 / scale;
    (value / step).round() * step
}

fn angular_tolerance() -> f32 {
    martensite_tkernel::precision::angular() as f32
}

/// `viewPoint`/`viewDirection`: float transform of a point (w = 1) or direction (w = 0).
fn view_vector(world_view: &Mat4<f32>, xyz: Xyz, w: f32) -> [f32; 3] {
    let input = Vec4::new(xyz.x() as f32, xyz.y() as f32, xyz.z() as f32, w);
    let view = *world_view * input;
    [view.x(), view.y(), view.z()]
}

fn with_w([x, y, z]: [f32; 3], w: f64) -> [f32; 4] {
    [x, y, z, w as f32]
}
