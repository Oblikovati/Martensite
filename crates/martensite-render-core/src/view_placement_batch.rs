//! Retained placement frames (#594, instancing.md step 4). When a view's frame is only
//! placements of a few plain lit draw templates, as the occurrences of an assembly are, the
//! view keeps the templates and the placements and builds the next frames from them: one draw
//! that stands for every placement (draw_instances.rs). It holds while no main-layer notice
//! arrived, no observed graphics input changed, the view state other
//! than the camera is unchanged and culling still keeps every structure. A frame then computes
//! only the camera-derived fields: the templates' camera fields and lights, and each
//! placement's clip matrix in the backend.
use super::batch::{BatchViewPolicy, batch_camera_matches, batchable_frame};
use super::*;
use crate::draw_instances::{
    DrawInstances, DrawInstancesRef, InstanceBounds, InstancePlacement, InstanceVisibility,
};
use crate::graphic_group::ReplayedFields;
use crate::group_record_frame::RecordCameraParts;
use crate::immediate_elision::ImmediateViewInputs;

/// The fewest draws a placement frame stands for; a few draws record faster one by one.
const MIN_PLACEMENTS: usize = 64;

#[path = "view_placement_inputs.rs"]
mod inputs;
pub(super) use inputs::PlacementInputs;
use inputs::PlacementStamp;
#[path = "view_placement_visibility.rs"]
mod visibility;

pub(super) struct PlacementBatch {
    seed: crate::RenderFrame,
    templates: Vec<crate::PrimitiveDraw>,
    /// The first placement of each template, whose matrices patch it.
    first_placements: Vec<usize>,
    placements: Arc<[InstancePlacement]>,
    normalize: bool,
    visibility_sources: Option<visibility::VisibilitySources>,
    hiz_history: Option<Arc<dyn crate::device_backend::RenderHiZHistory>>,
    unit_axial_scale: bool,
    policy: BatchViewPolicy,
    inputs: ImmediateViewInputs,
    stamp: PlacementStamp,
    hits: u64,
}

impl PlacementBatch {
    /// The batch of `frame`, composed after `stamp` was read, or `None` when the frame is not
    /// only placements of templates that one draw-table run can draw;
    /// `PlacementBatch::seed(view, &frame, stamp)?`.
    pub(super) fn seed(
        view: &Graphic3dRenderView,
        frame: &crate::RenderFrame,
        stamp: PlacementStamp,
    ) -> Result<Option<Self>, RenderError> {
        let Some((policy, camera)) = placement_camera_policy(view) else {
            return Ok(None);
        };
        if !policy.matches_seed(frame)
            || !batchable_frame(frame)
            || !placement_raster_compatible(frame)
            || frame.draws.len() < MIN_PLACEMENTS
        {
            return Ok(None);
        }
        let Some(parts) = camera
            .get()
            .and_then(|camera| RecordCameraParts::of(camera).ok())
        else {
            return Ok(None);
        };
        let Some(split) = split_placements(&frame.draws, &parts) else {
            return Ok(None);
        };
        if !view
            .device
            .read()
            .supports_placements(&split.templates, frame.draws.len())?
        {
            return Ok(None);
        }
        Ok(Some(Self::kept(
            view,
            frame,
            split,
            (policy, parts.unit_axial_scale),
            stamp,
        )))
    }

    fn kept(
        view: &Graphic3dRenderView,
        frame: &crate::RenderFrame,
        split: SplitPlacements,
        (policy, unit_axial_scale): (BatchViewPolicy, bool),
        stamp: PlacementStamp,
    ) -> Self {
        let visibility_sources =
            visibility::VisibilitySources::capture(view, frame, &split.templates);
        let mut seed = frame.clone();
        seed.draws = vec![frame.draws[0].clone()];
        seed.layers[0].draw_range = 0..1;
        seed.layers[0].placement_sources = Arc::from([]);
        Self {
            seed,
            templates: split.templates,
            first_placements: split.first_placements,
            placements: split.placements.into(),
            normalize: split.normalize,
            visibility_sources,
            hiz_history: view.device.read().create_hiz_history(),
            unit_axial_scale,
            policy,
            inputs: ImmediateViewInputs::capture(view, frame.extent()),
            stamp,
            hits: 0,
        }
    }

