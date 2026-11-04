//! GlView::updateCamera and setUniformState: the RaytraceUniforms block of
//! shaders/raytrace_uniforms.glsl, packed at its std140 offsets.
use martensite_tkernel::ncollection::{Mat4, Vec4};

/// Bytes of the std140 block (the last member ends at 416).
pub(crate) const UNIFORM_BYTES: usize = 416;

/// The view rays of the four window corners and the matrices they come from.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct RaytraceCamera {
    /// `uViewMat`: projection times orientation.
    pub view_projection: Mat4<f32>,
    /// `uUnviewMat`: its float inverse.
    pub unview: Mat4<f32>,
    /// Ray origins at LB, RB, LT, RT (near plane).
    pub origins: [[f32; 3]; 4],
    /// Ray directions (far minus near point) at LB, RB, LT, RT.
    pub directions: [[f32; 3]; 4],
}

fn unprojected(unview: &Mat4<f32>, x: f32, y: f32, z: f32) -> Vec4<f32> {
    let mut point = *unview * Vec4::new(x, y, z, 1.);
    point.x /= point.w;
    point.y /= point.w;
    point.z /= point.w;
    point
}

/// `updateCamera`: the corner rays of `projection * orientation`, in float as natively.
/// Example: `RaytraceCamera::new(&orientation, &projection)`.
impl RaytraceCamera {
    pub fn new(orientation: &Mat4<f32>, projection: &Mat4<f32>) -> Self {
        let view_projection = *projection * *orientation;
        let mut unview = Mat4::<f32>::identity();
        // A singular view matrix leaves the identity, as the ignored native Inverted result.
        if let Ok(inverse) = view_projection.inverted() {
            unview = inverse;
        }
        let mut camera = Self {
            view_projection,
            unview,
            origins: [[0.; 3]; 4],
            directions: [[0.; 3]; 4],
        };
        let corners = [(-1., -1.), (1., -1.), (-1., 1.), (1., 1.)];
        for (index, (x, y)) in corners.into_iter().enumerate() {
            let origin = unprojected(&unview, x, y, -1.);
            let mut direct = unprojected(&unview, x, y, 1.);
            direct -= origin;
            camera.origins[index] = [origin.x, origin.y, origin.z];
            camera.directions[index] = [direct.x, direct.y, direct.z];
        }
        camera
    }
}

/// The values of one ray-tracing pass.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct RaytraceUniforms {
    pub camera: RaytraceCamera,
    pub window: [i32; 2],
    pub ambient: [f32; 4],
    pub back_top: [f32; 4],
    pub back_bottom: [f32; 4],
    pub light_count: i32,
    pub shadows: bool,
    pub reflections: bool,
    pub scene_radius: f32,
    pub scene_epsilon: f32,
    pub aperture_radius: f32,
    pub focal_plane_distance: f32,
    /// The path-tracing members: `uFrameRndSeed`, `uBlockedRngEnabled`, `uAccumSamples` and
    /// `uMaxRadiance`, zero for ray tracing.
    pub path_tracing: PathtraceUniforms,
    /// `uEyeOrig`, `uEyeView`, `uEyeVert`, `uEyeSide` and `uEyeSize` (updatePerspCameraPT).
    pub eye: RaytraceEye,
    /// `uEnvMapEnabled`, `uEnvMapForBack`, `uYCoeff` and `uZCoeff`.
    pub environment: super::environment::EnvironmentUniforms,
}

/// The per-frame path-tracing uniforms of setUniformState and runPathtrace.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct PathtraceUniforms {
    pub frame_seed: i32,
    pub blocked_rng: bool,
    pub accum_samples: i32,
    pub max_radiance: f32,
    /// `uTileSize` of adaptive sampling (square tiles), zero otherwise.
    pub tile_size: i32,
}

/// The eye frame of updatePerspCameraPT (zero until a perspective path-traced frame sets it).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct RaytraceEye {
    pub origin: [f32; 3],
    pub view: [f32; 3],
    pub vertical: [f32; 3],
    pub side: [f32; 3],
    pub size: [f32; 2],
}

struct Block([u8; UNIFORM_BYTES]);
impl Block {
    fn floats(&mut self, offset: usize, values: &[f32]) {
        for (index, value) in values.iter().enumerate() {
            self.0[offset + 4 * index..][..4].copy_from_slice(&value.to_ne_bytes());
        }
    }
    fn int(&mut self, offset: usize, value: i32) {
        self.0[offset..][..4].copy_from_slice(&value.to_ne_bytes());
    }
}

impl RaytraceUniforms {
    /// The std140 bytes; the depth-of-field members stay zero without a perspective
    /// path-traced eye. Example: `uniforms.bytes().len() == UNIFORM_BYTES`.
    pub fn bytes(&self) -> [u8; UNIFORM_BYTES] {
        let mut block = Block([0; UNIFORM_BYTES]);
        block.floats(0, &self.camera.unview.data());
        block.floats(64, &self.camera.view_projection.data());
        block.floats(128, &self.ambient);
        block.floats(144, &self.back_top);
        block.floats(160, &self.back_bottom);
        // LT, LB, RT, RB order of the block; the camera keeps LB, RB, LT, RT.
        let [lb, rb, lt, rt] = self.camera.origins;
        let [dlb, drb, dlt, drt] = self.camera.directions;
        for (slot, value) in [lt, lb, rt, rb, dlt, dlb, drt, drb].iter().enumerate() {
            block.floats(176 + 16 * slot, value);
        }
        let ints = [self.window[0], self.window[1], self.light_count];
        ints.iter()
            .enumerate()
            .for_each(|(slot, v)| block.int(188 + 16 * slot, *v));
        block.int(236, i32::from(self.shadows));
        block.int(252, i32::from(self.reflections));
        let environment = &self.environment;
        block.int(268, i32::from(environment.enabled));
        block.int(284, i32::from(environment.for_background));
        block.floats(300, &[self.scene_radius]);
        block.floats(316, &[self.scene_epsilon]);
        block.floats(332, &[self.aperture_radius]);
        block.floats(348, &[self.focal_plane_distance]);
        let eye = &self.eye;
        for (slot, value) in [eye.origin, eye.view, eye.vertical, eye.side]
            .iter()
            .enumerate()
        {
            block.floats(304 + 16 * slot, value);
        }
        block.floats(368, &eye.size);
        let path = &self.path_tracing;
        block.int(400, path.frame_seed);
        block.int(404, i32::from(path.blocked_rng));
        block.int(408, path.accum_samples);
        block.floats(412, &[path.max_radiance]);
        block.int(384, path.tile_size);
        block.int(388, path.tile_size);
        block.int(392, environment.coefficients[0]);
        block.int(396, environment.coefficients[1]);
        block.0
    }
}

#[cfg(test)]
#[path = "uniforms_tests.rs"]
mod tests;
