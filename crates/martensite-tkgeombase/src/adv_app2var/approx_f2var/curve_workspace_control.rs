//! One outer workspace lifetime preserves caller store and resource failure order.
use super::curve::{IsoApproxRequest, cut_until_done};
use super::curve_integers::CurveIntegerOutputs;
use super::curve_output::{CurveCallerTables, CurveCoefficientStorage};
use super::curve_output_layout::CurveCallerLayout;
use super::curve_workspace::CurveProjectionFrame;
use crate::adv_app2var::evaluator::AdvApp2VarEvaluatorFunc2Var;
use crate::adv_app2var::math_base::mmveps3_;
use crate::adv_app2var::sys_base::{mvriraz_, numeric_workspace::NumericWorkspace};
pub(super) struct CurveScratch {
    pub(super) output_layout: CurveCallerLayout,
    pub(super) projection: CurveProjectionFrame,
}
pub(crate) fn mma2fnc_with_workspace<
    Curve: CurveCoefficientStorage,
    Integers: CurveIntegerOutputs,
>(
    req: &mut IsoApproxRequest<'_>,
    evaluator: &dyn AdvApp2VarEvaluatorFunc2Var,
    out: &mut CurveCallerTables<'_, Curve>,
    integers: &mut Integers,
    workspace: &mut dyn NumericWorkspace,
) -> i32 {
    let Some(layout) = prepare_caller(req, evaluator, out, integers) else {
        return 0;
    };
    let Ok(mut work) = allocate_scratch(req, layout, workspace) else {
        integers.set_status(13, req);
        return 0;
    };
    run_allocated_curve(req, evaluator, out, integers, &mut work);
    // retained  cleanup overrides an earlier callback failure after preserving its stores.
    if workspace.release(work.projection.into_values()).is_err() {
        integers.set_status(13, req);
    }
    0
}
fn prepare_caller<Curve: CurveCoefficientStorage, Integers: CurveIntegerOutputs>(
    req: &mut IsoApproxRequest<'_>,
    evaluator: &dyn AdvApp2VarEvaluatorFunc2Var,
    out: &mut CurveCallerTables<'_, Curve>,
    integers: &mut Integers,
) -> Option<CurveCallerLayout> {
    // retained  fixes caller strides before a status write can change incident inputs.
    let layout = CurveCallerLayout::capture(req);
    integers.set_status(0, req);
    out.courbe
        .with_curve_values(|values| mvriraz_(req.ndimen * req.ncflim * req.nbcrmx, values));
    let mut eps3 = 0.;
    mmveps3_(&mut eps3);
    // retained  clears COURBE before reading potentially incident UVFONC.
    // Validate here; ordered cut stores below can change caller domains before initial bounds are captured.
    // Later callback writes must not recapture already chosen piece bounds.
    let uv = evaluator.caller_iso_domain(req.uvfonc);
    if (uv[1] - uv[0]).abs() < eps3 || (uv[3] - uv[2]).abs() < eps3 {
        integers.set_status(1, req);
        return None;
    }
    Some(layout)
}
fn workspace_lengths(req: &IsoApproxRequest<'_>) -> [i32; 5] {
    let (dimension, roots, order) = (req.ndimen, req.nbroot, req.iordre);
    let samples = (dimension * (roots + 2))
        .max(((order + 1) << 1) * roots)
        .max((((req.ncflim - 1) / 2 + 1) << 1) * dimension);
    [
        roots + 2,
        samples,
        ((order + 1) << 2) * (order + 1),
        (roots / 2 + 1) * (req.ndgjac + 1 - ((order + 1) << 1)),
        (req.ndgjac + 1) * dimension,
    ]
}
fn allocate_scratch(
    req: &IsoApproxRequest<'_>,
    layout: CurveCallerLayout,
    workspace: &mut dyn NumericWorkspace,
) -> Result<CurveScratch, ()> {
    let lengths = workspace_lengths(req);
    let length = usize::try_from(lengths.iter().sum::<i32>()).map_err(|_| ())?;
    let values = workspace.allocate(length)?;
    // retained  advances by signed region lengths; an unused origin need not lie in the payload.
    let mut offset = 0;
    let starts = std::array::from_fn(|index| {
        offset += lengths[index];
        offset as usize
    });
    Ok(CurveScratch {
        output_layout: layout,
        projection: CurveProjectionFrame::with_zone_starts(starts, values),
    })
}
fn run_allocated_curve<Curve: CurveCoefficientStorage, Integers: CurveIntegerOutputs>(
    req: &mut IsoApproxRequest<'_>,
    evaluator: &dyn AdvApp2VarEvaluatorFunc2Var,
    out: &mut CurveCallerTables<'_, Curve>,
    integers: &mut Integers,
    work: &mut CurveScratch,
) {
    let mut piece_domain = [0.; 4];
    if super::curve_cut_initialization::initialize_cut_parameters(
        req,
        evaluator,
        out.tabdec,
        &mut piece_domain,
        integers,
    ) {
        cut_until_done(req, evaluator, out, work, &mut piece_domain, integers);
    }
}
