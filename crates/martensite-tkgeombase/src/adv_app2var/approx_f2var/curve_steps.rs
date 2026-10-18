//! The steps of the iso-curve approximation `mma2fnc_`: the file-static `mma1nop_`
//! (parameters), `mma1fdi_` (sampling), `mma1cdi_` (constraint removal), `mma1jak_`
//! and `mmmapcoe_` (Jacobi projection), `mma1cnt_` (constraint addition),
//! with `mma1fer_`, `mmaperm_` and `mma1noc_` in the `curve_errors` module.
use super::bases::{mma1her_, mmapptt_};
use super::curve::IsoApproxRequest;
use super::curve_endpoint_storage::ConstraintEnd;
use super::curve_folded_storage::FoldedTable::{AntisymmetricDifference, SymmetricSum};
use super::curve_workspace::CurveProjectionFrame;

/// `mma1nop_`: the `nbroot + 2` parameters of the iso sampling into `ttable`: the
/// interval ends around the roots mapped on the non-constant parameter of `uvfonc`
/// (U0, U1, V0, V1). `iercod` is 1 for an `isofav` other than 1 or 2 (and is not
/// cleared otherwise, as natively).
pub(super) fn mma1nop_(
    nbroot: i32,
    rootlg: &[f64],
    uvfonc: &[f64; 4],
    isofav: i32,
    ttable: &mut [f64],
    iercod: &mut i32,
) {
    let alinu: f64 = (uvfonc[1] - uvfonc[0]) / 2.;
    let blinu: f64 = (uvfonc[1] + uvfonc[0]) / 2.;
    let alinv: f64 = (uvfonc[3] - uvfonc[2]) / 2.;
    let blinv: f64 = (uvfonc[3] + uvfonc[2]) / 2.;
    let (ends, alin, blin) = match isofav {
        1 => ([uvfonc[2], uvfonc[3]], alinv, blinv),
        2 => ([uvfonc[0], uvfonc[1]], alinu, blinu),
        _ => {
            *iercod = 1;
            return;
        }
    };
    ttable[0] = ends[0];
    for ii in 1..=nbroot {
        ttable[ii as usize] = alin * rootlg[(ii - 1) as usize] + blin;
    }
    ttable[(nbroot + 1) as usize] = ends[1];
}

#[path = "curve_sampling.rs"]
mod sampling;
pub(crate) use sampling::layout::CurveSamplingEntry;
pub(super) use sampling::{PieceTables, mma1fdi_};

/// `mma1cdi_`: subtracts from the folded samples the Hermite interpolation of the
/// end constraints, sampled at the roots into `fpntab` with the basis `hermit`.
/// `iercod` is 1 when the Hermite basis fails.
///
/// retained quirk: for an odd number of roots the middle correction uses only the
/// highest order's term (the order loop overwrites `bid3` before it is applied).
pub(super) fn mma1cdi_(
    req: &IsoApproxRequest<'_>,
    piece: &mut PieceTables<'_>,
    workspace: &mut CurveProjectionFrame,
    iercod: &mut i32,
) {
    let (ndimen, nbroot, iordre) = (req.ndimen, req.nbroot, req.iordre);
    *iercod = 0;
    mma1her_(iordre, workspace.hermite_storage_mut(), iercod);
    if *iercod > 0 {
        *iercod = 1;
        return;
    }
    let ncfhe: i32 = (iordre + 1) << 1;
    for column in 1..=ncfhe as usize {
        workspace.sample_hermite_column(ncfhe as usize, column, &req.rootlg[..nbroot as usize]);
    }
    let fp = |root: i32, column: i32| workspace.sample_value(root, column, nbroot);
    let sd1: i32 = nbroot / 2 + 1;
    let endpoints = &piece.endpoints;
    let folded = &mut piece.folded;
    let nroo2: i32 = nbroot / 2;
    for nd in 1..=ndimen {
        for ii in 1..=iordre + 1 {
            let (bid1, bid2) = (
                endpoints.value(ConstraintEnd::Lower, (nd + ii * ndimen) - (ndimen + 1)),
                endpoints.value(ConstraintEnd::Upper, (nd + ii * ndimen) - (ndimen + 1)),
            );
            for kk in 1..=nroo2 {
                let kkm: i32 = nroo2 - kk + 1;
                let bid3: f64 = bid1 * fp(kkm, (ii << 1) - 1) + bid2 * fp(kkm, ii << 1);
                folded.set_value(
                    SymmetricSum,
                    kk + (nd - 1) * sd1,
                    folded.value(SymmetricSum, kk + (nd - 1) * sd1) - bid3,
                );
                folded.set_value(
                    AntisymmetricDifference,
                    kk + (nd - 1) * sd1,
                    folded.value(AntisymmetricDifference, kk + (nd - 1) * sd1) + bid3,
                );
            }
            for kk in 1..=nroo2 {
                let kkp: i32 = (nbroot + 1) / 2 + kk;
                let bid3: f64 = bid1 * fp(kkp, (ii << 1) - 1) + bid2 * fp(kkp, ii << 1);
                folded.set_value(
                    SymmetricSum,
                    kk + (nd - 1) * sd1,
                    folded.value(SymmetricSum, kk + (nd - 1) * sd1) - bid3,
                );
                folded.set_value(
                    AntisymmetricDifference,
                    kk + (nd - 1) * sd1,
                    folded.value(AntisymmetricDifference, kk + (nd - 1) * sd1) - bid3,
                );
            }
        }
    }
    if nbroot % 2 != 1 {
        return;
    }
    for nd in 1..=ndimen {
        let mut bid3: f64 = 0.;
        for ii in 1..=iordre + 1 {
            bid3 = fp(nroo2 + 1, (ii << 1) - 1)
                * endpoints.value(ConstraintEnd::Lower, (nd + ii * ndimen) - (ndimen + 1))
                + fp(nroo2 + 1, ii << 1)
                    * endpoints.value(ConstraintEnd::Upper, (nd + ii * ndimen) - (ndimen + 1));
        }
        folded.set_value(
            SymmetricSum,
            (nd - 1) * sd1,
            folded.value(SymmetricSum, (nd - 1) * sd1) - bid3,
        );
        folded.set_value(
            AntisymmetricDifference,
            (nd - 1) * sd1,
            folded.value(AntisymmetricDifference, (nd - 1) * sd1) - bid3,
        );
    }
}

