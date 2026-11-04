//! Explicit pole traversal, periodic closure and callback failure order.
use super::style_api::{KnotsStyle, draw_pole_net};
use martensite_raster_core::{PaletteColor, PaletteKind, RasterDisplay};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::Point3;
use std::cell::RefCell;
struct FakePoleGrid {
    calls: RefCell<Vec<(i32, i32)>>,
    fail_at: Option<usize>,
}
impl FakePoleGrid {
    fn new(fail_at: Option<usize>) -> Self {
        Self {
            calls: RefCell::new(Vec::new()),
            fail_at,
        }
    }
    fn pole(&self, u: i32, v: i32) -> Result<Point3, Failure> {
        self.calls.borrow_mut().push((u, v));
        if self.fail_at == Some(self.calls.borrow().len()) {
            return Err(Failure::with_message(
                "PoleUnavailable",
                &format!("grid=({u},{v}): expected available pole"),
            ));
        }
        Ok(Point3::new(f64::from(u), f64::from(v), 0.))
    }
}
fn draw(grid: &FakePoleGrid, periodic: bool) -> Result<(), Failure> {
    draw_pole_net(
        &mut RasterDisplay::batch(),
        PaletteColor::new(PaletteKind::Rouge),
        (2, 2),
        (periodic, periodic),
        |u, v| grid.pole(u, v),
    )
}
#[test]
fn open_net_visits_columns_before_rows() {
    let grid = FakePoleGrid::new(None);
    draw(&grid, false).unwrap();
    assert_eq!(
        *grid.calls.borrow(),
        [
            (1, 1),
            (2, 1),
            (1, 2),
            (2, 2),
            (1, 1),
            (1, 2),
            (2, 1),
            (2, 2)
        ]
    );
}
#[test]
fn periodic_net_revisits_the_first_pole_at_each_closure() {
    let grid = FakePoleGrid::new(None);
    draw(&grid, true).unwrap();
    assert_eq!(*grid.calls.borrow(), PERIODIC_CALLS);
}
#[test]
fn a_failed_pole_stops_traversal_with_the_case_error() {
    let grid = FakePoleGrid::new(Some(2));
    let error = draw(&grid, true).unwrap_err();
    assert_eq!(*grid.calls.borrow(), [(1, 1), (2, 1)]);
    assert_eq!(error.exception_type(), "PoleUnavailable");
    assert_eq!(error.message(), "grid=(2,1): expected available pole");
}
#[test]
fn marker_point_failure_is_returned_without_reading_further_points() {
    let failure = Failure::with_message("KnotUnavailable", "knot=2: expected available point");
    let points = [Ok(Point3::new(1., 2., 3.)), Err(failure.clone())];
    let error = KnotsStyle::default_shown()
        .mark(&mut RasterDisplay::batch(), points)
        .unwrap_err();
    assert_eq!(error, failure);
}

const PERIODIC_CALLS: [(i32, i32); 12] = [
    (1, 1),
    (2, 1),
    (1, 1),
    (1, 2),
    (2, 2),
    (1, 2),
    (1, 1),
    (1, 2),
    (1, 1),
    (2, 1),
    (2, 2),
    (2, 1),
];
