//! Sampling and symmetry folds retain the entry allocation strides.
use super::table_storage::{SamplingTableLayout, SquareTableRole as Role, SquareTableStorage};
use super::*;
use crate::adv_app2var::AdvApp2VarSquareRootAxis as RootAxis;
use crate::adv_app2var::{AdvApp2VarBufferEvaluation, AdvApp2VarDerivativeOrders};

pub(super) struct SquareRows<'a, 'b, 'buffer> {
    square: &'a SampledSquare<'b>,
    sampler: RowSampler<'a>,
    pub(super) state: MutableSquareState,
    tables: &'a mut dyn SquareTableStorage,
    layout: SamplingTableLayout,
    buffers: &'a mut AdvApp2VarEvaluationBuffers<'buffer>,
    integers: &'a mut dyn SquareIntegerStorage,
    alinv: f64,
    blinv: f64,
    transposed: bool,
    roots: SquareRootBindings,
}

impl<'a, 'b, 'buffer> SquareRows<'a, 'b, 'buffer> {
    pub(super) fn new(
        square: &'a SampledSquare<'b>,
        iiuouv: i32,
        integers: &'a mut dyn SquareIntegerStorage,
        foncnp: &'a dyn AdvApp2VarEvaluatorFunc2Var,
        tables: &'a mut dyn SquareTableStorage,
        buffers: &'a mut AdvApp2VarEvaluationBuffers<'buffer>,
        roots: SquareRootBindings,
    ) -> Self {
        initialize_parameter_roots(square, buffers, roots, iiuouv == 1);
        let state = mutable_square_state(square, iiuouv);
        Self::from_parts(square, foncnp, state, tables, buffers, integers, roots)
    }

    fn from_parts(
        square: &'a SampledSquare<'b>,
        foncnp: &'a dyn AdvApp2VarEvaluatorFunc2Var,
        state: MutableSquareState,
        tables: &'a mut dyn SquareTableStorage,
        buffers: &'a mut AdvApp2VarEvaluationBuffers<'buffer>,
        integers: &'a mut dyn SquareIntegerStorage,
        roots: SquareRootBindings,
    ) -> Self {
        let v = square.vintfn;
        Self {
            square,
            sampler: RowSampler { foncnp },
            transposed: state.favor_iso == 1,
            roots,
            state,
            tables,
            layout: SamplingTableLayout::new([square.nbpntu, square.nbpntv]),
            buffers,
            integers,
            alinv: (v[1] - v[0]) / 2.,
            blinv: (v[1] + v[0]) / 2.,
        }
    }

    pub(super) fn run(&mut self) {
        self.sample_negative_rows();
        if self.integers.integer(SquareInteger::ErrorCode) > 0 || !self.sample_middle_row() {
            return;
        }
        self.sample_positive_rows();
    }

    fn sample_negative_rows(&mut self) {
        for iv in 1..=self.square.nbpntv / 2 {
            let root = self.constant_root(iv);
            let row = self.nvroo() - iv + 1;
            if !self.sample_and_fold(self.blinv + self.alinv * root, row, -1.) {
                return;
            }
        }
    }

    fn sample_middle_row(&mut self) -> bool {
        if self.live_counts()[1] % 2 == 0 {
            return true;
        }
        if !self
            .sampler
            .row(&mut self.state, self.blinv, self.buffers, self.integers)
        {
            return false;
        }
        fold_middle_row(
            self.tables,
            self.buffers,
            self.square,
            &self.state,
            &self.layout,
        );
        true
    }

    fn sample_positive_rows(&mut self) {
        let nvroo = self.nvroo();
        for iv in 1..=nvroo {
            let root_index = (self.live_counts()[1] + 1) / 2 + iv;
            let root = self.constant_root(root_index);
            let constant = self.alinv * root + self.blinv;
            if !self.sample_and_fold(constant, iv, 1.) {
                return;
            }
        }
    }

    fn sample_and_fold(&mut self, constant: f64, row: i32, sign: f64) -> bool {
        if !self
            .sampler
            .row(&mut self.state, constant, self.buffers, self.integers)
        {
            return false;
        }
        fold_row(
            self.tables,
            self.buffers,
            self.square,
            &self.state,
            row,
            sign,
            &self.layout,
        );
        true
    }

    // Source  captures allocation strides/half-counts at6032–6034 but
    // rereads caller counts at6098 and6145; do not replace the entry layout.
    pub(super) fn live_counts(&self) -> [i32; 2] {
        self.integers.root_counts(self.transposed)
    }
    fn constant_root(&self, index: i32) -> f64 {
        let axis = if self.transposed {
            RootAxis::U
        } else {
            RootAxis::V
        };
        self.roots
            .read(axis, index as isize - 1, self.square.vrootb, self.buffers)
    }
    fn nvroo(&self) -> i32 {
        self.square.nbpntv / 2
    }
}

