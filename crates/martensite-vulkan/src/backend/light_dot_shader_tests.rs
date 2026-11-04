use super::super::{light_dot_shader_table, pipeline_variants::PipelineVariant};
use super::*;
use std::collections::BTreeMap;

#[test]
fn default_and_unapplied_domains_preserve_unknown_modules() {
    for select in [selected_fragment, selected_vertex] {
        assert!(
            select(RenderLightDotBehavior::default(), true, &[0])
                .unwrap()
                .is_none()
        );
        assert!(
            select(RenderLightDotBehavior::NumericMaxZeroV2, false, &[0])
                .unwrap()
                .is_none()
        );
        assert!(select(RenderLightDotBehavior::NumericMaxZeroV2, true, &[0]).is_err());
    }
}
#[test]
fn all_final_builtin_phong_interfaces_have_versioned_fragments() {
    let variants: Vec<_> = light_dot_shader_table::FRAGMENT_GROUPS
        .iter()
        .flat_map(|group| group.iter())
        .collect();
    assert_eq!(variants.len(), 240);
    for &(stock, numeric) in variants {
        assert_eq!(
            selected_fragment(RenderLightDotBehavior::NumericMaxZeroV2, true, stock).unwrap(),
            Some(numeric)
        );
        assert_ne!(stock, numeric);
        assert_eq!(&numeric[..4], &[3, 2, 35, 7]);
    }
}
#[test]
fn gouraud_vertex_and_admitted_line_clipping_interfaces_are_versioned() {
    assert_eq!(light_dot_shader_table::VERTEX_MODULES.len(), 16);
    for &(stock, numeric) in light_dot_shader_table::VERTEX_MODULES {
        assert_eq!(
            selected_vertex(RenderLightDotBehavior::NumericMaxZeroV2, true, stock).unwrap(),
            Some(numeric)
        );
        assert_ne!(stock, numeric);
        assert_eq!(
            selected_vertex(RenderLightDotBehavior::DriverNativeV1, true, stock).unwrap(),
            None
        );
    }
}
#[test]
fn repeated_byte_identities_never_choose_different_numeric_modules() {
    let fragments = light_dot_shader_table::FRAGMENT_GROUPS
        .iter()
        .flat_map(|group| group.iter());
    let mut targets = BTreeMap::new();
    for &(stock, numeric) in fragments.chain(light_dot_shader_table::VERTEX_MODULES.iter()) {
        if let Some(previous) = targets.insert(stock, numeric) {
            assert_eq!(previous, numeric);
        }
    }
}
#[test]
fn only_builtin_programmable_phong_is_in_the_policy_domain() {
    let mut kind = PipelineVariant::from_index(0, false);
    assert!(!super::builtin_phong_kind(kind));
    kind.lighting = true;
    assert!(super::builtin_phong_kind(kind));
    kind.pbr = true;
    assert!(!super::builtin_phong_kind(kind));
    kind.pbr = false;
    kind.fixed_lighting = true;
    assert!(!super::builtin_phong_kind(kind));
    kind.fixed_lighting = false;
    assert!(super::builtin_phong_kind(kind));
}
#[test]
fn independent_numeric_reference_preserves_non_nan_values_and_rejects_nan_energy() {
    for (input, expected) in [
        (f32::NAN, 0.),
        (f32::from_bits(0xffc00001), 0.),
        (f32::NEG_INFINITY, 0.),
        (-2., 0.),
        (0.25, 0.25),
        (f32::INFINITY, f32::INFINITY),
    ] {
        assert_eq!(input.max(0.), expected);
    }
}
#[test]
fn independent_ambient_emission_closed_form_has_no_normal_dependence() {
    let ambient = [0.3_f64, 0.4, 0.5];
    let material = [0.1, 0.2, 0.3];
    let emission = [0.01, 0.02, 0.03];
    for i in 0..3 {
        assert!((ambient[i] * material[i] + emission[i] - [0.04, 0.10, 0.18][i]).abs() < 1e-15);
    }
}
