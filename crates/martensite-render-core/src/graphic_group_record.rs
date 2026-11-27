//! Group draw records (#594, Phase 3): a group keeps the program its ordinary surfaces lowered
//! to and replays it on later frames while none of the inputs it read changed. Only the fields
//! derived from the camera are computed again. OCCT keeps a group's primitive arrays and binds
//! their aspects again on each frame; lowering every group again cost about 1.5 us per
//! structure, which put `frame_transparent_1k` at 8 times OCCT.
//!
//! A record is kept only when every lowered draw certifies that its camera- and model-derived
//! fields are exactly the ones `ReplayedFields` recomputes, so it replays under any placement
//! of the group's model class (graphic_group_record_fields.rs). The `draw-record-verification` feature lowers
//! the group again on each replay and panics when the two programs differ.
use super::Graphic3dDrawGroup;
use super::commands::GroupCommand;
use crate::RenderError;
use crate::group_flipping::FlipCarry;
use crate::primitive_cache::GeometryStamp;
use crate::render_element_pass::{RecordPassKey, RenderElementPass};
use crate::stencil_program::StencilProgram;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;
use record_sources::{observe_command_inputs, plain_aspects, recorded_sources, sources_match};
use replayed_fields::LazyReplayedFields;
pub(crate) use replayed_fields::ReplayedFields;
use std::sync::Weak;

#[path = "graphic_group_record_sources.rs"]
mod record_sources;
#[path = "graphic_group_record_fields.rs"]
mod replayed_fields;

/// A group's record state: nothing yet, lowered once, or a
/// record kept when the group was lowered again, by a later pass or by another instance of a
/// connected presentation in the same pass.
#[derive(Default)]
pub(super) enum RecordSlot {
    #[default]
    Empty,
    Lowered,
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
    /// Pins the address of the recorded commands, so that no other command list can reuse it.
    _commands: Weak<Vec<GroupCommand>>,
    /// Keeps the recorded base aspects alive for the same reason.
    aspects: Handle<dyn Graphic3dAspectsSource>,
    /// The last pass that kept or replayed this record. A lowering in the same pass is an
    /// instance of another model class (a mirrored placement, say), which must not replace
    /// the record that the other instances of the pass replay.
    pass: std::sync::atomic::AtomicU64,
    /// A revision of the aspects and buffers that this record reads.
    input_scope: Option<GraphicInputRevision>,
    validated_revision: std::sync::atomic::AtomicU64,
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
    /// Whether the structure's transformation mirrors (`structure_mirrored`): the winding is
    /// part of the shared draw body, so a replay cannot set it per placement.
    mirrored: bool,
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

/// The structure, rendering parameters and view clipping a structure lowers its groups with.
pub(crate) type StructureScene<'a> = (
    &'a Graphic3dCStructureFields,
    &'a Graphic3dRenderingParams,
    &'a Handle<dyn Graphic3dClipPlaneSequenceSource>,
);

/// The structure-level inputs of a probe: `(structure, rendering, clipping, pass, extent)`.
pub(super) type ProbeScene<'a> = (
    &'a Graphic3dCStructureFields,
    &'a Graphic3dRenderingParams,
    &'a Handle<dyn Graphic3dClipPlaneSequenceSource>,
    &'a RenderElementPass,
    Option<[u32; 2]>,
);

