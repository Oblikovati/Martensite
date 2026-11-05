//! Resource operations shared by virtual nodes and concrete format configuration cores.
use super::shape_fix_keys::{AVAILABLE_MODES, FIX_MODE_KEYS, TOLERANCE_KEYS};
use super::{DeConfigurationContext, DeShapeFixParameters, DeShapeFixParametersFixMode};
use martensite_tkernel::tcollection::AsciiString;

/// Updates healing settings in the given full scope; missing values remain unchanged.
/// Example: `load_shape_fix_parameters(&context, &"provider.IGES.Martensite.healing".into(), &mut parameters);`.
pub fn load_shape_fix_parameters(
    context: &DeConfigurationContext,
    scope: &AsciiString,
    parameters: &mut DeShapeFixParameters,
) {
    for entry in &TOLERANCE_KEYS {
        let value: &mut f64 = (entry.field)(parameters);
        *value = context.real_val(entry.key, *value, scope);
    }
    for entry in &FIX_MODE_KEYS {
        let mode: &mut DeShapeFixParametersFixMode = (entry.field)(parameters);
        *mode = DeShapeFixParametersFixMode(context.integer_val(entry.key, mode.0, scope));
    }
}

/// Writes the commented healing resource with a scope ending in a dot.
/// Example: `save_shape_fix_parameters(parameters, &"provider.IGES.Martensite.healing.".into());`.
pub fn save_shape_fix_parameters(
    mut parameters: DeShapeFixParameters,
    scope: &AsciiString,
) -> AsciiString {
    let mut result = AsciiString::new("!\n!Shape healing parameters:\n!\n");
    for entry in &TOLERANCE_KEYS {
        let value = AsciiString::from_real(*(entry.field)(&mut parameters));
        append_resource(&mut result, entry.comment, scope, entry.key, value);
    }
    append_modes(&mut result, &mut parameters, scope);
    result
}
fn append_modes(
    result: &mut AsciiString,
    parameters: &mut DeShapeFixParameters,
    scope: &AsciiString,
) {
    for entry in &FIX_MODE_KEYS {
        let mode = *(entry.field)(parameters);
        let comment = format!(
            "{}!Default value: {}. {AVAILABLE_MODES}",
            entry.description, entry.default_label
        );
        append_resource(
            result,
            &comment,
            scope,
            entry.key,
            AsciiString::from_integer(mode.0),
        );
    }
}
// The exact separators and source comments remain shared with virtual node serialization.
fn append_resource(
    result: &mut AsciiString,
    comment: &str,
    scope: &AsciiString,
    key: &str,
    value: AsciiString,
) {
    *result += "!\n";
    *result += comment;
    *result += scope;
    *result += key;
    *result += " :\t ";
    *result += &value;
    *result += "\n!\n";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::de::{DeSectionGlobal, DeShapeFixConfigurationNodeBase};

    #[test]
    fn owned_resources_keep_missing_values_and_read_raw_modes() {
        let context = DeConfigurationContext::new();
        assert!(context.load_str(&"s.tolerance3d : 0.0254\ns.free.shell : 7\ns.free.face : word\nother.free.wire : 1\n".into()).unwrap());
        let mut parameters = DeShapeFixParameters::default();
        let expected = parameters;
        load_shape_fix_parameters(&context, &"s".into(), &mut parameters);
        assert_eq!(parameters.tolerance3d, 0.0254);
        assert_eq!(parameters.fix_free_shell_mode.0, 7);
        assert_eq!(parameters.fix_free_face_mode, expected.fix_free_face_mode);
        assert_eq!(parameters.fix_free_wire_mode, expected.fix_free_wire_mode);
        assert_eq!(parameters.max_tolerance3d, expected.max_tolerance3d);
    }
    #[test]
    fn owned_serialization_has_all_fifty_two_resources_in_order() {
        let text = save_shape_fix_parameters(DeShapeFixParameters::default(), &"s.".into());
        let context = DeConfigurationContext::new();
        assert!(context.load_str(&text).unwrap());
        assert_eq!(context.get_internal_map().extent(), 52);
        assert_eq!(context.real_val("tolerance3d", 0.0, "s"), 1.0e-6);
        assert_eq!(context.integer_val("free.shell", 9, "s"), -1);
        assert!(
            text.as_bytes()
                .starts_with(b"!\n!Shape healing parameters:\n!\n!\n")
        );
        let mut restored = DeShapeFixParameters::default();
        load_shape_fix_parameters(&context, &"s".into(), &mut restored);
        assert_eq!(restored, DeShapeFixParameters::default());
    }
    #[test]
    fn copied_healing_base_preserves_independent_parameters_and_node_state() {
        let source = DeShapeFixConfigurationNodeBase::default();
        source.configuration_node_base().set_enabled(false);
        let units = DeSectionGlobal {
            length_unit: 2.0,
            system_unit: 3.0,
        };
        source
            .configuration_node_base()
            .set_global_parameters(units);
        let mut parameters = source.shape_fix_parameters();
        parameters.tolerance3d = 0.125;
        source.set_shape_fix_parameters(parameters);
        let copied = source.copied_state();
        source.set_shape_fix_parameters(DeShapeFixParameters::default());
        source.configuration_node_base().set_enabled(true);
        assert_eq!(copied.shape_fix_parameters(), parameters);
        let copied_base = copied.configuration_node_base();
        assert!(!copied_base.is_enabled());
        assert_eq!(copied_base.global_parameters(), units);
    }
}
