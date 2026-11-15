//! Shared two-column construction for legacy filling and owned patch assembly (#464).
use super::errors;
use martensite_tkernel::ncollection::{Array1, Array2, array2_failure};
use martensite_tkernel::standard::Failure;
use martensite_tkg2d::geom2d::arrays::at;
use martensite_tkmath::gp::Point3;

/// Owned pole grid and optional weight grid; `let (poles, weights) = grid;`.
pub type RuledControlGrid = (Array2<'static, Point3>, Option<Array2<'static, f64>>);
/// Copies the first `count` coefficients into two columns without numerical admission.
/// `ruled_control_grid([&first, &last], None, 2)?`.
#[inline(always)]
pub fn ruled_control_grid(
    poles: [&Array1<'_, Point3>; 2],
    weights: Option<[&Array1<'_, f64>; 2]>,
    count: i32,
) -> Result<RuledControlGrid, Failure> {
    Ok((
        two_columns(poles, count)?,
        weights.map(|w| two_columns(w, count)).transpose()?,
    ))
}
fn two_columns<T: Copy + Default>(
    columns: [&Array1<'_, T>; 2],
    count: i32,
) -> Result<Array2<'static, T>, Failure> {
    let mut grid = Array2::with_bounds(1, count, 1, 2).map_err(errors::array_bounds)?;
    if copy_columns(columns, count, grid.as_mut_slice()) {
        return Ok(grid);
    }
    for index in 1..=count {
        grid.set_value(index, 1, at(columns[0], index)?)
            .map_err(array2_failure)?;
        grid.set_value(index, 2, at(columns[1], index)?)
            .map_err(array2_failure)?;
    }
    Ok(grid)
}

fn indexed_prefix<'a, T>(column: &'a Array1<'_, T>, count: i32) -> Option<&'a [T]> {
    let start = usize::try_from(1_i64 - i64::from(column.lower())).ok()?;
    let end = start.checked_add(usize::try_from(count).ok()?)?;
    column.as_slice().get(start..end)
}
fn copy_columns<T: Copy>(columns: [&Array1<'_, T>; 2], count: i32, target: &mut [T]) -> bool {
    let (Some(first), Some(last)) = (
        indexed_prefix(columns[0], count),
        indexed_prefix(columns[1], count),
    ) else {
        return false;
    };
    for (pair, (&a, &b)) in target
        .as_chunks_mut::<2>()
        .0
        .iter_mut()
        .zip(first.iter().zip(last))
    {
        pair.copy_from_slice(&[a, b]);
    }
    true
}
