//! Group draw records (#594, Phase 3): a group keeps the program its ordinary surfaces lowered
//! to and replays it on later frames while none of the inputs it read changed. Only the fields
//! derived from the camera are computed again. OCCT keeps a group's primitive arrays and binds
//! their aspects again on each frame; lowering every group again cost about 1.5 us per
//! structure, which put `frame_transparent_1k` at 8 times OCCT.
//!
//! A record is kept only when every lowered draw certifies that its camera-derived fields are
//! exactly the ones `RecordCamera` recomputes. The `draw-record-verification` feature lowers
//! the group again on each replay and panics when the two programs differ.
use super::Graphic3dDrawGroup;
use super::commands::{GroupCommand, GroupGeometry};
use crate::group_flipping::FlipCarry;
use crate::group_record_frame::RecordCameraParts;
use crate::primitive_cache::GeometryStamp;
use crate::render_element_pass::{RecordPassKey, RenderElementPass};
use crate::stencil_program::StencilProgram;
use crate::{PrimitiveDraw, RenderError, RenderProjectionFactors};
use martensite_tkernel::ncollection::Mat4;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;
use std::sync::Weak;

/// A group's record state: nothing yet, the serial of the one pass that lowered it, or a
/// record kept when a second pass lowered it.
#[derive(Default)]
pub(super) enum RecordSlot {
    #[default]
    Empty,
    Lowered(u64),
    /// Lowered more than once in one pass: a connected presentation draws the group under
    /// many transforms, and a record would help only the first of them.
    Instanced,
    /// Boxed, so that the slot of a group without a record stays two words.
    Kept(Box<GroupDrawRecord>),
}

impl RecordSlot {
    fn record(&self) -> Option<&GroupDrawRecord> {
        match self {
            Self::Kept(record) => Some(record),
            _ => None,
        }
    }
}

/// The program a group lowered to on an earlier frame, with the inputs it read.
pub(super) struct GroupDrawRecord {
    key: RecordKey,
    /// Shared by the records built in one pass (`RecordFrame::recorded_rendering`).
    rendering: std::sync::Arc<Graphic3dRenderingParams>,
    program: StencilProgram,
    /// The filling arrays the lowering counted as skipped by the opaque pass.
    skipped: usize,
    /// The model stack the lowering left for the structure's next group; it holds model
    /// matrices only, so it does not depend on the camera.
    carry: FlipCarry,
    /// Pins the address of the recorded commands, so that no other command list can reuse it.
    _commands: Weak<Vec<GroupCommand>>,
    /// Keeps the recorded base aspects alive for the same reason.
    _aspects: Handle<dyn Graphic3dAspectsSource>,
    /// The last pass that kept or replayed this record. Another lowering in the same pass is
    /// another instance of the group (a connected presentation draws one group under many
    /// transforms), which must not replace the record on every instance.
    pass: std::sync::atomic::AtomicU64,
    #[cfg(test)]
    replays: std::sync::atomic::AtomicUsize,
}

/// The inputs of a group's lowering that do not come from the camera.
pub(super) struct RecordKey {
    scalars: RecordScalars,
    /// The address and write revision of the base aspects, then of each aspects command.
    aspects: Vec<(usize, u64)>,
    geometry: Vec<GeometryStamp>,
}

/// The fixed-size record inputs, compared on every frame without allocating.
#[derive(PartialEq)]
pub(super) struct RecordScalars {
    commands: usize,
    model: [u64; 16],
    /// Whether the structure has no transformation and the group an identity one: equal
    /// model matrices still leave different model stacks for the next group otherwise
    /// (`group_flipping::can_keep_implicit_stack`).
    implicit_stack: bool,
    closed: bool,
    perspective: bool,
    extent: Option<[u32; 2]>,
    pass: RecordPassKey,
    surface: (
        crate::RenderLightingPipeline,
        crate::RenderUnlitFaceColorPolicy,
    ),
}

/// A group's record inputs on this frame; `rendering` is compared without a copy, and the
/// aspects and geometry are compared in place against a record's.
pub(super) struct RecordProbe<'a> {
    scalars: RecordScalars,
    rendering: &'a Graphic3dRenderingParams,
    aspects: Handle<dyn Graphic3dAspectsSource>,
    commands: crate::draw::DrawStream<GroupCommand>,
}

