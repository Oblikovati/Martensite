//! A layer's program from its structures, in their order (#594). Structures whose groups all
//! replay draw records are independent, so worker threads replay chunks of them; a chunk
//! with a structure that needs lowering is assembled again on the calling thread, so lowering,
//! user callbacks and the view stay on that thread. OCCT draws one structure after another.
use crate::{
    RenderError, driver_structure::Graphic3dRenderStructure, error::rejected,
    render_element_pass::RenderElementPass, stencil_program::StencilProgram,
};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::Graphic3dClipPlaneSequenceSource;
use martensite_tkservice::graphic3d::*;
use rayon::prelude::*;

/// Layers with fewer structures assemble on the calling thread; below this, handing chunks to
/// the workers costs more than it saves. Verification builds use the parallel path for every
/// layer of two structures or more, to prove it against the calling-thread assembly.
const PARALLEL_STRUCTURES: usize = if cfg!(feature = "draw-record-verification") {
    2
} else {
    256
};
/// Structures per chunk: enough to amortize a task, few enough to balance the workers.
const CHUNK_STRUCTURES: usize = 64;

/// The inputs every structure of a layer pass draws with.
pub(super) struct StructureInputs<'a> {
    pub camera: &'a Graphic3dCamera,
    pub view_id: i32,
    pub extent: [u32; 2],
    pub rendering: &'a Graphic3dRenderingParams,
    pub clipping: &'a Handle<dyn Graphic3dClipPlaneSequenceSource>,
}

/// The layer's program for `pass`; `layer_program(&layer, &inputs, &pass)?`.
pub(super) fn layer_program(
    layer: &Graphic3dLayer,
    inputs: &StructureInputs<'_>,
    pass: &RenderElementPass,
) -> Result<StencilProgram, RenderError> {
    let structures = layer.structure_handles();
    if structures.len() < PARALLEL_STRUCTURES {
        return sequential_program(&structures, inputs, pass);
    }
    #[cfg(feature = "draw-record-verification")]
    return verified_program(&structures, inputs, pass);
    #[cfg(not(feature = "draw-record-verification"))]
    parallel_program(&structures, inputs, pass)
}

/// Replay each structure on the workers, then assemble the layer on the calling thread and
/// require each replay to equal its structure's part. Replays change no record state, so the
/// calling-thread assembly proves and lowers exactly as it would alone.
#[cfg(feature = "draw-record-verification")]
fn verified_program(
    structures: &[Handle<dyn Graphic3dCStructure>],
    inputs: &StructureInputs<'_>,
    pass: &RenderElementPass,
) -> Result<StencilProgram, RenderError> {
    let replays: Vec<_> = structures
        .par_chunks(1)
        .map(|chunk| replayed_chunk(chunk, inputs, pass))
        .collect();
    let mut program = StencilProgram::default();
    for (structure, replay) in structures.iter().zip(replays) {
        let (draws, boundaries) = program.mark();
        append_structure(structure, inputs, pass, &mut program)?;
        let Some((replayed, _)) = replay else {
            continue;
        };
        assert!(
            replayed.draws.as_slice() == &program.draws[draws..],
            "a parallel replay differs from the calling-thread assembly"
        );
        let shifted = program.boundaries[boundaries..]
            .iter()
            .map(|boundary| (boundary.before_draw - draws, boundary.command));
        assert!(
            replayed
                .boundaries
                .iter()
                .map(|boundary| (boundary.before_draw, boundary.command))
                .eq(shifted),
            "parallel replay boundaries differ"
        );
    }
    Ok(program)
}

/// The layer's structures assembled one after another on the calling thread.
fn sequential_program(
    structures: &[Handle<dyn Graphic3dCStructure>],
    inputs: &StructureInputs<'_>,
    pass: &RenderElementPass,
) -> Result<StencilProgram, RenderError> {
    let mut program = StencilProgram::default();
    for structure in structures {
        append_structure(structure, inputs, pass, &mut program)?;
    }
    Ok(program)
}

/// The layer's structures replayed in chunks on the workers, merged in order; a chunk that
/// needs lowering is assembled again on the calling thread.
#[cfg_attr(feature = "draw-record-verification", allow(dead_code))]
fn parallel_program(
    structures: &[Handle<dyn Graphic3dCStructure>],
    inputs: &StructureInputs<'_>,
    pass: &RenderElementPass,
) -> Result<StencilProgram, RenderError> {
    let mut program = StencilProgram::default();
    let replays: Vec<_> = structures
        .par_chunks(CHUNK_STRUCTURES)
        .map(|chunk| replayed_chunk(chunk, inputs, pass))
        .collect();
    for (chunk, replay) in structures.chunks(CHUNK_STRUCTURES).zip(replays) {
        match replay {
            Some((replayed, skipped)) => {
                pass.add_skipped(skipped);
                program.append(replayed);
            }
            None => {
                for structure in chunk {
                    append_structure(structure, inputs, pass, &mut program)?;
                }
            }
        }
    }
    Ok(program)
}

/// The replayed programs of a chunk of structures and the skips they counted, or `None` when
/// one of them needs lowering. It reads the scene only: records, fields and revisions.
fn replayed_chunk(
    chunk: &[Handle<dyn Graphic3dCStructure>],
    inputs: &StructureInputs<'_>,
    pass: &RenderElementPass,
) -> Option<(StencilProgram, usize)> {
    let mut program = StencilProgram::default();
    let mut skipped = 0;
    let scene = (
        inputs.camera,
        inputs.extent,
        inputs.rendering,
        inputs.clipping,
        pass,
    );
    for structure in chunk {
        if structure.is_null() {
            continue;
        }
        let retained = structure.cast::<Graphic3dRenderStructure>();
        let first = program.draws.len();
        skipped += retained
            .get()?
            .replayed_in_view(inputs.view_id, scene, &mut program)?;
        program.mark_structure(structure.identification(), first);
    }
    Some((program, skipped))
}

/// Append a structure's draws straight to the layer's program.
fn append_structure(
    structure: &Handle<dyn Graphic3dCStructure>,
    inputs: &StructureInputs<'_>,
    pass: &RenderElementPass,
    program: &mut StencilProgram,
) -> Result<(), RenderError> {
    if structure.is_null() {
        return Ok(());
    }
    let retained = structure.cast::<Graphic3dRenderStructure>();
    let scene = (
        inputs.camera,
        inputs.extent,
        inputs.rendering,
        inputs.clipping,
        pass,
    );
    let first = program.draws.len();
    retained
        .get()
        .ok_or_else(|| rejected(structure.object_address(), "a Martensite driver structure"))?
        .draws_into(inputs.view_id, scene, program)?;
    program.mark_structure(structure.identification(), first);
    Ok(())
}
