//! Draw-table twins exist only for plain lit surface kinds.
use super::*;

fn plain_lit() -> PipelineVariant {
    let mut kind = PipelineVariant::from_index(0, false);
    kind.lighting = true;
    kind
}

#[test]
fn only_plain_lit_surface_kinds_have_a_twin() {
    assert!(plain_lit_kind(plain_lit()));
    let mut unlit = plain_lit();
    unlit.lighting = false;
    assert!(!plain_lit_kind(unlit));
    let mut clipped = plain_lit();
    clipped.clipped = true;
    assert!(!plain_lit_kind(clipped));
    let mut factorized = plain_lit();
    factorized.factorized_projection = true;
    assert!(
        !plain_lit_kind(factorized),
        "the twin reads no model push constant"
    );
    let mut capping = plain_lit();
    capping.capping = crate::RenderCappingPass::MaskStart;
    assert!(!plain_lit_kind(capping));
}

#[test]
fn the_legacy_table_remains_available_at_the_previous_storage_limit() {
    let mut limits = vk::PhysicalDeviceLimits {
        max_per_stage_descriptor_storage_buffers: 8,
        max_descriptor_set_storage_buffers: 8,
        ..Default::default()
    };
    assert!(!indexed_storage_limits(&limits, true));
    limits.max_per_stage_descriptor_storage_buffers = 9;
    assert!(!indexed_storage_limits(&limits, false));
    assert!(indexed_storage_limits(&limits, true));
    limits.max_descriptor_set_storage_buffers = 9;
    assert!(indexed_storage_limits(&limits, false));
}
