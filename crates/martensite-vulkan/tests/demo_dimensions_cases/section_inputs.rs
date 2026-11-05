//! Genuine BREP operands are sectioned with the command's approximation and both p-curves.
use super::*;
use martensite_tkbo::brep_algo_api::BRepAlgoAPISection;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

pub(super) fn read_case_inputs()
-> Result<([Shape; 2], std::path::PathBuf), Box<dyn std::error::Error>> {
    // The sample resolves all three paths before importing either BREP.
    crate::dataset_shape::locate_dataset_file("occ/Top.brep")?;
    crate::dataset_shape::locate_dataset_file("occ/Bottom.brep")?;
    let hatch = crate::dataset_shape::locate_dataset_file("images/hatch_1.png")?;
    println!("Importing shapes...");
    let shapes = [
        crate::dataset_shape::read_dataset_shape("occ/Top.brep")?,
        crate::dataset_shape::read_dataset_shape("occ/Bottom.brep")?,
    ];
    Ok((shapes, hatch))
}
pub(super) fn cut_sections(shapes: &[Shape; 2]) -> Result<[Shape; 2], Failure> {
    let bounds = crate::primitive_shapes::box_at([-150., -250., 0.], [150., 250., 100.])?;
    let bottom = section(&shapes[1], &bounds)?;
    let top = section(&shapes[0], &bounds)?;
    Ok([top, bottom])
}
fn section(first: &Shape, second: &Shape) -> Result<Shape, Failure> {
    let mut operation = configured_section(first, second)?;
    operation.build()?;
    operation
        .dump_warnings(&mut std::io::stdout())
        .map_err(|cause| Failure::with_message("Standard_Failure", &cause.to_string()))?;
    if operation.has_errors() || !operation.is_done() || operation.shape().is_null() {
        return Err(Failure::with_message(
            "Standard_ConstructionError",
            &format!(
                "bsection errors={},done={},null={}; expected completed non-null section",
                operation.has_errors(),
                operation.is_done(),
                operation.shape().is_null()
            ),
        ));
    }
    Ok(operation.shape().clone())
}
fn configured_section(first: &Shape, second: &Shape) -> Result<BRepAlgoAPISection, Failure> {
    let mut operation = BRepAlgoAPISection::from_shapes(first, second, None, false)?;
    operation.approximation(true);
    operation.compute_pcurve_on1(true);
    operation.compute_pcurve_on2(true);
    operation.set_fuzzy_value(0.);
    operation.set_run_parallel(false);
    operation.set_non_destructive(false);
    operation.set_glue(martensite_tkbo::bop_algo::Glue::Off);
    operation.set_use_obb(false);
    Ok(operation)
}
