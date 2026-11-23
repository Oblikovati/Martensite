//! Retain an ordered batch of compatible, ordinary shaded surfaces.
use super::*;

impl Graphic3dRenderView {
    pub(super) fn try_replay_batch(
        &self,
        extent: [u32; 2],
    ) -> Result<Option<crate::RenderFrame>, RenderError> {
        let mut current = self.fused_replay.write();
        let Some(replay) = current.as_mut() else {
            return Ok(None);
        };
        let outcome = match self.batch_view_policy() {
            Some(policy) if replay.seed.extent() == extent => replay.refresh(self, &policy),
            _ => Ok(None),
        };
        // A weak command snapshot can expire but cannot become live again.
        if matches!(&outcome, Ok(None)) && replay.prepared.commands_expired() {
            let retired = current.take();
            drop(current);
            drop(retired);
        }
        outcome
    }

    pub(super) fn try_seed_batch(
        &self,
        frame: &crate::RenderFrame,
    ) -> Result<Option<crate::RenderFrame>, RenderError> {
        if frame.draws.len() < 2 || !crate::backend::directional_phong_draw(&frame.draws[0]) {
            return Ok(None);
        }
        if self.fused_replay.read().is_some() {
            return Ok(None);
        }
        let Some(policy) = self.batch_view_policy() else {
            return Ok(None);
        };
        let Some(replay) = ViewBatch::seed(self, frame, policy)? else {
            return Ok(None);
        };
        let first = replay.seed.clone();
        *self.fused_replay.write() = Some(replay);
        Ok(Some(first))
    }

    pub(super) fn discard_batch(&self) {
        let retired = self.fused_replay.write().take();
        drop(retired);
    }

    fn batch_view_policy(&self) -> Option<BatchViewPolicy> {
        if cfg!(any(
            feature = "command-state-diagnostics",
            feature = "vertex-output-diagnostics",
            feature = "lit-only-command-diagnostics",
            feature = "raytrace-input-diagnostics",
        )) || !self
            .fused_replay_enabled
            .load(std::sync::atomic::Ordering::Relaxed)
            || !self.device.read().supports_phong_batching()
            || self.frame_hooks.read().is_some()
            || !self.retained.render_clip_planes().is_null()
            || self.shown_graduated_trihedron().is_some()
            || self.shader_grid_state().read().is_shown()
        {
            return None;
        }
        BatchViewPolicy::capture(self)
    }
}

#[derive(PartialEq)]
struct BatchViewPolicy {
    shading: Graphic3dTypeOfShadingModel,
    backfacing: Graphic3dTypeOfBackfacingModel,
    transparency: Graphic3dRenderTransparentMethod,
    samples: i32,
    resolution: u32,
    depth_prepass: bool,
    alpha_coverage: bool,
    clear: [u32; 4],
}

impl BatchViewPolicy {
    fn capture(view: &Graphic3dRenderView) -> Option<Self> {
        let (mut policy, camera) = Self::capture_fields(view)?;
        if !camera.is_orthographic()
            || view.lighting_selection.read().policy == crate::RenderLightingPipeline::FixedFunction
        {
            return None;
        }
        policy.clear[3] = f32::from(view.options.read().buffers_opaque_alpha).to_bits();
        Some(policy)
    }

    fn capture_fields(view: &Graphic3dRenderView) -> Option<(Self, Handle<Graphic3dCamera>)> {
        let background_fill = view.gradient_background().bg_gradient_fill_method();
        let fields = view.cview_base().fields.read();
        let params = &fields.render_params;
        if !Self::allows_batch(&fields, params)
            || (fields.background_type == Graphic3dTypeOfBackground::Gradient
                && background_fill != AspectGradientFillMethod::None)
        {
            return None;
        }
        let rgb = fields.bg_color.rgb().rgb();
        Some((
            Self {
                shading: params.shading_model,
                backfacing: fields.backfacing,
                transparency: params.transparency_method,
                samples: params.nb_msaa_samples,
                resolution: params.resolution,
                depth_prepass: params.to_enable_depth_prepass,
                alpha_coverage: params.to_enable_alpha_to_coverage,
                clear: [rgb[0].to_bits(), rgb[1].to_bits(), rgb[2].to_bits(), 0],
            },
            fields.camera.clone(),
        ))
    }

    fn matches_seed(&self, frame: &crate::RenderFrame) -> bool {
        frame.clear_color().map(f32::to_bits) == self.clear
            && !frame.raster.fixed_function
            && frame.raster.requested_samples == self.samples
            && frame.raster.alpha_to_coverage == self.alpha_coverage
            && frame.raster.transparency == crate::RenderTransparency::Unordered
            && frame.immediate_start == Some(1)
    }