/// The camera-derived fields of an ordinary surface draw.
#[derive(Clone, Copy)]
pub(super) struct RecordCamera {
    clip: [f32; 16],
    depth_clamp: bool,
    view: [f32; 16],
    eye: [f32; 4],
    factors: Option<RenderProjectionFactors>,
    normalize: bool,
}

/// The structure-level inputs of a probe: `(structure, rendering, clipping, pass, extent)`.
pub(super) type ProbeScene<'a> = (
    &'a Graphic3dCStructureFields,
    &'a Graphic3dRenderingParams,
    &'a Handle<dyn Graphic3dClipPlaneSequenceSource>,
    &'a RenderElementPass,
    Option<[u32; 2]>,
);

impl Graphic3dDrawGroup {
    /// The group's program in `structure`: its record replayed with the current camera, or
    /// the commands lowered again and recorded;
    /// `group.program_in_structure(&camera, extent, &fields, &params, &clipping, (&pass, &mut carry))?`.
    pub(crate) fn program_in_structure(
        &self,
        camera: &Graphic3dCamera,
        extent: Option<[u32; 2]>,
        structure: &Graphic3dCStructureFields,
        rendering: &Graphic3dRenderingParams,
        clipping: &Handle<dyn Graphic3dClipPlaneSequenceSource>,
        (pass, carry): (&RenderElementPass, &mut FlipCarry),
    ) -> Result<StencilProgram, RenderError> {
        let persistence =
            extent.map(|raster| crate::render_scale::persistence_extent(raster, rendering));
        let scene = (structure, rendering, clipping, pass, persistence);
        let lower = |carry: &mut FlipCarry| {
            self.lower_in_structure(
                camera,
                extent,
                structure,
                rendering,
                clipping,
                (pass, carry),
            )
        };
        let Some((probe, current)) = self.record_inputs(scene, camera, carry) else {
            return lower(carry);
        };
        if let Some(replayed) = self.recorded_program(&probe, &current, pass) {
            #[cfg(feature = "draw-record-verification")]
            return verified_replay(replayed, pass, || {
                lower(carry).map(|program| (program, carry.clone()))
            });
            #[cfg(not(feature = "draw-record-verification"))]
            {
                let (program, skipped, recorded_carry) = replayed;
                pass.add_skipped(skipped);
                *carry = recorded_carry;
                return Ok(program);
            }
        }
        // Only a group's first lowering in a pass, after another pass lowered it, may keep a
        // record; the sources are read before lowering, so a write that ends during the
        // lowering cannot match later.
        let sources = self
            .lowered_before(pass.record_frame().serial())
            .then(|| recorded_sources(&probe.aspects, &probe.commands))
            .flatten();
        let before = pass.skipped();
        let program = lower(carry)?;
        let skipped = pass.skipped().saturating_sub(before);
        if let Some((aspects, geometry)) = sources {
            let key = RecordKey {
                scalars: probe.scalars,
                aspects,
                geometry,
            };
            let kept = (key, probe.rendering, &current.get(), pass);
            self.keep_record(kept, &program, (skipped, carry.clone()));
        }
        Ok(program)
    }

