//! The truncation and normalisation steps of `mma2fnc_`: the file-static `mma1fer_`
//! and `mmaperm_` (truncation and errors) and `mma1noc_` (normalisation).
use super::curve::IsoApproxRequest;
use super::curve_error_storage::{CurveErrorOutput, PieceErrorTables};
use super::curve_integers::{CurveCoefficientCount, CurveIntegerOutputs};
use super::curve_workspace::CurveProjectionFrame;
use crate::adv_app2var::f2c::F2cArray;
use crate::adv_app2var::math_base::{
    TruncationPrecisionAccess, mmaperx_with_real_access, mmtrpjj_with_real_access, pow_di,
};
#[path = "curve_truncation_access.rs"]
mod original_access;
use original_access::CurveTruncationReals;

struct SubspacePrecisionError<'view, 'tables, Integers> {
    precision: &'view [f64],
    index: usize,
    integers: &'view Integers,
    errors: &'view mut PieceErrorTables<'tables>,
}
impl<Integers: CurveIntegerOutputs> TruncationPrecisionAccess
    for SubspacePrecisionError<'_, '_, Integers>
{
    fn precision(&self) -> f64 {
        self.integers.subspace_tolerance(self.precision, self.index)
    }
    fn set_error(&mut self, value: f64) {
        self.errors
            .set_error(CurveErrorOutput::Maximum, self.index, value);
    }
}

/// The per-piece outputs of `mma1fer_`: the errors per subspace and the kept
/// coefficient count.
pub(super) struct PieceErrors<'a, Integers> {
    pub(super) values: PieceErrorTables<'a>,
    pub(super) ncoeff: CurveCoefficientCount<'a, Integers>,
}

/// `mma1fer_`: truncates each subspace of `crvjac` to its tolerance within `ncflim`
/// coefficients (zeroing the rest) and measures the errors; `iercod` is -1 when a
/// subspace needs more than `ncflim`, 1 when `mmaperx_` fails.
pub(super) fn mma1fer_<Integers: CurveIntegerOutputs>(
    req: &mut IsoApproxRequest<'_>,
    workspace: &mut CurveProjectionFrame,
    errors: &mut PieceErrors<'_, Integers>,
    iercod: &mut i32,
) {
    *iercod = 0;
    let storage_stride = req.ndgjac + 1;
    let mut idim: i32 = 1;
    errors.ncoeff.set(0, req);
    let ncfja = req.ndgjac + 1;
    let subspaces = req.nbsesp;
    for ii in 1..=subspaces {
        let k: usize = (ii - 1) as usize;
        let ndses: i32 = errors.ncoeff.dimension(req.ndimse, k);
        let column: usize = ((idim - 1) * storage_stride) as usize;
        let mut ncfnw: i32 = 0;
        {
            let mut precision = SubspacePrecisionError {
                precision: req.epsapr,
                index: k,
                integers: &*errors.ncoeff.integers,
                errors: &mut errors.values,
            };
            let mut access = CurveTruncationReals::new(workspace, column, &mut precision);
            mmtrpjj_with_real_access(ncfja, ndses, ncfja, req.iordre, &mut access, &mut ncfnw)
                .expect("initialized curve truncation slots are infallible");
        }
        if ncfnw <= req.ncflim {
            errors
                .values
                .with_error(CurveErrorOutput::RootMeanSquare, k, |value| {
                    mmaperm_(
                        ncfja,
                        ndses,
                        ncfja,
                        req.iordre,
                        &workspace.coefficients()[column..],
                        ncfnw,
                        value,
                    );
                });
            errors.ncoeff.set(errors.ncoeff.get().max(ncfnw), req);
            let nbr0: i32 = req.ncflim - ncfnw;
            if nbr0 > 0 {
                for kk in 1..=ndses {
                    let start: usize = (ncfnw + (idim + kk - 2) * storage_stride) as usize;
                    workspace.clear_coefficient_span(start, nbr0);
                }
            }
        } else {
            *iercod = -1;
            let mut ier: i32 = 0;
            {
                let mut precision = SubspacePrecisionError {
                    precision: req.epsapr,
                    index: k,
                    integers: &*errors.ncoeff.integers,
                    errors: &mut errors.values,
                };
                let mut access = CurveTruncationReals::new(workspace, column, &mut precision);
                mmaperx_with_real_access(
                    ncfja,
                    ndses,
                    ncfja,
                    req.iordre,
                    req.ncflim,
                    &mut access,
                    &mut ier,
                )
                .expect("initialized curve error-bound slots are infallible");
            }
            if ier > 0 {
                *iercod = 1;
                return;
            }
            errors.ncoeff.set(req.ncflim, req);
            errors
                .values
                .with_error(CurveErrorOutput::RootMeanSquare, k, |value| {
                    mmaperm_(
                        ncfja,
                        ndses,
                        ncfja,
                        req.iordre,
                        &workspace.coefficients()[column..],
                        req.ncflim,
                        value,
                    );
                });
        }
        idim += ndses;
    }
}

/// `mmaperm_`: the average error of dropping the coefficients past `ncfnew` (and
/// past the constraint ones) of the curve `crvjac` (`(ncofmx, ndim)`).
fn mmaperm_(
    ncofmx: i32,
    ndim: i32,
    ncoeff: i32,
    iordre: i32,
    crvjac: &[f64],
    ncfnew: i32,
    errmoy: &mut f64,
) {
    let crv = F2cArray::new(crvjac, ncofmx + 1);
    let ia: i32 = (iordre + 1) << 1;
    let mut ncfcut: i32 = ia + 1;
    if ncfnew + 1 > ncfcut {
        ncfcut = ncfnew + 1;
    }
    *errmoy = 0.;
    let mut bid: f64 = 0.;
    for nd in 1..=ndim {
        for i in ncfcut..=ncoeff {
            let bidj: f64 = crv[i + nd * ncofmx];
            bid += bidj * bidj;
        }
    }
    bid /= 2.;
    *errmoy = bid.sqrt();
}

/// `mma1noc_`: rescales `ndimen` values of a derivative of order `ideriv` across
/// and `iordre` along the iso from the domain `dfuvin` to `duvout`, in place
/// (the retained calls pass the same array as input and output).
pub(super) fn mma1noc_(
    dfuvin: &[f64; 4],
    ndimen: i32,
    iordre: i32,
    values: &mut [f64],
    duvout: &[f64; 4],
    isofav: i32,
    ideriv: i32,
) {
    let ratio_u: f64 = (dfuvin[1] - dfuvin[0]) / (duvout[1] - duvout[0]);
    let ratio_v: f64 = (dfuvin[3] - dfuvin[2]) / (duvout[3] - duvout[2]);
    let (rider, riord) = if isofav == 1 {
        (pow_di(ratio_u, ideriv), pow_di(ratio_v, iordre))
    } else {
        (pow_di(ratio_v, ideriv), pow_di(ratio_u, iordre))
    };
    let bid: f64 = rider * riord;
    for value in values.iter_mut().take(ndimen.max(0) as usize) {
        *value *= bid;
    }
}
