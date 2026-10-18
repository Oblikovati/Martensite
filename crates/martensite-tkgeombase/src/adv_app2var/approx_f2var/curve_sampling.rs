//! Curve sampling behavior and supporting operations.
use super::super::curve::IsoApproxRequest;
#[path = "curve_sampling_layout.rs"]
pub(crate) mod layout;
use super::super::curve_endpoint_storage::{ConstraintEnd, EndpointPieceTables};
use super::super::curve_workspace::CurveProjectionFrame;
use crate::adv_app2var::evaluator::AdvApp2VarEvaluatorFunc2Var;
use crate::adv_app2var::math_base::pow__di;
use crate::adv_app2var::{
    AdvApp2VarBufferEvaluation, AdvApp2VarDerivativeOrders, AdvApp2VarEvaluationBuffers,
};
use layout::{CurveSamplingEntry, fold_samples};

/// The per-piece tables of `mma1fdi_`, `mma1cdi_` and `mma1cnt_`: the sums and
/// differences `(0:nbroot/2, ndimen)` and the end constraints `(ndimen, iordre+1)`.
pub(crate) struct PieceTables<'a> {
    pub(crate) folded:
        crate::adv_app2var::approx_f2var::curve_folded_storage::FoldedPieceTables<'a>,
    pub(crate) endpoints: EndpointPieceTables<'a>,
}

/// `mma1fdi_`: samples the derivative `ideriv` of the function across the iso at
/// the parameters `ttable` into `fpntab`, folds the samples into sums and
/// differences and reads the end constraints up to order `iordre`, all scaled to
/// [-1,1]. `iercod` is the evaluator code plus 100 when it stops the sampling.
pub(crate) fn mma1fdi_(
    entry: CurveSamplingEntry,
    req: &mut IsoApproxRequest<'_>,
    uvfonc: &mut [f64; 4],
    foncnp: &dyn AdvApp2VarEvaluatorFunc2Var,
    workspace: &mut CurveProjectionFrame,
    piece: &mut PieceTables<'_>,
    iercod: &mut i32,
) {
    *iercod = 0;
    sample_iso(entry, req, uvfonc, foncnp, workspace, piece, iercod);
    if *iercod > 0 {
        *iercod += 100;
    }
}

fn sample_iso(
    entry: CurveSamplingEntry,
    req: &mut IsoApproxRequest<'_>,
    uvfonc: &mut [f64; 4],
    foncnp: &dyn AdvApp2VarEvaluatorFunc2Var,
    workspace: &mut CurveProjectionFrame,
    piece: &mut PieceTables<'_>,
    iercod: &mut i32,
) {
    let (ndimen, nbroot, iordre) = (req.ndimen, req.nbroot, req.iordre);
    let (ideb, ifin) = sampling_limits(iordre, nbroot);
    let (u_axis, span) = sampling_axis(req.isofav, uvfonc);
    let renor = pow__di(span / 2., req.ideriv);
    let mut state = sampling_call(ndimen, *uvfonc, req, ifin - ideb + 1, u_axis);
    let fd1 = entry.dimension;
    initial_sample(&mut state, foncnp, workspace, ideb, fd1, iercod);
    if *iercod > 0 {
        state.write_back(req, uvfonc);
        return;
    }
    fold_samples(
        state.dimension,
        entry,
        nbroot,
        state.caller_sampling[0],
        renor,
        workspace,
        piece,
    );
    if state.caller_orders[0] >= 0 {
        let domain = state.domain();
        fill_endpoint_constraints(
            &state,
            renor,
            state.caller_sampling[0],
            entry.dimension,
            workspace,
            piece,
        );
        let parameters = workspace.endpoint_parameter_storage_mut();
        let storage_request = IsoApproxRequest {
            ndimen: entry.dimension,
            ..*req
        };
        finish_endpoint_samples(
            &mut state,
            &storage_request,
            foncnp,
            &domain,
            renor,
            parameters,
            piece,
            iercod,
        );
    }
    state.write_back(req, uvfonc);
}

fn sampling_limits(iordre: i32, nbroot: i32) -> (i32, i32) {
    if iordre >= 0 {
        (0, nbroot + 1)
    } else {
        (1, nbroot)
    }
}

fn sampling_axis(isofav: i32, uvfonc: &[f64; 4]) -> (bool, f64) {
    if isofav == 1 {
        (true, uvfonc[1] - uvfonc[0])
    } else {
        (false, uvfonc[3] - uvfonc[2])
    }
}

fn sampling_call(
    dimension: i32,
    uvfonc: [f64; 4],
    req: &IsoApproxRequest<'_>,
    nb_params: i32,
    u_axis: bool,
) -> SamplingCall {
    let orders = if u_axis {
        (req.ideriv, 0)
    } else {
        (0, req.ideriv)
    };
    SamplingCall::new(dimension, uvfonc, req, nb_params, orders.0, orders.1)
}