    /// The probe and camera fields of a group that starts without inherited flips.
    fn record_inputs<'a>(
        &self,
        scene: ProbeScene<'a>,
        camera: &Graphic3dCamera,
        carry: &FlipCarry,
    ) -> Option<(RecordProbe<'a>, LazyCamera<'a>)> {
        if *carry != FlipCarry::default() {
            return None;
        }
        let model = crate::persistent_transform::model_transform(self, scene.0);
        let probe = self.record_probe(scene, camera, &model)?;
        let parts = scene.3.record_frame().camera_parts(camera).ok()?;
        Some((probe, LazyCamera::new(parts, scene.0, model)))
    }

    /// The record inputs of this group, or `None` when its lowering reads more than ordinary
    /// commands, aspects and geometry; `group.record_probe(scene, &camera, &model)`.
    fn record_probe<'a>(
        &self,
        (structure, rendering, clipping, pass, extent): ProbeScene<'a>,
        camera: &Graphic3dCamera,
        model: &Mat4<f64>,
    ) -> Option<RecordProbe<'a>> {
        if !recordable_scene(structure, clipping, pass) {
            return None;
        }
        let closed = self.recordable_group_state()?;
        let (aspects, commands, effects) = self.retained_commands();
        if effects.flips || effects.calls_user_code {
            return None;
        }
        let scalars = RecordScalars {
            commands: commands.identity()?.get(),
            model: model.data().map(f64::to_bits),
            implicit_stack: structure.trsf.is_null()
                && self.transformation().form() == martensite_tkmath::gp::TransformForm::Identity,
            closed,
            perspective: !camera.is_orthographic(),
            extent,
            pass: pass.record_key(),
            surface: pass.record_frame().surface_policies(&self.structure()),
        };
        Some(RecordProbe {
            scalars,
            rendering,
            aspects,
            commands,
        })
    }

    /// Whether the group has no persistence and no flipper, and then whether it is closed.
    fn recordable_group_state(&self) -> Option<bool> {
        let fields = self.base.fields.read();
        (fields.trsf_pers.is_null() && fields.flipper.is_null()).then_some(fields.is_closed)
    }

    /// The recorded program with `camera`, its skip count and the carry it leaves, when the
    /// record's inputs are the probe's.
    fn recorded_program(
        &self,
        probe: &RecordProbe<'_>,
        camera: &LazyCamera<'_>,
        pass: &RenderElementPass,
    ) -> Option<(StencilProgram, usize, FlipCarry)> {
        let slot = self.draw_record.read();
        let record = slot.record()?;
        if record.key.scalars != probe.scalars
            || !sources_match(&record.key, &probe.aspects, &probe.commands)
            || !pass
                .record_frame()
                .rendering_matches(&record.rendering, probe.rendering)
        {
            return None;
        }
        let mut program = record.program.clone();
        let camera = camera.get();
        program.draws.iter_mut().for_each(|draw| camera.patch(draw));
        record.pass.store(
            pass.record_frame().serial(),
            std::sync::atomic::Ordering::Relaxed,
        );
        #[cfg(test)]
        record
            .replays
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Some((program, record.skipped, record.carry.clone()))
    }

    /// Keep `program` as this group's record when each draw certifies `camera`, the commands
    /// are still the probed ones and no aspects read a shader program or textures;
    /// `group.keep_record((probe, &camera, pass), &program, (0, carry))`.
    fn keep_record(
        &self,
        (key, rendering, camera, pass): KeptInputs<'_>,
        program: &StencilProgram,
        (skipped, carry): (usize, FlipCarry),
    ) {
        if !program.draws.iter().all(|draw| camera.certifies(draw)) {
            return;
        }
        let (aspects, commands, _) = self.retained_commands();
        let Some(witness) = commands.witness() else {
            return;
        };
        if commands.identity().map(|id| id.get()) != Some(key.scalars.commands)
            || !plain_aspects(&aspects, &commands)
        {
            return;
        }
        *self.draw_record.write() = RecordSlot::Kept(Box::new(GroupDrawRecord {
            key,
            rendering: pass.record_frame().recorded_rendering(rendering),
            program: program.clone(),
            skipped,
            carry,
            _commands: witness,
            _aspects: aspects,
            pass: pass.record_frame().serial().into(),
            #[cfg(test)]
            replays: Default::default(),
        }));
    }
}

impl Graphic3dDrawGroup {
    /// Whether this is the group's first lowering in this pass and another pass lowered it
    /// before. The first lowering only leaves the pass serial, since most groups lowered once
    /// are never lowered again; a second lowering in the same pass is another instance.
    fn lowered_before(&self, serial: u64) -> bool {
        let mut slot = self.draw_record.write();
        let again_in_pass = match &*slot {
            RecordSlot::Lowered(previous) => *previous == serial,
            RecordSlot::Kept(record) => {
                record.pass.load(std::sync::atomic::Ordering::Relaxed) == serial
            }
            RecordSlot::Instanced => return false,
            RecordSlot::Empty => {
                *slot = RecordSlot::Lowered(serial);
                return false;
            }
        };
        if again_in_pass {
            *slot = RecordSlot::Instanced;
        }
        !again_in_pass
    }
}

