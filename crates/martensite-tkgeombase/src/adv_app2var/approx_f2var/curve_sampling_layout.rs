//! Fixed entry table strides are distinct from live post-clear sampling counts.
use super::super::super::curve_workspace::CurveProjectionFrame;
use super::{IsoApproxRequest, PieceTables};
use crate::adv_app2var::approx_f2var::curve_folded_storage::FoldedTable::{
    AntisymmetricDifference, SymmetricSum,
};
#[derive(Clone, Copy)]
pub(crate) struct CurveSamplingEntry {
    pub(super) dimension: i32,
    pub(super) root_count: i32,
}
impl CurveSamplingEntry {
    pub(crate) fn capture(request: &IsoApproxRequest<'_>) -> Self {
        Self {
            dimension: request.ndimen,
            root_count: request.nbroot,
        }
    }
}
/// The folding of `mma1fdi_`: sums and differences of the samples at `+t` and
/// `-t`, times `renor`, and the middle sample of an odd count.
/// Initial half-count/strides survive caller root-count mutation; sample selection is live.
pub(super) fn fold_samples(
    ndimen: i32,
    entry: CurveSamplingEntry,
    nbroot: i32,
    live_nbroot: i32,
    renor: f64,
    workspace: &CurveProjectionFrame,
    piece: &mut PieceTables<'_>,
) {
    let sd1: i32 = entry.root_count / 2 + 1;
    let stride = entry.dimension;
    let folded = &mut piece.folded;
    let nroo2: i32 = nbroot / 2;
    for nd in 1..=ndimen {
        for ii in 1..=nroo2 {
            let iip: i32 = (live_nbroot + 1) / 2 + ii;
            let iim: i32 = nroo2 - ii + 1;
            let (bid1, bid2) = (
                workspace.sample_subscript(i64::from(nd) + i64::from(iim) * i64::from(stride)),
                workspace.sample_subscript(i64::from(nd) + i64::from(iip) * i64::from(stride)),
            );
            folded.set_value(SymmetricSum, ii + (nd - 1) * sd1, renor * (bid2 + bid1));
            folded.set_value(
                AntisymmetricDifference,
                ii + (nd - 1) * sd1,
                renor * (bid2 - bid1),
            );
        }
    }
    for nd in 1..=ndimen {
        let middle: f64 = if live_nbroot % 2 == 1 {
            renor
                * workspace
                    .sample_subscript(i64::from(nd) + (i64::from(nroo2) + 1) * i64::from(stride))
        } else {
            0.
        };
        folded.set_value(SymmetricSum, (nd - 1) * sd1, middle);
        folded.set_value(AntisymmetricDifference, (nd - 1) * sd1, middle);
    }
}
