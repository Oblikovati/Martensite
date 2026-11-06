//! Path tracing of GlView: the output parameters of runPathtraceOut, the eye
//! frame of updatePerspCameraPT and the per-view accumulation of Redraw/runPathtrace
//! (`myAccumFrames`, `myRNG`).
use super::adaptive::{AdaptiveSampling, PathtraceAdaptive, pathtrace_adaptive};
use super::frame::RaytraceFrame;
use super::uniforms::RaytraceEye;
use crate::RenderError;
use martensite_tkmath::math::BullardGenerator;
use martensite_tkservice::graphic3d::{
    Graphic3dCamera, Graphic3dCameraFocusType, Graphic3dCameraIodType, Graphic3dCameraProjection,
    Graphic3dRenderingParams, Graphic3dToneMappingMethod, Graphic3dWorldViewProjState,
};

/// The Display.fs parameters of a path-traced frame and its accumulated frame count.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PathtraceFrame {
    /// `myAccumFrames`: frames accumulated before this one since the last reset.
    pub accum_frames: i32,
    pub exposure: f32,
    pub white_point: f32,
    /// Adaptive screen sampling (ADAPTIVE_SAMPLING), when enabled.
    pub adaptive: Option<PathtraceAdaptive>,
}

/// The path-tracing output of `params`, `None` without global illumination.
pub(crate) fn pathtrace_frame(params: &Graphic3dRenderingParams) -> Option<PathtraceFrame> {
    params
        .ray_tracing
        .is_global_illumination_enabled
        .then_some(PathtraceFrame {
            accum_frames: 0,
            exposure: params.ray_tracing.exposure,
            white_point: params.ray_tracing.white_point,
            adaptive: pathtrace_adaptive(params),
        })
}

/// Whether the output program applies filmic tone mapping (TONE_MAPPING_FILMIC).
pub(crate) fn is_filmic(params: &Graphic3dRenderingParams) -> bool {
    params.ray_tracing.tone_mapping_method == Graphic3dToneMappingMethod::Filmic
}

fn vec3(x: f64, y: f64, z: f64) -> [f32; 3] {
    [x as f32, y as f32, z as f32]
}

/// `updatePerspCameraPT`: eye, view, up, their cross product and the half extents of the image
/// plane at unit distance. A camera that is not a perspective one (orthographic included) takes
/// the native "stereo camera" branch: the eye moves half the intraocular distance along the side
/// and looks at the Z-focus point.
pub(crate) fn perspective_eye(camera: &Graphic3dCamera, window: [i32; 2]) -> RaytraceEye {
    let (eye, view, up) = (camera.eye(), camera.direction(), camera.up());
    let view = vec3(view.x(), view.y(), view.z());
    let vertical = vec3(up.x(), up.y(), up.z());
    let side = [
        view[1] * vertical[2] - view[2] * vertical[1],
        view[2] * vertical[0] - view[0] * vertical[2],
        view[0] * vertical[1] - view[1] * vertical[0],
    ];
    let scale_y = martensite_cmath::tan(camera.fov_y() / 360. * std::f64::consts::PI);
    let scale_x = f64::from(window[0]) * scale_y / f64::from(window[1]);
    let mono = RaytraceEye {
        origin: vec3(eye.x(), eye.y(), eye.z()),
        view,
        vertical,
        side,
        size: [scale_x as f32, scale_y as f32],
    };
    match camera.projection_type() {
        Graphic3dCameraProjection::Perspective => mono,
        projection => shifted_eye(camera, mono, projection),
    }
}

