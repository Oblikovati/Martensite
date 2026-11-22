use martensite_tkernel::standard::{Handle, WeakHandle};
use martensite_tkservice::graphic3d::{
    Graphic3dCStructure, Graphic3dCStructureMembers, Graphic3dLayer, Graphic3dZLayerSettings,
};

pub(super) struct LayerInput {
    layer: WeakHandle<Graphic3dLayer>,
    order: Vec<(usize, WeakHandle<dyn Graphic3dCStructure>)>,
    policy: LayerPolicy,
}

impl LayerInput {
    pub(super) fn capture(layer: &Handle<Graphic3dLayer>) -> Option<Self> {
        let live = layer.get()?;
        Some(Self {
            layer: layer.downgrade(),
            order: live.batch_structure_order(),
            policy: LayerPolicy::capture(&live.layer_settings())?,
        })
    }

    pub(super) fn structure_ids(&self) -> impl Iterator<Item = i32> + '_ {
        self.order.iter().filter_map(|(_, source)| {
            source
                .upgrade()
                .get()
                .map(Graphic3dCStructureMembers::identification)
        })
    }

    pub(super) fn matches_seed_span(&self, span: &crate::RenderLayerSpan) -> bool {
        self.policy.matches_span(span)
    }

    fn matches(&self, layer: &Handle<Graphic3dLayer>) -> bool {
        self.layer.object_address() == layer.object_address()
            && layer.get().is_some_and(|live| {
                LayerPolicy::capture(&live.layer_settings()).as_ref() == Some(&self.policy)
                    && live.matches_batch_structure_order(&self.order)
            })
    }
}

pub(super) fn same_layers(layers: &[Handle<Graphic3dLayer>], inputs: &[LayerInput]) -> bool {
    layers.len() == inputs.len()
        && layers
            .iter()
            .zip(inputs)
            .all(|(layer, input)| input.matches(layer))
}

#[derive(PartialEq)]
struct LayerPolicy {
    offset: (i32, u32, u32),
    culling: [u64; 2],
    render_flags: [bool; 4],
    depth: (bool, bool, bool),
}

impl LayerPolicy {
    fn capture(settings: &Graphic3dZLayerSettings) -> Option<Self> {
        if !settings.lights().is_null() || !settings.origin_transformation().is_null() {
            return None;
        }
        let offset = settings.polygon_offset();
        Some(Self {
            offset: (
                offset.mode.0,
                offset.factor.to_bits(),
                offset.units.to_bits(),
            ),
            culling: [settings.culling_distance(), settings.culling_size()].map(f64::to_bits),
            render_flags: [
                settings.is_immediate(),
                settings.is_raytracable(),
                settings.use_environment_texture(),
                settings.to_render_in_depth_prepass(),
            ],
            depth: (
                settings.to_enable_depth_test(),
                settings.to_enable_depth_write(),
                settings.to_clear_depth(),
            ),
        })
    }

    fn matches_span(&self, span: &crate::RenderLayerSpan) -> bool {
        let (mode, factor, units) = span.polygon_offset().parameters();
        !self.render_flags[0]
            && self.depth == span.depth_policy()
            && self.offset == (mode, factor.to_bits(), units.to_bits())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn batch_seed_refuses_a_layer_policy_that_changed_after_lowering() {
        let mut settings = Graphic3dZLayerSettings::default();
        let original = LayerPolicy::capture(&settings).unwrap();
        let (depth_test, depth_write, clear_depth) = original.depth;
        let span = crate::RenderLayerSpan {
            draw_range: 0..2,
            depth_test,
            depth_write,
            clear_depth,
            polygon_offset: crate::RenderPolygonOffset::new(1, 1., 1.).unwrap(),
            stencil_boundaries: Vec::new(),
        };
        assert!(original.matches_span(&span));
        settings.set_enable_depth_write(!depth_write);
        assert!(!LayerPolicy::capture(&settings).unwrap().matches_span(&span));
    }
}
