//! Per-pass state shared by the group draw records of one layer pass (#594): the camera
//! fields every recorded group needs, computed once, and the rendering parameters the records
//! of the pass were built with. Without it each of 1,000 groups inverted the same view matrix
//! and compared the same parameter set again.
use crate::RenderError;
use martensite_tkernel::ncollection::Mat4;
use martensite_tkservice::graphic3d::*;
use std::sync::{Arc, Mutex, OnceLock};

/// The camera values that do not depend on a group's model matrix.
#[derive(Clone, Copy, Debug)]
pub(crate) struct RecordCameraParts {
    pub view: Mat4<f32>,
    /// `projection * view`, in the order `ordinary_clip` multiplies.
    pub projection_view: Mat4<f32>,
    pub eye: [f32; 4],
    /// Whether the camera's axial scale is 1 on every axis.
    pub unit_axial_scale: bool,
    /// The retained factors with an identity model; each draw sets only its model matrix.
    pub factors: Option<crate::RenderProjectionFactors>,
}

/// The shared record state of one layer pass; `RenderElementPass::record_frame()`.
pub(crate) struct RecordFrame {
    /// A number no other pass of the process has; a group keeps a record only when a second
    /// pass lowers it, so scenes drawn once or replayed as one batch keep no records.
    serial: u64,
    camera: OnceLock<(usize, RecordCameraParts)>,
    rendering: OnceLock<Arc<Graphic3dRenderingParams>>,
    /// The (recorded, current) parameter addresses last found identical.
    matched: Mutex<Option<(usize, usize)>>,
    /// The structure manager address and the surface policies of its driver.
    surface: OnceLock<(usize, SurfacePolicies)>,
    text_unview: crate::text_transform::SharedTextUnview,
}

/// The lighting pipeline and unlit face policy of a structure's driver.
pub(crate) type SurfacePolicies = (
    crate::RenderLightingPipeline,
    crate::RenderUnlitFaceColorPolicy,
);

impl std::fmt::Debug for RecordFrame {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("RecordFrame")
            .field("serial", &self.serial)
            .field("camera", &self.camera.get())
            .field("rendering", &self.rendering.get().is_some())
            .finish_non_exhaustive()
    }
}

impl Default for RecordFrame {
    fn default() -> Self {
        static PASSES: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        Self {
            serial: PASSES.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            camera: OnceLock::new(),
            rendering: OnceLock::new(),
            matched: Mutex::new(None),
            surface: OnceLock::new(),
            text_unview: Default::default(),
        }
    }
}

impl RecordFrame {
    /// This pass's process-unique number; `frame.serial()`.
    pub(crate) fn serial(&self) -> u64 {
        self.serial
    }

    /// The camera parts of `camera`, computed on the first call of the pass;
    /// `frame.camera_parts(&camera)?`.
    /// `None` for a camera that cannot be inverted, or for a second camera in one pass, whose
    /// groups then lower; borrowing saves copying the parts for every group (#594).
    pub(crate) fn camera_parts(&self, camera: &Graphic3dCamera) -> Option<&RecordCameraParts> {
        let address = std::ptr::from_ref(camera).addr();
        let (key, parts) = match self.camera.get() {
            Some(kept) => kept,
            None => {
                let parts = RecordCameraParts::of(camera).ok()?;
                self.camera.get_or_init(|| (address, parts))
            }
        };
        (*key == address).then_some(parts)
    }

    /// The surface policies of `structure`'s driver, looked up once per pass for the
    /// structures of one manager; each group asked through three locks before (#594).
    /// `frame.surface_policies(&group.structure())`
    pub(crate) fn surface_policies(
        &self,
        structure: &martensite_tkernel::standard::Handle<dyn Graphic3dStructureSource>,
    ) -> SurfacePolicies {
        let manager = structure.structure_manager();
        let Some(manager) = manager.get() else {
            return Default::default();
        };
        let address = std::ptr::from_ref(manager).addr();
        let (kept, policies) = self.surface.get_or_init(|| {
            (
                address,
                crate::lighting_pipeline::manager_surface_policies(manager),
            )
        });
        if *kept == address {
            return *policies;
        }
        crate::lighting_pipeline::manager_surface_policies(manager)
    }

    /// The inverse of the text projection that the labels of this pass share;
    /// `frame.text_unview().inverse(&projection)`.
    pub(crate) fn text_unview(&self) -> &crate::text_transform::SharedTextUnview {
        &self.text_unview
    }

    /// The parameters the records built in this pass keep, one copy for all of them;
    /// `frame.recorded_rendering(&params)`.
    pub(crate) fn recorded_rendering(
        &self,
        current: &Graphic3dRenderingParams,
    ) -> Arc<Graphic3dRenderingParams> {
        self.rendering
            .get_or_init(|| Arc::new(current.clone()))
            .clone()
    }

