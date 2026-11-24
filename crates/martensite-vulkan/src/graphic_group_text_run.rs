//! Runs of scene text replay their label records on worker threads (#594). A label record
//! reads only its text's revision, its aspects and its kept draws, so the labels of a long run
//! are independent; the calling thread builds the group's text transform once, merges the
//! labels in command order and lowers any label without a valid record itself.
use super::*;
use rayon::prelude::*;

/// Shorter runs replay on the calling thread. Vertex-output diagnostics tag each label as it
/// lowers there; verification builds take this path from two labels on, to prove it.
const PARALLEL_LABELS: usize = if cfg!(feature = "vertex-output-diagnostics") {
    usize::MAX
} else if cfg!(feature = "draw-record-verification") {
    2
} else {
    256
};

/// The label inputs of one command: text, upload slot and record.
type LabelCommand<'a> = (
    &'a Handle<Graphic3dText>,
    &'a crate::element_upload::ElementUploadSlot,
    &'a crate::text_label_record::LabelSlot,
);

/// Labels per worker task; each task reads a private copy of the aspects.
const CHUNK_LABELS: usize = 64;

/// What every label of a run replays with, read once on the calling thread.
struct RunScene<'a> {
    aspects: &'a Handle<dyn Graphic3dAspectsSource>,
    style: crate::text_label_record::LabelStyle,
    transform: crate::text_transform::TextTransform,
    clipping: Option<crate::RenderClipping>,
    culling: crate::RenderFaceCulling,
}

/// How many commands at the start of `commands` are scene text, when that run is long enough
/// to replay on the workers; `parallel_text_run(&commands[index..])`.
pub(super) fn parallel_text_run(commands: &[GroupCommand]) -> usize {
    let run = commands
        .iter()
        .take_while(|command| label_command(command).is_some())
        .count();
    if run >= PARALLEL_LABELS { run } else { 0 }
}

fn label_command(command: &GroupCommand) -> Option<LabelCommand<'_>> {
    match command {
        GroupCommand::Geometry(GroupGeometry::Text(
            text,
            upload,
            crate::render_element::TextPlacement::Scene,
            label,
        )) => Some((text, upload, label)),
        _ => None,
    }
}

impl Graphic3dDrawGroup {
    /// Append a run of scene text in order: labels with valid records replay on the workers,
    /// the others lower here. A transform the context cannot build sends the whole run down
    /// the ordinary path, which reports the same error.
    pub(super) fn append_text_run(
        &self,
        program: &mut StencilProgram,
        run: &[GroupCommand],
        (aspects, flips, policy): RunState<'_>,
        context: &crate::text_submission::TextSceneContext<'_>,
        pass: &RenderElementPass,
    ) -> Result<(), RenderError> {
        let replays = match self.run_scene(aspects, flips, policy, context) {
            Some(scene) => replayed_labels(run, &scene),
            None => vec![None; run.len()],
        };
        for (command, replay) in run.iter().zip(replays) {
            let GroupCommand::Geometry(geometry) = command else {
                continue;
            };
            pass.note_builtin(geometry.is_fill(), || {
                check_is_transparent(aspects, &policy.highlight)
            });
            #[cfg(feature = "draw-record-verification")]
            {
                // The calling-thread assembly decides; a replay must equal its part.
                let start = program.draws.len();
                self.append_geometry(program, geometry, aspects, flips, policy, context)?;
                if let Some(draws) = replay {
                    assert!(
                        draws.as_slice() == &program.draws[start..],
                        "a parallel label replay differs from the calling-thread assembly"
                    );
                }
            }
            #[cfg(not(feature = "draw-record-verification"))]
            match replay {
                Some(draws) => program.ordered_geometry(draws, true),
                None => self.append_geometry(program, geometry, aspects, flips, policy, context)?,
            }
        }
        Ok(())
    }