fn initial_sample(
    state: &mut SamplingCall,
    func: &dyn AdvApp2VarEvaluatorFunc2Var,
    workspace: &mut CurveProjectionFrame,
    first: i32,
    stride: i32,
    error_code: &mut i32,
) {
    state.evaluate_buffers(
        func,
        workspace.callback_buffers(first as usize, stride as usize),
        error_code,
    );
}

fn fill_endpoint_constraints(
    state: &SamplingCall,
    renor: f64,
    nbroot: i32,
    stride: i32,
    workspace: &CurveProjectionFrame,
    piece: &mut PieceTables<'_>,
) {
    let endpoints = &mut piece.endpoints;
    for component in 1..=state.dimension {
        endpoints.set_value(
            ConstraintEnd::Lower,
            component + stride - (stride + 1),
            renor * workspace.sample_subscript(i64::from(component)),
        );
        endpoints.set_value(
            ConstraintEnd::Upper,
            component + stride - (stride + 1),
            renor
                * workspace.sample_subscript(
                    i64::from(component) + (i64::from(nbroot) + 1) * i64::from(stride),
                ),
        );
    }
}

fn finish_endpoint_samples(
    state: &mut SamplingCall,
    req: &IsoApproxRequest<'_>,
    func: &dyn AdvApp2VarEvaluatorFunc2Var,
    uvfonc: &[f64; 4],
    renor: f64,
    ttable: (&mut [f64], usize),
    piece: &mut PieceTables<'_>,
    error_code: &mut i32,
) {
    // The axis scale belongs to the selected endpoint loop, before callbacks mutate it.
    let half_span = endpoint_span(state.favor_iso, uvfonc) / 2.;
    evaluate_endpoint_derivatives(state, req, func, ttable, error_code, piece);
    if *error_code <= 0 {
        normalize_endpoint_derivatives(state, half_span, renor, req, piece);
    }
}

fn evaluate_endpoint_derivatives(
    state: &mut SamplingCall,
    req: &IsoApproxRequest<'_>,
    func: &dyn AdvApp2VarEvaluatorFunc2Var,
    ttable: (&mut [f64], usize),
    error_code: &mut i32,
    piece: &mut PieceTables<'_>,
) {
    state.nb_params = 1;
    let varying_v = state.favor_iso == 1;
    let mut sampler = EndpointSampler::new(func, ttable, req, varying_v);
    sampler.evaluate(
        state,
        EndpointParameter::Lower,
        (&mut piece.endpoints, ConstraintEnd::Lower),
        error_code,
    );
    if *error_code > 0 {
        return;
    }
    sampler.evaluate(
        state,
        EndpointParameter::Upper,
        (&mut piece.endpoints, ConstraintEnd::Upper),
        error_code,
    );
}

#[derive(Clone, Copy)]
enum EndpointParameter {
    Lower,
    Upper,
}
impl EndpointParameter {
    fn index(self, roots: i32) -> usize {
        match self {
            Self::Lower => 0,
            Self::Upper => (roots + 1) as usize,
        }
    }
}

struct EndpointSampler<'a> {
    func: &'a dyn AdvApp2VarEvaluatorFunc2Var,
    parameters: (&'a mut [f64], usize),
    stride: i32,
    varying_v: bool,
}

