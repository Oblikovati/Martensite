//! Shared source-faithful command-host naming command construction and acceptance.
use super::{
    naming_box_driver::real_argument,
    naming_draw_observations::{
        NamingShapeBindings, new_command, observe_reference, observe_shape,
    },
    naming_draw_selection::{SelectionBatch, setup},
    naming_function_graph::{add_function, add_object},
};
use martensite_tkernel::standard::Handle;
use martensite_tklcaf::{
    guid_from_text,
    tdata_std::TDataStdUAttribute,
    tdf::TdfAttributeState,
    tdoc_std::TDocStdDocument,
    tfunction::{TFunctionDriverTable, TFunctionFunction, TFunctionLogbook},
};
pub(super) fn function(
    object: &Handle<TDataStdUAttribute>,
    suffix: &str,
    name: &str,
) -> Handle<TFunctionFunction> {
    let function = add_function(
        object,
        &guid_from_text(&format!("12e945{suffix}-6dbc-11d4-b9c8-0060b0ee281b")),
    )
    .unwrap();
    setup::name_function(&function, name).unwrap();
    function
}
pub(super) fn reals(function: &Handle<TFunctionFunction>, values: [f64; 3]) {
    for (position, value) in values.into_iter().enumerate() {
        real_argument(function, position as i32 + 1)
            .unwrap()
            .set_value(value)
            .unwrap();
    }
}
pub(super) fn compute(table: &Handle<TFunctionDriverTable>, function: &Handle<TFunctionFunction>) {
    let mut log = TFunctionLogbook::set(&function.label()).unwrap();
    assert_eq!(
        setup::compute_function(table, function, &mut log).unwrap(),
        0
    );
}
pub(super) fn initialize_logbook(function: &Handle<TFunctionFunction>) {
    TFunctionLogbook::set(&function.label())
        .unwrap()
        .clear()
        .unwrap();
}
pub(super) fn initial_box_reads(
    bindings: &mut NamingShapeBindings,
    object: &Handle<TDataStdUAttribute>,
    function: &Handle<TFunctionFunction>,
    first: bool,
) {
    let name = if first { "Box2" } else { "Box" };
    box_reads(
        bindings,
        object,
        function,
        name,
        first.then_some("0:1:1:1:2"),
    );
}
/// Preserve actual GetShape, whatis and optional reference observation; `box_reads(&mut bindings,&object,&function,"Box",None)`.
pub(super) fn box_reads(
    bindings: &mut NamingShapeBindings,
    object: &Handle<TDataStdUAttribute>,
    function: &Handle<TFunctionFunction>,
    name: &'static str,
    expected_reference: Option<&str>,
) {
    bindings.bind(function, name).unwrap();
    observed_box_reads(bindings, object, name, expected_reference);
}

fn observed_box_reads(
    bindings: &NamingShapeBindings,
    object: &Handle<TDataStdUAttribute>,
    name: &str,
    expected_reference: Option<&str>,
) {
    observe_shape(bindings, name, "shape SOLID FORWARD");
    if let Some(expected) = expected_reference {
        observe_reference(object, expected);
    }
}
pub(super) type ModeledObject = (Handle<TDataStdUAttribute>, Handle<TFunctionFunction>);
pub(super) fn prepare_box(
    document: &Handle<TDocStdDocument>,
    sizes: [f64; 3],
    initialize_log: bool,
) -> ModeledObject {
    prepare_box_with_setup(document, sizes, initialize_log, || {})
}

fn prepare_box_with_setup(
    document: &Handle<TDocStdDocument>,
    sizes: [f64; 3],
    initialize_log: bool,
    setup: impl FnOnce(),
) -> ModeledObject {
    new_command(document);
    setup();
    let object = add_object(document).unwrap();
    let function = function(&object, "43", "Box_Function");
    reals(&function, sizes);
    if initialize_log {
        initialize_logbook(&function);
    }
    (object, function)
}
pub(super) fn assert_no_errors(batch: &SelectionBatch) {
    assert!(batch.failed_shapes.is_empty());
    assert!(
        batch
            .diagnostics
            .iter()
            .all(|message| message.contains("Warning") && !parse_rules_fail(message)),
        "{:?}",
        batch.diagnostics
    );
    assert!(!batch.first.is_null());
}