fn initialize_parameter_roots(
    square: &SampledSquare<'_>,
    buffers: &mut AdvApp2VarEvaluationBuffers<'_>,
    roots: SquareRootBindings,
    transposed: bool,
) {
    let u = square.uintfn;
    let (half_span, center) = ((u[1] - u[0]) / 2., (u[1] + u[0]) / 2.);
    let axis = if transposed { RootAxis::V } else { RootAxis::U };
    for index in 1..=square.nbpntu {
        let root = roots.read(axis, index as isize - 1, square.urootb, buffers);
        buffers.set_parameter((index - 1) as usize, center + half_span * root);
    }
}

fn mutable_square_state(square: &SampledSquare<'_>, iso: i32) -> MutableSquareState {
    let (first, second) = if iso == 1 {
        (square.vintfn, square.uintfn)
    } else {
        (square.uintfn, square.vintfn)
    };
    MutableSquareState::new(square, iso, first, second)
}

/// One `Evaluate` call of `mma2ds2_` along the U roots at the constant `tcons`.
pub(super) struct MutableSquareState {
    pub(super) dimension: i32,
    pub(super) parameter_count: i32,
    favor_iso: i32,
    intervals: [[f64; 2]; 2],
    derivative_order: i32,
    original_transposed: bool,
}

impl MutableSquareState {
    fn new(
        square: &SampledSquare<'_>,
        favor_iso: i32,
        first_interval: [f64; 2],
        second_interval: [f64; 2],
    ) -> Self {
        Self {
            dimension: square.ndimen,
            parameter_count: square.nbpntu,
            favor_iso,
            intervals: [first_interval, second_interval],
            derivative_order: 0,
            original_transposed: favor_iso == 1,
        }
    }
}

struct RowSampler<'a> {
    foncnp: &'a dyn AdvApp2VarEvaluatorFunc2Var,
}

impl RowSampler<'_> {
    /// Evaluates the row; false when the evaluator returns a positive code.
    fn row(
        &self,
        state: &mut MutableSquareState,
        constant: f64,
        buffers: &mut AdvApp2VarEvaluationBuffers<'_>,
        integers: &mut dyn SquareIntegerStorage,
    ) -> bool {
        evaluate_row(self.foncnp, state, constant, buffers, integers);
        integers.integer(SquareInteger::ErrorCode) <= 0
    }
}

fn evaluate_row(
    foncnp: &dyn AdvApp2VarEvaluatorFunc2Var,
    state: &mut MutableSquareState,
    constant: f64,
    buffers: &mut AdvApp2VarEvaluationBuffers<'_>,
    integers: &mut dyn SquareIntegerStorage,
) {
    state.dimension = integers.integer(SquareInteger::Dimension);
    let counts = integers.root_counts(state.original_transposed);
    state.parameter_count = counts[0];
    let mut code = integers.integer(SquareInteger::ErrorCode);
    let mut const_param = constant;
    let [u_start_end, v_start_end] = &mut state.intervals;
    let mut call = AdvApp2VarBufferEvaluation {
        dimension: &mut state.dimension,
        u_start_end,
        v_start_end,
        favor_iso: &mut state.favor_iso,
        const_param: &mut const_param,
        nb_params: &mut state.parameter_count,
        buffers: buffers.reborrow(),
        orders: AdvApp2VarDerivativeOrders::Shared(&mut state.derivative_order),
    };
    foncnp.evaluate_square_buffers(&mut call, &mut code);
    integers.set_integer(SquareInteger::Dimension, state.dimension);
    let varying = if state.original_transposed {
        SquareInteger::VRootCount
    } else {
        SquareInteger::URootCount
    };
    integers.set_integer(varying, state.parameter_count);
    integers.set_integer(SquareInteger::ErrorCode, code);
}

