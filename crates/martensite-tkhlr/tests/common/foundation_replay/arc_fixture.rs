//! One lazy bounded-arc constructor shared by independent Reader contexts.
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::{geom2d::Geom2dCurve, geom2d_adaptor::Geom2dAdaptorCurve};
use martensite_tkgeomalgo::int_start::TheArc;

/// Only consume range fields for bounded arcs; `from_fields(&curve, false, || unreachable!())`.
pub fn from_fields(
    curve: &Handle<dyn Geom2dCurve>,
    bounded: bool,
    range: impl FnOnce() -> (f64, f64),
) -> Result<TheArc, Failure> {
    if !bounded {
        return Ok(Handle::new(Geom2dAdaptorCurve::with_curve(curve)?).cast());
    }
    let (first, last) = range();
    Ok(Handle::new(Geom2dAdaptorCurve::with_range(curve, first, last)?).cast())
}

#[cfg(test)]
mod tests {
    use super::from_fields;
    use martensite_tkernel::standard::Handle;
    use martensite_tkg2d::geom2d::Geom2dLine;
    use martensite_tkhlr::contap::ContapHCurve2dTool;
    use martensite_tkmath::gp::{Direction2, Point2};

    #[test]
    fn natural_arc_skips_bounds_and_bounded_arc_reads_them_once() {
        let curve = Handle::new(Geom2dLine::from_point(Point2::new(1., 2.), Direction2::X)).cast();
        let natural =
            from_fields(&curve, false, || panic!("natural arc has no range fields")).unwrap();
        assert_eq!(
            ContapHCurve2dTool::value(&natural, 3.).unwrap(),
            Point2::new(4., 2.)
        );
        let mut reads = 0;
        let bounded = from_fields(&curve, true, || {
            reads += 1;
            (2., 4.)
        })
        .unwrap();
        assert_eq!(reads, 1);
        assert_eq!(ContapHCurve2dTool::first_parameter(&bounded).unwrap(), 2.);
        assert_eq!(ContapHCurve2dTool::last_parameter(&bounded).unwrap(), 4.);
    }
}
