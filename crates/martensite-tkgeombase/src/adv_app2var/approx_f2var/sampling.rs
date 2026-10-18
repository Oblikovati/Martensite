//! Sampling of the function on the square: `mma2ds1_` and the file-static
//! `mma2ds2_`.
#[path = "sampling_rows.rs"]
mod rows;
use super::discretise::SquareTables;
use super::sampling_integers::{BorrowedSquareIntegers, SquareInteger, SquareIntegerStorage};
use crate::adv_app2var::AdvApp2VarEvaluationBuffers;
use crate::adv_app2var::evaluator::AdvApp2VarEvaluatorFunc2Var;
use crate::adv_app2var::math_base::transpose_workspace::{
    HeapTransposeWorkspace, TransposeShape, TransposeWorkspace, transpose_in_place,
};
use crate::adv_app2var::square_intervals::SquareInputBindings;
use crate::adv_app2var::square_roots::SquareRootBindings;
use rows::SquareRows;
#[path = "sampling_tables.rs"]
pub(crate) mod table_storage;
use table_storage::{BorrowedSquareTables, SquareTableRole, SquareTableStorage};

/// The square of `mma2ds1_`: the parameter intervals, the Legendre roots on
/// [-1,1] and the favourite iso direction.
#[derive(Clone, Copy)]
pub struct SampledSquare<'a> {
    /// `ndimen`: dimension of the function.
    pub ndimen: i32,
    /// `uintfn`: the U interval.
    pub uintfn: [f64; 2],
    /// `vintfn`: the V interval.
    pub vintfn: [f64; 2],
    /// `nbpntu`: number of U roots.
    pub nbpntu: i32,
    /// `nbpntv`: number of V roots.
    pub nbpntv: i32,
    /// `urootb`: the U roots.
    pub urootb: &'a [f64],
    /// `vrootb`: the V roots.
    pub vrootb: &'a [f64],
    /// `isofav`: 1 to evaluate U-isos, 2 (or anything else) for V-isos.
    pub isofav: i32,
}

/// Adds the samples of `foncnp` at the roots of the square to the four symmetric
/// tables (`mma2ds1_`), evaluating along the favourite iso direction; `fpntab`
/// (`ndimen * max(nbpntu, nbpntv)`) and `ttable` are scratch. `iercod` is the
/// evaluator code plus 100 when it stops the sampling; returns 0.
///
/// retained quirk: for `isofav == 1` the tables are transposed back with
/// `mmfmtb1_` after the sampling whatever its outcome, and that call clears
/// `iercod`, so an evaluator failure is lost; this implementation keeps this.
/// `mma2ds1_(&square, &f, &mut tables, &mut fpntab, &mut ttable, &mut ier)`.
pub fn mma2ds1_(
    square: &SampledSquare<'_>,
    foncnp: &dyn AdvApp2VarEvaluatorFunc2Var,
    tables: &mut SquareTables<'_>,
    fpntab: &mut [f64],
    ttable: &mut [f64],
    iercod: &mut i32,
) -> i32 {
    let mut request = *square;
    mma2ds1_mut_(&mut request, foncnp, tables, fpntab, ttable, iercod)
}

/// Samples a square while writing callback mutations to the caller's dimension
/// and varying-axis root count, including on error. Interval copies and the
/// preferred-iso local remain private; `mma2ds1_mut_(&mut square, &f, &mut tables, &mut values, &mut parameters, &mut code)`.
pub fn mma2ds1_mut_(
    square: &mut SampledSquare<'_>,
    foncnp: &dyn AdvApp2VarEvaluatorFunc2Var,
    tables: &mut SquareTables<'_>,
    fpntab: &mut [f64],
    ttable: &mut [f64],
    iercod: &mut i32,
) -> i32 {
    let geometry = *square;
    let mut integers = BorrowedSquareIntegers {
        request: square,
        error: iercod,
        evaluator: foncnp,
    };
    mma2ds1_integer_store(geometry, &mut integers, foncnp, tables, fpntab, ttable)
}

