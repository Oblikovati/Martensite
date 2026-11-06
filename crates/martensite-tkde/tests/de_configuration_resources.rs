//! Component coverage for the original configuration fixtures and comparison/value-change
//! helpers. Real format nodes are not substituted; the complete configuration suites need format
//! nodes this kernel does not have, and do not run here.
mod support;
use martensite_tkernel::tcollection::AsciiString;
use support::resource_compare::{change_provider_values, context, same_configuration};

const INITIAL: &str = include_str!("data/wrapper_initial.conf");
const TWO_FORMATS: &str = include_str!("data/wrapper_two_formats.conf");

fn assert_changed_values(value: &str) {
    let changed = context(&change_provider_values(INITIAL, value));
    let original = context(INITIAL);
    assert_eq!(changed.get_internal_map().extent(), 256);
    for (key, original_value) in original.get_internal_map().items() {
        let expected = if key.as_bytes().starts_with(b"provider.") {
            AsciiString::new(value)
        } else {
            original_value.clone()
        };
        assert_eq!(
            changed.get_internal_map().seek(key),
            Some(&expected),
            "key={key:?}"
        );
    }
}

#[test]
fn case_initial_resource_preserves_defaults_and_empty_strings() {
    let parsed = context(INITIAL);
    assert_eq!(parsed.get_internal_map().extent(), 256);
    assert_eq!(
        parsed.integer_val("read.codepage", -1, "provider.STEP.Martensite"),
        4
    );
    assert_eq!(
        parsed.real_val("read.precision.val", -1., "provider.IGES.Martensite"),
        0.0001
    );
    assert_eq!(
        parsed.string_val(
            "write.comment",
            &"missing".into(),
            "provider.OBJ.Martensite"
        ),
        AsciiString::new("")
    );
    assert!(same_configuration(INITIAL, INITIAL));
}

#[test]
fn case_comparison_checks_count_keys_and_exact_values() {
    let reordered = INITIAL.lines().rev().collect::<Vec<_>>().join("\n");
    assert!(same_configuration(INITIAL, &reordered));
    assert!(!same_configuration(INITIAL, TWO_FORMATS));
    assert!(!same_configuration(
        INITIAL,
        &INITIAL.replace("unit :\t 1", "unit :\t 2")
    ));
    assert!(!same_configuration(
        INITIAL,
        &INITIAL.replace("system.unit", "different.unit")
    ));
}

#[test]
fn provider_value_change_helpers_keep_all_global_entries() {
    assert_changed_values("1");
    assert_changed_values("0");
}

#[test]
fn case_two_format_resource_retains_nine_global_priorities() {
    let parsed = context(TWO_FORMATS);
    assert_eq!(parsed.get_internal_map().extent(), 192);
    for format in [
        "VRML", "IGES", "OBJ", "GLTF", "PLY", "XCAF", "STEP", "STL", "BREP",
    ] {
        assert_eq!(
            parsed.string_val(format!("priority.{format}"), &"".into(), "global"),
            AsciiString::new("Martensite")
        );
    }
    assert!(parsed.is_param_set("read.color", "provider.IGES.Martensite"));
    assert!(parsed.is_param_set("read.color", "provider.STEP.Martensite"));
    assert!(!parsed.is_param_set("file.length.unit", "provider.OBJ.Martensite"));
}
