//! Pair the batch with the exact layer and primitive submissions.
use super::*;

#[path = "view_batch_layer_sources.rs"]
mod layer_inputs;

pub(crate) struct BatchSceneSources {
    extent: [u32; 2],
    draw_count: usize,
    layers: Vec<layer_inputs::LayerInput>,
    saved: Vec<Option<crate::graphic_group::batch_probe::BatchGraphicInput>>,
    group_id_base: i32,
    groups: Vec<Graphic3dBatchEntry>,
    /// The sources subscribed when this batch was seeded.
    input_scope: GraphicInputRevision,
    validated_revision: Option<u64>,
}

impl BatchSceneSources {
    pub(crate) fn new(extent: [u32; 2], draw_count: usize) -> Self {
        Self {
            extent,
            draw_count,
            layers: Vec::new(),
            saved: Vec::new(),
            group_id_base: 0,
            groups: Vec::new(),
            input_scope: GraphicInputRevision::default(),
            validated_revision: None,
        }
    }
}

impl BatchSceneSources {
    pub(crate) fn commands_expired(&self) -> bool {
        self.saved
            .iter()
            .flatten()
            .any(|source| source.commands_expired())
    }

    fn matches_seed(&self, frame: &RenderFrame) -> bool {
        let [span] = frame.layers.as_slice() else {
            return false;
        };
        let mut groups: std::collections::HashMap<_, _> = self
            .groups
            .iter()
            .map(|entry| (entry.id, &entry.group))
            .collect();
        if groups.len() != self.groups.len() {
            return false;
        }
        let mut remaining = frame.draws.iter();
        let mut source_layer = None;
        for (index, layer) in self.layers.iter().enumerate() {
            if !self.matches_layer_seed(
                index,
                layer,
                span,
                &mut remaining,
                &mut groups,
                &mut source_layer,
            ) {
                return false;
            }
        }
        remaining.next().is_none() && groups.is_empty()
    }

    fn matches_layer_seed(
        &self,
        index: usize,
        layer: &layer_inputs::LayerInput,
        span: &crate::RenderLayerSpan,
        remaining: &mut std::slice::Iter<'_, crate::PrimitiveDraw>,
        groups: &mut std::collections::HashMap<i32, &Handle<dyn Graphic3dGroup>>,
        source_layer: &mut Option<usize>,
    ) -> bool {
        layer.structure_ids().all(|id| {
            let Some(group) = groups.remove(&id) else {
                return true;
            };
            if source_layer.is_none() && !layer.matches_seed_span(span) {
                return false;
            }
            *source_layer.get_or_insert(index) == index
                && remaining
                    .next()
                    .is_some_and(|draw| self.matches_group(id, group, draw))
        })
    }

    fn matches_group(
        &self,
        id: i32,
        group: &Handle<dyn Graphic3dGroup>,
        draw: &crate::PrimitiveDraw,
    ) -> bool {
        let Some(saved) = source_slot(self.group_id_base, id)
            .and_then(|slot| self.saved.get(slot))
            .and_then(Option::as_ref)
        else {
            return false;
        };
        let Some(group) = group.get().and_then(|group| {
            martensite_tkernel::standard::project_concrete::<crate::Graphic3dDrawGroup>(group)
        }) else {
            return false;
        };
        group.matches_batch_seed(saved, draw)
    }
}