pub(crate) fn mma2ds1_integer_store(
    geometry: SampledSquare<'_>,
    integers: &mut dyn SquareIntegerStorage,
    foncnp: &dyn AdvApp2VarEvaluatorFunc2Var,
    tables: &mut SquareTables<'_>,
    fpntab: &mut [f64],
    ttable: &mut [f64],
) -> i32 {
    let legacy_parameters = ttable.len();
    let mut buffers = AdvApp2VarEvaluationBuffers::separate(ttable, fpntab, legacy_parameters);
    mma2ds1_buffer_store(geometry, integers, foncnp, tables, &mut buffers)
}
pub(crate) fn mma2ds1_buffer_store(
    geometry: SampledSquare<'_>,
    integers: &mut dyn SquareIntegerStorage,
    foncnp: &dyn AdvApp2VarEvaluatorFunc2Var,
    tables: &mut SquareTables<'_>,
    buffers: &mut AdvApp2VarEvaluationBuffers<'_>,
) -> i32 {
    mma2ds1_root_buffer_store(
        geometry,
        integers,
        foncnp,
        tables,
        buffers,
        SquareRootBindings::default(),
    )
}
pub(crate) fn mma2ds1_root_buffer_store(
    geometry: SampledSquare<'_>,
    integers: &mut dyn SquareIntegerStorage,
    foncnp: &dyn AdvApp2VarEvaluatorFunc2Var,
    tables: &mut SquareTables<'_>,
    buffers: &mut AdvApp2VarEvaluationBuffers<'_>,
    roots: SquareRootBindings,
) -> i32 {
    let mut tables = BorrowedSquareTables(tables);
    mma2ds1_real_store(
        geometry,
        integers,
        foncnp,
        &mut tables,
        buffers,
        SquareInputBindings {
            roots,
            ..SquareInputBindings::default()
        },
    )
}
pub(crate) fn mma2ds1_real_store(
    geometry: SampledSquare<'_>,
    integers: &mut dyn SquareIntegerStorage,
    foncnp: &dyn AdvApp2VarEvaluatorFunc2Var,
    tables: &mut dyn SquareTableStorage,
    buffers: &mut AdvApp2VarEvaluationBuffers<'_>,
    inputs: SquareInputBindings,
) -> i32 {
    mma2ds1_real_store_with_workspace(
        geometry,
        integers,
        foncnp,
        tables,
        buffers,
        inputs,
        &mut HeapTransposeWorkspace,
    )
}
fn mma2ds1_real_store_with_workspace(
    geometry: SampledSquare<'_>,
    integers: &mut dyn SquareIntegerStorage,
    foncnp: &dyn AdvApp2VarEvaluatorFunc2Var,
    tables: &mut dyn SquareTableStorage,
    buffers: &mut AdvApp2VarEvaluationBuffers<'_>,
    inputs: SquareInputBindings,
    workspace: &mut dyn TransposeWorkspace,
) -> i32 {
    let storage_counts = integers.root_counts(false);
    integers.set_integer(SquareInteger::ErrorCode, 0);
    let preference = integers.integer(SquareInteger::FavoriteIso);
    let iuouv = if preference == 1 { 1 } else { 2 };
    if iuouv == 2 {
        mma2ds2_store(geometry, iuouv, integers, foncnp, tables, buffers, inputs);
    } else if transpose_tables(integers, false, storage_counts, tables, buffers, workspace) {
        sample_transposed(geometry, integers, foncnp, tables, buffers, inputs);
        transpose_tables(integers, true, storage_counts, tables, buffers, workspace);
    }
    let code = integers.integer(SquareInteger::ErrorCode);
    if code > 0 {
        integers.set_integer(SquareInteger::ErrorCode, code + 100);
    }
    0
}
fn sample_transposed(
    geometry: SampledSquare<'_>,
    integers: &mut dyn SquareIntegerStorage,
    evaluator: &dyn AdvApp2VarEvaluatorFunc2Var,
    tables: &mut dyn SquareTableStorage,
    buffers: &mut AdvApp2VarEvaluationBuffers<'_>,
    inputs: SquareInputBindings,
) {
    mma2ds2_store(geometry, 1, integers, evaluator, tables, buffers, inputs);
}