    fn run_scene<'a>(
        &self,
        aspects: &'a Handle<dyn Graphic3dAspectsSource>,
        flips: &crate::group_flipping::GroupFlipStack,
        policy: &GroupDrawPolicy,
        context: &crate::text_submission::TextSceneContext<'a>,
    ) -> Option<RunScene<'a>> {
        let mut active = *context;
        active.model_override = flips.override_model();
        active.view_override = Some(flips.world_view());
        let closed = self.is_closed() && policy.allow_culling;
        Some(RunScene {
            aspects,
            style: crate::text_label_record::LabelStyle::current(aspects, context.rendering)?,
            transform: active.transform().ok()?,
            clipping: super::super::command_clipping(flips, context.clipping).ok()?,
            culling: crate::text_culling::text_face_culling(aspects.get()?, closed),
        })
    }
}

/// The aspects, flip stack and draw policy a run of commands lowers with.
pub(super) type RunState<'a> = (
    &'a Handle<dyn Graphic3dAspectsSource>,
    &'a crate::group_flipping::GroupFlipStack,
    &'a GroupDrawPolicy,
);

/// Each label's placed draws, or `None` for a label that must lower.
fn replayed_labels(run: &[GroupCommand], scene: &RunScene<'_>) -> Vec<Option<Vec<PrimitiveDraw>>> {
    let labels: Vec<_> = run.iter().filter_map(label_command).collect();
    let chunks: Vec<Vec<_>> = labels
        .par_chunks(CHUNK_LABELS)
        .map(|chunk| {
            let aspects = private_aspects(scene.aspects);
            chunk
                .iter()
                .map(|label| replayed_label(*label, scene, &aspects))
                .collect()
        })
        .collect();
    chunks.into_iter().flatten().collect()
}

/// A copy of `aspects` for one worker. All labels of a run read the same aspects, and their
/// lock's shared counter made the workers wait on each other (#594). The aspects classes
/// add nothing to their fields, so the copy answers every read the same way.
fn private_aspects(
    aspects: &Handle<dyn Graphic3dAspectsSource>,
) -> Handle<dyn Graphic3dAspectsSource> {
    let copy = Graphic3dAspects::default();
    if let Some(source) = aspects.get() {
        *copy.aspects_base().fields.write() = source.aspects_base().fields.snapshot();
    }
    Handle::new(copy).cast()
}

/// A label's kept draws placed, culled and clipped as `GroupGeometry::lower` would leave them.
fn replayed_label(
    (text, upload, label): LabelCommand<'_>,
    scene: &RunScene<'_>,
    aspects: &Handle<dyn Graphic3dAspectsSource>,
) -> Option<Vec<PrimitiveDraw>> {
    let replay = label.unchanged_in(text, &scene.style)?;
    let mut draws =
        crate::text_submission::placed_label(replay, &scene.transform, aspects, Some(upload))
            .ok()??;
    for draw in &mut draws {
        draw.face_culling = scene.culling;
    }
    super::super::apply_clipping(&mut draws, &scene.clipping);
    Some(draws)
}

#[cfg(test)]
mod tests {
    use super::private_aspects;
    use martensite_tkernel::standard::Handle;
    use martensite_tkservice::graphic3d::*;

    #[test]
    fn a_private_copy_answers_the_reads_of_its_source() {
        let source = Graphic3dAspectText3d::default();
        source.set_text_angle(30.);
        source.set_text_display_type(martensite_tkservice::aspect::AspectTypeOfDisplayText::Shadow);
        let source: Handle<dyn Graphic3dAspectsSource> = Handle::new(source).cast();
        let copy = private_aspects(&source);
        let (a, b) = (source.get().unwrap(), copy.get().unwrap());
        assert_eq!(a.text_angle(), b.text_angle());
        assert_eq!(a.text_display_type(), b.text_display_type());
        assert_eq!(a.is_text_zoomable(), b.is_text_zoomable());
        assert_eq!(a.face_culling(), b.face_culling());
        assert_ne!(source.object_address(), copy.object_address());
    }
}
