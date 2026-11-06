//! Original direct and prepared Boolean command paths shared by fixture regressions.
#![allow(dead_code)] // Each source case uses only its original command path.
use martensite_tkbo::bop_algo::{
    BOPAlgoBOP, BOPAlgoBuilder, BOPAlgoPaveFiller, BOPAlgoSection, Glue, Operation, Options,
};
#[path = "../bop_alerts.rs"]
pub mod bop_alerts;
use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_tools::{self, History},
    topods::Shape,
};
use martensite_tkernel::{
    message::{Gravity, ProgressRange},
    ncollection::List,
    precision,
    standard::Handle,
    storage::IoStream,
};
use martensite_tkg3d::top_abs::ShapeEnum;

/// Restore the complete drawable fixture; `restored_shape(bytes, ShapeEnum::compound)`.
pub fn restored_shape(bytes: &[u8], expected: ShapeEnum) -> Shape {
    let end = bytes.iter().position(u8::is_ascii_whitespace).unwrap();
    assert_eq!(&bytes[..end], b"DBRep_DrawableShape");
    let mut shape = Shape::default();
    let mut stream = IoStream::from_bytes(bytes[end..].to_vec());
    brep_tools::read(
        &mut shape,
        &mut stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )
    .unwrap();
    assert_eq!(shape.shape_type().unwrap(), expected);
    shape
}

/// Configure bop defaults before caller-owned execution; `configured_intersections(&first, &second)`.
pub fn configured_intersections(first: &Shape, second: &Shape) -> BOPAlgoPaveFiller {
    let arguments = [first.clone(), second.clone()]
        .into_iter()
        .collect::<List<_>>();
    let mut filler = BOPAlgoPaveFiller::new();
    filler.set_arguments(&arguments);
    filler.options_mut().set_run_parallel(false);
    filler.options_mut().set_fuzzy_value(precision::confusion());
    filler.options_mut().set_use_obb(false);
    filler.set_non_destructive(false);
    filler.set_glue(Glue::Off);
    filler
}

/// Prepare the bop once; `intersections(&first, &second)`.
pub fn intersections(first: &Shape, second: &Shape) -> BOPAlgoPaveFiller {
    let mut filler = configured_intersections(first, second);
    filler.perform();
    assert!(!filler.options().has_errors(), "source common: bop failed");
    filler
}

/// Reuse the completed intersection data; `common(&object, &tool)`.
pub(super) fn common(first: &Shape, second: &Shape) -> Shape {
    prepared(first, second, Operation::Common)
}

/// Replay bop followed by its selected result command; `prepared(&a, &b, Operation::fuse)`.
pub fn prepared(first: &Shape, second: &Shape, kind: Operation) -> Shape {
    let mut filler = intersections(first, second);
    prepared_with_filler(first, second, kind, &mut filler)
}

/// Configure the prepared result before execution; `configured_prepared(a, b, Operation::common)`.
pub fn configured_prepared(first: &Shape, second: &Shape, kind: Operation) -> BOPAlgoBOP {
    let mut operation = BOPAlgoBOP::new();
    operation.add_argument(first);
    operation.add_tool(second);
    operation.set_operation(kind);
    operation.options_mut().set_run_parallel(false);
    operation.set_check_inverted(true);
    operation.set_to_fill_history(true);
    operation
}

/// Perform with caller-owned intersections; `performed_with_filler(a, b, kind, filler)`.
/// The caller retains the command's publication and diagnostic ordering.
pub fn performed_with_filler(
    first: &Shape,
    second: &Shape,
    kind: Operation,
    filler: &mut BOPAlgoPaveFiller,
) -> BOPAlgoBOP {
    let mut operation = configured_prepared(first, second, kind);
    operation.perform_with_filler(filler);
    operation
}

/// Keep the warning-before-error publication; `report_result_alerts(operation.options())`.
pub fn report_result_alerts(options: &Options) {
    for (gravity, prefix) in [(Gravity::Warning, "Warning: "), (Gravity::Fail, "Error: ")] {
        // The scripts' patterns and the reference logs name each alert with its package separator.
        let text = bop_alerts::gravity_alert_text(options.report(), gravity, prefix);
        print!("{}", text.replace("BOPAlgoAlert", "BOPAlgo_Alert"));
    }
}

fn prepared_shape(operation: &BOPAlgoBOP, kind: Operation) -> Shape {
    assert!(
        !operation.options().has_errors(),
        "source prepared {kind:?}: result command failed"
    );
    operation.shape().clone()
}

/// Build another result from the bop intersections; `prepared_with_filler(a, b, kind, filler)`.
pub(super) fn prepared_with_filler(
    first: &Shape,
    second: &Shape,
    kind: Operation,
    filler: &mut BOPAlgoPaveFiller,
) -> Shape {
    let operation = performed_with_filler(first, second, kind, filler);
    prepared_shape(&operation, kind)
}

/// Retain actual result History with reused intersections; `prepared_with_filler_and_history(a,b,kind,filler)`.
pub(super) fn prepared_with_filler_and_history(
    first: &Shape,
    second: &Shape,
    kind: Operation,
    filler: &mut BOPAlgoPaveFiller,
) -> (Shape, Handle<History>) {
    let mut operation = performed_with_filler(first, second, kind, filler);
    report_result_alerts(operation.options());
    let history = operation
        .history()
        .unwrap_or_else(|| panic!("source prepared {kind:?}: expected enabled result History"));
    (prepared_shape(&operation, kind), history)
}

