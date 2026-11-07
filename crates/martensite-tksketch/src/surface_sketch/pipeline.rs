//! Construct, bind, encode and validate before immutable publication.
use super::*;

pub(super) fn build(
    admitted: &sweep::Sweep,
    semantic_check: &impl Fn(&history::Bindings) -> failure::Checked<()>,
) -> Result<NormalExtrusionResult, SurfaceSketchFailure> {
    let (section, mut maker, solid) = construction::construct(admitted)
        .map_err(|cause| admitted.failure(SurfaceStage::Construction, cause))?;
    let bindings = history::collect(admitted, &section, &mut maker, &solid)
        .map_err(|cause| admitted.failure(SurfaceStage::History, cause))?;
    semantic_check(&bindings).map_err(|cause| admitted.failure(SurfaceStage::Validation, cause))?;
    let bytes = encoding::encode(&solid, &bindings)
        .map_err(|cause| admitted.failure(SurfaceStage::Encoding, cause))?;
    publish(
        admitted,
        semantic_check,
        bytes,
        bindings.iter().map(|(key, _)| *key).collect(),
    )
}
fn publish(
    admitted: &sweep::Sweep,
    semantic_check: &impl Fn(&history::Bindings) -> failure::Checked<()>,
    bytes: Vec<u8>,
    keys: Vec<HistoryKey>,
) -> Result<NormalExtrusionResult, SurfaceSketchFailure> {
    let (solid, bindings) = encoding::decode(&bytes, &keys)
        .map_err(|cause| admitted.failure(SurfaceStage::Encoding, cause))?;
    let checked = validation::check_body(&solid, &bindings, admitted, semantic_check)
        .map_err(|cause| admitted.failure(SurfaceStage::Validation, cause))?;
    let (volume, errors) = volume::measure(&solid, admitted)
        .map_err(|cause| admitted.failure(SurfaceStage::Volume, cause))?;
    Ok(result::assemble(
        admitted, bytes, keys, checked, volume, errors,
    ))
}
