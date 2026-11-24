//! Dependencies of one view's retained placement frame (#594).
use super::*;
use std::collections::HashSet;

pub(in super::super) struct PlacementInputs(PlacementStamp);

impl PlacementInputs {
    pub(super) fn observe(view: &Graphic3dRenderView) -> Option<Self> {
        let stamp = PlacementStamp {
            scope: GraphicInputRevision::default(),
            notices: view.retained.main_notices.count(),
        };
        let mut visited = HashSet::new();
        view.layers()
            .iter()
            .all(|layer| observe_layer(layer, &stamp.scope, &mut visited))
            .then_some(Self(stamp))
            .filter(|inputs| inputs.0.current(view))
    }

    pub(super) fn stamp(&self, view: &Graphic3dRenderView) -> Option<PlacementStamp> {
        self.0.current(view).then(|| self.0.clone())
    }
}

fn observe_layer(
    layer: &Graphic3dLayer,
    scope: &GraphicInputRevision,
    visited: &mut HashSet<usize>,
) -> bool {
    layer.observe_scene_inputs(scope);
    layer.structure_handles().into_iter().all(|handle| {
        let structure = handle.cast::<crate::Graphic3dRenderStructure>();
        structure
            .get()
            .is_some_and(|source| source.observe_placement_inputs(scope, visited))
    })
}

#[derive(Clone)]
pub(in super::super) struct PlacementStamp {
    scope: GraphicInputRevision,
    notices: u64,
}

impl PlacementStamp {
    pub(super) fn current(&self, view: &Graphic3dRenderView) -> bool {
        self.scope.revision() == 0 && self.notices == view.retained.main_notices.count()
    }

    pub(super) fn matches(&self, other: &Self, view: &Graphic3dRenderView) -> bool {
        self.scope.same_scope(&other.scope) && self.current(view)
    }
}