/// The stereo branch of updatePerspCameraPT, in the native float order.
fn shifted_eye(
    camera: &Graphic3dCamera,
    mono: RaytraceEye,
    projection: Graphic3dCameraProjection,
) -> RaytraceEye {
    let distance = camera.distance();
    let iod = match camera.iod_type() {
        Graphic3dCameraIodType::Relative => (camera.iod() * distance) as f32,
        Graphic3dCameraIodType::Absolute => camera.iod() as f32,
    };
    let focus = match camera.z_focus_type() {
        Graphic3dCameraFocusType::Relative => (camera.z_focus() * distance) as f32,
        Graphic3dCameraFocusType::Absolute => camera.z_focus() as f32,
    };
    let focus_point: [f32; 3] = std::array::from_fn(|i| mono.origin[i] + mono.view[i] * focus);
    let shift = match projection {
        Graphic3dCameraProjection::MonoRightEye => 0.5 * iod,
        _ => -0.5 * iod,
    };
    let side = normalized(mono.side);
    let origin: [f32; 3] = std::array::from_fn(|i| mono.origin[i] + side[i] * shift);
    let view = normalized(std::array::from_fn(|i| focus_point[i] - origin[i]));
    let dot = mono.view[0] * view[0] + mono.view[1] * view[1] + mono.view[2] * view[2];
    let scale = 1. / dot;
    RaytraceEye {
        origin,
        view: view.map(|c| c * scale),
        ..mono
    }
}

/// `NCollectionVec3<float>::Normalized`: unchanged when the modulus is zero.
fn normalized(v: [f32; 3]) -> [f32; 3] {
    let modulus = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    match modulus != 0. {
        true => v.map(|c| c / modulus),
        false => v,
    }
}

/// runPathtrace's `aFBOIdx` of a frame drawn with `projection`: the right eye's images are the
/// second set. Example: `eye_targets(Graphic3dCameraProjection::MonoRightEye) == 1`.
pub(crate) fn eye_targets(projection: Graphic3dCameraProjection) -> usize {
    usize::from(projection == Graphic3dCameraProjection::MonoRightEye)
}

/// The accumulation state of one view. A stereo Redraw path traces both eyes with one
/// `myAccumFrames`, counted once per Redraw, and one `myRNG`; each eye accumulates into its own
/// images (`eye_targets`).
#[derive(Default)]
pub(crate) struct RaytraceAccumulation {
    /// The last path-traced frame of each eye's images without its per-frame members; another
    /// one restarts.
    previous: [Option<RaytraceFrame>; 2],
    /// The last Redraw: whether it was a stereo one and the images its first frame used.
    last_redraw: Option<(bool, usize)>,
    /// `myWorldViewProjState`: the camera's change counters at the last path-traced Redraw.
    camera_state: Option<Graphic3dWorldViewProjState>,
    frames: i32,
    random: BullardGenerator,
    adaptive: AdaptiveSampling,
    /// The eye of the stereo Redraw being drawn (0 left, 1 right) and the stereo camera's
    /// counters before the eye projections, `None` outside one.
    stereo_eye: Option<(usize, Graphic3dWorldViewProjState)>,
}

/// `frame` without the members that change from one accumulated frame to the next.
fn steady(frame: &RaytraceFrame) -> RaytraceFrame {
    let mut steady = frame.clone();
    steady.uniforms.path_tracing.frame_seed = 0;
    steady.uniforms.path_tracing.accum_samples = 0;
    if let Some(path) = steady.path_tracing.as_mut() {
        path.accum_frames = 0;
        if let Some(adaptive) = path.adaptive.as_mut() {
            adaptive.passes = None;
        }
    }
    steady.span = 0;
    steady.rebuilt = false;
    steady.camera_state = Graphic3dWorldViewProjState::default();
    steady
}

impl RaytraceAccumulation {
    /// Enter (`Some((eye, state))`, with the stereo camera's counters before the eye
    /// projections) or leave (`None`) one eye of a stereo Redraw: its right eye reuses the frame
    /// count of the left. Example: `accumulation.set_stereo_eye(Some((1, state)))`.
    pub fn set_stereo_eye(&mut self, eye: Option<(usize, Graphic3dWorldViewProjState)>) {
        self.stereo_eye = eye;
    }
    /// After a stereo Redraw: the counters the camera has once its stereo projection is
    /// restored. Natively the eyes never change the camera, so these are its counters of the
    /// Redraw. Example: `accumulation.settle_camera(camera.world_view_proj_state())`.
    pub fn settle_camera(&mut self, state: Graphic3dWorldViewProjState) {
        if self.camera_state.is_some() {
            self.camera_state = Some(state);
        }
    }

