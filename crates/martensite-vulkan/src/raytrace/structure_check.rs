//! GlView::updateRaytraceGeometry in GlGUM_CHECK mode for the
//! structures' modification counters: each ray-traced frame compares the ray-traced layer list
//! and every structure's counter with the states of the last rebuild, and `toUpdateStructure`
//! resets the counter of the first structure (in layer and priority order) that lost its
//! ray-traced geometry. The Vulkan frame rebuilds its geometry from content (`RaytraceBvhCache`),
//! so only this bookkeeping, visible through the structures' DumpJson, is kept here.
use crate::Graphic3dRenderStructure;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::Graphic3dCView;
use std::collections::BTreeMap;

/// One structure's `StructState`: its counter and its instanced structure's counter.
type StructState = (usize, Option<usize>);

/// The per-view states of the last rebuild (`myStructureStates`) and the ray-traced layer list
/// they were built from (`myRaytraceLayerListState`); `RaytraceStructureCheck::default()`.
#[derive(Clone, Debug, Default)]
pub(crate) struct RaytraceStructureCheck {
    /// Ray-traceable structures of the ray-traced layers at the last rebuild, in layer order.
    listed: Vec<usize>,
    states: BTreeMap<usize, StructState>,
}

impl RaytraceStructureCheck {
    /// The check of one ray-traced frame of `view`: whether the native view rebuilds its
    /// geometry, which uploadRaytraceData follows with a restart of the accumulation
    /// (`myAccumFrames = 0`). Example: `let rebuilt = check.check(&*view);`.
    pub fn check(&mut self, view: &dyn Graphic3dCView) -> bool {
        let structures = raytraced_layer_structures(view);
        if structures.is_empty() {
            return false;
        }
        // The native layer list counts ray-traceable structures entering or leaving a layer;
        // a structure that loses its geometry in place does not change it.
        let listed: Vec<usize> = structures
            .iter()
            .filter(|s| s.is_raytracable() || self.states.contains_key(&address(s)))
            .map(address)
            .collect();
        let rebuilt = listed != self.listed || structures.iter().any(|s| self.to_update(s));
        if rebuilt {
            self.rebuild(&structures);
        }
        rebuilt
    }
    /// `toUpdateStructure`: a structure without ray-traced geometry whose counter is positive
    /// had its geometry removed; its counter is reset and the scene rebuilt.
    fn to_update(&self, structure: &Handle<Graphic3dRenderStructure>) -> bool {
        if !structure.is_raytracable() {
            return structure.reset_counted_modification();
        }
        self.states.get(&address(structure)) != Some(&structure.raytrace_struct_state())
    }
    /// The rebuild after a change: the states of every ray-traceable structure.
    fn rebuild(&mut self, structures: &[Handle<Graphic3dRenderStructure>]) {
        let traced = structures.iter().filter(|s| s.is_raytracable());
        self.listed = traced.clone().map(address).collect();
        self.states = traced
            .map(|s| (address(s), s.raytrace_struct_state()))
            .collect();
    }
}

fn address(structure: &Handle<Graphic3dRenderStructure>) -> usize {
    structure.object_address() as usize
}

/// The Vulkan structures of the ray-traced, non-immediate layers by layer and priority.
fn raytraced_layer_structures(view: &dyn Graphic3dCView) -> Vec<Handle<Graphic3dRenderStructure>> {
    let layers = view.layers();
    let traced = layers
        .iter()
        .filter(|layer| layer.layer_settings().is_raytracable() && !layer.is_immediate());
    let priorities = traced.flat_map(|layer| layer.array_of_structures());
    let weak = priorities.flat_map(|priority| priority.iter().cloned().collect::<Vec<_>>());
    weak.map(|weak| weak.upgrade().cast::<Graphic3dRenderStructure>())
        .filter(|structure| !structure.is_null())
        .collect()
}