/// Folds one row of samples at V root row `jv` into the tables: `sign` is -1 for
/// the rows at `-t` (subtracted in the V differences) and +1 for those at `+t`.
fn fold_row(
    t: &mut dyn SquareTableStorage,
    buffers: &mut AdvApp2VarEvaluationBuffers<'_>,
    square: &SampledSquare<'_>,
    state: &MutableSquareState,
    jv: i32,
    sign: f64,
    layout: &SamplingTableLayout,
) {
    let [soso, diso, sodi, didi] = sampling_roles(state.original_transposed);
    let ndimen: i32 = state.dimension;
    let nuroo: i32 = square.nbpntu / 2;
    let jdec: i32 = (square.nbpntu + 1) / 2;
    for id in 1..=ndimen {
        for iu in 1..=nuroo {
            let up: f64 = sample_value(buffers, square.ndimen, id + (iu + jdec) * square.ndimen);
            let um: f64 = sample_value(
                buffers,
                square.ndimen,
                id + (nuroo - iu + 1) * square.ndimen,
            );
            let (f, h) = (layout.f(iu, jv, id), layout.h(iu, jv, id));
            let value = t.table_at(soso, f, buffers)[0] + up + um;
            t.table_at_mut(soso, f, buffers)[0] = value;
            let value = t.table_at(diso, h, buffers)[0] + up - um;
            t.table_at_mut(diso, h, buffers)[0] = value;
            if sign < 0. {
                let value = t.table_at(sodi, h, buffers)[0] - up - um;
                t.table_at_mut(sodi, h, buffers)[0] = value;
                let value = t.table_at(didi, f, buffers)[0] - up + um;
                t.table_at_mut(didi, f, buffers)[0] = value;
            } else {
                let value = t.table_at(sodi, h, buffers)[0] + up + um;
                t.table_at_mut(sodi, h, buffers)[0] = value;
                let value = t.table_at(didi, f, buffers)[0] + up - um;
                t.table_at_mut(didi, f, buffers)[0] = value;
            }
        }
        if state.parameter_count % 2 != 0 {
            let up: f64 = sample_value(buffers, square.ndimen, id + jdec * square.ndimen);
            let f: isize = layout.f(0, jv, id);
            let value = t.table_at(soso, f, buffers)[0] + up;
            t.table_at_mut(soso, f, buffers)[0] = value;
            if sign < 0. {
                let value = t.table_at(didi, f, buffers)[0] - up;
                t.table_at_mut(didi, f, buffers)[0] = value;
            } else {
                let value = t.table_at(didi, f, buffers)[0] + up;
                t.table_at_mut(didi, f, buffers)[0] = value;
            }
        }
    }
}

/// Folds the row at the middle V root of an odd count into the `jv = 0` entries.
fn fold_middle_row(
    t: &mut dyn SquareTableStorage,
    buffers: &mut AdvApp2VarEvaluationBuffers<'_>,
    square: &SampledSquare<'_>,
    state: &MutableSquareState,
    layout: &SamplingTableLayout,
) {
    let [soso, _, _, didi] = sampling_roles(state.original_transposed);
    let ndimen: i32 = state.dimension;
    let nuroo: i32 = square.nbpntu / 2;
    let jdec: i32 = (square.nbpntu + 1) / 2;
    for id in 1..=ndimen {
        for iu in 1..=nuroo {
            let up: f64 = sample_value(buffers, square.ndimen, id + (jdec + iu) * square.ndimen);
            let um: f64 = sample_value(
                buffers,
                square.ndimen,
                id + (nuroo - iu + 1) * square.ndimen,
            );
            let f: isize = layout.f(iu, 0, id);
            let value = t.table_at(soso, f, buffers)[0] + up + um;
            t.table_at_mut(soso, f, buffers)[0] = value;
            let value = t.table_at(didi, f, buffers)[0] + up - um;
            t.table_at_mut(didi, f, buffers)[0] = value;
        }
        if state.parameter_count % 2 != 0 {
            let up: f64 = sample_value(buffers, square.ndimen, id + jdec * square.ndimen);
            let f: isize = layout.f(0, 0, id);
            let value = t.table_at(soso, f, buffers)[0] + up;
            t.table_at_mut(soso, f, buffers)[0] = value;
        }
    }
}

fn sample_value(
    buffers: &AdvApp2VarEvaluationBuffers<'_>,
    entry_dimension: i32,
    index: i32,
) -> f64 {
    buffers
        .result_at_signed_offset(sample_offset(index, entry_dimension))
        .expect("MMA2ds1: result outside initialized owner; expected a valid signed sample origin")
}

fn sample_offset(index: i32, entry_dimension: i32) -> isize {
    // retained  subtracts a pointer shift, not two i32 array subscripts.
    let offset = i64::from(index) - i64::from(entry_dimension) - 1;
    isize::try_from(offset).unwrap_or_else(|_| {
        panic!(
            "MMA2ds1: sample offset={offset}; expected signed address in {}..={}",
            isize::MIN,
            isize::MAX
        )
    })
}

fn sampling_roles(transposed: bool) -> [Role; 4] {
    if transposed {
        return [Role::Soso, Role::Sodi, Role::Diso, Role::Didi];
    }
    [Role::Soso, Role::Diso, Role::Sodi, Role::Didi]
}

#[cfg(test)]
mod address_tests {
    use super::*;
    #[test]
    fn sample_address_delta_preserves_retained_pointer_adjustment_range() {
        assert_eq!(sample_offset(1, 1), -1);
        assert_eq!(sample_offset(-1, 1), -3);
        #[cfg(target_pointer_width = "64")]
        assert_eq!(sample_offset(1, i32::MIN), 2_147_483_648);
    }
    #[cfg(target_pointer_width = "64")]
    #[test]
    #[should_panic(expected = "result offset=2147483648")]
    fn oversized_sample_delta_reaches_physical_validation_without_i32_overflow() {
        let mut values = [0.; 4];
        let buffers = AdvApp2VarEvaluationBuffers::shared(&mut values, 0, 0, 0, 4);
        sample_value(&buffers, i32::MIN, 1);
    }
}