    /// Whether a record's parameters are `current`, compared once per pair in the pass;
    /// `frame.rendering_matches(&record.rendering, &params)`.
    pub(crate) fn rendering_matches(
        &self,
        recorded: &Arc<Graphic3dRenderingParams>,
        current: &Graphic3dRenderingParams,
    ) -> bool {
        let key = (
            Arc::as_ptr(recorded).addr(),
            std::ptr::from_ref(current).addr(),
        );
        let mut matched = self.matched.lock().unwrap_or_else(|e| e.into_inner());
        if *matched == Some(key) {
            return true;
        }
        let same = same_rendering(recorded, current);
        if same {
            *matched = Some(key);
        }
        same
    }
}

impl RecordCameraParts {
    pub(crate) fn of(camera: &Graphic3dCamera) -> Result<Self, RenderError> {
        let projection = camera.projection_matrix_f();
        let view = camera.orientation_matrix_f();
        // Read both values together, as RenderProjectionFactors::retained_camera does.
        let (orientation, scale) = camera.orientation_with_axial_scale_f();
        let unit_axial_scale = [scale.x(), scale.y(), scale.z()] == [1.; 3];
        let isometry = unit_axial_scale
            && view.data().map(f32::to_bits) == orientation.data().map(f32::to_bits);
        Ok(Self {
            view,
            projection_view: projection * view,
            eye: crate::lighting::lighting_eye(view.data(), !camera.is_orthographic())?,
            unit_axial_scale,
            factors: crate::RenderProjectionFactors::retained_with_isometry(
                projection.data(),
                view.data(),
                Mat4::<f32>::identity().data(),
                isometry,
            ),
        })
    }
}

/// Whether two rendering parameter sets are identical, floats by bits. The destructuring
/// names every field, so a new field fails to compile here until it is compared.
pub(crate) fn same_rendering(a: &Graphic3dRenderingParams, b: &Graphic3dRenderingParams) -> bool {
    let Graphic3dRenderingParams {
        method,
        shading_model,
        transparency_method,
        resolution,
        font_hinting,
        line_feather,
        pbr_env_pow2_size,
        pbr_env_spec_map_nb_levels,
        pbr_env_baking_diff_nb_samples,
        pbr_env_baking_spec_nb_samples,
        pbr_env_baking_probability,
        oit_depth_factor,
        nb_oit_depth_peeling_layers,
        nb_msaa_samples,
        render_resolution_scale,
        shadow_map_resolution,
        shadow_map_bias,
        to_enable_depth_prepass,
        to_enable_alpha_to_coverage,
        ray_tracing,
        stereo,
        stats,
    } = a;
    let floats = [
        (line_feather, b.line_feather),
        (pbr_env_baking_probability, b.pbr_env_baking_probability),
        (oit_depth_factor, b.oit_depth_factor),
        (render_resolution_scale, b.render_resolution_scale),
        (shadow_map_bias, b.shadow_map_bias),
    ];
    let integers = [
        (pbr_env_pow2_size, b.pbr_env_pow2_size),
        (pbr_env_spec_map_nb_levels, b.pbr_env_spec_map_nb_levels),
        (
            pbr_env_baking_diff_nb_samples,
            b.pbr_env_baking_diff_nb_samples,
        ),
        (
            pbr_env_baking_spec_nb_samples,
            b.pbr_env_baking_spec_nb_samples,
        ),
        (nb_oit_depth_peeling_layers, b.nb_oit_depth_peeling_layers),
        (nb_msaa_samples, b.nb_msaa_samples),
        (shadow_map_resolution, b.shadow_map_resolution),
    ];
    *method == b.method
        && *shading_model == b.shading_model
        && *transparency_method == b.transparency_method
        && *resolution == b.resolution
        && *font_hinting == b.font_hinting
        && floats.iter().all(|(x, y)| x.to_bits() == y.to_bits())
        && integers.iter().all(|(x, y)| **x == *y)
        && *to_enable_depth_prepass == b.to_enable_depth_prepass
        && *to_enable_alpha_to_coverage == b.to_enable_alpha_to_coverage
        && *ray_tracing == b.ray_tracing
        && *stereo == b.stereo
        && same_stats(stats, &b.stats)
}

/// `same_rendering` for the statistics fields.
fn same_stats(a: &Graphic3dStatsParams, b: &Graphic3dStatsParams) -> bool {
    let Graphic3dStatsParams {
        stats_position,
        chart_position,
        chart_size,
        stats_text_aspect,
        stats_update_interval,
        stats_text_height,
        stats_nb_frames,
        stats_max_chart_time,
        collected_stats,
        to_show_stats,
    } = a;
    *stats_position == b.stats_position
        && *chart_position == b.chart_position
        && [chart_size.x(), chart_size.y()] == [b.chart_size.x(), b.chart_size.y()]
        && *stats_text_aspect == b.stats_text_aspect
        && stats_update_interval.to_bits() == b.stats_update_interval.to_bits()
        && *stats_text_height == b.stats_text_height
        && *stats_nb_frames == b.stats_nb_frames
        && stats_max_chart_time.to_bits() == b.stats_max_chart_time.to_bits()
        && *collected_stats == b.collected_stats
        && *to_show_stats == b.to_show_stats
}

#[cfg(test)]
#[path = "group_record_frame_tests.rs"]
mod tests;
