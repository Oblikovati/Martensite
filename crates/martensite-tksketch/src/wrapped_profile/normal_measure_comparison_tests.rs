//! Completed measure reports must be recaptured against current decoded geometry and keys.
use super::*;
use crate::surface_sketch::HistoryRole;
use martensite_tkbrep::{brep::BRepBuilder, topods};
struct Fixture {
    original: Request,
    prepared: Request,
    bindings: Vec<(BindingKey, Shape)>,
    stored: Vec<storage::StoredProof>,
    flux: CertifiedCylindricalFlux,
    normal: NormalMeasureEvidence,
}
impl Fixture {
    fn new() -> Self {
        let (original, prepared, bindings) = storage::tests::frozen();
        let stored = storage::certify(&original, &prepared, &bindings).unwrap();
        let flux = storage::certify_flux(&original, &prepared, &bindings, &stored).unwrap();
        let normal = certify(&original, &prepared, &stored).unwrap();
        Self {
            original,
            prepared,
            bindings,
            stored,
            flux,
            normal,
        }
    }
    fn compare(self) -> Result<pipeline::MeasureEvidence, NormalMeasureFailure> {
        compare(
            &self.original,
            &self.prepared,
            &self.bindings,
            &self.stored,
            (self.flux, self.normal),
        )
    }
    fn binding(&self, role: HistoryRole) -> Shape {
        self.bindings
            .iter()
            .find(|(key, _)| key.role == role)
            .unwrap()
            .1
            .clone()
    }
}
#[test]
fn reordered_semantic_span_receipt_cannot_reuse_a_completed_measure() {
    let mut f = Fixture::new();
    f.normal.spans.reverse();
    assert!(matches!(
        f.compare(),
        Err(NormalMeasureFailure::SpanSequence { .. })
    ));
}
#[test]
fn removed_source_proof_cannot_reuse_completed_measures() {
    let mut f = Fixture::new();
    f.stored.remove(0);
    assert!(matches!(f.compare(), Err(NormalMeasureFailure::Sources(_))));
}
#[test]
fn changed_retained_trim_domain_is_refused_during_measure_recapture() {
    let f = Fixture::new();
    let edge = topods::edge(&f.binding(HistoryRole::FirstDepthEdge)).unwrap();
    let face = topods::face(&f.binding(HistoryRole::FirstDepthFace)).unwrap();
    BRepBuilder::default()
        .range_on_face(&edge, &face, 0., 0.5)
        .unwrap();
    let Err(NormalMeasureFailure::Storage(error)) = f.compare() else {
        panic!("changed trim must be refused");
    };
    assert!(error.to_string().contains("retained pcurve range/storage"));
}