#[path = "../direct_options.rs"]
mod direct_options;
pub(super) use direct_options::direct_options;

/// Retain the actual performed direct operation for reference-order report and History publication.
/// Example: `let operation = performed_direct(&a, &b, Operation::cut, precision::confusion());`.
pub fn performed_direct(first: &Shape, second: &Shape, kind: Operation, fuzzy: f64) -> BOPAlgoBOP {
    let mut operation = BOPAlgoBOP::new();
    operation.add_argument(first);
    operation.add_tool(second);
    direct_options(&mut operation, kind, fuzzy);
    operation.perform();
    operation
}

/// [`performed_direct`] after `brunparallel 1`; `performed_direct_parallel(&a, &b, kind, 1e-7)`.
#[allow(dead_code)] // Each standalone test target uses a different subset.
pub(super) fn performed_direct_parallel(
    first: &Shape,
    second: &Shape,
    kind: Operation,
    fuzzy: f64,
) -> BOPAlgoBOP {
    let mut operation = BOPAlgoBOP::new();
    operation.add_argument(first);
    operation.add_tool(second);
    direct_options(&mut operation, kind, fuzzy);
    operation.options_mut().set_run_parallel(true);
    operation.perform();
    operation
}

/// Replay bcommon/bfuse/bcut; `direct(&object, &tool, Operation::cut)`.
pub fn direct(first: &Shape, second: &Shape, kind: Operation) -> Shape {
    let fuzzy = precision::confusion();
    direct_with_fuzzy(first, second, kind, fuzzy).0
}

fn check_direct_shape(operation: &BOPAlgoBOP, kind: Operation) {
    assert!(
        !operation.options().has_errors(),
        "source direct {kind:?}: operation failed"
    );
    assert!(
        !operation.shape().is_null(),
        "source direct {kind:?}: null result"
    );
}

/// Apply an explicit source fuzzy request and return the actual effective setting.
/// Example: `let (shape, effective) = direct_with_fuzzy(&a, &b, Operation::fuse, 1e-3);`.
pub fn direct_with_fuzzy(
    first: &Shape,
    second: &Shape,
    kind: Operation,
    fuzzy: f64,
) -> (Shape, f64) {
    // Existing callers keep the identical operation through a side-effect-free observer.
    direct_with_fuzzy_observed(first, second, kind, fuzzy, |_| {})
}

/// Observe the actual performed operation before source guards; `direct_with_fuzzy_observed(a,b,kind,tol,inspect)`.
pub fn direct_with_fuzzy_observed(
    first: &Shape,
    second: &Shape,
    kind: Operation,
    fuzzy: f64,
    observe: impl FnOnce(&mut BOPAlgoBOP),
) -> (Shape, f64) {
    let mut operation = performed_direct(first, second, kind, fuzzy);
    observe(&mut operation);
    check_direct_shape(&operation, kind);
    (operation.shape().clone(), operation.options().fuzzy_value())
}

/// Reuse the existing Section builder options and supplied filler; `performed_section(&a, &b, filler)`.
pub(super) fn performed_section(
    first: &Shape,
    second: &Shape,
    filler: &mut BOPAlgoPaveFiller,
) -> BOPAlgoSection {
    let arguments = [first.clone(), second.clone()];
    let mut operation = configured_section_arguments(&arguments);
    operation.perform_with_filler(filler);
    operation
}

/// Preserve supplied Section argument order before execution; `configured_section_arguments(&[a,b,c])`.
pub fn configured_section_arguments(arguments: &[Shape]) -> BOPAlgoSection {
    let mut operation = BOPAlgoSection::new();
    for argument in arguments {
        operation.add_argument(argument);
    }
    operation.options_mut().set_run_parallel(false);
    operation.set_check_inverted(true);
    operation.set_to_fill_history(true);
    operation
}

fn section_shape(operation: &BOPAlgoSection) -> Shape {
    assert!(
        !operation.options().has_errors(),
        "bopsection: result command failed"
    );
    operation.shape().clone()
}

/// Replay bopsection using the already intersected inputs; `section(&first, &second)`.
pub(super) fn section(first: &Shape, second: &Shape) -> Shape {
    let mut filler = intersections(first, second);
    let operation = performed_section(first, second, &mut filler);
    section_shape(&operation)
}

/// Reuse the filler and retain Section History; `section_with_filler_and_history(a,b,filler)`.
pub(super) fn section_with_filler_and_history(
    first: &Shape,
    second: &Shape,
    filler: &mut BOPAlgoPaveFiller,
) -> (Shape, Handle<History>) {
    let mut operation = performed_section(first, second, filler);
    report_result_alerts(operation.options());
    let history = operation
        .history()
        .expect("source bopsection: expected enabled result History");
    (section_shape(&operation), history)
}

/// Replay bbuild from the ordered objects and tools; `general_fuse(&first, &second)`.
pub(super) fn general_fuse(first: &Shape, second: &Shape) -> Shape {
    let mut filler = intersections(first, second);
    let mut builder = BOPAlgoBuilder::new();
    builder.add_argument(first);
    builder.add_argument(second);
    builder.options_mut().set_run_parallel(false);
    builder.set_check_inverted(true);
    builder.set_to_fill_history(true);
    builder.perform_with_filler(&mut filler);
    assert!(
        !builder.options().has_errors(),
        "bbuild: prepared general fuse failed"
    );
    builder.shape().clone()
}