impl<'a> EndpointSampler<'a> {
    fn new(
        func: &'a dyn AdvApp2VarEvaluatorFunc2Var,
        parameters: (&'a mut [f64], usize),
        req: &IsoApproxRequest<'_>,
        varying_v: bool,
    ) -> Self {
        Self {
            func,
            parameters,
            stride: req.ndimen,
            varying_v,
        }
    }

    fn evaluate(
        &mut self,
        state: &mut SamplingCall,
        parameter: EndpointParameter,
        constraints: (&mut EndpointPieceTables<'_>, ConstraintEnd),
        error_code: &mut i32,
    ) {
        set_cross_order(state, self.varying_v);
        // Each endpoint loop captures the live caller bound, independently of local orders.
        let max_order = state.caller_orders[0];
        evaluate_cross_orders(
            state,
            self.func,
            (&mut *self.parameters.0, self.parameters.1, parameter),
            (constraints.0, constraints.1, self.stride + 1),
            max_order,
            self.stride,
            self.varying_v,
            error_code,
        );
    }
}

fn set_cross_order(state: &mut SamplingCall, varying_v: bool) {
    let selected = if varying_v {
        &mut state.v_order
    } else {
        &mut state.u_order
    };
    *selected = 1;
}

fn normalize_endpoint_derivatives(
    state: &SamplingCall,
    half_span: f64,
    renor: f64,
    req: &IsoApproxRequest<'_>,
    piece: &mut PieceTables<'_>,
) {
    let endpoints = &mut piece.endpoints;
    let mut scale = renor;
    for order in 1..=state.caller_orders[0] {
        scale *= half_span;
        scale_endpoint_order(order, state.dimension, req.ndimen, scale, endpoints);
    }
}

fn endpoint_span(favorite_iso: i32, domain: &[f64; 4]) -> f64 {
    if favorite_iso == 1 {
        return domain[3] - domain[2];
    }
    domain[1] - domain[0]
}

fn scale_endpoint_order(
    order: i32,
    dimension: i32,
    stride: i32,
    scale: f64,
    endpoints: &mut EndpointPieceTables<'_>,
) {
    for component in 1..=dimension {
        let index = component + (order + 1) * stride - (stride + 1);
        for end in [ConstraintEnd::Lower, ConstraintEnd::Upper] {
            endpoints.set_value(end, index, endpoints.value(end, index) * scale);
        }
    }
}

struct SamplingCall {
    subspace_count: i32,
    caller_orders: [i32; 2],
    caller_limits: [i32; 2],
    caller_sampling: [i32; 2],
    dimension: i32,
    u_start_end: [f64; 2],
    v_start_end: [f64; 2],
    favor_iso: i32,
    const_param: f64,
    nb_params: i32,
    u_order: i32,
    v_order: i32,
}

impl SamplingCall {
    fn new(
        dimension: i32,
        uvfonc: [f64; 4],
        req: &IsoApproxRequest<'_>,
        nb_params: i32,
        u_order: i32,
        v_order: i32,
    ) -> Self {
        Self {
            subspace_count: req.nbsesp,
            caller_orders: [req.iordre, req.ideriv],
            caller_limits: [req.nbcrmx, req.ncflim],
            caller_sampling: [req.nbroot, req.ndgjac],
            dimension,
            u_start_end: [uvfonc[0], uvfonc[1]],
            v_start_end: [uvfonc[2], uvfonc[3]],
            favor_iso: req.isofav,
            const_param: req.tconst,
            nb_params,
            u_order,
            v_order,
        }
    }

    fn domain(&self) -> [f64; 4] {
        [
            self.u_start_end[0],
            self.u_start_end[1],
            self.v_start_end[0],
            self.v_start_end[1],
        ]
    }

    fn write_back(&self, request: &mut IsoApproxRequest<'_>, domain: &mut [f64; 4]) {
        request.nbsesp = self.subspace_count;
        [request.iordre, request.ideriv] = self.caller_orders;
        [request.nbcrmx, request.ncflim] = self.caller_limits;
        [request.nbroot, request.ndgjac] = self.caller_sampling;
        request.ndimen = self.dimension;
        request.isofav = self.favor_iso;
        request.tconst = self.const_param;
        *domain = self.domain();
    }

    fn evaluate_buffers(
        &mut self,
        func: &dyn AdvApp2VarEvaluatorFunc2Var,
        buffers: AdvApp2VarEvaluationBuffers<'_>,
        error_code: &mut i32,
    ) {
        let mut call = AdvApp2VarBufferEvaluation {
            dimension: &mut self.dimension,
            u_start_end: &mut self.u_start_end,
            v_start_end: &mut self.v_start_end,
            favor_iso: &mut self.favor_iso,
            const_param: &mut self.const_param,
            nb_params: &mut self.nb_params,
            buffers,
            orders: AdvApp2VarDerivativeOrders::Independent {
                u: &mut self.u_order,
                v: &mut self.v_order,
            },
        };
        let mut callers = crate::adv_app2var::AdvApp2VarIsoCallerIntegers::new(
            &mut self.subspace_count,
            &mut self.caller_orders,
        )
        .with_limits(&mut self.caller_limits)
        .with_sampling(&mut self.caller_sampling);
        func.evaluate_iso_buffers(&mut call, &mut callers, error_code);
    }
}

fn evaluate_cross_orders(
    state: &mut SamplingCall,
    func: &dyn AdvApp2VarEvaluatorFunc2Var,
    parameters: (&mut [f64], usize, EndpointParameter),
    table: (&mut EndpointPieceTables<'_>, ConstraintEnd, i32),
    max_order: i32,
    stride: i32,
    varying_v: bool,
    error_code: &mut i32,
) {
    let (parameters, legacy_end, endpoint) = parameters;
    let (table, end, origin) = table;
    while cross_order(state, varying_v) <= max_order {
        let order = cross_order(state, varying_v);
        let result_start = (order + 1) * stride + 1;
        // Root count is a live caller control; reselect the upper sample each invocation.
        let parameter = endpoint.index(state.caller_sampling[0]);
        let legacy_parameters = legacy_end
            .saturating_sub(parameter)
            .max(state.nb_params.max(0) as usize);
        let buffers = table.callback_buffers(
            end,
            result_start - origin,
            &mut parameters[parameter..],
            legacy_parameters,
        );
        state.evaluate_buffers(func, buffers, error_code);
        if *error_code > 0 {
            return;
        }
        increment_cross_order(state, varying_v);
    }
}

fn cross_order(state: &SamplingCall, varying_v: bool) -> i32 {
    if varying_v {
        state.v_order
    } else {
        state.u_order
    }
}

fn increment_cross_order(state: &mut SamplingCall, varying_v: bool) {
    if varying_v {
        state.v_order += 1;
    } else {
        state.u_order += 1;
    }
}

#[cfg(test)]
#[path = "curve_steps_tests.rs"]
mod tests;
