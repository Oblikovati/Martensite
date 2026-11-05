use martensite_tkbrep::topods::Solid;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::State;
use martensite_tktopalgo::brep_class3d::BRepClass3dSolidClassifier;
/// Classify the infinite point at the native 1e-7 tolerance; `is_inverted_solid(&solid)?`.
pub fn is_inverted_solid(solid: &Solid) -> Result<bool, Failure> {
    let mut classifier = BRepClass3dSolidClassifier::with_shape(solid)?;
    classifier.perform_infinite_point(1e-7)?;
    Ok(classifier.state() == State::In)
}
