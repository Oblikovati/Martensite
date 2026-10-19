//! The approximation of an iso-curve of the function: `mma2fnc_`.
use super::curve_errors::{PieceErrors, mma1noc_};
use super::curve_integers::CurveCoefficientCount;
use super::curve_integers::CurveIntegerOutputs;
use super::curve_output::{CurveCallerTables, CurveCoefficientStorage, borrowed_curve_outputs};
use super::curve_steps::{CurveSamplingEntry, PieceTables, mma1fdi_, mma1nop_};
use super::curve_workspace_control::{CurveScratch, mma2fnc_with_workspace};
use crate::adv_app2var::evaluator::AdvApp2VarEvaluatorFunc2Var;
use crate::adv_app2var::math_base::mmfmca8_;
use crate::adv_app2var::sys_base::numeric_workspace::HeapNumericWorkspace;

/// The inputs of `mma2fnc_` in the retained order.
#[derive(Clone, Copy)]
pub struct IsoApproxRequest<'a> {
    /// `ndimen`: total dimension.
    pub ndimen: i32,
    /// `nbsesp`: number of subspaces.
    pub nbsesp: i32,
    /// `ndimse`: dimension of each subspace.
    pub ndimse: &'a [i32],
    /// `uvfonc`: the domain (U0, U1, V0, V1).
    pub uvfonc: [f64; 4],
    /// `tconst`: the value of the constant parameter.
    pub tconst: f64,
    /// `isofav`: 1 for an iso U = tconst, 2 for V = tconst.
    pub isofav: i32,
    /// `nbroot`: number of Legendre roots.
    pub nbroot: i32,
    /// `rootlg`: the Legendre roots.
    pub rootlg: &'a [f64],
    /// `iordre`: continuity order at the ends (-1..=2).
    pub iordre: i32,
    /// `ideriv`: order of the cross derivative approximated.
    pub ideriv: i32,
    /// `ndgjac`: degree of the Jacobi approximation.
    pub ndgjac: i32,
    /// `nbcrmx`: maximum number of pieces.
    pub nbcrmx: i32,
    /// `ncflim`: maximum number of coefficients per piece.
    pub ncflim: i32,
    /// `epsapr`: tolerance of each subspace.
    pub epsapr: &'a [f64],
}

/// The outputs of `mma2fnc_`: per piece the coefficient count, the canonical curve
/// (`(ncflim, ndimen, nbcrmx)`), the folded samples (`(0:nbroot/2, ndimen, nbcrmx)`),
/// the end constraints (`(ndimen, iordre+2, nbcrmx)`), the cut parameters and the
/// errors (`(nbsesp, nbcrmx)`); `nbcrbe` is the number of pieces made.
pub struct IsoApproxCallerOutput<'a, Curve> {
    /// `ncoeff`.
    pub ncoeff: &'a mut [i32],
    /// `courbe`.
    pub courbe: Curve,
    /// `nbcrbe`.
    pub nbcrbe: i32,
    /// `somtab`.
    pub somtab: &'a mut [f64],
    /// `diftab`.
    pub diftab: &'a mut [f64],
    /// `contr1`.
    pub contr1: &'a mut [f64],
    /// `contr2`.
    pub contr2: &'a mut [f64],
    /// `tabdec`: the `nbcrmx + 1` cut parameters.
    pub tabdec: &'a mut [f64],
    /// `errmax`.
    pub errmax: &'a mut [f64],
    /// `errmoy`.
    pub errmoy: &'a mut [f64],
}

/// Slice-backed output with the original field coercion and caller borrowing;
/// `let output: IsoApproxOutput<'_> = /* caller slices */;`.
pub type IsoApproxOutput<'a> = IsoApproxCallerOutput<'a, &'a mut [f64]>;

/// Approximates the derivative `ideriv` across the iso `tconst` by up to `nbcrmx`
/// polynomial pieces, halving a piece whose tolerance is not met (`mma2fnc_`).
/// `iercod`: 0 done, -1 when the last allowed piece misses the tolerance, 1 for a
/// degenerate domain or a failed step, the `mma1fdi_` code (above 100) when the
/// evaluator stops; returns 0.
/// `mma2fnc_(&request, &f, &mut output, &mut ier)`.
pub fn mma2fnc_(
    req: &IsoApproxRequest<'_>,
    foncnp: &dyn AdvApp2VarEvaluatorFunc2Var,
    out: &mut IsoApproxOutput<'_>,
    iercod: &mut i32,
) -> i32 {
    let mut request = *req;
    mma2fnc_mut_(&mut request, foncnp, out, iercod)
}