impl Graphic3dDrawGroup {
    pub(crate) fn observe_group_inputs(&self, scope: &GraphicInputRevision) -> bool {
        self.base.fields.observe(scope);
        self.submissions.observe(scope);
        let (aspects, commands, _) = self.retained_commands();
        observe_command_inputs(scope, &aspects, &commands) && plain_aspects(&aspects, &commands)
    }

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
            let kept = (key, probe.rendering, &current, pass);
            self.keep_record(kept, &program, (skipped, &*carry));
        }
        Ok(program)
    }

    /// The group's recorded program in `structure` with this camera, appended to `target`,
    /// with its skip count and the carry it leaves, or `None` with nothing appended when the
    /// group has no valid record. It never lowers, so a structure can try every group before
    /// it commits to the replayed draws;
    /// `group.replayed_in_structure(&camera, extent, (&fields, &params, &clipping), (&pass, &carry), &mut target)`.
    pub(crate) fn replayed_in_structure(
        &self,
        camera: &Graphic3dCamera,
        extent: Option<[u32; 2]>,
        (structure, rendering, clipping): StructureScene<'_>,
        (pass, carry): (&RenderElementPass, &FlipCarry),
        target: &mut StencilProgram,
    ) -> Option<(usize, FlipCarry)> {
        let persistence =
            extent.map(|raster| crate::render_scale::persistence_extent(raster, rendering));
        let scene = (structure, rendering, clipping, pass, persistence);
        let (probe, current) = self.record_inputs(scene, camera, carry)?;
        self.recorded_program_into(&probe, &current, pass, target)
    }

    /// The probe and camera fields of a group that starts without inherited flips.
    fn record_inputs<'a>(
        &self,
        scene: ProbeScene<'a>,
        camera: &Graphic3dCamera,
        carry: &FlipCarry,
    ) -> Option<(RecordProbe<'a>, LazyReplayedFields<'a>)> {
        if *carry != FlipCarry::default() || !recordable_scene(scene.0, scene.2, scene.3) {
            return None;
        }
        // One read of the group fields gives the record state and the model (#594).
        let (closed, local) = self.recordable_group_state()?;
        let model = crate::persistent_transform::model_with_local(scene.0, &local);
        let probe = self.record_probe(scene, camera, (closed, &local))?;
        let parts = scene.3.record_frame().camera_parts(camera)?;
        Some((probe, LazyReplayedFields::new(parts, scene.0, model, local)))
    }

    /// The record inputs of this group, or `None` when its lowering reads more than ordinary
    /// commands, aspects and geometry; `group.record_probe(scene, &camera, (closed, &local))`.
    fn record_probe<'a>(
        &self,
        (structure, rendering, _, pass, extent): ProbeScene<'a>,
        camera: &Graphic3dCamera,
        (closed, local): (bool, &martensite_tkmath::gp::Transform3),
    ) -> Option<RecordProbe<'a>> {
        let (aspects, commands, effects) = self.retained_commands();
        if effects.flips || effects.calls_user_code {
            return None;
        }
        let scalars = RecordScalars {
            commands: commands.identity()?.get(),
            mirrored: super::extraction::structure_mirrored(structure),
            implicit_stack: structure.trsf.is_null()
                && local.form() == martensite_tkmath::gp::TransformForm::Identity,
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

    /// Whether the group has no persistence and no flipper, and then whether it is closed and
    /// its transformation.
    fn recordable_group_state(&self) -> Option<(bool, martensite_tkmath::gp::Transform3)> {
        let fields = self.base.fields.read();
        (fields.trsf_pers.is_null() && fields.flipper.is_null())
            .then_some((fields.is_closed, fields.trsf))
    }

    /// The recorded program with `camera`, its skip count and the carry it leaves, when the
    /// record's inputs are the probe's.
    fn recorded_program(
        &self,
        probe: &RecordProbe<'_>,
        camera: &LazyReplayedFields<'_>,
        pass: &RenderElementPass,
    ) -> Option<(StencilProgram, usize, FlipCarry)> {
        let mut program = StencilProgram::default();
        let (skipped, carry) = self.recorded_program_into(probe, camera, pass, &mut program)?;
        Some((program, skipped, carry))
    }

    /// `recorded_program` appended to `target`; it appends nothing without a match.
    fn recorded_program_into(
        &self,
        probe: &RecordProbe<'_>,
        camera: &LazyReplayedFields<'_>,
        pass: &RenderElementPass,
        target: &mut StencilProgram,
    ) -> Option<(usize, FlipCarry)> {
        let slot = self.draw_record.read();
        let record = slot.record()?;
        let revision = record
            .input_scope
            .as_ref()
            .map(GraphicInputRevision::revision);
        let unwritten = revision.is_some_and(|revision| {
            record
                .validated_revision
                .load(std::sync::atomic::Ordering::Relaxed)
                == revision
        });
        if record.aspects.object_address() != probe.aspects.object_address()
            || record.key.scalars != probe.scalars
            || !(unwritten || sources_match(&record.key, &probe.aspects, &probe.commands))
            || !pass
                .record_frame()
                .rendering_matches(&record.rendering, probe.rendering)
        {
            return None;
        }
        if let Some(revision) = revision {
            record
                .validated_revision
                .store(revision, std::sync::atomic::Ordering::Relaxed);
        }
        let fields = camera.get()?;
        target.append_patched(&record.program, |draw| fields.patch(draw));
        record.pass.store(
            pass.record_frame().serial(),
            std::sync::atomic::Ordering::Relaxed,
        );
        #[cfg(test)]
        record
            .replays
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Some((record.skipped, camera.carry()))
    }

    /// Keep `program` as this group's record when each draw certifies the replayed fields
    /// and the model class, the lowered carry is the one a replay computes, the commands are
    /// still the probed ones and no aspects read a shader program or textures;
    /// `group.keep_record((key, &rendering, &fields, pass), &program, (0, &carry))`.
    fn keep_record(
        &self,
        (key, rendering, fields, pass): KeptInputs<'_>,
        program: &StencilProgram,
        (skipped, carry): (usize, &FlipCarry),
    ) {
        if !certified(program, &key.scalars, fields) || *carry != fields.carry() {
            return;
        }
        let input_scope = GraphicInputRevision::default();
        self.submissions.observe(&input_scope);
        let (aspects, commands, _) = self.retained_commands();
        let Some(witness) = commands.witness() else {
            return;
        };
        if commands.identity().map(|id| id.get()) != Some(key.scalars.commands)
            || !plain_aspects(&aspects, &commands)
        {
            return;
        }
        let input_scope =
            observe_command_inputs(&input_scope, &aspects, &commands).then_some(input_scope);
        *self.draw_record.write() = RecordSlot::Kept(Box::new(GroupDrawRecord {
            key,
            rendering: pass.record_frame().recorded_rendering(rendering),
            program: program.clone(),
            skipped,
            _commands: witness,
            aspects,
            pass: pass.record_frame().serial().into(),
            input_scope,
            validated_revision: u64::MAX.into(),
            #[cfg(test)]
            replays: Default::default(),
        }));
    }
}

