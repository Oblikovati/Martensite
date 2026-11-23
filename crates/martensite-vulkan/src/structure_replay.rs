//! Structures whose groups all hold valid draw records replay them directly (#594, retained
//! render list step 1). This skips the structure snapshot, the capping scene and the group
//! batches, which cost about half of a structure's scene time once its groups had records.
//! The group records stay the only proof: one group without a valid record sends the whole
//! structure down the lowering path, before anything of the replay is committed.
use super::Graphic3dRenderStructure;
use crate::Graphic3dDrawGroup;
use crate::render_element_pass::RenderElementPass;
use crate::stencil_program::StencilProgram;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;

/// What a structure draws with in one pass: camera, extent, parameters, view clipping, pass.
pub(crate) type StructureFrame<'a> = (
    &'a Graphic3dCamera,
    [u32; 2],
    &'a Graphic3dRenderingParams,
    &'a Handle<dyn Graphic3dClipPlaneSequenceSource>,
    &'a RenderElementPass,
);

impl Graphic3dRenderStructure {
    /// The replayed program of every group and the skips they counted, or `None` when the
    /// structure or one of its groups needs lowering; `structure.replayed_groups(scene)`.
    pub(crate) fn replayed_groups(
        &self,
        (camera, extent, rendering, clipping, pass): StructureFrame<'_>,
    ) -> Option<(StencilProgram, usize)> {
        if !self.replays_local_groups(clipping, pass) {
            return None;
        }
        let fields = self.base.fields.read();
        if fields.highlight || !fields.trsf_pers.is_null() || !fields.clip_planes.is_null() {
            return None;
        }
        let mut program = StencilProgram::default();
        let mut carry = crate::group_flipping::FlipCarry::default();
        let mut skipped = 0;
        for slot in fields.groups.iter() {
            let group = slot.cast::<Graphic3dDrawGroup>();
            let group = group.get()?;
            // GlWorkspace::ShouldRender: the shadow-map pass skips groups with persistence.
            if pass.skips_trsf_persistence() && !group.transform_persistence().is_null() {
                continue;
            }
            let scene = (&*fields, rendering, clipping);
            let (replayed, skips, next) =
                group.replayed_in_structure(camera, Some(extent), scene, (pass, &carry))?;
            program.append(replayed);
            skipped += skips;
            carry = next;
        }
        Some((program, skipped))
    }

    /// Whether the structure draws only its own groups, in a pass and view that move none of
    /// them off the camera.
    fn replays_local_groups(
        &self,
        clipping: &Handle<dyn Graphic3dClipPlaneSequenceSource>,
        pass: &RenderElementPass,
    ) -> bool {
        !cfg!(feature = "vertex-output-diagnostics")
            && self.parent.is_none()
            && self.connection.read().is_none()
            && clipping.is_null()
            && pass.layer_origin().is_none()
            && pass.lowers_group_geometry()
    }
}

/// Require the replay to equal the lowering path and return the lowered program, as group
/// records do under the `draw-record-verification` feature.
#[cfg(feature = "draw-record-verification")]
pub(super) fn verified(
    replayed: StencilProgram,
    _skipped: usize,
    lowered: Result<StencilProgram, crate::RenderError>,
) -> Result<StencilProgram, crate::RenderError> {
    let lowered = lowered?;
    assert_eq!(
        replayed.draws, lowered.draws,
        "a structure replayed stale draws"
    );
    assert_eq!(
        replayed.boundaries, lowered.boundaries,
        "a structure replayed stale stencil boundaries"
    );
    Ok(lowered)
}