/// Approximates an iso while retaining caller-visible dimension, preferred iso,
/// subspace count, caller orders and constant writes, including evaluation failure. Domain bounds
/// passed to the callback are local piece copies; `mma2fnc_mut_(&mut request, &f, &mut output, &mut code)`.
pub fn mma2fnc_mut_(
    req: &mut IsoApproxRequest<'_>,
    foncnp: &dyn AdvApp2VarEvaluatorFunc2Var,
    out: &mut IsoApproxOutput<'_>,
    iercod: &mut i32,
) -> i32 {
    mma2fnc_curve_store(req, foncnp, out, iercod)
}
pub(crate) fn mma2fnc_curve_store<Curve: CurveCoefficientStorage>(
    req: &mut IsoApproxRequest<'_>,
    foncnp: &dyn AdvApp2VarEvaluatorFunc2Var,
    out: &mut IsoApproxCallerOutput<'_, Curve>,
    iercod: &mut i32,
) -> i32 {
    let (mut tables, mut integers) = borrowed_curve_outputs(out, iercod);
    mma2fnc_integer_store(req, foncnp, &mut tables, &mut integers)
}
pub(crate) fn mma2fnc_integer_store<
    Curve: CurveCoefficientStorage,
    Integers: CurveIntegerOutputs,
>(
    req: &mut IsoApproxRequest<'_>,
    foncnp: &dyn AdvApp2VarEvaluatorFunc2Var,
    out: &mut CurveCallerTables<'_, Curve>,
    integers: &mut Integers,
) -> i32 {
    mma2fnc_with_workspace(req, foncnp, out, integers, &mut HeapNumericWorkspace)
}
/// The piece loop of `mma2fnc_` (label L1000): approximate the next piece, keep
/// it when it meets the tolerance (or when no more cut is allowed), otherwise
/// insert the middle of the piece in `tabdec` and retry.
pub(super) fn cut_until_done<Curve: CurveCoefficientStorage, Integers: CurveIntegerOutputs>(
    req: &mut IsoApproxRequest<'_>,
    foncnp: &dyn AdvApp2VarEvaluatorFunc2Var,
    out: &mut CurveCallerTables<'_, Curve>,
    work: &mut CurveScratch,
    uvpav: &mut [f64; 4],
    integers: &mut Integers,
) {
    let mut nupil: i32 = 1;
    integers.set_pieces(0, req);
    let mut ier: i32 = 0;
    while nupil - integers.pieces() != 0 {
        let nbcrbe: i32 = integers.pieces();
        let piece: [f64; 2] = [
            integers.cut_parameter(out.tabdec, nbcrbe),
            integers.cut_parameter(out.tabdec, nbcrbe + 1),
        ];
        if req.isofav == 1 {
            uvpav[2..4].copy_from_slice(&piece);
        } else {
            uvpav[0..2].copy_from_slice(&piece);
        }
        match approximate_piece(req, foncnp, out, work, uvpav, &mut ier, integers) {
            PieceOutcome::Failed => return,
            PieceOutcome::Approximated => {}
        }
        if ier == 0 || (ier == -1 && nupil == req.nbcrmx) {
            if ier == -1 {
                integers.set_status(-1, req);
            }
            store_piece(req, out, work, uvpav, nbcrbe, integers);
            integers.set_pieces(integers.pieces() + 1, req);
        } else {
            let tmil: f64 = (integers.cut_parameter(out.tabdec, nbcrbe + 1)
                + integers.cut_parameter(out.tabdec, nbcrbe))
                / 2.;
            let ideb: i32 = nbcrbe + 1;
            integers.shift_cut_parameters(
                (nupil - integers.pieces()) << 3,
                out.tabdec,
                ideb,
                ideb + 1,
                req,
            );
            integers.set_cut_parameter(out.tabdec, ideb, tmil, req);
            nupil += 1;
        }
    }
}

enum PieceOutcome {
    Approximated,
    Failed,
}

