//! GlFrameStats::updateStructures: groups and elements of the structures of one
//! layer set, following each structure's instanced (connected or shadow) chain.
use crate::Graphic3dDrawGroup;
use crate::Graphic3dRenderStructure as RenderStructure;
use crate::element_stats::CountedElement;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::{
    Graphic3dCStructure, Graphic3dCStructureIndexedMap, Graphic3dCStructureMembers as _,
    Graphic3dFrameStatsCounter as Counter, Graphic3dFrameStatsDataTmp as Counters, Graphic3dGroup,
};

/// What the requested counters need; `StructureCounting { view_id: 0, elements: true, .. }`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct StructureCounting {
    pub view_id: i32,
    pub elements: bool,
    pub detailed: bool,
    pub memory: bool,
}

impl StructureCounting {
    /// Count every structure of a set into the active frame, visiting elements in native order
    /// so user hooks see the counters accumulated so far.
    /// Example: `counting.count(&layer.cullable_structures(), &mut tmp)`.
    pub fn count(&self, structures: &Graphic3dCStructureIndexedMap, tally: &mut Counters) {
        for structure in structures.iter().map(|weak| weak.upgrade()) {
            if let Some(cstructure) = structure.get() {
                self.structure(tally, cstructure, &structure);
            }
        }
    }
    fn structure(
        &self,
        tally: &mut Counters,
        cstructure: &dyn Graphic3dCStructure,
        structure: &Handle<dyn Graphic3dCStructure>,
    ) {
        // Visibility is decided by the outer structure for the whole instanced chain.
        let hidden = cstructure.is_culled() || !cstructure.is_visible_in_view(self.view_id);
        for groups in instanced_group_batches(cstructure, structure) {
            if hidden {
                self.hidden_groups(tally, &groups);
            } else {
                self.visible_groups(tally, &groups);
            }
        }
    }
    fn hidden_groups(&self, tally: &mut Counters, groups: &[Handle<dyn Graphic3dGroup>]) {
        if !self.memory {
            return;
        }
        for element in groups.iter().flat_map(group_elements) {
            element.update_mem_stats(tally);
        }
    }
    fn visible_groups(&self, tally: &mut Counters, groups: &[Handle<dyn Graphic3dGroup>]) {
        let slot = tally.change_counter_value(Counter::NbGroupsNotCulled);
        *slot = slot.wrapping_add(groups.len());
        if !self.elements {
            return;
        }
        for element in groups.iter().flat_map(group_elements) {
            if self.memory {
                element.update_mem_stats(tally);
            }
            element.update_draw_stats(tally, self.detailed);
        }
    }
}

/// The groups of the structure, then of each instanced structure (`InstancedStructure`).
fn instanced_group_batches(
    cstructure: &dyn Graphic3dCStructure,
    structure: &Handle<dyn Graphic3dCStructure>,
) -> Vec<Vec<Handle<dyn Graphic3dGroup>>> {
    let render = structure.cast::<RenderStructure>();
    match render.get() {
        Some(render) => render.instanced_group_batches(),
        None => vec![cstructure.groups()],
    }
}

fn group_elements(group: &Handle<dyn Graphic3dGroup>) -> Vec<CountedElement> {
    group
        .cast::<Graphic3dDrawGroup>()
        .get()
        .map(Graphic3dDrawGroup::element_stats)
        .unwrap_or_default()
}
