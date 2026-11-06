//! The healing precision is authored; the maximal tolerance keeps its tool default.
use crate::dataset_shape::read_dataset_shape;
use crate::shape_checks::{MassCheck, MassKind, check_mass, check_shape_valid};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tkshhealing::shape_fix::ShapeFixShape;
#[cfg(test)]
mod cpu;
type Outcome = Result<(), Box<dyn std::error::Error>>;

fn case_repair(shape: Shape) -> Result<Shape, Failure> {
    let fixer = ShapeFixShape::default();
    fixer.init(shape);
    fixer.set_precision(1e-7);
    fixer.perform()?;
    Ok(fixer.shape())
}

/// Repair the original shape, check its area and validity, then display its faces.
/// Example: `healed_face_presentation()?`.
pub(crate) fn healed_face_presentation() -> Outcome {
    crate::session_run::banner(&["================", "OCC1360", "================", ""]);
    let result = case_repair(read_dataset_shape("OCC1360.brep")?)?;
    check_mass(&result, &MassCheck::near(MassKind::Area, 407.599))?;
    check_shape_valid(&result)?;
    crate::shape_view_checks::checkview_3d(&result, "bug1360.png")
}