/// The `mmfmtb1_` transpositions of `mma2ds1_`, per dimension, of the four tables
/// sampled on `nbpnt = [first, second]` roots; false when one fails.
fn transpose_tables(
    integers: &mut dyn SquareIntegerStorage,
    transposed: bool,
    storage_counts: [i32; 2],
    tables: &mut dyn SquareTableStorage,
    buffers: &mut AdvApp2VarEvaluationBuffers<'_>,
    workspace: &mut dyn TransposeWorkspace,
) -> bool {
    let full_stride = i64::from(storage_counts[0] / 2 + 1) * i64::from(storage_counts[1] / 2 + 1);
    let half_stride = i64::from(storage_counts[0] / 2) * i64::from(storage_counts[1] / 2);
    let dimensions = integers.integer(SquareInteger::Dimension);
    for dimension in 1..=dimensions {
        let [first, second] = integers.root_counts(transposed);
        let full = [first / 2 + 1, second / 2 + 1];
        if !transpose_pair(
            integers,
            dimension,
            full,
            full_stride,
            [SquareTableRole::Soso, SquareTableRole::Didi],
            tables,
            buffers,
            workspace,
        ) {
            return false;
        }
        let [first, second] = integers.root_counts(transposed);
        let half = [first / 2, second / 2];
        if !transpose_pair(
            integers,
            dimension,
            half,
            half_stride,
            [SquareTableRole::Sodi, SquareTableRole::Diso],
            tables,
            buffers,
            workspace,
        ) {
            return false;
        }
    }
    true
}
fn transpose_pair(
    integers: &mut dyn SquareIntegerStorage,
    dimension: i32,
    shape: [i32; 2],
    stride: i64,
    roles: [SquareTableRole; 2],
    tables: &mut dyn SquareTableStorage,
    buffers: &mut AdvApp2VarEvaluationBuffers<'_>,
    workspace: &mut dyn TransposeWorkspace,
) -> bool {
    for role in roles {
        if !transpose_block(
            integers,
            tables.table_at_mut(
                role,
                ((i64::from(dimension) - 1) * stride) as isize,
                buffers,
            ),
            shape,
            workspace,
        ) {
            return false;
        }
    }
    true
}
fn transpose_block(
    integers: &mut dyn SquareIntegerStorage,
    table: &mut [f64],
    shape: [i32; 2],
    workspace: &mut dyn TransposeWorkspace,
) -> bool {
    let [rows, columns] = shape;
    let shape = TransposeShape {
        input_stride: rows,
        rows,
        columns,
        output_stride: columns,
    };
    let mut sizes = [0, 0];
    let mut code = integers.integer(SquareInteger::ErrorCode);
    transpose_in_place(shape, table, &mut sizes, &mut code, workspace);
    integers.set_integer(SquareInteger::ErrorCode, code);
    code <= 0
}

/// `mma2ds2_`: the sampling itself; like `mma2ds1_` it adds 100 to a positive
/// evaluator code, so a failure seen through `mma2ds1_` carries 200.
fn mma2ds2_store(
    geometry: SampledSquare<'_>,
    iso: i32,
    integers: &mut dyn SquareIntegerStorage,
    foncnp: &dyn AdvApp2VarEvaluatorFunc2Var,
    tables: &mut dyn SquareTableStorage,
    buffers: &mut AdvApp2VarEvaluationBuffers<'_>,
    inputs: SquareInputBindings,
) {
    integers.set_integer(SquareInteger::ErrorCode, 0);
    let mut geometry = inputs.intervals.capture(geometry, buffers);
    if iso == 1 {
        std::mem::swap(&mut geometry.uintfn, &mut geometry.vintfn);
        std::mem::swap(&mut geometry.urootb, &mut geometry.vrootb);
    }
    let square = integers.snapshot(geometry, iso == 1);
    let mut rows = SquareRows::new(
        &square,
        iso,
        integers,
        foncnp,
        tables,
        buffers,
        inputs.roots,
    );
    rows.run();
    let code = integers.integer(SquareInteger::ErrorCode);
    if code > 0 {
        integers.set_integer(SquareInteger::ErrorCode, code + 100);
    }
}

#[cfg(test)]
#[path = "sampling_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "sampling_workspace_tests.rs"]
mod workspace_tests;