    /// Count `frame` (Redraw's `myAccumFrames++` once per Redraw, reset to 0 by any change of
    /// the scene, camera, lights or parameters) and draw its seed (`myRNG.NextInt() >> 2`, the
    /// generator restarting whenever an eye draws frame 0). An adaptive frame draws one seed
    /// per pass instead and takes its tile maps from the view's tile sampler.
    /// Example: `accumulation.advance(&mut frame)?`.
    pub fn advance(&mut self, frame: &mut RaytraceFrame) -> Result<(), RenderError> {
        if frame.path_tracing.is_none() {
            self.previous = [None, None];
            self.last_redraw = None;
            self.camera_state = None;
            return Ok(());
        }
        let key = steady(frame);
        let eye = frame.eye_targets;
        let unchanged = !frame.rebuilt && self.previous[eye].as_ref() == Some(&key);
        let second_eye = self.stereo_eye.is_some_and(|(eye, _)| eye == 1);
        let camera = self
            .stereo_eye
            .map_or(frame.camera_state, |(_, state)| state);
        self.frames = match second_eye {
            // The right eye of a stereo Redraw: the count of its left eye.
            true if unchanged => self.frames,
            true => 0,
            false => {
                let redraw = (self.stereo_eye.is_some(), eye);
                let same_redraw = self.last_redraw == Some(redraw);
                self.last_redraw = Some(redraw);
                // GlView::render: `myWorldViewProjState != aWVPState` restarts.
                let same_camera = self.camera_state == Some(camera);
                self.camera_state = Some(camera);
                match unchanged && same_redraw && same_camera {
                    true => self.frames + 1,
                    false => 0,
                }
            }
        };
        if self.frames == 0 {
            self.random = BullardGenerator::default();
        }
        let window = frame.uniforms.window;
        let path_uniforms = &mut frame.uniforms.path_tracing;
        let path = frame.path_tracing.as_mut().expect("path-traced frame");
        path.accum_frames = self.frames;
        path_uniforms.accum_samples = self.frames;
        match path.adaptive.as_mut() {
            None => path_uniforms.frame_seed = (self.random.next_int() >> 2) as i32,
            Some(adaptive) => {
                let inputs = (window, self.frames);
                self.adaptive.prepare(adaptive, inputs, &mut self.random)?;
                let passes = adaptive.passes.as_ref().expect("prepared adaptive passes");
                path_uniforms.frame_seed = passes.seeds[0];
                path_uniforms.tile_size = passes.tile_size;
            }
        }
        self.previous[eye] = Some(key);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::raytrace::frame::RaytraceConstants;
    use crate::raytrace::uniforms::{RaytraceCamera, RaytraceUniforms};
    use martensite_tkernel::ncollection::Mat4;

    fn frame(exposure: f32) -> RaytraceFrame {
        let constants = RaytraceConstants {
            stack_size: 10,
            bounces: 3,
            zero_to_one_depth: false,
            transparent_shadows: false,
            two_sided_bxdf: false,
            filmic: false,
            ignore_normal_map: false,
            depth_of_field: false,
            textures: false,
            cubemap_background: false,
        };
        RaytraceFrame {
            span: 1,
            buffers: Default::default(),
            materials: Vec::new(),
            textures: Default::default(),
            environment: Default::default(),
            lights: Default::default(),
            uniforms: RaytraceUniforms {
                camera: RaytraceCamera::new(&Mat4::identity(), &Mat4::identity()),
                window: [8, 8],
                ambient: [0.; 4],
                back_top: [0.; 4],
                back_bottom: [0.; 4],
                light_count: 0,
                shadows: true,
                reflections: false,
                scene_radius: 1.,
                scene_epsilon: 1e-6,
                aperture_radius: 0.,
                focal_plane_distance: 1.,
                path_tracing: Default::default(),
                eye: Default::default(),
                environment: Default::default(),
            },
            constants,
            antialiasing: false,
            path_tracing: Some(PathtraceFrame {
                accum_frames: 0,
                exposure,
                white_point: 1.,
                adaptive: None,
            }),
            eye_targets: 0,
            rebuilt: false,
            camera_state: Default::default(),
        }
    }

    #[test]
    fn orthographic_eyes_shift_by_half_the_intraocular_distance_like_native() {
        let camera = Graphic3dCamera::default();
        camera.set_projection_type(Graphic3dCameraProjection::Orthographic);
        let mono = {
            let perspective = Graphic3dCamera::default();
            perspective.set_projection_type(Graphic3dCameraProjection::Perspective);
            perspective_eye(&perspective, [8, 8])
        };
        let shifted = perspective_eye(&camera, [8, 8]);
        assert_eq!(shifted.size, mono.size);
        assert_ne!(shifted.origin, mono.origin);
        let side = normalized(mono.side);
        let moved: Vec<f32> = (0..3)
            .map(|i| (shifted.origin[i] - mono.origin[i]) / side[i])
            .collect();
        let half = -0.5 * (camera.iod() * camera.distance()) as f32;
        assert!(
            moved
                .iter()
                .filter(|m| m.is_finite())
                .all(|m| (m - half).abs() < 1e-5)
        );
        assert_eq!(normalized([0., 0., 0.]), [0., 0., 0.]);
    }
    #[test]
    fn unchanged_frames_accumulate_with_the_native_seed_sequence() {
        let mut accumulation = RaytraceAccumulation::default();
        let mut random = BullardGenerator::default();
        for expected in 0..3 {
            let mut next = frame(0.);
            accumulation.advance(&mut next).unwrap();
            assert_eq!(next.uniforms.path_tracing.accum_samples, expected);
            assert_eq!(next.path_tracing.unwrap().accum_frames, expected);
            assert_eq!(
                next.uniforms.path_tracing.frame_seed,
                (random.next_int() >> 2) as i32
            );
        }
    }

    #[test]
    fn a_changed_frame_restarts_the_accumulation_and_the_generator() {
        let mut accumulation = RaytraceAccumulation::default();
        let (mut first, mut second, mut changed) = (frame(0.), frame(0.), frame(1.));
        accumulation.advance(&mut first).unwrap();
        accumulation.advance(&mut second).unwrap();
        accumulation.advance(&mut changed).unwrap();
        assert_eq!(changed.uniforms.path_tracing.accum_samples, 0);
        assert_eq!(
            changed.uniforms.path_tracing.frame_seed,
            first.uniforms.path_tracing.frame_seed
        );
        let mut ray_traced = frame(0.);
        ray_traced.path_tracing = None;
        accumulation.advance(&mut ray_traced).unwrap();
        assert_eq!(ray_traced.uniforms.path_tracing, Default::default());
    }

    #[test]
    fn stereo_eyes_share_one_count_per_redraw_and_the_generator() {
        let mut accumulation = RaytraceAccumulation::default();
        let mut random = BullardGenerator::default();
        let first_seed = (random.next_int() >> 2) as i32;
        let mut seeds = Vec::new();
        for redraw in 0..3 {
            for eye in 0..2 {
                accumulation.set_stereo_eye(Some((eye, Default::default())));
                let mut next = frame(0.);
                next.eye_targets = eye;
                // Each eye differs from the other (its own camera), not from its last frame.
                next.uniforms.window = [8 + eye as i32, 8];
                accumulation.advance(&mut next).unwrap();
                assert_eq!(next.uniforms.path_tracing.accum_samples, redraw);
                seeds.push(next.uniforms.path_tracing.frame_seed);
            }
        }
        accumulation.set_stereo_eye(None);
        // Frame 0 restarts the generator in each eye; later eyes draw consecutive seeds.
        assert_eq!(&seeds[..2], &[first_seed, first_seed]);
        let later: Vec<i32> = (0..4).map(|_| (random.next_int() >> 2) as i32).collect();
        assert_eq!(&seeds[2..], &later[..]);
        // A mono redraw of the right eye's images after the stereo ones restarts.
        let mut mono = frame(0.);
        mono.eye_targets = 1;
        mono.uniforms.window = [9, 8];
        accumulation.advance(&mut mono).unwrap();
        assert_eq!(mono.uniforms.path_tracing.accum_samples, 0);
    }
}