    /// The next frame, or `None` when the batch no longer holds and the view must compose
    /// its scene; `batch.refresh(view, extent, stamp)?`.
    pub(super) fn refresh(
        &mut self,
        view: &Graphic3dRenderView,
        extent: [u32; 2],
        stamp: &Option<PlacementStamp>,
    ) -> Result<Option<crate::RenderFrame>, RenderError> {
        let Some(camera) = self.holding_camera(view, extent, stamp.as_ref()) else {
            return Ok(None);
        };
        let mut frame = self.seed.clone();
        frame.attach_stencil_context(&view.stencil_feedback);
        frame.attach_fragment_alpha(&view.fragment_alpha_feedback);
        if !placement_raster_compatible(&frame) {
            return Ok(None);
        }
        let revision = camera.world_view_proj_state();
        let Some(parts) = camera
            .get()
            .and_then(|camera| RecordCameraParts::of(camera).ok())
        else {
            return Ok(None);
        };
        if parts.unit_axial_scale != self.unit_axial_scale {
            return Ok(None);
        }
        let supports_hiz = self.hiz_supported(view);
        let visibility = if !keeps_every_structure(view, &camera, extent)? {
            let Some(sources) = &self.visibility_sources else {
                return Ok(None);
            };
            if !view.supports_placement_compaction() {
                return Ok(None);
            }
            Some(sources.for_camera(view, &camera, extent)?)
        } else if supports_hiz {
            self.visibility_sources
                .as_ref()
                .map(visibility::VisibilitySources::all_visible)
        } else {
            None
        };
        let draw = self.placement_draw(view, &parts, visibility, supports_hiz)?;
        if !self.stamp.current(view) || !batch_camera_matches(view, &camera, revision) {
            return Ok(None);
        }
        frame.draws = vec![draw];
        frame.attach_raster_feedback(&view.raster_feedback);
        self.hits = self.hits.saturating_add(1);
        Ok(Some(frame))
    }

    fn hiz_supported(&self, view: &Graphic3dRenderView) -> bool {
        self.seed.layers[0].depth_test
            && self.seed.layers[0].depth_write
            && self.seed.raster.requested_samples <= 1
            && !self.seed.raster.alpha_writes
            && view.device.read().supports_hiz()
    }

    /// The camera when everything but the camera is as at the seed.
    fn holding_camera(
        &self,
        view: &Graphic3dRenderView,
        extent: [u32; 2],
        stamp: Option<&PlacementStamp>,
    ) -> Option<Handle<Graphic3dCamera>> {
        let (policy, camera) = placement_camera_policy(view)?;
        let holds = policy == self.policy
            && stamp.is_some_and(|stamp| self.stamp.matches(stamp, view))
            && self
                .inputs
                .matches_but_camera(&ImmediateViewInputs::capture(view, extent));
        holds.then_some(camera)
    }

    /// The draw that stands for every placement, with this camera's templates.
    fn placement_draw(
        &self,
        view: &Graphic3dRenderView,
        parts: &RecordCameraParts,
        visibility: Option<InstanceVisibility>,
        supports_hiz: bool,
    ) -> Result<crate::PrimitiveDraw, RenderError> {
        let lights = crate::retained_lighting::lights_for_camera_view(view, parts.view)?;
        let templates: Vec<_> = self
            .templates
            .iter()
            .zip(&self.first_placements)
            .map(|(template, &first)| {
                let placement = &self.placements[first];
                let mut draw = template.clone();
                ReplayedFields::placed(parts, (placement.world, placement.normal), self.normalize)
                    .patch(&mut draw);
                if let Some(lighting) = draw.lighting.as_mut() {
                    lighting.lights = lights.clone();
                }
                draw
            })
            .collect();
        let mut head = templates[0].clone();
        head.instances = Some(DrawInstancesRef(Arc::new(DrawInstances {
            templates,
            placements: self.placements.clone(),
            projection_view: parts.projection_view.data(),
            hiz_history: if supports_hiz {
                self.hiz_history.clone()
            } else {
                None
            },
            visibility,
        })));
        Ok(head)
    }
}

fn placement_raster_compatible(frame: &crate::RenderFrame) -> bool {
    // Alpha-to-coverage can be enabled in single-sample views without changing the draw state.
    frame.initial_fragment_alpha(Default::default()).0.is_none()
        && !frame.stencil_state().is_enabled()
        && frame.raster.requested_samples <= 1
        && !frame.raster.alpha_writes
}

fn placement_camera_policy(
    view: &Graphic3dRenderView,
) -> Option<(BatchViewPolicy, Handle<Graphic3dCamera>)> {
    // An empty sequence or disabled plane can begin clipping without a view or layer notice.
    if !view.retained.render_clip_planes().is_null() {
        return None;
    }
    BatchViewPolicy::capture_any_camera(view)
}

/// Whether culling with `camera` keeps every structure of every layer, as the seed's did.
fn keeps_every_structure(
    view: &Graphic3dRenderView,
    camera: &Handle<Graphic3dCamera>,
    extent: [u32; 2],
) -> Result<bool, RenderError> {
    let selector = crate::view_scene::culling_selector(view, camera, extent);
    let state = view.rendering_params().ray_tracing.frustum_culling_state;
    for layer in view.layers() {
        let Some(layer) = layer.get() else {
            return Ok(false);
        };
        let kept = layer
            .keeps_every_structure(&selector, state)
            .map_err(|failure| rejected(failure, "a valid culling probe"))?;
        if !kept {
            return Ok(false);
        }
    }
    Ok(true)
}