/// Whether a logged line fails the case by tests/caf/parse.rules (`\binvalid\b`) or the
/// common tests/parse.rules bad words (`exception`, `error`, `fail<digit>`, `failed`,
/// `failure`, case-insensitive whole words, and the process-limit kills); the selection
/// commands log their diagnostics, so `parse_rules_fail(" Warning => ...") == false`.
pub(super) fn parse_rules_fail(line: &str) -> bool {
    if line.contains("Process killed by CPU limit")
        || line.contains("Process killed by elapsed limit")
    {
        return true;
    }
    let lower = line.to_lowercase();
    lower
        .split(|c: char| !(c.is_alphanumeric() || c == '_'))
        .any(|word| {
            matches!(
                word,
                "invalid" | "exception" | "error" | "failed" | "failure"
            ) || (word.len() == 5
                && word.starts_with("fail")
                && word.as_bytes()[4].is_ascii_digit())
        })
}

#[test]
fn caf_parse_rules_pass_warnings_and_fail_bad_words() {
    assert!(!parse_rules_fail(
        " Warning => Type migration to Compound at 0:1:223"
    ));
    assert!(!parse_rules_fail("invalidated errors_count fail12"));
    assert!(parse_rules_fail(
        " Selection at label = 0:1:5 failed, shape type = EDGE"
    ));
    assert!(parse_rules_fail("Shape is INVALID"));
    assert!(parse_rules_fail("fail1"));
    assert!(parse_rules_fail("Process killed by CPU limit (300 sec)"));
}

pub(super) fn initial_box(
    table: &Handle<TFunctionDriverTable>,
    document: &Handle<TDocStdDocument>,
    bindings: &mut NamingShapeBindings,
    sizes: [f64; 3],
    initialize_log: bool,
) -> ModeledObject {
    let (object, function) = prepare_box(document, sizes, initialize_log);
    compute(table, &function);
    initial_box_reads(bindings, &object, &function, initialize_log);
    (object, function)
}

/// Shared original box dimensions; `standard_box(&table,&document,&mut bindings)`.
pub(super) fn standard_box(
    table: &Handle<TFunctionDriverTable>,
    document: &Handle<TDocStdDocument>,
    bindings: &mut NamingShapeBindings,
    fit: bool,
) -> ModeledObject {
    let (object, function) = prepare_standard_box(document, bindings, fit);
    let presentation = bindings.presentation();
    compute(table, &function);
    bindings.bind(&function, "Box2").unwrap();
    if fit {
        presentation.fit();
    }
    observed_box_reads(bindings, &object, "Box2", Some("0:1:1:1:2"));
    (object, function)
}

fn prepare_standard_box(
    document: &Handle<TDocStdDocument>,
    bindings: &NamingShapeBindings,
    fit: bool,
) -> ModeledObject {
    let presentation = bindings.presentation();
    prepare_box_with_setup(document, [190., 290., 390.], true, || {
        if fit {
            presentation.isos(12);
        }
    })
}

/// Construct a real named Boolean command function; `binary_operation(&table,&object,&tool,("48","Cut"))`.
pub(super) fn binary_operation(
    table: &Handle<TFunctionDriverTable>,
    object: &Handle<TDataStdUAttribute>,
    tool: &Handle<TDataStdUAttribute>,
    kind: (&str, &str),
) -> Handle<TFunctionFunction> {
    let function = function(object, kind.0, kind.1);
    super::naming_transform_arguments::set_object_argument(&function, 1, tool).unwrap();
    compute(table, &function);
    function
}