/// `mma1jak_`: the Jacobi coefficients of the folded samples by Gauss projection
/// (`mmapptt_` then `mmmapcoe_`) into `crvjac` (`(0:ndgjac, ndimen)`); `iercod` is 33
/// when the projection table fails.
pub(super) fn mma1jak_(
    req: &IsoApproxRequest<'_>,
    piece: &PieceTables<'_>,
    workspace: &mut CurveProjectionFrame,
    iercod: &mut i32,
) {
    *iercod = 0;
    mmapptt_(
        req.ndgjac,
        req.nbroot,
        req.iordre,
        workspace.gauss_storage_mut(),
        iercod,
    );
    if *iercod > 0 {
        *iercod = 33;
        return;
    }
    mmmapcoe_(req, piece, workspace);
}

/// `mmmapcoe_`: the Gauss projections of the sums (even degrees) and differences
/// (odd degrees) from degree `2*(iordre+1)` on.
fn mmmapcoe_(
    req: &IsoApproxRequest<'_>,
    piece: &PieceTables<'_>,
    workspace: &mut CurveProjectionFrame,
) {
    let (ndim, ndgjac, nbpnts) = (req.ndimen, req.ndgjac, req.nbroot);
    let cd1: i32 = ndgjac + 1;
    let sd1: i32 = nbpnts / 2 + 1;
    let folded = &piece.folded;
    let ikdeb: i32 = (req.iordre + 1) << 1;
    let nbroot: i32 = nbpnts / 2;
    for nd in 1..=ndim {
        for (start, table) in [(ikdeb, SymmetricSum), (ikdeb + 1, AntisymmetricDifference)] {
            for ik in (start..=ndgjac).step_by(2) {
                let igss: i32 = ik - ikdeb;
                let mut bidon: f64 = 0.;
                for ir in 1..=nbroot {
                    bidon += folded.value(table, ir + (nd - 1) * sd1)
                        * workspace.gauss_value((ir + igss * sd1) as usize);
                }
                workspace.set_coefficient((ik + (nd - 1) * cd1) as usize, bidon);
            }
        }
    }
    if nbpnts % 2 == 0 {
        return;
    }
    for nd in 1..=ndim {
        for ik in (ikdeb..=ndgjac).step_by(2) {
            let igss: i32 = ik - ikdeb;
            let index = (ik + (nd - 1) * cd1) as usize;
            let coefficient = workspace.coefficient(index)
                + folded.value(SymmetricSum, (nd - 1) * sd1)
                    * workspace.gauss_value((igss * sd1) as usize);
            workspace.set_coefficient(index, coefficient);
        }
    }
}

/// `mma1cnt_`: the constraint coefficients `0..=2*iordre+1` of `crvjac`, the
/// Hermite interpolation of the end constraints.
pub(super) fn mma1cnt_(
    req: &IsoApproxRequest<'_>,
    piece: &PieceTables<'_>,
    workspace: &mut CurveProjectionFrame,
) {
    let (ndimen, iordre) = (req.ndimen, req.iordre);
    let hd1: i32 = (iordre << 1) + 2;
    let endpoints = &piece.endpoints;
    let cd1: i32 = req.ndgjac + 1;
    for nd in 1..=ndimen {
        for ii in 0..=(iordre << 1) + 1 {
            let mut bid: f64 = 0.;
            for jj in 1..=iordre + 1 {
                bid = bid
                    + endpoints.value(ConstraintEnd::Lower, (nd + jj * ndimen) - (ndimen + 1))
                        * workspace.hermite_value((ii + ((jj << 1) - 2) * hd1) as usize)
                    + endpoints.value(ConstraintEnd::Upper, (nd + jj * ndimen) - (ndimen + 1))
                        * workspace.hermite_value((ii + ((jj << 1) - 1) * hd1) as usize);
            }
            workspace.set_coefficient((ii + (nd - 1) * cd1) as usize, bid);
        }
    }
}