/// The views of piece `ncb1` (1-based) in the output tables.
fn piece_tables<'a, Curve: CurveCoefficientStorage, Integers: CurveIntegerOutputs>(
    out: &'a mut CurveCallerTables<'_, Curve>,
    ncb1: i32,
    storage_dimension: i32,
    storage_root_count: i32,
    storage_continuity: i32,
    integers: &Integers,
) -> PieceTables<'a> {
    // Caller table blocks keep entry dimensions while each piece samples the live count.
    let sblock: i32 = (storage_root_count / 2 + 1) * storage_dimension * (ncb1 - 1);
    let cblock: i32 = storage_dimension * (storage_continuity + 2) * (ncb1 - 1);
    PieceTables {
        folded: match integers.folded_piece(sblock) {
            Some(slots) => super::curve_folded_storage::FoldedPieceTables::owned(slots),
            None => super::curve_folded_storage::FoldedPieceTables::new(
                &mut out.somtab[sblock as usize..],
                &mut out.diftab[sblock as usize..],
            ),
        },
        endpoints: match integers.endpoint_piece(cblock) {
            Some(slots) => super::curve_endpoint_storage::EndpointPieceTables::owned(slots),
            None => super::curve_endpoint_storage::EndpointPieceTables::new(
                &mut out.contr1[cblock as usize..],
                &mut out.contr2[cblock as usize..],
            ),
        },
    }
}

/// The steps of one piece: parameters, samples, constraint removal, Jacobi
/// projection, constraint addition, truncation. `ier` receives the `mma1fer_`
/// code; a failed step sets `iercod` and ends `mma2fnc_`.
fn approximate_piece<Curve: CurveCoefficientStorage, Integers: CurveIntegerOutputs>(
    req: &mut IsoApproxRequest<'_>,
    foncnp: &dyn AdvApp2VarEvaluatorFunc2Var,
    out: &mut CurveCallerTables<'_, Curve>,
    work: &mut CurveScratch,
    uvpav: &mut [f64; 4],
    ier: &mut i32,
    integers: &mut Integers,
) -> PieceOutcome {
    let ncb1: i32 = integers.pieces() + 1;
    let roots = foncnp.caller_iso_roots(req.rootlg);
    mma1nop_(
        req.nbroot,
        roots.as_ref(),
        uvpav,
        req.isofav,
        work.projection.parameter_storage_mut(),
        ier,
    );
    if *ier > 0 {
        integers.set_status(1, req);
        return PieceOutcome::Failed;
    }
    let mut piece: PieceTables<'_> = piece_tables(
        out,
        ncb1,
        work.output_layout.dimension,
        work.output_layout.root_count,
        work.output_layout.continuity,
        integers,
    );
    let entry = CurveSamplingEntry::capture(req);
    integers.prepare_sampling(req);
    integers.with_status(|status| {
        mma1fdi_(
            entry,
            req,
            uvpav,
            foncnp,
            &mut work.projection,
            &mut piece,
            status,
        )
    });
    integers.refresh_request(req);
    if integers.status() > 0 {
        return PieceOutcome::Failed;
    }
    if req.iordre >= 0 {
        // The callback may replace owned roots after parameter generation; reread them for the basis stage.
        let roots = foncnp.caller_iso_roots(req.rootlg);
        let constraint_request = IsoApproxRequest {
            rootlg: roots.as_ref(),
            ..*req
        };
        work.projection
            .remove_constraints(&constraint_request, &mut piece, ier);
        if *ier > 0 {
            integers.set_status(1, req);
            return PieceOutcome::Failed;
        }
    }
    work.projection.project(req, &piece, ier);
    if *ier > 0 {
        integers.set_status(1, req);
        return PieceOutcome::Failed;
    }
    if req.iordre >= 0 {
        work.projection.add_constraints(req, &piece);
    }
    // retained error-table strides are fixed before callbacks; live counts only select rows.
    let column = work.output_layout.subspaces * (ncb1 - 1);
    let values = match integers.error_piece(column) {
        Some(slots) => super::curve_error_storage::PieceErrorTables::owned(slots),
        None => super::curve_error_storage::PieceErrorTables::new(
            &mut out.errmax[column as usize..],
            &mut out.errmoy[column as usize..],
        ),
    };
    let mut errors = PieceErrors {
        values,
        ncoeff: CurveCoefficientCount {
            integers,
            values: out.ncoeff,
            index: ncb1 - 1,
        },
    };
    // Owned caller dimensions and requested limits can change in the evaluator; this stage runs no callbacks.
    let dimensions = foncnp.caller_subspace_dimensions(req.ndimse);
    let tolerances = foncnp.caller_subspace_tolerances(req.epsapr);
    let mut truncation_request = IsoApproxRequest {
        ndimse: dimensions.as_ref(),
        epsapr: tolerances.as_ref(),
        ..*req
    };
    work.projection
        .truncate(&mut truncation_request, &mut errors, ier);
    integers.refresh_request(req);
    if *ier > 0 {
        integers.set_status(1, req);
        return PieceOutcome::Failed;
    }
    PieceOutcome::Approximated
}

