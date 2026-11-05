//! The whole-registry resource cycles of the native reference: an empty load, at defaults and after
//! other values, and a load of a saved resource with every provider value changed.
use super::probe_node::ProbeNode;
use super::reference_lines::{Lines, b, esc, real, s};
use super::reference_registry::{order, probe_value, registry};
use super::resource_compare::{change_provider_values, context, resource_text, same_configuration};
use martensite_tkde::de::{DeShapeFixConfigurationNode, DeWrapper};
use martensite_tkernel::{ncollection::List, tcollection::AsciiString};

/// What an empty load did to a registry at its defaults.
pub struct EmptyLoadAtDefaults {
    pub result: bool,
    pub same: bool,
    pub bytes: bool,
    pub order: String,
}

/// What an empty load did to a registry that had loaded other values.
pub struct EmptyLoadAfterValues {
    pub result: bool,
    pub same: bool,
    pub order: String,
    pub value: i32,
    pub tolerance: f64,
    pub unit: f64,
}

/// A saved resource with every provider value changed, and the registry after loading it.
pub struct ChangedValues {
    pub changed: String,
    pub saved: String,
    pub keys: usize,
    pub result: bool,
    pub same: bool,
    pub order: String,
    pub value: i32,
    pub tolerance: f64,
    pub node_text: AsciiString,
    pub shallow_same: bool,
}

/// `Save()` with its default arguments.
pub fn save_all(wrapper: &DeWrapper) -> AsciiString {
    let none: List<AsciiString> = List::default();
    wrapper.save(true, &none, &none)
}

fn probe_int(wrapper: &DeWrapper) -> i32 {
    let node = probe_value(wrapper, "STEP", "Martensite").cast::<ProbeNode>();
    node.get().expect("the STEP node").value()
}

fn probe_tolerance(wrapper: &DeWrapper) -> f64 {
    let node = probe_value(wrapper, "STEP", "ACME").cast::<dyn DeShapeFixConfigurationNode>();
    node.get()
        .expect("the ACME node")
        .shape_fix_parameters()
        .tolerance3d
}

pub fn empty_load_at_defaults() -> EmptyLoadAtDefaults {
    let handle = registry();
    let wrapper = handle.get().expect("a wrapper");
    let before = save_all(wrapper);
    let result = wrapper.load(&s(""), true).expect("an empty resource");
    let after = save_all(wrapper);
    EmptyLoadAtDefaults {
        result,
        same: same_configuration(resource_text(&before), resource_text(&after)),
        bytes: before == after,
        order: order(wrapper),
    }
}

pub fn empty_load_after_values() -> EmptyLoadAfterValues {
    let handle = registry();
    let wrapper = handle.get().expect("a wrapper");
    let values = "global.priority.STEP : ACME Martensite\nglobal.general.length.unit : 10\n\
provider.STEP.Martensite.value : 3\nprovider.STEP.ACME.healing.tolerance3d : 0.5\n";
    wrapper.load(&s(values), true).expect("a valid resource");
    let before = save_all(wrapper);
    let result = wrapper.load(&s(""), true).expect("an empty resource");
    EmptyLoadAfterValues {
        result,
        same: same_configuration(resource_text(&before), resource_text(&save_all(wrapper))),
        order: order(wrapper),
        value: probe_int(wrapper),
        tolerance: probe_tolerance(wrapper),
        unit: wrapper.global_parameters().length_unit,
    }
}

/// A load that does not recurse into the nodes must not match, which shows the comparison fails.
fn shallow_load_matches(changed: &str) -> bool {
    let handle = registry();
    let wrapper = handle.get().expect("a wrapper");
    wrapper.load(&s(changed), false).expect("a valid resource");
    same_configuration(changed, resource_text(&save_all(wrapper)))
}

pub fn changed_values(value: &str) -> ChangedValues {
    let handle = registry();
    let wrapper = handle.get().expect("a wrapper");
    let changed = change_provider_values(resource_text(&save_all(wrapper)), value);
    let result = wrapper.load(&s(&changed), true).expect("a valid resource");
    let saved = resource_text(&save_all(wrapper)).to_owned();
    let acme = probe_value(wrapper, "STEP", "ACME");
    ChangedValues {
        keys: context(&changed).get_internal_map().extent(),
        result,
        same: same_configuration(&changed, &saved),
        order: order(wrapper),
        value: probe_int(wrapper),
        tolerance: probe_tolerance(wrapper),
        node_text: acme.get().expect("the ACME node").save(),
        shallow_same: shallow_load_matches(&changed),
        changed,
        saved,
    }
}

fn changed_value_lines(lines: &mut Lines, key: &str, value: &str) {
    let case = changed_values(value);
    lines.line(&format!("roundtrip.{key}.keys"), case.keys);
    lines.line(&format!("roundtrip.{key}.result"), b(case.result));
    lines.line(&format!("roundtrip.{key}.same"), b(case.same));
    lines.line(&format!("roundtrip.{key}.order"), case.order);
    lines.line(&format!("roundtrip.{key}.value"), case.value);
    lines.line(&format!("roundtrip.{key}.tol"), real(case.tolerance));
    lines.line(&format!("roundtrip.{key}.node"), esc(&case.node_text));
    lines.line(
        &format!("roundtrip.{key}.shallow.same"),
        b(case.shallow_same),
    );
}

pub fn round_trip_cases(lines: &mut Lines) {
    let empty = empty_load_at_defaults();
    lines.line("roundtrip.empty.result", b(empty.result));
    lines.line("roundtrip.empty.same", b(empty.same));
    lines.line("roundtrip.empty.bytes", b(empty.bytes));
    lines.line("roundtrip.empty.order", empty.order);
    let kept = empty_load_after_values();
    lines.line("roundtrip.kept.result", b(kept.result));
    lines.line("roundtrip.kept.same", b(kept.same));
    lines.line("roundtrip.kept.order", kept.order);
    lines.line("roundtrip.kept.value", kept.value);
    lines.line("roundtrip.kept.tol", real(kept.tolerance));
    lines.line("roundtrip.kept.unit", real(kept.unit));
    changed_value_lines(lines, "one", "1");
    changed_value_lines(lines, "zero", "0");
}
