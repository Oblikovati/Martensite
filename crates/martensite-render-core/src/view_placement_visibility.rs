//! Pair candidate draws with the ordinary CPU layer visibility and counters (#594).
use super::*;
use std::collections::HashSet;

pub(super) struct VisibilitySources {
    structure_ids: Vec<i32>,
    bounds: Arc<[Option<InstanceBounds>]>,
}

impl VisibilitySources {
    pub(super) fn capture(
        view: &Graphic3dRenderView,
        frame: &crate::RenderFrame,
        templates: &[crate::PrimitiveDraw],
    ) -> Option<Self> {
        let [span] = frame.layers.as_slice() else {
            return None;
        };
        if view
            .layers()
            .iter()
            .any(|layer| layer.nb_structures() != layer.nb_structures_not_culled())
        {
            return None;
        }
        Some(Self {
            structure_ids: draw_source_ids(&span.placement_sources, frame.draws.len())?,
            bounds: templates.iter().map(InstanceBounds::of).collect(),
        })
    }

    pub(super) fn all_visible(&self) -> InstanceVisibility {
        InstanceVisibility {
            visible: vec![true; self.structure_ids.len()],
            bounds: self.bounds.clone(),
        }
    }

    pub(super) fn for_camera(
        &self,
        view: &Graphic3dRenderView,
        camera: &Handle<Graphic3dCamera>,
        extent: [u32; 2],
    ) -> Result<InstanceVisibility, RenderError> {
        let selector = crate::view_scene::culling_selector(view, camera, extent);
        let state = view.rendering_params().ray_tracing.frustum_culling_state;
        let view_id = view.identification();
        let mut visible = HashSet::new();
        for layer in view.layers() {
            layer
                .update_culling(view_id, &selector, state)
                .map_err(|failure| rejected(failure, "ordinary placement visibility"))?;
            visible.extend(visible_structure_ids(&layer, view_id));
        }
        Ok(InstanceVisibility {
            visible: self
                .structure_ids
                .iter()
                .map(|id| visible.contains(id))
                .collect(),
            bounds: self.bounds.clone(),
        })
    }
}

fn draw_source_ids(
    spans: &[crate::stencil_program::StructureDrawSpan],
    draws: usize,
) -> Option<Vec<i32>> {
    let mut sources = Vec::with_capacity(draws);
    for span in spans {
        if span.range.start != sources.len() || span.range.end > draws || span.range.is_empty() {
            return None;
        }
        sources.resize(span.range.end, span.structure);
    }
    (sources.len() == draws).then_some(sources)
}

fn visible_structure_ids(layer: &Graphic3dLayer, view_id: i32) -> impl Iterator<Item = i32> {
    layer
        .structure_handles()
        .into_iter()
        .filter_map(move |handle| {
            let structure = handle.get()?;
            (structure.is_visible_in_view(view_id) && !structure.is_culled())
                .then(|| structure.identification())
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stencil_program::StructureDrawSpan;

    #[test]
    fn one_structure_can_contribute_multiple_draws_but_every_draw_must_have_a_source() {
        let spans = [
            StructureDrawSpan {
                range: 0..2,
                structure: 42,
            },
            StructureDrawSpan {
                range: 2..3,
                structure: 8,
            },
        ];
        assert_eq!(draw_source_ids(&spans, 3), Some(vec![42, 42, 8]));
        assert_eq!(draw_source_ids(&spans[1..], 3), None);
        assert_eq!(draw_source_ids(&spans, 4), None);
    }
}