    fn allows_batch(fields: &Graphic3dCViewFields, params: &Graphic3dRenderingParams) -> bool {
        params.method == Graphic3dRenderingMode::Rasterization
            && params.transparency_method == Graphic3dRenderTransparentMethod::BlendUnordered
            && crate::render_scale::active_render_scale(params).is_none()
            && !params.stats.to_show_stats
            && !params.to_enable_depth_prepass
            && matches!(
                fields.background_type,
                Graphic3dTypeOfBackground::None | Graphic3dTypeOfBackground::Gradient
            )
            && fields.background_image.is_null()
            && fields.texture_env_data.is_null()
            && fields.xr_session.is_null()
            && !fields.is_subview_composer
            && !fields.z_layer_redraw_mode
            && fields.z_layer_target == GRAPHIC3D_ZLAYER_ID_BOT_OSD
    }
}

pub(super) struct ViewBatch {
    seed: crate::RenderFrame,
    prepared: crate::view_scene::BatchSceneSources,
    policy: BatchViewPolicy,
    connection_revision: Arc<()>,
    hits: u64,
}
impl Graphic3dRenderView {
    /// Toggle native batch selection for the parity benchmark.
    /// Example: `view.enable_fused_replay(false);`.
    #[doc(hidden)]
    pub fn enable_fused_replay(&self, enabled: bool) {
        self.fused_replay_enabled
            .store(enabled, std::sync::atomic::Ordering::Relaxed);
    }

    /// Read replay admissions for the parity benchmark.
    /// Example: `let accepted = view.fused_replay_hits();`.
    #[doc(hidden)]
    pub fn fused_replay_hits(&self) -> u64 {
        self.fused_replay
            .read()
            .as_ref()
            .map_or(0, |replay| replay.hits)
    }
}
impl ViewBatch {
    fn seed(
        view: &Graphic3dRenderView,
        frame: &crate::RenderFrame,
        policy: BatchViewPolicy,
    ) -> Result<Option<Self>, RenderError> {
        if !policy.matches_seed(frame) {
            return Ok(None);
        }
        let Some(merged) = batch_packet(frame) else {
            return Ok(None);
        };
        let mut prepared =
            crate::view_scene::BatchSceneSources::new(frame.extent(), frame.draws.len());
        let camera = crate::view_render_camera::owned_render_camera(view);
        let revision = camera.world_view_proj_state();
        let connection_revision = view.driver_objects.connection_revision();
        if !crate::view_scene::match_batch_sources(
            view,
            &camera,
            &mut prepared,
            Some(frame),
            Some(&view.driver_objects),
        )? || !view.driver_objects.connections_match(&connection_revision)
            || !batch_camera_matches(view, &camera, revision)
        {
            return Ok(None);
        }
        let mut frame = frame.clone();
        let sources = std::mem::replace(&mut frame.draws, vec![merged]);
        let Some(geometry) = view.device.read().prepare_batch(sources)? else {
            return Ok(None);
        };
        frame.geometry_batch = Some(Arc::new(geometry));
        frame.layers[0].draw_range = 0..1;
        Ok(Some(Self {
            seed: frame,
            prepared,
            policy,
            connection_revision,
            hits: 0,
        }))
    }

    fn refresh(
        &mut self,
        view: &Graphic3dRenderView,
        policy: &BatchViewPolicy,
    ) -> Result<Option<crate::RenderFrame>, RenderError> {
        use martensite_tkernel::ncollection::{Mat4, Vec4};
        if policy != &self.policy {
            return Ok(None);
        }
        let camera = crate::view_render_camera::owned_render_camera(view);
        let revision = camera.world_view_proj_state();
        let check_connections = !view
            .driver_objects
            .connections_match(&self.connection_revision);
        let current_connections =
            check_connections.then(|| view.driver_objects.connection_revision());
        if !camera.is_orthographic()
            || !crate::view_scene::match_batch_sources(
                view,
                &camera,
                &mut self.prepared,
                None,
                check_connections.then_some(&view.driver_objects),
            )?
        {
            return Ok(None);
        }
        let mut frame = self.seed.clone();
        let draw = &mut frame.draws[0];
        let lighting = draw.lighting.as_mut().unwrap();
        let orientation = camera.orientation_matrix_f();
        draw.clip_transform = crate::persistent_transform::ordinary_clip(
            &camera,
            Mat4::<f64>::from_other(&Mat4::<f32>::from_data(lighting.world)),
        );
        let Ok(inverse) = orientation.inverted() else {
            return Ok(None);
        };
        lighting.view = orientation.data();
        lighting.eye = (inverse * Vec4::new(0., 0., 1., 0.)).data();
        lighting.eye[3] = 0.;
        lighting.lights =
            crate::RenderLightSet::for_view(policy.shading, &view.lights(), inverse.data(), None)?;
        let connections = current_connections
            .as_ref()
            .unwrap_or(&self.connection_revision);
        if !crate::backend::directional_lights(&lighting.lights)
            || !view.driver_objects.connections_match(connections)
            || !batch_camera_matches(view, &camera, revision)
        {
            return Ok(None);
        }
        if let Some(current) = current_connections {
            self.connection_revision = current;
        }
        frame.attach_stencil_context(&view.stencil_feedback);
        frame.attach_fragment_alpha(&view.fragment_alpha_feedback);
        self.hits = self.hits.saturating_add(1);
        Ok(Some(frame))
    }
}
fn batch_packet(frame: &crate::RenderFrame) -> Option<crate::PrimitiveDraw> {
    if frame.draws.len() < 2
        || frame.layers.len() != 1
        || !frame.layers[0].stencil_boundaries.is_empty()
        || frame.gui_draw_start.is_some()
        || frame.environment.is_some()
        || frame.raytrace.is_some()
        || frame.shadow_maps.is_some()
        || frame.colored_quad.is_some()
        || frame.stereo_pair.is_some()
        || !frame.subview_blits.is_empty()
        || frame.immediate_main.is_some()
    {
        return None;
    }
    let packet = probe_packet(&frame.draws[0])?;
    compatible_draws(&frame.draws, &packet).then_some(packet)
}