/// Keeps the current piece: its Jacobi coefficients go to the canonical base on
/// [-1,1] in `courbe`, and every table of the piece is rescaled to its domain.
fn store_piece<Curve: CurveCoefficientStorage, Integers: CurveIntegerOutputs>(
    req: &IsoApproxRequest<'_>,
    out: &mut CurveCallerTables<'_, Curve>,
    work: &mut CurveScratch,
    uvpav: &[f64; 4],
    completed_pieces: i32,
    integers: &Integers,
) {
    let ncb1: i32 = completed_pieces + 1;
    let ndimen: i32 = req.ndimen;
    let ncoeff: i32 = integers.coefficient_count(out.ncoeff, ncb1 - 1);
    let ncfja: i32 = req.ndgjac + 1;
    work.projection.compress(ndimen, ncfja, ncoeff);
    work.projection
        .clear_coefficient_span(0, ndimen * req.ncflim);
    let ndgre: i32 = ncoeff - 1;
    for nd in 1..=ndimen {
        work.projection.canonical_component(req.iordre, ndgre, nd);
    }
    // retained caller layout is fixed on entry; the matrix copy still uses the live limit.
    let cblock: i32 =
        work.output_layout.coefficient_limit * work.output_layout.dimension * (ncb1 - 1);
    out.courbe.with_curve_values_at(cblock, |values| {
        mmfmca8_(
            ncoeff,
            ndimen,
            1,
            req.ncflim,
            ndimen,
            1,
            work.projection.coefficients(),
            values,
        )
    });
    rescale_piece(
        req,
        out,
        ncb1,
        ncoeff,
        uvpav,
        work.output_layout.dimension,
        work.output_layout.coefficient_limit,
        work.output_layout.root_count,
        work.output_layout.continuity,
        integers,
    );
}

/// The `mma1noc_` calls of the kept piece: constraints of each order, folded
/// samples and curve coefficients, from [-1,1]x[-1,1] to the piece's domain.
fn rescale_piece<Curve: CurveCoefficientStorage, Integers: CurveIntegerOutputs>(
    req: &IsoApproxRequest<'_>,
    out: &mut CurveCallerTables<'_, Curve>,
    ncb1: i32,
    ncoeff: i32,
    uvpav: &[f64; 4],
    storage_dimension: i32,
    storage_coefficient_limit: i32,
    storage_root_count: i32,
    storage_continuity: i32,
    integers: &Integers,
) {
    const UV11: [f64; 4] = [-1., 1., -1., 1.];
    let (ndimen, isofav, ideriv) = (req.ndimen, req.isofav, req.ideriv);
    let mut piece: PieceTables<'_> = piece_tables(
        out,
        ncb1,
        storage_dimension,
        storage_root_count,
        storage_continuity,
        integers,
    );
    for ii in 0..=req.iordre {
        let start = ii * storage_dimension;
        for end in [
            super::curve_endpoint_storage::ConstraintEnd::Lower,
            super::curve_endpoint_storage::ConstraintEnd::Upper,
        ] {
            piece.endpoints.with_values(end, start, |values| {
                mma1noc_(&UV11, ndimen, ii, values, uvpav, isofav, ideriv)
            });
        }
    }
    let ibid1: i32 = (req.nbroot / 2 + 1) * ndimen;
    for table in [
        super::curve_folded_storage::FoldedTable::SymmetricSum,
        super::curve_folded_storage::FoldedTable::AntisymmetricDifference,
    ] {
        piece.folded.with_table_values(table, |values| {
            mma1noc_(&UV11, ibid1, 0, values, uvpav, isofav, ideriv)
        });
    }
    let cblock: i32 = storage_coefficient_limit * storage_dimension * (ncb1 - 1);
    for nd in 1..=ndimen {
        let start: i32 = cblock + (nd - 1) * storage_coefficient_limit;
        out.courbe.with_curve_values_at(start, |values| {
            mma1noc_(&UV11, ncoeff, 0, values, uvpav, isofav, ideriv)
        });
    }
}