/// The templates and placements of a frame's draws.
struct SplitPlacements {
    templates: Vec<crate::PrimitiveDraw>,
    first_placements: Vec<usize>,
    placements: Vec<InstancePlacement>,
    normalize: bool,
}

/// Split `draws` into templates, one per draw body, and placements, or `None` when a draw
/// is not exactly its template under its own world and normal matrices and this camera.
fn split_placements(
    draws: &[crate::PrimitiveDraw],
    parts: &RecordCameraParts,
) -> Option<SplitPlacements> {
    let normalize = draws.first()?.lighting.as_ref()?.normalize_fixed;
    let mut split = SplitPlacements {
        templates: Vec::new(),
        first_placements: Vec::new(),
        placements: Vec::with_capacity(draws.len()),
        normalize,
    };
    for draw in draws {
        let placement = split.place(draw)?;
        let mut expected = split.templates[placement.template as usize].clone();
        ReplayedFields::placed(parts, (placement.world, placement.normal), normalize)
            .patch(&mut expected);
        if expected != *draw {
            return None;
        }
        split.placements.push(placement);
    }
    Some(split)
}

impl SplitPlacements {
    /// The placement of `draw`, adding its body as a template when it is new.
    fn place(&mut self, draw: &crate::PrimitiveDraw) -> Option<InstancePlacement> {
        let lighting = draw.lighting.as_ref()?;
        if draw.instances.is_some() || lighting.fixed.is_some() || lighting.pbr {
            return None;
        }
        let template = match self
            .templates
            .iter()
            .position(|template| Arc::ptr_eq(&template.body, &draw.body))
        {
            Some(index) => index,
            None => {
                self.templates.push(draw.clone());
                self.first_placements.push(self.placements.len());
                self.templates.len() - 1
            }
        };
        Some(InstancePlacement {
            template: u32::try_from(template).ok()?,
            world: lighting.world,
            normal: lighting.normal,
        })
    }
}

impl Graphic3dRenderView {
    /// The next frame of the view's placement batch, if it holds;
    /// `view.try_replay_placements(extent, stamp)?`.
    pub(super) fn try_replay_placements(
        &self,
        extent: [u32; 2],
        stamp: &Option<PlacementStamp>,
    ) -> Result<Option<crate::RenderFrame>, RenderError> {
        if !self.placement_replay_enabled() {
            return Ok(None);
        }
        let mut current = self.placement_replay.write();
        let Some(batch) = current.as_mut() else {
            return Ok(None);
        };
        let frame = batch.refresh(self, extent, stamp)?;
        if frame.is_none() {
            *current = None;
        }
        Ok(frame)
    }

    /// Keep a placement batch of `frame`, composed after `stamp` was read.
    pub(super) fn try_seed_placements(
        &self,
        frame: &crate::RenderFrame,
        stamp: Option<PlacementStamp>,
    ) -> Result<(), RenderError> {
        if !self.placement_replay_enabled() || self.placement_replay.read().is_some() {
            return Ok(());
        }
        if frame.draws.len() < MIN_PLACEMENTS || !batchable_frame(frame) {
            return Ok(());
        }
        // The frame used as a seed must be composed after its sources were subscribed.
        let Some(stamp) = stamp.filter(|stamp| stamp.current(self)) else {
            *self.placement_inputs.write() = PlacementInputs::observe(self);
            return Ok(());
        };
        *self.placement_replay.write() = PlacementBatch::seed(self, frame, stamp)?;
        Ok(())
    }

    /// The observed inputs before the frame's scene is read.
    pub(super) fn input_stamp(&self) -> Option<PlacementStamp> {
        self.placement_inputs.read().as_ref()?.stamp(self)
    }

    fn placement_replay_enabled(&self) -> bool {
        !cfg!(any(
            feature = "command-state-diagnostics",
            feature = "vertex-output-diagnostics",
            feature = "lit-only-command-diagnostics",
            feature = "raytrace-input-diagnostics",
            feature = "draw-record-verification",
        )) && self
            .fused_replay_enabled
            .load(std::sync::atomic::Ordering::Relaxed)
            && self.frame_hooks.read().is_none()
    }

    /// Whether this device admits GPU placement compaction for the parity benchmark.
    /// Example: `view.supports_placement_compaction()`.
    #[doc(hidden)]
    pub fn supports_placement_compaction(&self) -> bool {
        self.device.read().supports_placement_compaction()
    }

    /// Frames the placement batch built, for the parity benchmark and the tests;
    /// `view.placement_replay_hits()`.
    #[doc(hidden)]
    pub fn placement_replay_hits(&self) -> u64 {
        self.placement_replay
            .read()
            .as_ref()
            .map_or(0, |batch| batch.hits)
    }
}
