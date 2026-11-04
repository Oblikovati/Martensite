//! Curvature gating and failure order use named independent local properties.
use super::style_api::{CurvatureDisplay, CurvatureProps, CurveLook, CurveStyle};
use martensite_raster_core::{PaletteColor, PaletteKind, RasterDisplay};
use martensite_tkernel::{ncollection::Array1, standard::Failure};
use martensite_tkmath::gp::Point3;
use std::{cell::RefCell, rc::Rc};
struct FakeCurvatureProps {
    curvature: Option<f64>,
    calls: Rc<RefCell<Vec<&'static str>>>,
    fail_centre: bool,
}
impl CurvatureProps for FakeCurvatureProps {
    type Point = Point3;
    fn curvature_at(&mut self, _t: f64) -> Result<Option<f64>, Failure> {
        self.calls.borrow_mut().push("curvature");
        Ok(self.curvature)
    }
    fn centre(&mut self) -> Result<Point3, Failure> {
        self.calls.borrow_mut().push("centre");
        if self.fail_centre {
            return Err(Failure::with_message(
                "CentreUnavailable",
                "radius centre: expected available curvature centre",
            ));
        }
        Ok(Point3::new(0., 0., 0.))
    }
}
fn radii() -> CurvatureDisplay {
    let style = CurveStyle::new(CurveLook::new(PaletteColor::new(PaletteKind::Rouge)));
    CurvatureDisplay::of(4, &style)
}
fn bounds() -> Array1<'static, f64> {
    Array1::from_shared(&[0., 1.], 1)
}
#[test]
fn undefined_or_tiny_curvature_does_not_evaluate_points_or_centres() {
    for curvature in [None, Some(0.), Some(1e-9)] {
        let calls = Rc::new(RefCell::new(Vec::new()));
        let mut props = FakeCurvatureProps {
            curvature,
            calls: calls.clone(),
            fail_centre: false,
        };
        radii()
            .draw(&mut RasterDisplay::batch(), &bounds(), &mut props, |_| {
                panic!("undefined radius: point must not be read")
            })
            .unwrap();
        assert_eq!(*calls.borrow(), ["curvature"; 4]);
    }
}
#[test]
fn centre_failure_follows_point_evaluation_and_stops_the_interval() {
    let calls = Rc::new(RefCell::new(Vec::new()));
    let mut props = FakeCurvatureProps {
        curvature: Some(0.2),
        calls: calls.clone(),
        fail_centre: true,
    };
    let point = |_| {
        calls.borrow_mut().push("point");
        Ok(Point3::new(5., 0., 0.))
    };
    let error = radii()
        .draw(&mut RasterDisplay::batch(), &bounds(), &mut props, point)
        .unwrap_err();
    assert_centre_failure(&error, &calls.borrow());
}

fn assert_centre_failure(error: &Failure, calls: &[&str]) {
    assert_eq!(calls, ["curvature", "point", "centre"]);
    assert_eq!(error.exception_type(), "CentreUnavailable");
    assert_eq!(
        error.message(),
        "radius centre: expected available curvature centre"
    );
}
