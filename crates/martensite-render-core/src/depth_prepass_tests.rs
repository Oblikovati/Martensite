use super::RenderDepthPass::{DepthEqual, DepthOnly, Shaded};
use super::*;
use crate::{PrimitiveDraw, RenderVertex};

const IDENTITY: [f32; 16] = [
    1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
];

fn draw(blended: bool) -> PrimitiveDraw {
    let vertices = vec![RenderVertex::new([0.; 3], [1.; 4]); 3];
    let mut draw = PrimitiveDraw::new(vertices, vec![0, 1, 2], IDENTITY).unwrap();
    draw.surface_blend = blended;
    draw
}

fn layer(clear: bool, member: bool, draws: Vec<PrimitiveDraw>) -> RenderLayer {
    RenderLayer::new(draws, true, true, clear).with_depth_prepass_membership(member)
}

/// (role, draw count, depth write, depth clear) of each expanded layer.
fn shape(layers: &[RenderLayer]) -> Vec<(RenderDepthPass, usize, bool, bool)> {
    layers
        .iter()
        .map(|l| (l.depth_pass, l.draws.len(), l.depth_write, l.clear_depth))
        .collect()
}

#[test]
fn disabled_pre_pass_keeps_the_layers() {
    let layers = vec![layer(true, true, vec![draw(false)])];
    let (out, immediate) = expand_view_layers(layers, 0, false);
    assert_eq!(shape(&out), [(Shaded, 1, true, true)]);
    assert_eq!(immediate, 0);
}

#[test]
fn bunch_runs_depth_only_outside_and_equal_passes_in_native_order() {
    let layers = vec![
        layer(true, true, vec![draw(false), draw(true)]),
        layer(false, false, vec![draw(false)]),
    ];
    let (out, _) = expand_view_layers(layers, 0, true);
    assert_eq!(
        shape(&out),
        [
            // Pass 0 keeps only opaque draws and the bunch's depth clear.
            (DepthOnly, 1, true, true),
            // Pass 1 renders the layer outside the pre-pass normally.
            (Shaded, 1, true, false),
            // Pass 2 renders every layer again without depth writes.
            (DepthEqual, 2, false, false),
            (DepthEqual, 1, false, false),
        ]
    );
}

#[test]
fn depth_clear_starts_a_new_bunch_and_immediate_layers_expand_separately() {
    let layers = vec![
        layer(true, true, vec![draw(false)]),
        layer(true, true, vec![draw(false)]),
        layer(false, true, vec![draw(false)]),
    ];
    let (out, immediate) = expand_view_layers(layers, 1, true);
    assert_eq!(
        shape(&out),
        [
            (DepthOnly, 1, true, true),
            (DepthEqual, 1, false, false),
            (DepthOnly, 1, true, true),
            (DepthEqual, 1, false, false),
            (DepthOnly, 1, true, false),
            (DepthEqual, 1, false, false),
        ]
    );
    assert_eq!(immediate, 2);
}

#[test]
fn flattened_draws_carry_their_role_and_blended_content_stays_shaded() {
    let layers = vec![layer(true, true, vec![draw(false), draw(true)])];
    let (out, _) = expand_view_layers(layers, 0, true);
    let (draws, spans, _) = crate::render_layers::flatten_layers_at(out, None);
    let roles: Vec<_> = draws.iter().map(|d| d.depth_pass).collect();
    assert_eq!(roles, [DepthOnly, DepthEqual, Shaded]);
    assert_eq!(spans.len(), 3);
}