/// Lower the group again for a replayed program and panic when the two differ: the
/// `draw-record-verification` proof that a record's inputs cover everything lowering reads.
#[cfg(feature = "draw-record-verification")]
fn verified_replay(
    (replayed, recorded_skips, recorded_carry): (StencilProgram, usize, FlipCarry),
    pass: &RenderElementPass,
    lower: impl FnOnce() -> Result<(StencilProgram, FlipCarry), RenderError>,
) -> Result<StencilProgram, RenderError> {
    let before = pass.skipped();
    let (lowered, carry) = lower()?;
    assert_eq!(
        replayed.draws, lowered.draws,
        "a draw record replayed stale draws"
    );
    assert_eq!(
        replayed.boundaries, lowered.boundaries,
        "stale stencil boundaries"
    );
    assert_eq!(
        recorded_skips,
        pass.skipped() - before,
        "stale opaque-pass skip count"
    );
    assert_eq!(
        recorded_carry, carry,
        "stale model stack for the next group"
    );
    Ok(lowered)
}

/// Whether nothing at the structure, layer or view level moves the group off the camera:
/// no highlight, persistence, clipping or layer origin.
fn recordable_scene(
    structure: &Graphic3dCStructureFields,
    clipping: &Handle<dyn Graphic3dClipPlaneSequenceSource>,
    pass: &RenderElementPass,
) -> bool {
    !structure.highlight
        && structure.trsf_pers.is_null()
        && structure.clip_planes.is_null()
        && clipping.is_null()
        && pass.layer_origin().is_none()
        && pass.lowers_group_geometry()
}

/// The aspects addresses with their revisions, and the geometry stamps, of a group.
type RecordedSources = (Vec<(usize, u64)>, Vec<GeometryStamp>);

/// What `keep_record` stores: the key read before lowering, the parameters, the camera
/// fields the draws must certify and the pass.
type KeptInputs<'a> = (
    RecordKey,
    &'a Graphic3dRenderingParams,
    &'a RecordCamera,
    &'a RenderElementPass,
);

/// Whether the base aspects and the commands still have the record's aspects revisions and
/// geometry stamps, compared in order without building new lists.
fn sources_match(
    key: &RecordKey,
    base: &Handle<dyn Graphic3dAspectsSource>,
    commands: &[GroupCommand],
) -> bool {
    let (mut aspects, mut geometry) = (key.aspects.iter(), key.geometry.iter());
    if aspects.next() != aspects_revision(base).as_ref() {
        return false;
    }
    for command in commands {
        let same = match command {
            GroupCommand::Aspects(style) => aspects.next() == aspects_revision(style).as_ref(),
            GroupCommand::Stencil(_) => true,
            GroupCommand::Geometry(GroupGeometry::Primitives(primitive)) => {
                geometry.next() == GeometryStamp::capture(primitive).as_ref()
            }
            _ => false,
        };
        if !same {
            return false;
        }
    }
    aspects.next().is_none() && geometry.next().is_none()
}

/// The aspects revisions and geometry stamps of commands that hold only aspects, stencil
/// switches and primitive arrays.
fn recorded_sources(
    base: &Handle<dyn Graphic3dAspectsSource>,
    commands: &[GroupCommand],
) -> Option<RecordedSources> {
    let mut aspects = vec![aspects_revision(base)?];
    let mut geometry = Vec::new();
    for command in commands {
        match command {
            GroupCommand::Aspects(style) => aspects.push(aspects_revision(style)?),
            GroupCommand::Stencil(_) => {}
            GroupCommand::Geometry(GroupGeometry::Primitives(primitive)) => {
                geometry.push(GeometryStamp::capture(primitive)?);
            }
            _ => return None,
        }
    }
    Some((aspects, geometry))
}

/// The address and write revision of aspects. A write that changes their fields advances the
/// revision, so `plain_aspects` needs to read the fields only when a record is kept.
fn aspects_revision(style: &Handle<dyn Graphic3dAspectsSource>) -> Option<(usize, u64)> {
    let revision = style.get()?.aspects_base().fields.revision();
    Some((style.object_address().addr(), revision))
}

/// Whether no aspects of the group read a shader program or textures, whose own contents
/// change without an aspects revision.
fn plain_aspects(base: &Handle<dyn Graphic3dAspectsSource>, commands: &[GroupCommand]) -> bool {
    let styles = commands.iter().filter_map(|command| match command {
        GroupCommand::Aspects(style) => Some(style),
        _ => None,
    });
    std::iter::once(base).chain(styles).all(|style| {
        style.get().is_some_and(|source| {
            let fields = source.aspects_base().fields.read();
            fields.program.is_null() && fields.texture_set.is_null() && !fields.to_map_texture
        })
    })
}

