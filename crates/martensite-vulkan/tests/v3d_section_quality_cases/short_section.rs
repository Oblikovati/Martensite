use super::*;
use crate::boolean_typed_explode::all_distinct;
use crate::case_view::viewer_session::DisplayRequest;
use crate::shape_checks::{MassCheck, MassKind, check_mass};
use crate::shape_report_checks::{
    ShapeCounts, check_max_tolerance, check_shape_counts, check_shape_structure, print_tolerances,
};
use martensite_tkbo::brep_algo_api::BRepAlgoAPISection;
use martensite_tkernel::message::Gravity;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
const HEADING: &[&str] = &[
    "##################################################",
    "0024097: BRepAlgoAPISection returns wrong result",
    "##################################################",
];

fn checked_input(input: &str) -> Result<Shape, Failure> {
    let shape = read_dataset_shape(input)?;
    print_tolerances(&shape)?;
    check_shape_structure(&shape)?;
    Ok(shape)
}

fn report_gravity(operation: &BRepAlgoAPISection, gravity: Gravity, prefix: &str) {
    print!(
        "{}",
        crate::source_bop_alerts::gravity_alert_text(operation.get_report(), gravity, prefix)
    );
}

fn report_section(operation: &BRepAlgoAPISection) {
    report_gravity(operation, Gravity::Warning, "Warning: ");
    if !operation.is_done() {
        report_gravity(operation, Gravity::Fail, "Error: ");
    }
}

pub(super) fn source_section() -> Result<(Shape, [Shape; 2]), Failure> {
    let inputs = [
        checked_input("bug24097_face1.brep")?,
        checked_input("bug24097_face2.brep")?,
    ];
    let result = crate::direct_section_adapter::direct_section_observed(
        &inputs[0],
        &inputs[1],
        report_section,
    );
    Ok((result, inputs))
}

fn authored_checks(shape: &Shape) -> Result<(), Failure> {
    let mut expected = ShapeCounts::default();
    expected.expected[0] = Some(2);
    expected.expected[1] = Some(1);
    check_shape_counts(shape, "result", &expected)?;
    check_max_tolerance(shape, "0.0001")?;
    check_mass(shape, &MassCheck::near(MassKind::Length, 0.125))?;
    Ok(())
}

fn source_display_bindings(result: &Shape) -> Vec<(String, Shape)> {
    let mut bindings = vec![("result".to_owned(), result.clone())];
    bindings.extend(
        all_distinct(result, ShapeEnum::Vertex)
            .into_iter()
            .enumerate()
            .map(|(index, vertex)| (format!("result_{}", index + 1), vertex)),
    );
    bindings
}

fn display_section(run: &mut SessionRun, result: &Shape, inputs: &[Shape; 2]) -> Outcome {
    let bindings = source_display_bindings(result);
    let borrowed: Vec<(&str, &Shape)> = bindings
        .iter()
        .map(|(name, shape)| (name.as_str(), shape))
        .collect();
    run.session().clear()?;
    run.session()
        .display_many(&borrowed, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().display_many(
        &[("a", &inputs[0]), ("b", &inputs[1])],
        &DisplayRequest::default(),
    )?;
    Ok(())
}

/// Build the real direct section, check its open edge, then fit before displaying both faces.
/// Example: `checked_short_section()?`.
pub(crate) fn checked_short_section() -> Outcome {
    banner(HEADING);
    let (result, inputs) = source_section()?;
    authored_checks(&result)?;
    let mut run = SessionRun::new()?;
    display_section(&mut run, &result, &inputs)?;
    run.dump("bug24097.png")?;
    run.finish()
}
