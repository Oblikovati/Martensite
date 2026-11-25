//! Native-only fixtures retain the source and real production history projection inputs.
use super::*;
pub(super) struct FrozenNative {
    pub inputs: Inputs,
    pub source: DevelopedRegionRequest,
    pub shape: Shape,
    pub bindings: Vec<(BindingKey, Shape)>,
    pub history: history::HistoryEvidence,
}
pub(super) fn capture(request: &WrappedConeNormalRequest) -> FrozenNative {
    let inputs = input::capture(request, &mut || false).unwrap();
    let mut work = storage::Work::new(request.proof.arithmetic);
    let selection = prepare::select(
        &ConeRun::from(request),
        &inputs.originals,
        &mut work,
        &mut || false,
    )
    .unwrap();
    let prepared = &selection.prepared[0];
    let built = native::construct(prepared, request.support).unwrap();
    let keys: Vec<_> = built.bindings.iter().map(|(key, _)| *key).collect();
    let history =
        history::certify_developed(&inputs.originals, &selection.prepared, &keys).unwrap();
    retain(inputs, prepared.source.clone(), built, keys, history)
}
fn retain(
    inputs: Inputs,
    source: DevelopedRegionRequest,
    built: Constructed,
    keys: Vec<BindingKey>,
    history: history::HistoryEvidence,
) -> FrozenNative {
    let bytes = crate::surface_sketch::encoding::encode(&built.solid, &built.bindings).unwrap();
    let (shape, bindings) = super::super::super::super::pipeline::decode(&bytes, &keys).unwrap();
    FrozenNative {
        inputs,
        source,
        shape,
        bindings,
        history,
    }
}
