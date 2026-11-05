//! Booleans between the parts of dataset assemblies whose results must not be empty or
//! unchanged, shown in the view (OCC23378, OCC27472, OCC27760).
use super::prelude::*;
use crate::common_operation;
use crate::shape_checks::six_significant_digits;
use martensite_tkbo::bop_algo::{BOPAlgoBOP, Operation};
use martensite_tkbrep::topods::ShapeIterator;
use martensite_tkernel::standard::Failure;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::BRepGProp;

/// The direct members of a container, as an explode without type lists them.
fn members(shape: &Shape) -> Result<Vec<Shape>, Failure> {
    let mut found = Vec::new();
    let mut iterator = ShapeIterator::new(shape, true, true);
    while iterator.more() {
        found.push(iterator.value()?.clone());
        iterator.next();
    }
    Ok(found)
}

/// Display `result`, optionally shaded, fit, and dump the view as it is.
fn fitted_screenshot(result: &Shape, shaded: bool, image: &'static str) -> Outcome {
    fitted_run(result, shaded)?.dump_and_finish(image, false)
}

/// `fitted_screenshot` with the half-step camera shift controls of the cross-driver grader.
fn fitted_screenshot_with_shifts(result: &Shape, image: &'static str) -> Outcome {
    fitted_run(result, false)?.dump_and_finish(image, true)
}

fn fitted_run(result: &Shape, shaded: bool) -> Result<SessionRun, Box<dyn std::error::Error>> {
    let mut run = SessionRun::new()?;
    if shaded {
        run.session().set_display_mode(1)?;
    }
    run.session()
        .display("result", result, &DisplayRequest::default())?;
    run.session().fit()?;
    Ok(run)
}

/// `bug23378_1/2`: cutting the second blower part from the first gives two solids; part
/// `part` (1 or 2) keeps area `area`.
/// Example: `blower_cut_view(1, 864274., "bug23378_1.png")?`.
pub(crate) fn blower_cut_view(part: usize, area: f64, image: &'static str) -> Outcome {
    banner(&["========", "OCC23378", "========", ""]);
    let b = members(&read_dataset_shape("bug23378_blower1.brep")?)?;
    let r = common_operation::prepared(&b[0], &b[1], Operation::Cut);
    let pieces = members(&r)?;
    if pieces.len() != 2 {
        println!("ERROR: Result of BrepAlgoAPI_Cut is unexpected");
    }
    let result = pieces
        .get(part - 1)
        .cloned()
        .ok_or_else(|| format!("cut has {} parts; expected part {part}", pieces.len()))?;
    check_shape_structure(&result)?;
    check_mass(&result, &MassCheck::near(MassKind::Area, area))?;
    fitted_screenshot(&result, true, image)
}

/// `bug27472`: cutting a dataset tool from a dataset object changes its volume.
pub(crate) fn volume_changing_cut_view() -> Outcome {
    banner(&["========", "OCC27472", "========", ""]);
    let o = read_dataset_shape("bug27472_o.brep")?;
    let t = read_dataset_shape("bug27472_t.brep")?;
    let result = common_operation::direct(&o, &t, Operation::Cut);
    check_shape_structure(&result)?;
    if printed_volume(&result)? == printed_volume(&o)? {
        println!("ERROR: OCC27472 is reproduced. Result of bcut operation is WRONG.");
    }
    fitted_screenshot(&result, false, "bug27472.png")
}

/// The volume as a default stream prints it (six significant digits).
fn printed_volume(shape: &Shape) -> Result<f64, Failure> {
    let mut props = GProps::new();
    BRepGProp::volume_properties(shape, &mut props, false, false, false)?;
    Ok(six_significant_digits(props.mass()))
}

/// `bug27760`: the common of the two parts of a dataset shape, computed in parallel, is
/// not empty.
pub(crate) fn parallel_common_view() -> Outcome {
    banner(&["========", "OCC27760", "========", ""]);
    let l = members(&read_dataset_shape("bug27760_leid.brep")?)?;
    let result = parallel_common(&l[0], &l[1]);
    if members(&result)?.is_empty() {
        println!("ERROR: OCC27760 is reproduced. Result of bopcommon operation is empty compound.");
    }
    fitted_screenshot_with_shifts(&result, "bug27760.png")
}

/// The common of prepared intersections with both stages run in parallel.
fn parallel_common(first: &Shape, second: &Shape) -> Shape {
    let mut filler = common_operation::configured_intersections(first, second);
    filler.options_mut().set_run_parallel(true);
    filler.perform();
    assert!(!filler.options().has_errors(), "bop failed");
    let mut operation = BOPAlgoBOP::new();
    operation.add_argument(first);
    operation.add_tool(second);
    operation.set_operation(Operation::Common);
    operation.options_mut().set_run_parallel(true);
    operation.set_check_inverted(true);
    operation.set_to_fill_history(true);
    operation.perform_with_filler(&mut filler);
    assert!(!operation.options().has_errors(), "bopcommon failed");
    operation.shape().clone()
}