impl Graphic3dDrawGroup {
    /// Whether the group was lowered before and its lowering in this pass may keep a record.
    /// The first lowering only leaves the pass serial, since most groups lowered once are
    /// never lowered again. A later one keeps a record, whether a later pass or another
    /// instance in the same pass lowered it, unless an instance of this pass already kept one.
    fn lowered_before(&self, serial: u64) -> bool {
        let mut slot = self.draw_record.write();
        match &*slot {
            RecordSlot::Lowered => true,
            RecordSlot::Kept(record) => {
                record.pass.load(std::sync::atomic::Ordering::Relaxed) != serial
            }
            RecordSlot::Empty => {
                *slot = RecordSlot::Lowered;
                false
            }
        }
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

/// What `keep_record` stores: the key read before lowering, the parameters, the replayed
/// fields the draws must certify and the pass.
type KeptInputs<'a> = (
    RecordKey,
    &'a Graphic3dRenderingParams,
    &'a LazyReplayedFields<'a>,
    &'a RenderElementPass,
);

/// Whether every draw of `program` certifies the replayed fields and has the key's winding.
fn certified(
    program: &StencilProgram,
    scalars: &RecordScalars,
    fields: &LazyReplayedFields<'_>,
) -> bool {
    let Some(fields) = fields.get() else {
        return false;
    };
    program
        .draws
        .iter()
        .all(|draw| draw.mirrored == scalars.mirrored && fields.certifies(draw))
}

#[cfg(test)]
#[path = "graphic_group_record_tests.rs"]
mod tests;
