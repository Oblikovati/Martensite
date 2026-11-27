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
    /// Append the replayed program of every group to `target` and return the skips they
    /// counted, or `None` with `target` unchanged when the structure or one of its groups
    /// needs lowering; `structure.replayed_groups(scene, &mut target)`. A connected structure
    /// replays the groups of its references first, deepest first, then its own, in the order
    /// and with the model stack of `CappingScene::draws`.
    pub(crate) fn replayed_groups(
        &self,
        (camera, extent, rendering, clipping, pass): StructureFrame<'_>,
        target: &mut StencilProgram,
    ) -> Option<usize> {
        if !replays_in_pass(clipping, pass) {
            return None;
        }
        // Read before this structure's fields: each reference reads its own.
        let batches = self.geometry_batches(Vec::new()).ok()?;
        let fields = self.base.fields.read();
        if fields.highlight || !fields.trsf_pers.is_null() || !fields.clip_planes.is_null() {
            return None;
        }
        let mark = target.mark();
        let mut replay = GroupReplay {
            camera,
            extent,
            scene: (&*fields, rendering, clipping),
            pass,
            carry: crate::group_flipping::FlipCarry::default(),
            skipped: 0,
        };
        let references = batches.into_batches().rev().flatten();
        let replayed = references
            .map(|group| group.cast::<Graphic3dDrawGroup>())
            .chain(fields.groups.iter().map(|slot| slot.cast()))
            .all(|group| replay.group(&group, target));
        if !replayed {
            target.truncate(mark);
            return None;
        }
        Some(replay.skipped)
    }

    /// `replayed_groups` for a structure in a view: `Some(0)` and nothing appended when the
    /// view does not draw it; `structure.replayed_in_view(view_id, scene, &mut target)`.
    pub(crate) fn replayed_in_view(
        &self,
        view_id: i32,
        scene: StructureFrame<'_>,
        target: &mut StencilProgram,
    ) -> Option<usize> {
        if !self.is_visible_in_view(view_id) || self.is_culled() {
            return Some(0);
        }
        self.replayed_groups(scene, target)
    }
}

/// Whether nothing at the view or pass level moves the groups off the camera.
fn replays_in_pass(
    clipping: &Handle<dyn Graphic3dClipPlaneSequenceSource>,
    pass: &RenderElementPass,
) -> bool {
    !cfg!(feature = "vertex-output-diagnostics")
        && clipping.is_null()
        && pass.layer_origin().is_none()
        && pass.lowers_group_geometry()
}

/// The replay of one structure's groups: the model stack each group leaves for the next,
/// and the skips they count.
struct GroupReplay<'a> {
    camera: &'a Graphic3dCamera,
    extent: [u32; 2],
    scene: (
        &'a Graphic3dCStructureFields,
        &'a Graphic3dRenderingParams,
        &'a Handle<dyn Graphic3dClipPlaneSequenceSource>,
    ),
    pass: &'a RenderElementPass,
    carry: crate::group_flipping::FlipCarry,
    skipped: usize,
}

impl GroupReplay<'_> {
    /// Append the replay of `group` to `target`; false when it has no valid record.
    fn group(&mut self, group: &Handle<Graphic3dDrawGroup>, target: &mut StencilProgram) -> bool {
        let Some((skips, next)) = group.get().and_then(|group| {
            let inputs = (self.pass, &self.carry);
            group.replayed_in_structure(self.camera, Some(self.extent), self.scene, inputs, target)
        }) else {
            return false;
        };
        self.skipped += skips;
        self.carry = next;
        true
    }
}

/// Require the replay to equal the lowering path, as group records do under the
/// `draw-record-verification` feature.
#[cfg(feature = "draw-record-verification")]
pub(super) fn verify(replayed: &StencilProgram, lowered: &StencilProgram) {
    assert_eq!(
        replayed.draws, lowered.draws,
        "a structure replayed stale draws"
    );
    assert_eq!(
        replayed.boundaries, lowered.boundaries,
        "a structure replayed stale stencil boundaries"
    );
}
