//! Integer-cell coverage; inputs here are projected coordinates, not observed GPU outputs.
use crate::{RenderLineGrid, RenderLineOrigin};

pub(super) fn minor_cell(coordinate: f32, grid: RenderLineGrid) -> f32 {
    let scale = grid.scale();
    let fixed = ((f64::from(coordinate) - 0.5) * scale).round_ties_even();
    (fixed / scale - 0.5).ceil() as f32
}
pub(super) fn oriented(point: [f32; 2], height: u32, origin: RenderLineOrigin) -> [f32; 2] {
    match origin {
        RenderLineOrigin::BottomUp => point,
        RenderLineOrigin::TopDown => [point[0], height as f32 - point[1]],
    }
}
pub(super) fn window_bounds(
    points: [[f32; 2]; 2],
    extent: [u32; 2],
    grid: RenderLineGrid,
    origin: RenderLineOrigin,
    axis: usize,
) -> Option<[[f32; 2]; 2]> {
    let p = points.map(|point| oriented(point, extent[1], origin));
    let [start, end] = crate::wide_line_columns::covered_centres(p[0], p[1], axis)?;
    let minor = minor_cell(p[0][1 - axis], grid);
    let mut bounds = [[0.; 2]; 2];
    for (point, major) in bounds.iter_mut().zip(major_bounds(start, end)) {
        point[axis] = major;
    }
    bounds[0][1 - axis] = minor;
    bounds[1][1 - axis] = minor + 1.;
    Some(bounds.map(|point| oriented(point, extent[1], origin)))
}

fn major_bounds(start: f32, end: f32) -> [f32; 2] {
    if start <= end {
        [start - 0.5, end - 0.5]
    } else {
        [end + 0.5, start + 0.5]
    }
}