/// The camera fields of a group, computed only when a replay or a kept record needs them.
pub(super) struct LazyCamera<'a> {
    parts: RecordCameraParts,
    structure: &'a Graphic3dCStructureFields,
    model: Mat4<f64>,
    fields: std::cell::OnceCell<RecordCamera>,
}

impl<'a> LazyCamera<'a> {
    fn new(
        parts: RecordCameraParts,
        structure: &'a Graphic3dCStructureFields,
        model: Mat4<f64>,
    ) -> Self {
        Self {
            parts,
            structure,
            model,
            fields: std::cell::OnceCell::new(),
        }
    }
    fn get(&self) -> RecordCamera {
        *self
            .fields
            .get_or_init(|| RecordCamera::current(&self.parts, self.structure, self.model))
    }
}

impl RecordCamera {
    /// The camera-derived fields of a group without persistence, as
    /// `persistent_transform::ordinary_clip`, `retained_camera` and
    /// `native_normalize_state` compute them; `RecordCamera::current(&parts, &fields, model)`.
    fn current(
        parts: &RecordCameraParts,
        structure: &Graphic3dCStructureFields,
        model: Mat4<f64>,
    ) -> Self {
        let model = Mat4::<f32>::from_other(&model);
        let view = parts.view.data();
        Self {
            clip: (parts.projection_view * model).data(),
            depth_clamp: false,
            view,
            eye: parts.eye,
            factors: parts
                .factors
                .and_then(|factors| factors.with_model_world(model.data())),
            normalize: !parts.unit_axial_scale
                || crate::fixed_normalization::structure_scale_normalizes(structure),
        }
    }

    /// Whether `draw` is an ordinary surface whose camera-derived fields are these.
    fn certifies(&self, draw: &PrimitiveDraw) -> bool {
        ordinary_surface(draw)
            && bits(&draw.clip_transform) == bits(&self.clip)
            && draw.depth_clamp == self.depth_clamp
            && draw
                .projection_factors
                .is_none_or(|f| Some(f) == self.factors)
            && draw
                .fixed_function_factors
                .is_none_or(|f| Some(f) == self.factors)
            && draw.lighting.as_ref().is_none_or(|lighting| {
                bits(&lighting.view) == bits(&self.view)
                    && lighting.eye.map(f32::to_bits) == self.eye.map(f32::to_bits)
                    && lighting.normalize_fixed == self.normalize
            })
    }

    /// Set the camera-derived fields of a recorded draw.
    fn patch(&self, draw: &mut PrimitiveDraw) {
        draw.clip_transform = self.clip;
        draw.depth_clamp = self.depth_clamp;
        if draw.projection_factors.is_some() {
            draw.projection_factors = self.factors;
        }
        if draw.fixed_function_factors.is_some() {
            draw.fixed_function_factors = self.factors;
        }
        if let Some(lighting) = &mut draw.lighting {
            lighting.view = self.view;
            lighting.eye = self.eye;
            lighting.normalize_fixed = self.normalize;
        }
    }
}

fn bits(matrix: &[f32; 16]) -> [u32; 16] {
    matrix.map(f32::to_bits)
}

/// A triangle draw with no state that lowering derives from the camera besides the fields
/// `RecordCamera` recomputes: no lines, points, text, textures, edges or user program.
fn ordinary_surface(draw: &PrimitiveDraw) -> bool {
    draw.topology() == crate::PrimitiveTopology::Triangles
        && draw.line_style.is_none()
        && draw.point_style.is_none()
        && draw.projected_dash.is_none()
        && draw.guard_band_source.is_none()
        && draw.quad_corners.is_empty()
        && draw.mesh_edges.is_none()
        && draw.polygon_edge_style.is_none()
        && draw.textured_style.is_none()
        && draw.surface_texture.is_none()
        && draw.user_program.is_none()
        && !draw.annotation
        && !draw.stencil_rect
}

#[cfg(test)]
#[path = "graphic_group_record_tests.rs"]
mod tests;