pub(crate) fn match_batch_sources(
    view: &dyn Graphic3dCView,
    camera: &Handle<Graphic3dCamera>,
    prepared: &mut BatchSceneSources,
    seed_frame: Option<&RenderFrame>,
    check_connections: Option<&std::sync::Arc<crate::driver_device_lost::DriverObjects>>,
) -> Result<bool, RenderError> {
    let layers = view.layers();
    let register_sources = seed_frame.is_some();
    if view.rendering_params().ray_tracing.frustum_culling_state
        != Graphic3dRenderingParamsFrustumCulling::On
    {
        return Ok(false);
    }
    if register_sources {
        let Some(inputs) = layers
            .iter()
            .map(layer_inputs::LayerInput::capture)
            .collect()
        else {
            return Ok(false);
        };
        prepared.layers = inputs;
    } else if !layer_inputs::same_layers(&layers, &prepared.layers) {
        return Ok(false);
    }
    let mut revision = prepared.input_scope.revision();
    let unwritten = !register_sources && prepared.validated_revision == Some(revision);
    if !unwritten {
        // A source may have been replaced by an equal input since the previous scan.
        prepared.input_scope = GraphicInputRevision::default();
        revision = prepared.input_scope.revision();
    }
    let selector = culling_selector(view, camera, prepared.extent);
    let mut visited = 0;
    prepared.groups.clear();
    for layer in layers {
        let result = layer.collect_batch_groups(
            view.identification(),
            &selector,
            check_connections.is_some(),
            &mut prepared.groups,
        );
        let Some(count) = result.map_err(|error| rejected(error, "valid culling probe"))? else {
            prepared.groups.clear();
            return Ok(false);
        };
        visited += count;
    }
    if register_sources {
        prepared.group_id_base = prepared
            .groups
            .iter()
            .map(|entry| entry.id)
            .min()
            .unwrap_or(0);
    }
    let matched = prepared.groups.iter().all(|entry| {
        check_connections.is_none_or(|driver| locally_owned(entry, driver))
            && register_or_match(
                entry,
                (prepared.group_id_base, prepared.draw_count),
                &mut prepared.saved,
                (register_sources, unwritten, &prepared.input_scope),
            )
    });
    let matches_seed = matched
        && visited == prepared.draw_count
        && seed_frame.is_none_or(|frame| {
            prepared.matches_seed(frame)
                && layer_inputs::same_layers(&view.layers(), &prepared.layers)
        })
        && prepared.input_scope.revision() == revision;
    prepared.groups.clear();
    prepared.validated_revision = matches_seed.then_some(revision);
    Ok(matches_seed)
}

fn source_slot(first: i32, id: i32) -> Option<usize> {
    usize::try_from(id.checked_sub(first)?).ok()
}

fn locally_owned(
    entry: &Graphic3dBatchEntry,
    driver: &std::sync::Arc<crate::driver_device_lost::DriverObjects>,
) -> bool {
    entry
        .structure
        .get()
        .and_then(|structure| {
            martensite_tkernel::standard::project_concrete::<Graphic3dRenderStructure>(structure)
        })
        .is_some_and(|structure| structure.has_only_local_geometry(driver))
}

fn register_or_match(
    entry: &Graphic3dBatchEntry,
    (first, draw_count): (i32, usize),
    saved: &mut Vec<Option<crate::graphic_group::batch_probe::BatchGraphicInput>>,
    (register, unwritten, scope): (bool, bool, &GraphicInputRevision),
) -> bool {
    use martensite_tkernel::standard::project_concrete;
    let Some(group) = entry
        .group
        .get()
        .and_then(|group| project_concrete::<crate::Graphic3dDrawGroup>(group))
    else {
        return false;
    };
    let Some(slot) = source_slot(first, entry.id) else {
        return false;
    };
    if let Some(captured) = saved.get(slot).and_then(Option::as_ref) {
        // Nothing the probe reads was written since every probe last passed.
        if unwritten && captured.is_group(&entry.group) {
            return true;
        }
        return group.observe_group_inputs(scope) && group.probe_unchanged(captured);
    }
    register_group(
        slot,
        draw_count,
        saved,
        (register, scope),
        (group, &entry.group),
    )
}

fn register_group(
    slot: usize,
    draw_count: usize,
    saved: &mut Vec<Option<crate::graphic_group::batch_probe::BatchGraphicInput>>,
    (register, scope): (bool, &GraphicInputRevision),
    (group, handle): (&crate::Graphic3dDrawGroup, &Handle<dyn Graphic3dGroup>),
) -> bool {
    // IDs come from the structure manager. Limit storage by emitted work, even across gaps.
    if !register || slot >= draw_count.saturating_mul(4) || !group.observe_group_inputs(scope) {
        return false;
    }
    if slot >= saved.len() {
        saved.resize_with(slot + 1, || None);
    }
    saved[slot] = group.probe_graphic_input().map(|mut input| {
        input.pin_group(handle);
        input
    });
    saved[slot].is_some()
}