fn compatible_draws(draws: &[crate::PrimitiveDraw], packet: &crate::PrimitiveDraw) -> bool {
    let mut size = (0u32, 0u32);
    let shares_packet = draws.iter().all(|draw| {
        let Some(vertices) = u32::try_from(draw.vertices.len())
            .ok()
            .and_then(|count| size.0.checked_add(count))
        else {
            return false;
        };
        let Some(indices) = u32::try_from(draw.indices.len())
            .ok()
            .and_then(|count| size.1.checked_add(count))
        else {
            return false;
        };
        size = (vertices, indices);
        draw.clip_transform.map(f32::to_bits) == packet.clip_transform.map(f32::to_bits)
            && shares_packet(draw, packet)
    });
    shares_packet && size.0 as usize >= crate::backend::MIN_PHONG_BATCH_VERTICES
}

/// The draw without its geometry streams: the state every draw of a batch must share.
fn probe_packet(draw: &crate::PrimitiveDraw) -> Option<crate::PrimitiveDraw> {
    if !crate::backend::directional_phong_draw(draw) {
        return None;
    }
    let mut packet = draw.clone();
    packet.body = std::sync::Arc::new(packet_body(&draw.body));
    let lighting = packet.lighting.as_mut()?;
    lighting.body = std::sync::Arc::new(packet_lighting(&lighting.body));
    Some(packet)
}

/// Whether `draw` equals `packet` apart from its geometry. The stripped bodies stay on the
/// stack, so checking ten thousand draws allocates nothing (#594).
fn shares_packet(draw: &crate::PrimitiveDraw, packet: &crate::PrimitiveDraw) -> bool {
    let (Some(lighting), Some(packed)) = (draw.lighting.as_ref(), packet.lighting.as_ref()) else {
        return false;
    };
    if !crate::backend::directional_phong_draw(draw)
        || packet_body(&draw.body) != *packet.body
        || packet_lighting(&lighting.body) != *packed.body
    {
        return false;
    }
    // The bodies match, so the packet's bodies stand in for them to compare the rest.
    let mut probe = draw.clone();
    probe.body = std::sync::Arc::clone(&packet.body);
    if let Some(lighting) = probe.lighting.as_mut() {
        lighting.body = std::sync::Arc::clone(&packed.body);
    }
    probe == *packet
}

fn packet_body(body: &crate::draw::DrawBody) -> crate::draw::DrawBody {
    let mut body = body.clone();
    body.vertices.clear();
    body.indices.clear();
    body.native_flat_sources.clear();
    body.guard_band_bounds = Default::default();
    body.geometry_revision = Default::default();
    body
}

fn packet_lighting(body: &crate::lighting::LightingBody) -> crate::lighting::LightingBody {
    let mut body = body.clone();
    body.normals.clear();
    body
}

fn batch_camera_matches(
    view: &Graphic3dRenderView,
    camera: &Handle<Graphic3dCamera>,
    previous: Graphic3dWorldViewProjState,
) -> bool {
    let current = camera.world_view_proj_state();
    !current.is_projection_changed(&previous)
        && !current.is_world_view_changed(&previous)
        && *camera == crate::view_render_camera::owned_render_camera(view)
}

#[cfg(test)]
#[path = "view_batch_tests.rs"]
mod tests;
